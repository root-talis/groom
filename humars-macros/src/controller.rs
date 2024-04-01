use indexmap::IndexMap;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use syn::{parse2, Error, Item, ItemMod};
use quote::{format_ident, quote, ToTokens};
use darling::FromMeta;
use syn::Attribute;

use crate::{http::HTTPMethod, attrs::{parse_attr, remove_attrs}};

// region: RouteArgs ---------------------------------------------------------------
//

#[derive(FromMeta)]
pub(crate) struct RouteArgs {
    pub(crate) method: HTTPMethod,
    pub(crate) path: String,
}

impl RouteArgs {
    pub(crate) fn parse_from_attrs(attrs: &[Attribute]) -> Result<Option<Self>, darling::Error> {
        parse_attr("Route", attrs)  
    }

    pub(crate) fn remove_from_attrs(attrs: &mut Vec<Attribute>) {
        remove_attrs("Route", attrs)
    }
}

//
// endregion: RouteArgs ------------------------------------------------------------

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
    let item_mod = match parse2::<ItemMod>(input) {
        Ok(syntax_tree) => syntax_tree,
        Err(error) => return error.to_compile_error(),
    };

    let ident = item_mod.ident.clone();
    let vis = item_mod.vis;

    if item_mod.content.is_none() {
        return Error::new_spanned(&item_mod.ident, "module should have content").to_compile_error();
    }

    let items = item_mod.content.unwrap().1;

    //
    // Walk through all handlers and parse them
    //

    let mut seen_handlers: IndexMap<String, IndexMap<HTTPMethod, String>> = IndexMap::new(); // for routes deduplication
    let mut module_items: Vec<TokenStream> = Vec::with_capacity(items.len());                // all module items in the original order
    let mut routes_setup: Vec<TokenStream> = Vec::new();                                     // code to setup routes in merge_into_router()
    let mut paths_setup: IndexMap<String, Vec<TokenStream>> = IndexMap::new();               // code to setup paths in merge_into_openapi_builder()

    let mut type_assertions: Vec<TokenStream> = Vec::new(); // compile-time checks of trait implementation (for better error messages)

    for item in items {
        if let Item::Fn(mut function) = item {
            let args = match RouteArgs::parse_from_attrs(&function.attrs) {
                Ok(args) => {
                    RouteArgs::remove_from_attrs(&mut function.attrs);
                    args
                },
                Err(error) => return error.write_errors(),
            };

            if let Some(route) = args {
                if function.sig.asyncness.is_none() {
                    return Error::new_spanned(&function.sig.fn_token, "handler should be async fn").to_compile_error();
                }

                let path = route.path;
                let method = route.method;

                let duplicate_handler = seen_handlers
                    .entry(path.clone())
                    .or_default()
                    .insert(method, function.sig.ident.to_string())
                ;

                if duplicate_handler.is_some() {
                    return Error::new_spanned(
                        &function.sig,
                        format!(
                            "duplicate handler: function named `{}` is already assigned to route `{} {}`",
                            duplicate_handler.unwrap(),
                            method,
                            path,
                        )
                    ).to_compile_error();
                }

                let mut extractors: Vec<TokenStream> = Vec::new(); // mounting extractors to modify openapi operations
                for item in function.sig.inputs.clone() {
                    match item {
                        syn::FnArg::Receiver(receiver) => {
                            return Error::new_spanned(
                                &receiver,
                                "handlers with receiver are not supported, remove `self` and use State instead: https://docs.rs/axum/latest/axum/extract/struct.State.html"
                            ).to_compile_error();
                        },
                        syn::FnArg::Typed(arg) => {
                            let ty = arg.ty.as_ref();
                            type_assertions.push(quote!{
                                assert_impl_all!(#ty: ::humars::extract::HumarsExtractor);
                            });
                            extractors.push(quote! {
                                op_builder = <#ty>::__openapi_modify_operation(op_builder);
                            });
                        },
                    }
                }

                //
                // new module item instead of current one:
                //

                // make new module item:
                let method_str = method.to_string().to_ascii_uppercase();
                let handler_comment = format!(" HTTP handler: {method_str} {path}");

                // change comment:
                let (summary, description) = crate::utils::get_summary_and_description(&function.attrs).unwrap_or_default();

                let mut new_comment: Vec<TokenStream> = Vec::new();
                if let Some(s) = summary.clone() {
                    let s = format!(" {s}");
                    new_comment.push(quote!{#[doc = #s]});
                    new_comment.push(quote!{#[doc = ""]});
                }
                new_comment.push(quote!{#[doc = #handler_comment]});
                if let Some(s) = description.clone() {
                    let s = format!(" {s}");
                    new_comment.push(quote!{#[doc = ""]});
                    new_comment.push(quote!{#[doc = #s]});
                }

                crate::utils::remove_description(&mut function.attrs);

                // generate module item:
                module_items.push(quote! {
                    #(#new_comment)*
                    #function
                });

                //
                // web route setup:
                //

                let routing_method = format_ident!("{}", method.to_string());
                let fn_name = function.sig.ident.clone();

                routes_setup.push(quote! {
                    .route(#path, ::axum::routing::#routing_method(#fn_name))
                });

                //
                // openapi path setup:
                //

                let operation = match method {
                    HTTPMethod::Connect => quote! {::utoipa::openapi::PathItemType::Connect},
                    HTTPMethod::Delete  => quote! {::utoipa::openapi::PathItemType::Delete },
                    HTTPMethod::Get     => quote! {::utoipa::openapi::PathItemType::Get    },
                    HTTPMethod::Head    => quote! {::utoipa::openapi::PathItemType::Head   },
                    HTTPMethod::Options => quote! {::utoipa::openapi::PathItemType::Option },
                    HTTPMethod::Patch   => quote! {::utoipa::openapi::PathItemType::Patch  },
                    HTTPMethod::Post    => quote! {::utoipa::openapi::PathItemType::Post   },
                    HTTPMethod::Put     => quote! {::utoipa::openapi::PathItemType::Put    },
                    HTTPMethod::Trace   => quote! {::utoipa::openapi::PathItemType::Trace  },
                };

                let summary_tk = match summary {
                    Some(s) => quote! { Some(#s) },
                    None => quote! { None as Option<String> },
                };

                let description_tk = match description {
                    Some(s) => quote! { Some(#s) },
                    None => quote! { None as Option<String> },
                };
                paths_setup.entry(path.clone()).or_default().push(quote! {
                    {
                        let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                                .summary(#summary_tk)
                                .description(#description_tk);
                        
                        #(#extractors)*

                        ::utoipa::openapi::path::PathItemBuilder::new()
                            .operation(#operation, op_builder.build())
                            .build()
                    }
                });
            } else {
                module_items.push(function.into_token_stream());
            }
        } else {
            module_items.push(item.into_token_stream());
        }
    }

    //
    // Regenerate the entire module
    //

    let mut paths: Vec<TokenStream> = Vec::new();
    for p in paths_setup {
        let url = p.0;

        for m in p.1 {
            paths.push(quote! {
                paths = paths.path(#url, #m);
            });
        }
    }

    quote! {
        #vis mod #ident {
            use ::static_assertions::assert_impl_all;
            
            #(#module_items)*

            pub fn merge_into_router(other: ::axum::Router) -> ::axum::Router {
                let this_router = ::axum::Router::new()
                    #(#routes_setup)*
                ;

                other.merge(this_router)
            }

            pub fn merge_into_openapi_builder(other: ::utoipa::openapi::OpenApiBuilder) -> ::utoipa::openapi::OpenApiBuilder {
                let mut paths = ::utoipa::openapi::path::PathsBuilder::new();

                #(#paths)*

                other.paths(paths)
            }

            #(#type_assertions)*
        }
    }
}

//
// endregion: AST parsing and generation ---------------------------------------------------
