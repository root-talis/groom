use indexmap::IndexMap;
use proc_macro2::{Ident, TokenStream};
use syn::{parse2, Error, Item, ItemMod, ItemFn, Visibility};
use quote::{format_ident, quote, ToTokens};
use darling::FromMeta;
use syn::Attribute;

use crate::{http::HTTPMethod, annotation_attrs::{parse_attr, remove_attrs}};
use crate::comments::DocblockParts;

// region: ControllerArgs  -----------------------------------------------------------
//

#[derive(FromMeta, Default)]
pub(crate) struct ControllerArgs {
    /// State type for router. `S` from `::axum::Router<S>`.
    ///
    /// Value of this type goes into `Router::new().with_state(s)`.
    ///
    /// Defaults to `()` (unit type).
    #[darling(default)]
    pub(crate) state_type: Option<syn::Expr>,
}

//
// endregion: ControllerArgs -------------------------------------------------------

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

pub(crate) fn generate(args_t: TokenStream, args: ControllerArgs, input: TokenStream) -> TokenStream {
    generate_impl(args_t, args, input).unwrap_or_else(|e| e)
}

struct ModuleASTFragments {
    /// for routes deduplication
    module_items: Vec<TokenStream>,

    /// all module items in the original order
    seen_handlers: IndexMap<String, IndexMap<HTTPMethod, String>>,

    /// code to set up routes in merge_into_router()
    routes_setup: Vec<TokenStream>,

    /// code to set up paths in merge_into_openapi_builder()
    openapi_paths_setup: IndexMap<String, Vec<TokenStream>>,

    /// compile-time checks of trait implementation (for better error messages)
    type_assertions: Vec<TokenStream>,
}

struct HandlerASTFragments {
    /// name of the wrapping function which implements stuff like content negotiation
    wrapper_name: Ident,

    /// input arguments of the wrapper (AST for wrapper definition)
    wrapper_inputs: Vec<TokenStream>,

    /// input arguments which get delegated into the original handler
    delegated_inputs: Vec<TokenStream>,

    /// ast fragments to add extractors into OpenAPI spec
    openapi_extractors_modifiers: Vec<TokenStream>,

    /// entire AST to set up this handler OpenAPI spec
    openapi_setup: TokenStream,
}

/// Generates implementation for mod annotated with `#[Controller()]`
fn generate_impl(_args_t: TokenStream, args: ControllerArgs, input: TokenStream) -> Result<TokenStream, TokenStream> {
    let item_mod = match parse2::<ItemMod>(input) {
        Ok(syntax_tree) => syntax_tree,
        Err(error) => return Err(error.to_compile_error()),
    };

    if item_mod.content.is_none() {
        return Err(Error::new_spanned(&item_mod.ident, "module should have content").to_compile_error());
    }

    let items = item_mod.content.unwrap().1;

    //
    // Walk through all handlers and parse them
    //

    let mut fragments = ModuleASTFragments {
        seen_handlers: IndexMap::new(),
        module_items: Vec::with_capacity(items.len()),
        routes_setup: Vec::new(),
        openapi_paths_setup: IndexMap::new(),
        type_assertions: Vec::new(),
    };

    for item in items {
        if let Item::Fn(mut function) = item {
            parse_handler_function(&mut function, &mut fragments)?;
        } else {
            fragments.module_items.push(item.into_token_stream());
        }
    }

    //
    // Regenerate the entire module
    //

    Ok(generate_new_mod_ast(args, &item_mod.vis, &item_mod.ident, fragments))
}

/// Analyzes handler's AST and breaks it into meaningful fragments
fn parse_handler_function(
    function: &mut ItemFn,
    mod_fragments: &mut ModuleASTFragments,
) -> Result<(), TokenStream> {
    if function.sig.asyncness.is_none() {
        return Err(Error::new_spanned(&function.sig.fn_token, "handler should be async fn").to_compile_error());
    }

    let route = match extract_route_args(function, mod_fragments)? {
        Some(r) => r,
        None => return Ok(())
    };

    deduplicate_handler(function, &route, mod_fragments)?;

    let mut fn_fragments = generate_handler_fragments(function, mod_fragments)?;

    generate_route_add_ast(&fn_fragments.wrapper_name, &route, mod_fragments);
    fn_fragments.openapi_setup = generate_openapi_modify_op_ast(function, mod_fragments)?;

    //
    // new module item instead of current one:
    //

    // change comment:
    let docblock = crate::comments::get_docblock_parts(&function.attrs).unwrap_or_default();
    crate::comments::remove_docblock(&mut function.attrs);

    generate_new_handler_ast(&function, &route, &docblock, &fn_fragments, mod_fragments);
    generate_openapi_paths_setup_ast(&fn_fragments, &route, &docblock, mod_fragments);

    Ok(())
}

/// Extracts the arguments of `#[Route(...)]` annotation
fn extract_route_args(function: &mut ItemFn, mod_fragments: &mut ModuleASTFragments) -> Result<Option<RouteArgs>, TokenStream> {
    let args = match RouteArgs::parse_from_attrs(&function.attrs) {
        Ok(args) => {
            RouteArgs::remove_from_attrs(&mut function.attrs);
            args
        },
        Err(error) => return Err(error.write_errors()),
    };

    if args.is_none() {
        mod_fragments.module_items.push(function.into_token_stream());
        return Ok(None);
    }

    let route = args.expect("args parse result is checked right above");
    Ok(Some(route))
}

/// Deduplicates handler. If a duplicate is found, emits a compile error.
fn deduplicate_handler(handler: &mut ItemFn, route: &RouteArgs, mod_fragments: &mut ModuleASTFragments) -> Result<(), TokenStream> {
    let path = &route.path;
    let method = &route.method;

    let fn_name = &handler.sig.ident;

    let duplicate_handler = mod_fragments.seen_handlers
        .entry(path.clone())
        .or_default()
        .insert(*method, fn_name.to_string())
    ;

    if duplicate_handler.is_some() {
        return Err(Error::new_spanned(
            &handler.sig,
            format!(
                "duplicate handler: function named `{}` is already assigned to route `{} {}`",
                duplicate_handler.unwrap(),
                method,
                path,
            )
        ).to_compile_error());
    }
    Ok(())
}

/// Generates AST fragments for handler function
fn generate_handler_fragments(handler: &mut ItemFn, mod_fragments: &mut ModuleASTFragments) -> Result<HandlerASTFragments, TokenStream> {
    let mut fn_fragments = HandlerASTFragments {
        wrapper_name: format_ident!("__groom_wrapper_{}", &handler.sig.ident),
        openapi_extractors_modifiers: Vec::new(),
        wrapper_inputs: Vec::new(),
        delegated_inputs: Vec::new(),
        openapi_setup: Default::default(),
    };

    for item in &handler.sig.inputs {
        match item {
            syn::FnArg::Receiver(receiver) => {
                return Err(
                    Error::new_spanned(
                        &receiver,
                        "handlers with receiver are not supported, remove `self` and use State instead: https://docs.rs/axum/latest/axum/extract/struct.State.html"
                    ).to_compile_error()
                );
            },
            syn::FnArg::Typed(arg) => {
                let ty = arg.ty.as_ref();

                mod_fragments.type_assertions.push(quote! {
                    assert_impl_all!(#ty: ::groom::extract::GroomExtractor);
                });

                fn_fragments.openapi_extractors_modifiers.push(quote! {
                    op_builder = <#ty>::__openapi_modify_operation(op_builder);
                });

                let input_ident = format_ident!("input{}", fn_fragments.delegated_inputs.len());

                fn_fragments.wrapper_inputs.push(quote! {
                    #input_ident: #ty,
                });

                fn_fragments.delegated_inputs.push(quote! {
                    #input_ident,
                });
            },
        }
    }

    Ok(fn_fragments)
}

/// Generates an AST to add OpenAPI spec modifier for this particular handler
fn generate_openapi_modify_op_ast(handler: &mut ItemFn, mod_fragments: &mut ModuleASTFragments) -> Result<TokenStream, TokenStream> {
    Ok(match &handler.sig.output {
        syn::ReturnType::Default => {
            return Err(
                Error::new_spanned(
                    &handler.sig,
                    "handlers must return something"
                ).to_compile_error()
            );
        },
        syn::ReturnType::Type(_arrow, ty) => {
            mod_fragments.type_assertions.push(quote! {
                assert_impl_all!(#ty: ::groom::response::Response);
            });

            quote! {op_builder = <#ty>::__openapi_modify_operation(op_builder);}
        },
    })
}

/// Generates an AST to configure all paths of this mod for the OpenAPI spec
fn generate_openapi_paths_setup_ast(
    fn_fragments: &HandlerASTFragments,
    route: &RouteArgs,
    docblock: &DocblockParts,
    mod_fragments: &mut ModuleASTFragments
) {
    let path = &route.path;
    let method = &route.method;

    let operation = match method {
        HTTPMethod::Connect  => quote! {::utoipa::openapi::PathItemType::Connect},
        HTTPMethod::Delete   => quote! {::utoipa::openapi::PathItemType::Delete },
        HTTPMethod::Get      => quote! {::utoipa::openapi::PathItemType::Get    },
        HTTPMethod::Head     => quote! {::utoipa::openapi::PathItemType::Head   },
        HTTPMethod::Options  => quote! {::utoipa::openapi::PathItemType::Option },
        HTTPMethod::Patch    => quote! {::utoipa::openapi::PathItemType::Patch  },
        HTTPMethod::Post     => quote! {::utoipa::openapi::PathItemType::Post   },
        HTTPMethod::Put      => quote! {::utoipa::openapi::PathItemType::Put    },
        HTTPMethod::Trace    => quote! {::utoipa::openapi::PathItemType::Trace  },
    };

    let summary_tk = match &docblock.summary {
        Some(s) => quote! { Some(#s) },
        None => quote! { None as Option<String> },
    };

    let description_tk = match &docblock.description {
        Some(s) => quote! { Some(#s) },
        None => quote! { None as Option<String> },
    };

    let extractors = &fn_fragments.openapi_extractors_modifiers;
    let openapi_setup = &fn_fragments.openapi_setup;

    mod_fragments.openapi_paths_setup.entry(path.clone()).or_default().push(quote! {
        {
            let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                    .summary(#summary_tk)
                    .description(#description_tk);

            #(#extractors)*

            #openapi_setup

            ::utoipa::openapi::path::PathItemBuilder::new()
                .operation(#operation, op_builder.build())
                .build()
        }
    });
}

/// Generates AST to install a new route into Router
fn generate_route_add_ast(
    wrapper_name: &Ident,
    route: &RouteArgs,
    mod_fragments: &mut ModuleASTFragments
) {
    let path = &route.path;
    let method = &route.method;

    let routing_method = format_ident!("{}", method.to_string());

    mod_fragments.routes_setup.push(quote! {
        .route(#path, ::axum::routing::#routing_method(#wrapper_name))
    });
}

/// Generates new AST for the original handler and it's wrapper
fn generate_new_handler_ast(
    original_handler: &ItemFn,
    route: &RouteArgs,
    docblock: &DocblockParts,
    fn_fragments: &HandlerASTFragments,
    mod_fragments: &mut ModuleASTFragments,
) {
    let path = &route.path;
    let method = &route.method;

    // make new module item:
    let method_str = method.to_string().to_ascii_uppercase();
    let handler_comment = format!(" HTTP handler: {method_str} {path}");

    let mut new_comment: Vec<TokenStream> = Vec::new();
    if let Some(s) = &docblock.summary {
        let s = format!(" {s}");
        new_comment.push(quote!{#[doc = #s]});
        new_comment.push(quote!{#[doc = ""]});
    }
    new_comment.push(quote!{#[doc = #handler_comment]});
    if let Some(s) = &docblock.description {
        let s = format!(" {s}");
        new_comment.push(quote!{#[doc = ""]});
        new_comment.push(quote!{#[doc = #s]});
    }

    let wrapper_name = &fn_fragments.wrapper_name;
    let wrapper_inputs = &fn_fragments.wrapper_inputs;
    let delegated_inputs = &fn_fragments.delegated_inputs;
    let fn_name = &original_handler.sig.ident;

    // generate module item:
    mod_fragments.module_items.push(quote! {
        #(#new_comment)*
        #original_handler

        async fn #wrapper_name(headers: ::axum::http::header::HeaderMap, #(#wrapper_inputs)*) -> impl ::axum::response::IntoResponse {
            let accept = ::groom::content_negotiation::parse_accept_header(&headers);

            // todo: check that accept is valid for output before running code

            let result = #fn_name(#(#delegated_inputs)*).await;

            result.__groom_into_response(accept)
        }
    });

    // todo: #wrapper_output should be a Result with error type indicating bad accept header;
    // todo: that error should serialize into an appropriate header and response code.
}

/// Generates new AST for the entire mod based on parsed fragments
fn generate_new_mod_ast(
    args: ControllerArgs,
    vis: &Visibility,
    ident: &Ident,
    fragments: ModuleASTFragments
) -> TokenStream {
    let mut paths: Vec<TokenStream> = Vec::new();
    for p in fragments.openapi_paths_setup {
        let url = p.0;

        for m in p.1 {
            paths.push(quote! {
                paths = paths.path(#url, #m);
            });
        }
    }

    let state_ty = args.state_type.unwrap_or_else(
        || syn::parse_str::<syn::Expr>("()").unwrap()
    );

    let module_items = fragments.module_items;
    let routes_setup = fragments.routes_setup;
    let type_assertions = fragments.type_assertions;

    quote! {
        #vis mod #ident {
            use ::static_assertions::{assert_impl_all, assert_impl_any};

            #(#module_items)*

            // todo: validate return codes of types in runtime?

            pub fn merge_into_router(other: ::axum::Router<#state_ty>) -> ::axum::Router<#state_ty> {
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
