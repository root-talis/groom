use darling::FromMeta;
use syn::{Error, Item, Attribute};
use proc_macro2::TokenStream;
use syn::{ItemEnum, parse2};
use quote::{quote, ToTokens};

use crate::{attrs::{parse_attr, remove_attrs}, http::{HTTPStatusCode, ResponseContentTypesList}, utils::get_description};

// region: ResponseArgs ------------------------------------------------------------
//

#[derive(FromMeta)]
pub(crate) struct ResponseArgs {
    #[darling(default)]
    pub(crate) format: ResponseContentTypesList,
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

fn generate_impl_enum(_resp_args_t: TokenStream, resp_args: ResponseArgs, enum_impl: ItemEnum) -> TokenStream {
    let ident = enum_impl.ident;
    let vis = enum_impl.vis;

    let mut variants: Vec<TokenStream> = Vec::new();
    let mut openapi_impls: Vec<TokenStream> = Vec::new();

    // todo: make this more dynamic and extensible somehow...
    let mut response_bodies_match_text_plain: Vec<TokenStream> = Vec::new();
    let mut response_bodies_match_text_html: Vec<TokenStream> = Vec::new();
    let mut response_bodies_match_application_json: Vec<TokenStream> = Vec::new();

    let mut type_assertions: Vec<TokenStream> = Vec::new(); // compile-time checks of trait implementation (for better error messages)
    
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

        let name = variant.ident.clone();

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
        
        if resp_args.format.plain_text {
            response_bodies_match_text_plain.push(match variant_field.clone() {
                None => quote! {
                    Self::#name => (#code_ts).into_response(),
                },
                Some(_single_field) => quote!{
                    Self::#name(body) => (
                        #code_ts,
                        Into::<String>::into(body)
                    ).into_response(),
                },
            });
        } else {
            // todo: one default match for all?
            response_bodies_match_text_plain.push(quote! {
                _ => (::axum::http::StatusCode::BAD_REQUEST).into_response(),
            });
        }

        if resp_args.format.html {
            response_bodies_match_text_html.push(match variant_field.clone() {
                None => quote! {
                    Self::#name => (#code_ts).into_response(),
                },
                Some(_single_field) => quote!{
                    Self::#name(body) => (
                        #code_ts,
                        ::axum::response::Html(body)
                    ).into_response(),
                },
            });
        } else {
            // todo: one default match for all?
            response_bodies_match_text_html.push(quote! {
                _ => (::axum::http::StatusCode::BAD_REQUEST).into_response(),
            });
        }

        if resp_args.format.json {
            response_bodies_match_application_json.push(match variant_field.clone() {
                None => quote! {
                    Self::#name => (#code_ts).into_response(),
                },
                Some(_single_field) => quote!{
                    Self::#name(body) => (
                        #code_ts,
                        ::axum::Json(body)
                    ).into_response(),
                },
            });
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
                            "text/plain",
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
                            "text/html",
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
                            "application/json",
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

    quote! {
        #vis enum #ident {
            #(#variants)*
        }

        #(#type_assertions)*

        impl ::humars::response::Response for #ident {
            fn __openapi_modify_operation(op: ::utoipa::openapi::path::OperationBuilder) -> ::utoipa::openapi::path::OperationBuilder {
                #(#openapi_impls)*
                op
            }

            fn __humars_into_response(self, accept: Option<&::axum::http::HeaderValue>) -> ::axum::response::Response {
                // response code that was annotated on current enum variant
                /*let code = match self {
                    #(#response_codes_match)*
                };*/

                match accept.map(|x| x.as_bytes()) {
                    Some(b"text/plain") => {
                        match self {
                            #(#response_bodies_match_text_plain)*
                        }
                    },
                    Some(b"text/html") => {
                        match self {
                            #(#response_bodies_match_text_html)*
                        }
                    },
                    Some(b"application/json") => {
                        match self {
                            #(#response_bodies_match_application_json)*
                        }
                    },
                    None => {
                        // todo: if there is a default content-type specified in #[Response()] args, use it;
                        //       else:
                        //           if there is no response body, return response code
                        //           else return BadRequest
                        (::axum::http::StatusCode::BAD_REQUEST).into_response()
                    },
                    _ => {
                        (::axum::http::StatusCode::BAD_REQUEST).into_response()
                    }
                }

                // Some(body) or None
                /*let body = match self {
                    #(#response_bodies_match)*
                };*/

                // serialize body into appropriate response type

                // if there is no body, just return into_response()
                //(code).into_response()
            }
        }
    }
}

//
// endregion: AST parsing and generation ---------------------------------------------------
