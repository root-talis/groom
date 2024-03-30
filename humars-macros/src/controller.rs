use indexmap::IndexMap;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use syn::{parse2, ItemImpl, ImplItem, Error, ImplItemFn};
use quote::{quote, format_ident};
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

struct ControllerParts {
    handlers: IndexMap<String, IndexMap<HTTPMethod, ImplItemFn>>,
    ignored_functions: Vec<ImplItemFn>,
}

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
    let mut item_impl = match parse2::<ItemImpl>(input) {
        Ok(syntax_tree) => syntax_tree,
        Err(error) => return error.to_compile_error(),
    };

    let ident = item_impl.self_ty.clone();
    /*let ident_name = match &*ident {
        syn::Type::Path(tp) => tp.path.segments.first().unwrap().ident.clone(),
        _ => panic!("failed to parse the ident path"),
    };*/

    //
    // Walk through all handlers and parse them
    //

    let mut controller_parts = ControllerParts {
        handlers: Default::default(),
        ignored_functions: Default::default(),
    };


    let mut type_assertions: Vec<TokenStream> = Vec::new();

    for item in &mut item_impl.items {
        if let ImplItem::Fn(function) = item {
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

                let duplicate_handler = controller_parts.handlers
                    .entry(path.to_string())
                    .or_default()
                    .insert(method, function.clone());

                if duplicate_handler.is_some() {
                    return Error::new_spanned(
                        &function.sig,
                        format!(
                            "duplicate handler: function named `{}` is already assigned to route `{} {}`",
                            duplicate_handler.unwrap().sig.ident.to_string(),
                            method.to_string().to_ascii_uppercase(),
                            path
                        )
                    ).to_compile_error();
                }

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
                        },
                    }
                }
            } else {
                controller_parts.ignored_functions.push(function.clone());
            }
        }
    }

    //
    // Convert everything we've gathered into handlers and router setup calls
    //

    let ignored_functions = controller_parts.ignored_functions;
    let (handlers, routes_setup) = {
        let mut handlers: Vec<TokenStream> = Vec::new();
        let mut routes_setup: Vec<TokenStream> = Vec::new();

        for (path, routes) in controller_parts.handlers {
            for (method, impl_item_fn) in routes {
                let impl_item_fn = impl_item_fn.clone();
                let method_str = method.to_string().to_ascii_uppercase();

                let comment = format!(" HTTP handler: {method_str} {path}");

                handlers.push(quote! {
                    #[doc = #comment]
                    #impl_item_fn
                });

                let routing_method = format_ident!("{}", method.to_string());
                let fn_name = impl_item_fn.sig.ident.clone();

                routes_setup.push(quote! {
                    .route(#path, ::axum::routing::#routing_method(Self::#fn_name))
                });
           }
        }

        (handlers, routes_setup)
    };

    //
    // Regenerate the entire impl
    //

    quote! {
        impl #ident {
            #(#handlers)*

            #(#ignored_functions)*
        }

        #(#type_assertions)*

        impl ::humars::Controller for #ident {
            fn merge_into_router(other: ::axum::Router) -> ::axum::Router {
                let this_router = ::axum::Router::new()
                    #(#routes_setup)*
                ;

                other.merge(this_router)
            }

            fn merge_into_openapi_builder(other: ::utoipa::openapi::OpenApiBuilder) -> ::utoipa::openapi::OpenApiBuilder {
                other
            }
        }
    }
}

//
// endregion: AST parsing and generation ---------------------------------------------------
