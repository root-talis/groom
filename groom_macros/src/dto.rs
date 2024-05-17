
use darling::FromMeta;
use syn::{Error, Item, ItemStruct};
use proc_macro2::TokenStream;
use syn::parse2;
use quote::{quote};

// region: DTO args --------------------------------------------------------------------------------
//

#[derive(FromMeta)]
pub(crate) struct DtoArgs {
    #[darling(default)]
    pub(crate) response: bool,

    #[darling(default)]
    pub(crate) request: bool,
}

//
// endregion: DTO args -----------------------------------------------------------------------------

// region: AST parsing and generation --------------------------------------------------------------
//

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

    quote! {
        #deserialize_derive
        #serialize_derive
        #[derive(::utoipa::ToSchema)]
        #item_struct

        impl ::groom::DTO for #ident {}

        #dto_request_impl
        #dto_response_impl
    }
}

//
// endregion: AST parsing and generation -----------------------------------------------------------
