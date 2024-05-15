use std::collections::HashSet;
use darling::FromMeta;
use mime::Mime;
use syn::{Attribute, Error, Item};
use proc_macro2::TokenStream;
use syn::{ItemEnum, parse2};
use quote::{format_ident, quote, ToTokens};
use strum_macros::Display;

use crate::{attrs::{parse_attr, remove_attrs}, http::HTTPStatusCode, utils::get_description};

// region: ResponseArgs ------------------------------------------------------------
//

#[derive(FromMeta)]
pub(crate) struct ResponseArgs {
    #[darling(default)]
    pub(crate) format: ResponseContentTypesList,

    #[darling(default)]
    pub(crate) default_format: Option<ResponseContentType>,
}

#[derive(FromMeta)]
pub(crate) struct ResponseVariantArgs {
    #[darling(default)]
    pub(crate) code: HTTPStatusCode,
}


impl ResponseVariantArgs {
    pub(crate) fn parse_from_attrs(attrs: &[Attribute]) -> Result<Option<Self>, darling::Error> {
        parse_attr("Response", attrs)  
    }

    pub(crate) fn remove_from_attrs(attrs: &mut Vec<Attribute>) {
        remove_attrs("Response", attrs)
    }
}

//
// endregion: ResponseArgs ---------------------------------------------------------

// region: AST parsing and generation ----------------------------------------------
//

pub(crate) fn generate(args_t: TokenStream, args: ResponseArgs, input: TokenStream) -> TokenStream {
    let generated_impl = generate_impl(args_t, args, input);

    quote! {
        #generated_impl
    }
}

fn generate_impl(args_t: TokenStream, args: ResponseArgs, input: TokenStream) -> TokenStream {
    match parse2::<Item>(input) {
        Err(error) => error.to_compile_error(),
        Ok(item) => match item {
            Item::Enum(item_enum) => generate_impl_enum(args_t, args, item_enum),
            _ => Error::new_spanned(item, "Response should be an enum.").to_compile_error(),
        }
    }
}

fn generate_impl_enum(resp_args_t: TokenStream, resp_args: ResponseArgs, enum_impl: ItemEnum) -> TokenStream {
    let ident = enum_impl.ident;
    let vis = enum_impl.vis;

    let mut variants: Vec<TokenStream> = Vec::new();
    let mut openapi_impls: Vec<TokenStream> = Vec::new();

    // todo: make this more dynamic and extensible somehow...
    let mut response_bodies_match_blank: Vec<TokenStream> = Vec::new();
    let mut response_bodies_match_text_plain: Vec<TokenStream> = Vec::new();
    let mut response_bodies_match_text_html: Vec<TokenStream> = Vec::new();
    let mut response_bodies_match_application_json: Vec<TokenStream> = Vec::new();


    let available_mimes_ident = format_ident!("__HUMARS_RESPONSE_AVAILABLE_MIMES_{}", ident);
    let mut available_mimes_list: Vec<TokenStream> = Vec::new(); // token streams (cannot be hashed, so we also have a set of unique mime types)
    let mut available_mimes_set: HashSet<Mime> = HashSet::new(); // for deduplication (because token streams cannot be hashed)

    let mut type_assertions: Vec<TokenStream> = Vec::new(); // compile-time checks of trait implementation (for better error messages)

    let default_format = if resp_args.format.is_any() {
        let default_format = resp_args.default_format.map_or_else(
            || resp_args.format.get_single_value(),
            |val| Some(val),
        );

        if default_format.is_none() {
            return syn::Error::new_spanned(
                resp_args_t,
                format!("specify default_format for enum `{ident}` (e.g. #[Response(default_format=\"json\")])")
            ).into_compile_error();
        } else if !resp_args.format.has(default_format.unwrap()) {
            return syn::Error::new_spanned(
                resp_args_t,
                format!("default_format `{}` of enum `{ident}` is not mentioned in it's formats list", default_format.unwrap())
            ).into_compile_error();
        }

        default_format
    } else {
        None
    };

    for mut variant in enum_impl.variants {
        let variant_args = match ResponseVariantArgs::parse_from_attrs(&variant.attrs) {
            Ok(Some(args)) => {
                ResponseVariantArgs::remove_from_attrs(&mut variant.attrs);
                args
            },
            Ok(None) => {
                return syn::Error::new_spanned(
                    variant.into_token_stream(),
                    "response variant should be annotated with #[Response()]"
                ).to_compile_error()
            },
            Err(error) => return error.write_errors(),
        };

        let code_u16 = variant_args.code.0;

        let code_ts = match ::axum::http::StatusCode::from_u16(code_u16) {
            Ok(code) => {
                let code = code.as_u16();
                quote! {
                    ::axum::http::StatusCode::from_u16(#code).unwrap()
                }
            },
            Err(e) => {
                return syn::Error::new_spanned(
                    &variant,
                    format!("{e}: \"{}\"", code_u16)
                ).to_compile_error()
            },
        };

        let code_str = format!("{code_u16}");

        let variant_name = variant.ident.clone();

        let variant_field = match variant.fields.clone() {
            syn::Fields::Named(fields) => {
                return syn::Error::new_spanned(fields, "named fields are not supported").into_compile_error();
                // todo: support something like http::response::Parts
            },

            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    return syn::Error::new_spanned(fields, "only exactly one unnamed field is supported").into_compile_error();
                }

                Some(fields.unnamed.first().expect("length is checked right above").clone())
            },

            syn::Fields::Unit => None,
        };
        
        // Let's make enum variants conversion for when no Accept header was supplied by the client.
        if !resp_args.format.is_any() {
            // If no formats were specified for this Response, we need to ensure that we:
            //  - either have only variants without fields in this enum and respond only with HTTP codes,
            //  - or raise a compile error telling the dev that they need to specify some response format.
            response_bodies_match_blank.push(match variant_field.clone() {
                None => quote! {
                    Self::#variant_name => (#code_ts).into_response(),
                },
                Some(single_field) => {
                    return syn::Error::new_spanned(
                        single_field,
                        format!("specify at least one response format for enum `{ident}` to be able to return data through it's variant `{variant_name}` (e.g. #[Response(format(json))])")
                    ).into_compile_error();
                }
            });
        }

        if resp_args.format.plain_text {
            response_bodies_match_text_plain.push(match variant_field.clone() {
                None => quote! {
                    Self::#variant_name => (#code_ts).into_response(),
                },
                Some(_single_field) => quote!{
                    Self::#variant_name(body) => (
                        #code_ts,
                        Into::<String>::into(body)
                    ).into_response(),
                },
            });

            if available_mimes_set.insert(::mime::TEXT_PLAIN) {
                available_mimes_list.push(quote! {
                    ::mime::TEXT_PLAIN,
                });
            }
        } else {
            // todo: one default match for all?
            response_bodies_match_text_plain.push(quote! {
                _ => (::axum::http::StatusCode::BAD_REQUEST).into_response(),
            });
        }

        if resp_args.format.html {
            response_bodies_match_text_html.push(match variant_field.clone() {
                None => quote! {
                    Self::#variant_name => (#code_ts).into_response(),
                },
                Some(single_field) => {
                    let ty = single_field.ty;

                    quote!{
                        Self::#variant_name(body) => (
                            #code_ts,
                            <#ty as ::humars::response::HtmlFormat>::render(body)
                        ).into_response(),
                    }
                },
            });

            if available_mimes_set.insert(::mime::TEXT_HTML) {
                available_mimes_list.push(quote! {
                    ::mime::TEXT_HTML,
                });
            }
        } else {
            // todo: one default match for all?
            response_bodies_match_text_html.push(quote! {
                _ => (::axum::http::StatusCode::BAD_REQUEST).into_response(),
            });
        }

        if resp_args.format.json {
            response_bodies_match_application_json.push(match variant_field.clone() {
                None => quote! {
                    Self::#variant_name => (#code_ts).into_response(),
                },
                Some(_single_field) => quote!{
                    Self::#variant_name(body) => (
                        #code_ts,
                        ::axum::Json(body)
                    ).into_response(),
                },
            });

            if available_mimes_set.insert(::mime::APPLICATION_JSON) {
                available_mimes_list.push(quote! {
                    ::mime::APPLICATION_JSON,
                });
            }
        } else {
            // todo: one default match for all?
            response_bodies_match_application_json.push(quote! {
                _ => (::axum::http::StatusCode::BAD_REQUEST).into_response(),
            });
        }

        let description_tk = match get_description(&variant.attrs).unwrap_or_default() {
            Some(s) => quote! { #s },
            None            => quote! { "" },
        };

        match variant_field.clone() {
            None => {
                openapi_impls.push(quote! {
                    let op = op.response(
                        #code_str,
                        ::utoipa::openapi::ResponseBuilder::new()
                            .description(#description_tk)
                            .build()
                    );
                });
            },
            
            Some(single_field) => {
                let ty = single_field.ty;

                type_assertions.push(quote!{
                    assert_impl_any!(#ty: ::utoipa::PartialSchema, ::humars::DTO_Response);
                });

                let mut response_impls: Vec<TokenStream> = Vec::new();

                if resp_args.format.plain_text {
                    response_impls.push(quote!{
                        .content(
                            ::mime::TEXT_PLAIN_UTF_8.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(<String as utoipa::PartialSchema>::schema())
                                //.example(Some("Hello, world!".into()))
                                .build()
                        )
                    });
                }
                if resp_args.format.html {
                    response_impls.push(quote!{
                        .content(
                            ::mime::TEXT_HTML_UTF_8.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(<String as utoipa::PartialSchema>::schema())
                                //.example(Some("<h1>Hello, world!</h1>".into()))
                                .build()
                        )
                    });
                }
                if resp_args.format.json {
                    response_impls.push(quote!{
                        .content(
                            ::mime::APPLICATION_JSON.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(#ty::schema().1)
                                .build()
                        )
                    });
                }

                openapi_impls.push(quote! {
                    let op = op.response(
                        #code_str, 
                        ::utoipa::openapi::ResponseBuilder::new()
                            .description(#description_tk)
                            #(#response_impls)*
                            .build()
                    );
                })
            },
        };

        variants.push(quote! {
            #variant,
        });
    }

    let content_type_negotiation = if !resp_args.format.is_any() {
        quote! {
            match self {
                #(#response_bodies_match_blank)*
            }
        }
    } else {
        // todo: these should be generated as methods on Response enum
        let response_text_plain = quote! {
            match self {
                #(#response_bodies_match_text_plain)*
            }
        };

        let response_text_html = quote! {
            match self {
                #(#response_bodies_match_text_html)*
            }
        };

        let response_application_json = quote! {
            match self {
                #(#response_bodies_match_application_json)*
            }
        };

        let default_content_response = if let Some(default_format) = default_format {
            match default_format {
                ResponseContentType::PlainText => quote!{ #response_text_plain },
                ResponseContentType::Html      => quote!{ #response_text_html },
                ResponseContentType::Json      => quote!{ #response_application_json },
                // todo: option to force BadRequest response if client hasn't specified any format in Accept header
            }
        } else {
            return syn::Error::new_spanned(
                resp_args_t,
                format!("cannot infer default_format for enum `{ident}` - this is a bug in humars, please report it; thank you~")
            ).into_compile_error();
        };

        quote! {
            match accept {
                Some(accept) => {
                    match accept.negotiate(&#available_mimes_ident) {
                        Ok(negotiated) => {
                            match (negotiated.type_(), negotiated.subtype()) {
                                (::mime::TEXT, mime::HTML) => {
                                    #response_text_html
                                },
                                (::mime::TEXT, mime::PLAIN) => {
                                    #response_text_plain
                                },
                                (::mime::APPLICATION, mime::JSON) => {
                                    #response_application_json
                                },
                                _ => {
                                    (::axum::http::StatusCode::BAD_REQUEST, "Negotiated some weird poo.").into_response()
                                }
                            }
                        },
                        Err(_) => {
                            (::axum::http::StatusCode::BAD_REQUEST, "Requested content-type not supported.").into_response()
                        }
                    }
                },
                None => {
                    #default_content_response
                }
            }
        }
    };

    quote! {
        #vis enum #ident {
            #(#variants)*
        }

        #(#type_assertions)*

        #[allow(non_upper_case_globals)]
        const #available_mimes_ident: &[::mime::Mime] = &[
            #(#available_mimes_list)*
        ];

        impl ::humars::response::Response for #ident {
            fn __openapi_modify_operation(op: ::utoipa::openapi::path::OperationBuilder) -> ::utoipa::openapi::path::OperationBuilder {
                #(#openapi_impls)*
                op
            }

            fn __humars_into_response(self, accept: Option<::accept_header::Accept>) -> ::axum::response::Response {
                #content_type_negotiation
            }
            
            // todo: __humars_content_type_supported
        }
    }
}

//
// endregion: AST parsing and generation ---------------------------------------------------

// region: ResponseContentType -------------------------------------------------------------
//

#[derive(Debug, Copy, Clone, FromMeta, Eq, PartialEq, Hash, Display)]
#[darling(rename_all = "snake_case")]
#[strum(serialize_all="snake_case")]
pub(crate) enum ResponseContentType {
    PlainText,
    Html,
    Json,
}

#[derive(FromMeta, Default)]
pub(crate) struct ResponseContentTypesList {
    #[darling(default)]
    pub(crate) plain_text: bool,

    #[darling(default)]
    pub(crate) html: bool,

    #[darling(default)]
    pub(crate) json: bool,
}


impl ResponseContentTypesList {
    pub(crate) fn is_any(&self) -> bool {
        return self.plain_text || self.html || self.json;
    }

    pub(crate) fn count(&self) -> usize {
        let mut result = 0;

        if self.plain_text {
            result += 1;
        }

        if self.html {
            result += 1;
        }

        if self.json {
            result += 1;
        }

        result
    }

    pub(crate) fn get_single_value(&self) -> Option<ResponseContentType> {
        if self.count() != 1 {
            None
        } else if self.plain_text {
            Some(ResponseContentType::PlainText)
        } else if self.html {
            Some(ResponseContentType::Html)
        } else if self.json {
            Some(ResponseContentType::Json)
        } else {
            panic!("bug in ResponseContentTypesList::count() or ResponseContentTypesList::get_the_only_value()")
        }
    }

    pub(crate) fn has(&self, t: ResponseContentType) -> bool {
        match t {
            ResponseContentType::PlainText => self.plain_text,
            ResponseContentType::Html => self.html,
            ResponseContentType::Json => self.json,
        }
    }
}

//
// endregion: ResponseContentType ------------------------------------------------------------
