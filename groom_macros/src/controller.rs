use convert_case::{Case, Casing};
use indexmap::IndexMap;
use proc_macro2::{Ident, TokenStream};
use syn::{parse2, Error, Item, ItemMod, ItemFn, Visibility, ReturnType};
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
    generate_controller_impl(args_t, args, input).unwrap_or_else(|e| e)
}

struct ModuleASTFragments {
    /// Module items in original order (handlers replaced by wrapper AST).
    module_items: Vec<TokenStream>,

    /// Seen (path, method) → handler name for route deduplication.
    seen_handlers: IndexMap<String, IndexMap<HTTPMethod, String>>,

    /// Route install fragments for `into_router` / `merge_into_router`.
    routes_setup: Vec<TokenStream>,

    /// OpenAPI path-item setup fragments keyed by URL path.
    openapi_paths_setup: IndexMap<String, Vec<TokenStream>>,

    /// Compile-time trait assertions (clearer expand errors).
    type_assertions: Vec<TokenStream>,

    /// Runtime HTTP status/format check fragments.
    runtime_checks: Vec<TokenStream>,
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
    openapi_modification_code: TokenStream,
}

/// Generates implementation for mod annotated with `#[Controller()]`
fn generate_controller_impl(_args_t: TokenStream, args: ControllerArgs, input: TokenStream) -> Result<TokenStream, TokenStream> {
    let item_mod = parse2::<ItemMod>(input).map_err(|error| error.to_compile_error())?;

    let Some((_, items)) = item_mod.content else {
        return Err(Error::new_spanned(&item_mod.ident, "module should have content").to_compile_error());
    };

    let mut fragments = ModuleASTFragments {
        seen_handlers: IndexMap::new(),
        module_items: Vec::with_capacity(items.len()),
        routes_setup: Vec::new(),
        openapi_paths_setup: IndexMap::new(),
        type_assertions: Vec::new(),
        runtime_checks: Vec::new(),
    };

    for item in items {
        if let Item::Fn(mut function) = item {
            parse_handler_function(&mut function, &mut fragments)?;
        } else {
            fragments.module_items.push(item.into_token_stream());
        }
    }

    Ok(generate_new_mod_ast(args, &item_mod.vis, &item_mod.ident, fragments))
}

/// Analyzes handler's AST and breaks it into meaningful fragments
fn parse_handler_function(
    function: &mut ItemFn,
    mod_fragments: &mut ModuleASTFragments,
) -> Result<(), TokenStream> {
    let Some(route) = extract_route_args(function, mod_fragments)? else {
        return Ok(());
    };

    if function.sig.asyncness.is_none() {
        return Err(Error::new_spanned(function.sig.fn_token, "handler should be async fn").to_compile_error());
    }

    ensure_handler_is_unique(function, &route, mod_fragments)?;

    let mut fn_fragments = generate_handler_fragments(function, mod_fragments)?;

    generate_router_modifier_for_handler(&fn_fragments.wrapper_name, &route, mod_fragments);
    fn_fragments.openapi_modification_code = generate_openapi_modifier_for_handler(function, mod_fragments)?;

    let docblock = crate::comments::get_docblock_parts(&function.attrs).unwrap_or_default();
    crate::comments::remove_docblock(&mut function.attrs);

    generate_new_handler_ast(function, &route, &docblock, &fn_fragments, mod_fragments);
    generate_openapi_paths_setup_ast(function, &fn_fragments, &route, &docblock, mod_fragments);

    if let ReturnType::Type(_, ty) = &function.sig.output {
        let ident = &function.sig.ident;
        let context_format = format!("{{context}}: handler `{ident}`");
        mod_fragments.runtime_checks.push(quote! {
            let mut codes = ::groom::runtime_checks::HTTPCodeSet::new();
            <#ty>::__groom_check_response_codes(format_args!(#context_format), &mut codes);

            let mut formats = ::groom::runtime_checks::HTTPFormatsSet::new();
            <#ty>::__groom_check_response_formats(format_args!(#context_format), &mut formats);
        });
    }

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

    let Some(route) = args else {
        mod_fragments.module_items.push(function.into_token_stream());
        return Ok(None);
    };

    Ok(Some(route))
}

/// Checks if there is a duplicate of handler. If a duplicate is found, emits a compile error.
fn ensure_handler_is_unique(handler: &mut ItemFn, route: &RouteArgs, mod_fragments: &mut ModuleASTFragments) -> Result<(), TokenStream> {
    let path = &route.path;
    let method = &route.method;

    let fn_name = &handler.sig.ident;

    let duplicate_handler = mod_fragments.seen_handlers
        .entry(path.clone())
        .or_default()
        .insert(*method, fn_name.to_string())
    ;

    if let Some(name) = duplicate_handler {
        return Err(Error::new_spanned(
            &handler.sig,
            format!(
                "duplicate handler: function named `{}` is already assigned to route `{} {}`",
                name,
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
        openapi_modification_code: Default::default(),
    };

    for item in &handler.sig.inputs {
        match item {
            syn::FnArg::Receiver(receiver) => {
                return Err(
                    Error::new_spanned(
                        receiver,
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
                    op_builder = <#ty>::__openapi_modify_operation(op_builder, &mut components);
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
fn generate_openapi_modifier_for_handler(handler: &ItemFn, mod_fragments: &mut ModuleASTFragments) -> Result<TokenStream, TokenStream> {
    let syn::ReturnType::Type(_arrow, ty) = &handler.sig.output else {
        return Err(
            Error::new_spanned(
                &handler.sig,
                "handlers must return something"
            ).to_compile_error()
        );
    };

    mod_fragments.type_assertions.push(quote! {
        assert_impl_all!(#ty: ::groom::response::Response);
    });

    Ok(quote! {op_builder = <#ty>::__openapi_modify_operation(op_builder, &mut components);})
}

/// Generates an AST to configure all paths of this mod for the OpenAPI spec
fn generate_openapi_paths_setup_ast(
    handler: &ItemFn,
    fn_fragments: &HandlerASTFragments,
    route: &RouteArgs,
    docblock: &DocblockParts,
    mod_fragments: &mut ModuleASTFragments
) {
    let path = &route.path;
    let method = &route.method;

    if matches!(method, HTTPMethod::Connect) {
        // CONNECT routes keep axum routing (installed by generate_router_modifier_for_handler)
        // but are omitted from OpenAPI: OpenAPI 3 path items cannot model CONNECT via
        // utoipa's HttpMethod (the enum has no Connect variant).
        return;
    }

    let operation = match method {
        HTTPMethod::Delete   => quote! {::utoipa::openapi::path::HttpMethod::Delete },
        HTTPMethod::Get      => quote! {::utoipa::openapi::path::HttpMethod::Get    },
        HTTPMethod::Head     => quote! {::utoipa::openapi::path::HttpMethod::Head   },
        HTTPMethod::Options  => quote! {::utoipa::openapi::path::HttpMethod::Options},
        HTTPMethod::Patch    => quote! {::utoipa::openapi::path::HttpMethod::Patch  },
        HTTPMethod::Post     => quote! {::utoipa::openapi::path::HttpMethod::Post   },
        HTTPMethod::Put      => quote! {::utoipa::openapi::path::HttpMethod::Put    },
        HTTPMethod::Trace    => quote! {::utoipa::openapi::path::HttpMethod::Trace  },
        // The compiler cannot see that the early return above excludes CONNECT,
        // so the match needs this arm. It is never reached.
        HTTPMethod::Connect  => unreachable!("CONNECT is handled by the early return above"),
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
    let openapi_setup = &fn_fragments.openapi_modification_code;

    let operation_id = handler.sig.ident.to_string().to_case(Case::Camel);

    mod_fragments.openapi_paths_setup.entry(path.clone()).or_default().push(quote! {
        {
            let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                    .summary(#summary_tk)
                    .description(#description_tk)
                    .operation_id(Some(#operation_id))
            ;

            #(#extractors)*

            #openapi_setup

            ::utoipa::openapi::path::PathItemBuilder::new()
                .operation(#operation, op_builder.build())
                .build()
        }
    });
}

/// Generates AST to install a new route into Router
fn generate_router_modifier_for_handler(
    handler_wrapper_name: &Ident,
    route: &RouteArgs,
    mod_fragments: &mut ModuleASTFragments
) {
    let path = &route.path;
    let method = &route.method;

    let routing_method = format_ident!("{}", method.to_string());

    mod_fragments.routes_setup.push(quote! {
        .route(#path, ::axum::routing::#routing_method(#handler_wrapper_name))
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

    // The wrapper needs the handler's return type to call its associated
    // `__groom_negotiate_content_type` for the pre-run Accept check.
    // `generate_openapi_modifier_for_handler()` has already rejected handlers
    // without a return type before this point.
    let return_ty = match &original_handler.sig.output {
        syn::ReturnType::Type(_, ty) => ty,
        syn::ReturnType::Default => {
            unreachable!("handlers must return something (checked earlier in generate_openapi_modifier_for_handler)")
        }
    };

    // generate module item:
    mod_fragments.module_items.push(quote! {
        #(#new_comment)*
        #original_handler

        async fn #wrapper_name(headers: ::axum::http::header::HeaderMap, #(#wrapper_inputs)*) -> impl ::axum::response::IntoResponse {
            let accept = match ::groom::content_negotiation::parse_accept_header(&headers) {
                Err(_) => return ::groom::response::bad_accept_header(),
                Ok(accept) => accept,
            };

            let negotiated = match accept {
                None => None,
                Some(accept) => match <#return_ty>::__groom_negotiate_content_type(&accept) {
                    Err(response) => return response,
                    Ok(negotiated) => negotiated,
                },
            };

            let result = #fn_name(#(#delegated_inputs)*).await;

            result.__groom_into_response(negotiated)
        }
    });
}

/// Generates new AST for the entire mod based on parsed fragments
fn generate_new_mod_ast(
    args: ControllerArgs,
    vis: &Visibility,
    ident: &Ident,
    fragments: ModuleASTFragments
) -> TokenStream {
    let mut path_assignments: Vec<TokenStream> = Vec::new();
    for p in fragments.openapi_paths_setup {
        let url = p.0;

        for m in p.1 {
            path_assignments.push(quote! {
                __groom_paths.push((#url.to_string(), #m));
            });
        }
    }

    let state_ty = args.state_type.unwrap_or_else(
        || syn::parse_str::<syn::Expr>("()").unwrap()
    );

    let module_items = fragments.module_items;
    let routes_setup = fragments.routes_setup;
    let type_assertions = fragments.type_assertions;
    let runtime_checks = fragments.runtime_checks;

    let runtime_checks_context = format!("Groom runtime check of mod `{ident}`");

    quote! {
        #vis mod #ident {
            use ::static_assertions::{assert_impl_all, assert_impl_any};

            #(#module_items)*

            fn __groom_runtime_checks() {
                // Static module label — no String allocation on the success path.
                let context = #runtime_checks_context;
                #(#runtime_checks)*
            }

            pub fn into_router() -> ::groom::router::GroomRouter<#state_ty> {
                __groom_runtime_checks();

                let this_router: ::axum::Router<#state_ty> = ::axum::Router::new()
                    #(#routes_setup)*
                ;

                let mut components = ::groom::extract::ComponentsRegistry::new();
                let mut __groom_paths: ::std::vec::Vec<(::std::string::String, ::utoipa::openapi::path::PathItem)> = ::std::vec::Vec::new();
                #(#path_assignments)*
                ::groom::router::GroomRouter::from_controller_parts(
                    this_router, components, __groom_paths
                )
            }

            pub fn merge_into_router(other: impl Into<::groom::router::GroomRouter<#state_ty>>) -> ::std::result::Result<::groom::router::GroomRouter<#state_ty>, ::groom::router::MergeError> {
                __groom_runtime_checks();

                let this_router: ::axum::Router<#state_ty> = ::axum::Router::new()
                    #(#routes_setup)*
                ;

                let mut components = ::groom::extract::ComponentsRegistry::new();
                let mut __groom_paths: ::std::vec::Vec<(::std::string::String, ::utoipa::openapi::path::PathItem)> = ::std::vec::Vec::new();
                #(#path_assignments)*
                let __groom_this = ::groom::router::GroomRouter::from_controller_parts(
                    this_router, components, __groom_paths
                );
                let __groom_other = other.into();
                __groom_other.merge(__groom_this)
            }

            #(#type_assertions)*
        }
    }
}


//
// endregion: AST parsing and generation ---------------------------------------------------
