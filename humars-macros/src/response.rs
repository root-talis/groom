use darling::FromMeta;
use syn::{Error, Item, Attribute};
use proc_macro2::TokenStream;
use syn::{ItemEnum, parse2};
use quote::{quote, ToTokens};

use crate::{attrs::{parse_attr, remove_attrs}, http::HTTPStatusCode, utils::get_description};

// region: ResponseArgs ------------------------------------------------------------
//

#[derive(FromMeta)]
pub(crate) struct ResponseArgs {
    #[darling(default)]
    pub(crate) code: HTTPStatusCode,
}

impl ResponseArgs {
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

pub(crate) fn generate(input: TokenStream) -> TokenStream {
    let generated_impl = generate_impl(input);

    quote! {
        #generated_impl
    }
}

fn generate_impl(input: TokenStream) -> TokenStream {
    match parse2::<Item>(input) {
        Err(error) => error.to_compile_error(),
        Ok(item) => match item {
            Item::Enum(item_enum) => generate_impl_enum(item_enum),
            _ => Error::new_spanned(item, "Response should be an enum.").to_compile_error(),
        }
    }
}

fn generate_impl_enum(enum_impl: ItemEnum) -> TokenStream {
    let ident = enum_impl.ident;
    let vis = enum_impl.vis;

    let mut variants: Vec<TokenStream> = Vec::new();
    let mut into_response_impls: Vec<TokenStream> = Vec::new();
    let mut openapi_response_impls: Vec<TokenStream> = Vec::new();

    let mut type_assertions: Vec<TokenStream> = Vec::new(); // compile-time checks of trait implementation (for better error messages)
    
    for mut variant in enum_impl.variants {
        let args = match ResponseArgs::parse_from_attrs(&variant.attrs) {
            Ok(Some(args)) => {
                ResponseArgs::remove_from_attrs(&mut variant.attrs);
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

        let code_u16 = args.code.0;

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

        let fields = match variant.fields.clone() {
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

        into_response_impls.push(match fields.clone() {
            None => quote! {
                Self::#name => (#code_ts).into_response(),
            },
            Some(_single_field) => quote!{
                Self::#name(body) => (#code_ts, body).into_response(),
            },
        });

        let description_tk = match get_description(&variant.attrs).unwrap_or_default() {
            Some(s) => quote! { #s },
            None => quote! { "" },
        };

        openapi_response_impls.push(match fields.clone() {
            None => quote! {
                let op = op.response(
                    #code_str,
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description(#description_tk)
                        .build()
                );
            },
            Some(single_field) => {
                let ty = single_field.ty;

                type_assertions.push(quote!{
                    assert_impl_any!(#ty: ::utoipa::PartialSchema, ::utoipa::ToSchema<'static>);
                });

                quote!{
                    let op = op.response(
                        #code_str,
                        ::utoipa::openapi::ResponseBuilder::new()
                            .description(#description_tk)
                            .build()
                    );
                }
            },
        });

        variants.push(quote! {
            #variant,
        });
    }
    
    quote! {
        #vis enum #ident {
            #(#variants)*
        }

        #(#type_assertions)*

        impl ::axum::response::IntoResponse for #ident {
            fn into_response(self) -> ::axum::response::Response {
                match self {
                    #(#into_response_impls)*
                }
            }
        }

        impl ::humars::response::Response for #ident {
            fn __openapi_modify_operation(op: ::utoipa::openapi::path::OperationBuilder) -> ::utoipa::openapi::path::OperationBuilder {
                #(#openapi_response_impls)*
                op
            }
        }
    }
}

//
// endregion: AST parsing and generation ---------------------------------------------------
