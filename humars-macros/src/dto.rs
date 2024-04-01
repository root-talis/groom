
use syn::{Error, Item, ItemStruct};
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use syn::parse2;
use quote::quote;

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
            Item::Struct(item_struct) => generate_impl_struct(item_struct),
            // todo: support enums
            _ => Error::new_spanned(item, "DTO should be a struct.").to_compile_error(),
        }
    }
}

fn generate_impl_struct(item_struct: ItemStruct) -> TokenStream {
    let ident = item_struct.ident.clone();

    quote! {
        #[derive(::serde::Serialize, ::serde::Deserialize, ::utoipa::ToSchema)]
        #item_struct

        impl ::humars::DTO for #ident {}
    }
}

//
// endregion: AST parsing and generation ---------------------------------------------------
