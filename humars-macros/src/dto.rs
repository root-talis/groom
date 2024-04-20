
use darling::FromMeta;
use syn::{Error, Item, ItemStruct};
use proc_macro2::TokenStream;
use syn::parse2;
use quote::quote;

// region: DTO args ----------------------------------------------------------------
//

#[derive(FromMeta)]
pub(crate) struct DtoArgs {
    #[darling(default)]
    pub(crate) request: bool,

    #[darling(default)]
    pub(crate) response: bool,
}

//
// endregion: DTO args -------------------------------------------------------------

// region: AST parsing and generation ----------------------------------------------
//

pub(crate) fn generate(args: DtoArgs, input: TokenStream) -> TokenStream {
    let generated_impl = generate_impl(args, input);

    quote! {
        #generated_impl
    }
}

fn generate_impl(args: DtoArgs, input: TokenStream) -> TokenStream {
    match parse2::<Item>(input) {
        Err(error) => error.to_compile_error(),
        Ok(item) => match item {
            Item::Struct(item_struct) => generate_impl_struct(args, item_struct),
            // todo: support enums
            _ => Error::new_spanned(item, "DTO should be a struct.").to_compile_error(),
        }
    }
}

fn generate_impl_struct(args: DtoArgs, item_struct: ItemStruct) -> TokenStream {
    let ident = item_struct.ident.clone();

    let request_derive = match args.request {
        false => Default::default(),
        true  => quote!{ #[derive(::serde::Deserialize)] },
    };

    let response_derive = match args.response {
        false => Default::default(),
        true  => quote!{ #[derive(::serde::Serialize)] },
    };

    let into_response_derive = match args.response {
        false => Default::default(),
        true => quote! {
            impl ::axum::response::IntoResponse for #ident {
                fn into_response(self) -> axum::http::Response<::axum::body::Body> {
                    ::axum::Json(self).into_response()
                }
            }
        },
    };

    quote! {
        #request_derive
        #response_derive
        #[derive(::utoipa::ToSchema)]
        #item_struct

        impl ::humars::DTO for #ident {}

        #into_response_derive
    }
}

//
// endregion: AST parsing and generation ---------------------------------------------------
