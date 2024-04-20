
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
    pub(crate) response: Option<DtoResponseArg>,
}

#[derive(FromMeta, Default)]
pub(crate) struct DtoResponseArg {
    #[darling(default)]
    pub(crate) json: bool,

    /*#[darling(default)]
    pub(crate) bson: bool,*/

    /*#[darling(default)]
    pub(crate) xml: bool,*/
}

impl DtoResponseArg {
    const fn is_any(&self) -> bool {
        self.json /*|| self.bson */ /*|| self.xml*/
    }

    fn get_response_types(&self) -> Vec<&str> {
        let mut response_types: Vec<&str> = Vec::with_capacity(1);

        if self.json {
            response_types.push("application/json");
        }

        response_types
    }
}

//
// endregion: DTO args -------------------------------------------------------------

// region: AST parsing and generation ----------------------------------------------
//

pub(crate) fn generate(args_t: TokenStream, args: DtoArgs, input: TokenStream) -> TokenStream {
    let generated_impl = generate_impl(args_t, args, input);

    quote! {
        #generated_impl
    }
}

fn generate_impl(args_t: TokenStream, args: DtoArgs, input: TokenStream) -> TokenStream {
    match parse2::<Item>(input) {
        Err(error) => error.to_compile_error(),
        Ok(item) => match item {
            Item::Struct(item_struct) => generate_impl_struct(args_t, args, item_struct),
            // todo: support enums
            _ => Error::new_spanned(item, "DTO should be a struct.").to_compile_error(),
        }
    }
}

fn generate_impl_struct(args_t: TokenStream, args: DtoArgs, item_struct: ItemStruct) -> TokenStream {
    let ident = item_struct.ident.clone();

    let request_derive = match args.request {
        false => Default::default(),
        true  => quote!{ #[derive(::serde::Deserialize)] },
    };

    let (response_derive, response_impl) =
        if args.response.is_none() {
            Default::default()
        } else {
            let response = args.response.unwrap();
            if !response.is_any() {
                return Error::new_spanned(args_t, "specify at least one format (e.g. `#[DTO(response(json))]`).").to_compile_error();
            }

            let response_derive = quote!{ #[derive(::serde::Serialize)] };
            let response_types = response.get_response_types();

            let content_impls = response_types.iter().map(|ct| {
                quote! {
                    let rb = rb.content(
                        #ct,
                        ::utoipa::openapi::ContentBuilder::new()
                            .schema(Self::schema().1)
                            .build()
                    );
                }
            });
        
            let response_impl = quote! {
                impl ::axum::response::IntoResponse for #ident {
                    fn into_response(self) -> axum::http::Response<::axum::body::Body> {
                        ::axum::Json(self).into_response()
                    }
                }

                impl ::humars::DTO_Response for #ident {
                    fn __openapi_build_responses(rb: ::utoipa::openapi::ResponseBuilder) -> ::utoipa::openapi::ResponseBuilder {
                        #(#content_impls)*
                        rb
                    }
                }
            };

            (response_derive, response_impl)
        };

    quote! {
        #request_derive
        #response_derive
        #[derive(::utoipa::ToSchema)]
        #item_struct

        impl ::humars::DTO for #ident {}

        #response_impl
    }
}

//
// endregion: AST parsing and generation ---------------------------------------------------
