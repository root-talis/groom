use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Error, Item, ItemStruct, parse2};

// region: RequestBodyArgs args ----------------------------------------------------------------
//

#[derive(FromMeta)]
pub(crate) struct RequestBodyArgs {
    #[darling(default)]
    pub(crate) format: RequestBodyTypesList,
}

//
// endregion: RequestBodyArgs args -------------------------------------------------------------


// region: AST parsing and generation ----------------------------------------------
//

pub(crate) fn generate(args_t: TokenStream, args: RequestBodyArgs, input: TokenStream) -> TokenStream {
    let generated_impl = generate_impl(args_t, args, input);

    quote! {
        #generated_impl
    }
}

fn generate_impl(args_t: TokenStream, args: RequestBodyArgs, input: TokenStream) -> TokenStream {
    match parse2::<Item>(input) {
        Err(error) => error.to_compile_error(),
        Ok(item) => match item {
            Item::Struct(item_struct) => generate_impl_struct(args_t, args, item_struct),
            _ => Error::new_spanned(item, "RequestBody should be a struct.").to_compile_error(),
        }
    }
}

fn generate_impl_struct(args_t: TokenStream, args: RequestBodyArgs, item_struct: ItemStruct) -> TokenStream {
    let ident = item_struct.ident.clone();
    let vis = item_struct.vis.clone();

    let rejection_ident = format_ident!("{ident}Rejection");

    let mut body_extractors: Vec<TokenStream> = Vec::new();
    let mut rejection_types: Vec<TokenStream> = Vec::new();
    let mut rejections_into_response: Vec<TokenStream> = Vec::new();
    let mut openapi_generators: Vec<TokenStream> = Vec::new();

    if !args.format.is_any() {
        return syn::Error::new_spanned(
            args_t,
            format!("specify at least one request format for struct `{ident}` to be able to extract data from request body (e.g. #[RequestBody(format(json))])")
        ).into_compile_error();
    }

    if args.format.json {
        body_extractors.push(quote! {
            Some(Json) => {
                Ok(
                    ::axum::extract::Json::<#ident>::from_request(req, state)
                        .await
                        .map_err(|e| #rejection_ident::JsonRejection(e))?
                        .0
                )
            },
        });

        rejection_types.push(quote! {
            JsonRejection(::axum::extract::rejection::JsonRejection),
        });

        rejections_into_response.push(quote! {
            #rejection_ident::JsonRejection(r) => r.into_response(),
        });

        openapi_generators.push(quote! {
            .content(
                ::mime::APPLICATION_JSON.as_ref(),
                ::utoipa::openapi::ContentBuilder::new()
                    .schema(#ident::schema().1)
                    .build()
            )
        });
    }

    quote! {
        #[derive(::serde::Deserialize)]
        #[derive(::utoipa::ToSchema)]
        #item_struct

        impl ::humars::extract::HumarsExtractor for #ident {
            fn __openapi_modify_operation(op: ::utoipa::openapi::path::OperationBuilder) -> ::utoipa::openapi::path::OperationBuilder {
                op.request_body(Some(
                    ::utoipa::openapi::request_body::RequestBodyBuilder::new()
                        #(#openapi_generators)*
                        .required(Some(::utoipa::openapi::Required::True))
                        .build()
                ))
            }
        }

        #[::async_trait::async_trait]
        impl<S> ::axum::extract::FromRequest<S> for #ident
        where
            S: Send + Sync,
        {
            type Rejection = #rejection_ident;

            async fn from_request(req: ::axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
                let content_type = ::humars::content_negotiation::parse_content_type(req.headers());

                match (::humars::content_negotiation::get_body_content_type(content_type)) {
                    #(#body_extractors)*

                    _ => {
                        Err(#rejection_ident::BadContentType)
                    },
                }
            }
        }

        #vis enum #rejection_ident {
            BadContentType,
            #(#rejection_types)*
        }

        impl ::axum::response::IntoResponse for #rejection_ident {
            fn into_response(self) -> ::axum::response::Response {
                match self {
                    #rejection_ident::BadContentType =>
                        (::axum::http::StatusCode::BAD_REQUEST, "Unsupported Content-Type").into_response(),

                    #(#rejections_into_response)*
                }
            }
        }
    }
}

// region: RequestBodyTypesList ------------------------------------------------------------
//

#[derive(FromMeta, Default)]
pub(crate) struct RequestBodyTypesList {
    /*#[darling(default)]
    pub(crate) form: bool,*/

    #[darling(default)]
    pub(crate) json: bool,
}

impl RequestBodyTypesList {
    pub(crate) fn is_any(&self) -> bool {
        return /*self.form || */self.json;
    }
}

//
// endregion: RequestBodyTypesList ---------------------------------------------------------
