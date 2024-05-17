use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use darling::FromMeta;
use mime::Mime;
use syn::{Attribute, Error, Item};
use proc_macro2::TokenStream;
use syn::{ItemEnum, parse2};
use quote::{format_ident, quote, ToTokens};
use strum_macros::Display;

use crate::{attrs::{parse_attr, remove_attrs}, http::HTTPStatusCode, utils::get_description};

// region: ResponseArgs ----------------------------------------------------------------------------
//

#[derive(FromMeta)]
pub(crate) struct ResponseArgs {
    #[darling(default)]
    pub(crate) format: ResponseContentTypesList,

    #[darling(default)]
    pub(crate) default_format: Option<ResponseContentType>,
}

#[derive(FromMeta)]
pub(crate) struct ResponseVariantAnnotation {
    #[darling(default)]
    pub(crate) code: HTTPStatusCode,
}


impl ResponseVariantAnnotation {
    pub(crate) fn parse_from_attrs(attrs: &[Attribute]) -> Result<Option<Self>, darling::Error> {
        parse_attr("Response", attrs)
    }

    pub(crate) fn remove_from_attrs(attrs: &mut Vec<Attribute>) {
        remove_attrs("Response", attrs)
    }
}

//
// endregion: ResponseArgs -------------------------------------------------------------------------

// region: AST parsing and generation --------------------------------------------------------------
//

pub(crate) fn generate(args_t: TokenStream, args: ResponseArgs, input: TokenStream) -> TokenStream {
    match parse2::<Item>(input) {
        Err(error) =>
            error.to_compile_error(),

        Ok(item) => match item {
            Item::Enum(item_enum) =>
                generate_impl_for_enum(args_t, args, item_enum),

            _ =>
                Error::new_spanned(item, "Response should be an enum.").to_compile_error(),
        }
    }
}

fn generate_impl_for_enum(resp_args_t: TokenStream, resp_args: ResponseArgs, enum_impl: ItemEnum) -> TokenStream {
    let ident = &enum_impl.ident;
    let vis = &enum_impl.vis;

    let mut variants: Vec<TokenStream> = Vec::new();      // variants of output enum
    let mut openapi_impls: Vec<TokenStream> = Vec::new(); // openapi docs for each enum variant

    // todo: make this more dynamic and extensible somehow...
    let mut match_enum_when_no_accept_header_found: Vec<TokenStream> = Vec::new();
    let mut match_enum_for_text_plain: Vec<TokenStream> = Vec::new();
    let mut match_enum_for_text_html: Vec<TokenStream> = Vec::new();
    let mut match_enum_for_application_json: Vec<TokenStream> = Vec::new();

    // List of available content-types that can be produced from this Response (generated as a constant array):
    //
    // `supported_mimes` is a BTreeMap for deduplication of entries and to maintain a deterministic
    // order of elements so that macro expansion tests don't fail because of HashMap's shuffling.
    let supported_mimes_ident = format_ident!("__GROOM_RESPONSE_SUPPORTED_MIMES_{}", ident);
    let mut supported_mimes: BTreeMap<Mime, TokenStream> = BTreeMap::new();

    // compile-time checks of trait implementation (for better error messages):
    let mut type_assertions: Vec<TokenStream> = Vec::new();

    let default_format =
        match get_response_default_format(&enum_impl, &resp_args, &resp_args_t) {
            Ok(value) => value,
            Err(value) => return value,
        };

    for mut variant in enum_impl.variants {
        let variant_annotation = match ResponseVariantAnnotation::parse_from_attrs(&variant.attrs) {
            Ok(Some(annotation)) => {
                ResponseVariantAnnotation::remove_from_attrs(&mut variant.attrs);
                annotation
            }
            Ok(None) => {
                return syn::Error::new_spanned(
                    variant.into_token_stream(),
                    "response variant should be annotated with #[Response()]"
                ).to_compile_error()
            },
            Err(error) => return error.write_errors(),
        };

        let response_code_u16 = variant_annotation.code.0;
        let response_code_str_for_openapi = format!("{response_code_u16}");
        let response_code_ts = match ::axum::http::StatusCode::from_u16(response_code_u16) {
            Ok(code) => {
                let code = code.as_u16();
                quote! {
                    ::axum::http::StatusCode::from_u16(#code).unwrap()
                }
            },
            Err(e) => {
                return syn::Error::new_spanned(
                    &variant,
                    format!("{e}: response code `{}`", response_code_u16)
                ).to_compile_error()
            },
        };

        let variant_ident = &variant.ident;

        let response_body_field = match &variant.fields {
            syn::Fields::Named(fields) => {
                return syn::Error::new_spanned(fields, "named fields are not supported").into_compile_error();
                // todo: support something like http::response::Parts
            },

            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    return syn::Error::new_spanned(fields, "only exactly one unnamed field is supported").into_compile_error();
                }

                Some(fields.unnamed.first().expect("length is checked right above"))
            },

            syn::Fields::Unit => None,
        };

        // Let's make enum variants conversion for when no Accept header was supplied by the client.
        if !resp_args.format.is_any() {
            // If no formats were specified for this Response, we need to ensure that we:
            //  - either have only variants without fields in this enum and respond only with HTTP codes,
            //  - or raise a compile error telling the dev that they need to specify some response format.
            match_enum_when_no_accept_header_found.push(match &response_body_field {
                None =>
                    quote! {
                        Self::#variant_ident => (#response_code_ts).into_response(),
                    },

                Some(single_field) => {
                    return syn::Error::new_spanned(
                        single_field,
                        format!("specify at least one response format for enum `{ident}` to be able to return data through it's variant `{variant_ident}` (e.g. #[Response(format(json))])")
                    ).into_compile_error();
                }
            });
        }

        if resp_args.format.plain_text {
            match_enum_for_text_plain.push(match &response_body_field {
                None =>
                    quote! {
                        Self::#variant_ident => (#response_code_ts).into_response(),
                    },

                Some(_single_field) =>
                    quote!{
                        Self::#variant_ident(body) => (
                            #response_code_ts,
                            Into::<String>::into(body)
                        ).into_response(),
                    },
            });

            if let Entry::Vacant(e) = supported_mimes.entry(::mime::TEXT_PLAIN) {
                e.insert(quote! {
                    ::mime::TEXT_PLAIN,
                });
            }
        }

        if resp_args.format.html {
            match_enum_for_text_html.push(match &response_body_field {
                None =>
                    quote! {
                        Self::#variant_ident => (#response_code_ts).into_response(),
                    },

                Some(single_field) => {
                    let ty = &single_field.ty;

                    quote!{
                        Self::#variant_ident(body) => (
                            #response_code_ts,
                            <#ty as ::groom::response::HtmlFormat>::render(body)
                        ).into_response(),
                    }
                },
            });

            if let Entry::Vacant(e) = supported_mimes.entry(::mime::TEXT_HTML) {
                e.insert(quote! {
                    ::mime::TEXT_HTML,
                });
            }
        }

        if resp_args.format.json {
            match_enum_for_application_json.push(match &response_body_field {
                None =>
                    quote! {
                        Self::#variant_ident => (#response_code_ts).into_response(),
                    },

                Some(_single_field) =>
                    quote!{
                        Self::#variant_ident(body) => (
                            #response_code_ts,
                            ::axum::Json(body)
                        ).into_response(),
                    },
            });

            if let Entry::Vacant(e) = supported_mimes.entry(::mime::APPLICATION_JSON) {
                e.insert(quote! {
                    ::mime::APPLICATION_JSON,
                });
            }
        }

        let description_tk = match get_description(&variant.attrs).unwrap_or_default() {
            Some(s) => quote! { #s },
            None           => quote! { "" },
        };

        match &response_body_field {
            None => {
                openapi_impls.push(quote! {
                    let op = op.response(
                        #response_code_str_for_openapi,
                        ::utoipa::openapi::ResponseBuilder::new()
                            .description(#description_tk)
                            .build()
                    );
                });
            },

            Some(single_field) => {
                let ty = &single_field.ty;

                type_assertions.push(quote!{
                    assert_impl_any!(#ty: ::utoipa::PartialSchema, ::groom::DTO_Response);
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
                        #response_code_str_for_openapi,
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

    let mut mime_type_matches: Vec<TokenStream> = Vec::new();

    let response_default_ident = format_ident!("into_response_any_content_type");
    let response_default_fn = if !resp_args.format.is_any() {
        quote! {
            fn #response_default_ident(self) -> ::axum::response::Response {
                match self {
                    #(#match_enum_when_no_accept_header_found)*
                }
            }
        }
    } else {
        Default::default()
    };

    let response_text_plain_ident = format_ident!("into_response_text_plain");
    let response_text_plain_fn = if resp_args.format.plain_text {
        mime_type_matches.push(quote! {
            (::mime::TEXT, mime::PLAIN) => self.#response_text_plain_ident(),
        });

        quote! {
            fn #response_text_plain_ident(self) -> ::axum::response::Response {
                match self {
                    #(#match_enum_for_text_plain)*
                }
            }
        }
    } else {
        Default::default()
    };

    let response_text_html_ident = format_ident!("into_response_text_html");
    let response_text_html_fn = if resp_args.format.html {
        mime_type_matches.push(quote! {
            (::mime::TEXT, mime::HTML) => self.#response_text_html_ident(),
        });

        quote! {
            fn #response_text_html_ident(self) -> ::axum::response::Response {
                match self {
                    #(#match_enum_for_text_html)*
                }
            }
        }
    } else {
        Default::default()
    };

    let response_application_json_ident = format_ident!("into_response_application_json");
    let response_application_json_fn = if resp_args.format.json {
        mime_type_matches.push(quote! {
            (::mime::APPLICATION, mime::JSON) => self.#response_application_json_ident(),
        });

        quote! {
            fn #response_application_json_ident(self) -> ::axum::response::Response {
                match self {
                    #(#match_enum_for_application_json)*
                }
            }
        }
    } else {
        Default::default()
    };

    let content_type_negotiation = if !resp_args.format.is_any() {
        quote! {
            self.#response_default_ident()
        }
    } else {
        let default_content_response = if let Some(default_format) = default_format {
            match default_format {
                ResponseContentType::PlainText => quote!{ self.#response_text_plain_ident() },
                ResponseContentType::Html      => quote!{ self.#response_text_html_ident() },
                ResponseContentType::Json      => quote!{ self.#response_application_json_ident() },
                // todo: option to force BadRequest response if client hasn't specified any format in Accept header
            }
        } else {
            return syn::Error::new_spanned(
                resp_args_t,
                format!("cannot infer default_format for enum `{ident}` - this is a bug in groom, please report it; thank you~")
            ).into_compile_error();
        };

        quote! {
            match accept {
                None => {
                    // no Accept header found
                    #default_content_response
                },
                Some(accept) => {
                    // some Accept header found
                    // #available_mimes_ident is a const array with a list of supported mime types for this enum
                    match accept.negotiate(&#supported_mimes_ident) {
                        Err(_) => {
                            // todo: is this response body good enough?
                            (::axum::http::StatusCode::BAD_REQUEST, "Requested Content-Type is not supported.").into_response()
                        },

                        Ok(negotiated) => {
                            // todo: this match's arms should not include formats that aren't valid for this enum
                            match (negotiated.type_(), negotiated.subtype()) {
                                #(#mime_type_matches)*

                                _ => {
                                    // todo: somehow log this error?
                                    (::axum::http::StatusCode::BAD_REQUEST, "Content-Type negotiation produced an unexpected type/subtype pair.")
                                        .into_response()
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    let supported_mimes = supported_mimes.values();

    quote! {
        #vis enum #ident {
            #(#variants)*
        }

        #[allow(non_upper_case_globals)]
        const #supported_mimes_ident: &[::mime::Mime] = &[
            #(#supported_mimes)*
        ];

        impl #ident {
            #response_default_fn
            #response_text_plain_fn
            #response_text_html_fn
            #response_application_json_fn
        }

        impl ::groom::response::Response for #ident {
            fn __groom_into_response(self, accept: Option<::accept_header::Accept>) -> ::axum::response::Response {
                #content_type_negotiation
            }

            fn __openapi_modify_operation(op: ::utoipa::openapi::path::OperationBuilder) -> ::utoipa::openapi::path::OperationBuilder {
                #(#openapi_impls)*
                op
            }

            // todo: __groom_content_type_supported
        }

        #(#type_assertions)*
    }
}

fn get_response_default_format(
    enum_impl: &ItemEnum,
    resp_args: &ResponseArgs,
    resp_args_span: &TokenStream
) -> Result<Option<ResponseContentType>, TokenStream> {
    let ident = &enum_impl.ident;

    Ok(
        if !resp_args.format.is_any() {
            None
        } else {
            let default_format = resp_args.default_format.map_or_else(
                || resp_args.format.get_single_value(),
                |val| Some(val),
            );

            if default_format.is_none() {
                return Err(syn::Error::new_spanned(
                    resp_args_span,
                    format!("specify default_format for enum `{ident}` (e.g. #[Response(default_format=\"json\")])")
                ).into_compile_error());
            } else if !resp_args.format.has(default_format.unwrap()) {
                return Err(syn::Error::new_spanned(
                    resp_args_span,
                    format!("default_format `{}` of enum `{ident}` is not mentioned in it's formats list", default_format.unwrap())
                ).into_compile_error());
            }

            default_format
        }
    )
}

//
// endregion: AST parsing and generation -----------------------------------------------------------

// region: ResponseContentType ---------------------------------------------------------------------
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
// endregion: ResponseContentType ------------------------------------------------------------------
