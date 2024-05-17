
use darling::FromMeta;
use syn::{Error, Item, ItemStruct};
use proc_macro2::TokenStream;
use syn::parse2;
use quote::{quote};

// region: Annotation args -------------------------------------------------------------------------
//

/// Arguments of `#[DTO(...)]` annotation.
#[derive(FromMeta)]
pub(crate) struct DtoArgs {
    #[darling(default)]
    pub(crate) response: bool,

    #[darling(default)]
    pub(crate) request: bool,
}

//
// endregion: Annotation args  ---------------------------------------------------------------------

// region: AST parsing and generation --------------------------------------------------------------
//

/// Entrypoint for `#[DTO(...)]` macro.
pub(crate) fn generate(args_t: TokenStream, args: DtoArgs, input: TokenStream) -> TokenStream {
    match parse2::<Item>(input) {
        Err(error) =>
            error.to_compile_error(),

        Ok(item) => match item {
            Item::Struct(item_struct) =>
                generate_impl_for_struct(args_t, args, item_struct),

            _ =>
                Error::new_spanned(item, "DTO should be a struct.").to_compile_error(),
        }
    }
}

/// Generates `#[DTO]` from a struct.
fn generate_impl_for_struct(_args_t: TokenStream, args: DtoArgs, item_struct: ItemStruct) -> TokenStream {
    let ident = &item_struct.ident;

    let (deserialize_derive, dto_request_impl) =
        if !args.request {
            Default::default()
        } else {
            let deserialize_derive = quote! { #[derive(::serde::Deserialize)] };
            let request_impl = quote! { impl ::groom::DTO_Request for #ident {} };

            (deserialize_derive, request_impl)
        };

    let (serialize_derive, dto_response_impl) =
        if !args.response {
            Default::default()
        } else {
            let serialize_derive = quote!{ #[derive(::serde::Serialize)] };
            let response_impl = quote! { impl ::groom::DTO_Response for #ident {} };

            (serialize_derive, response_impl)
        };

    let openapi_derive = derive_openapi_schema_generation();

    quote! {
        #deserialize_derive
        #serialize_derive
        #openapi_derive
        #item_struct

        impl ::groom::DTO for #ident {}

        #dto_request_impl
        #dto_response_impl
    }
}

/// Returns TokenStream for invocation of `#[derive(::utoipa::ToSchema)]`.
fn derive_openapi_schema_generation() -> TokenStream {
    quote! {
        #[derive(::utoipa::ToSchema)]
    }
}

//
// endregion: AST parsing and generation -----------------------------------------------------------
