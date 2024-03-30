use darling::FromMeta;
use syn::{Error, Item, Attribute};
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use syn::{ItemEnum, parse2};
use quote::{quote, ToTokens};

use crate::{attrs::{parse_attr, remove_attrs}, http::HTTPStatusCode};

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

pub(crate) fn generate(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        abort!(args, "no args yet")
    }

    let generated_impl = generate_impl(input.clone());

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
    let (variants, into_response_implementations) = {
        let mut variants: Vec<TokenStream> = Vec::new();
        let mut into_response_impls: Vec<TokenStream> = Vec::new();

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

            let code = match ::axum::http::StatusCode::from_u16(args.code.0) {
                Ok(code) => {
                    let code = code.as_u16();
                    quote! {
                        ::axum::http::StatusCode::from_u16(#code).unwrap()
                    }
                },
                Err(e) => {
                    return syn::Error::new_spanned(
                        &variant,
                        format!("{e}: \"{}\"", args.code.0)
                    ).to_compile_error()
                },
            };

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

                    Some(fields.unnamed.first().expect("length checked right above").clone())
                },

                syn::Fields::Unit => None,
            };

            into_response_impls.push(match fields {
                None => quote! {
                    Self::#name => (#code).into_response(),
                },
                Some(_single_field) => quote!{
                    Self::#name(body) => (#code, body).into_response(),
                },
            });

            variants.push(quote! {
                #variant,
            })
        }

        (variants, into_response_impls)
    };
    
    quote! {
        enum #ident {
            #(#variants)*
        }

        impl ::axum::response::IntoResponse for #ident {
            fn into_response(self) -> ::axum::response::Response {
                match self {
                    #(#into_response_implementations)*
                }
            }
        }
    }
}

//
// endregion: AST parsing and generation ---------------------------------------------------