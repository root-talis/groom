use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Error, Fields, Item, ItemStruct, parse2};
use crate::utils::get_description;

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
            Item::Struct(item_struct) => generate_impl_for_struct(args_t, args, item_struct),
            _ => Error::new_spanned(item, "RequestBody should be a struct.").to_compile_error(),
        }
    }
}

fn generate_impl_for_struct(args_t: TokenStream, args: RequestBodyArgs, item_struct: ItemStruct) -> TokenStream {
    let ident = item_struct.ident.clone();
    let vis = item_struct.vis.clone();

    let rejection_ident = format_ident!("{ident}Rejection");

    let mut body_extractors: Vec<TokenStream> = Vec::new();
    let mut rejection_types: Vec<TokenStream> = Vec::new();
    let mut rejections_into_response: Vec<TokenStream> = Vec::new();
    let mut openapi_generators: Vec<TokenStream> = Vec::new();

    let mut type_assertions: Vec<TokenStream> = Vec::new(); // compile-time checks of trait implementation (for better error messages)

    if !args.format.is_any() {
        return syn::Error::new_spanned(
            args_t,
            format!("specify at least one request format for struct `{ident}` to be able to extract data from request body (e.g. #[RequestBody(format(json))])")
        ).into_compile_error();
    }

    // schema     - schema of #ident for OpenAPI spec
    // extract_ty - typename into which request body should be extracted
    //              (either #ident for named struct or typename of the DTO that it's wrapping for unnamed struct)
    // pack_dto   - code to pack the extracted DTO into #ident (if #ident is an unnamed struct that wraps DTO)
    let (schema, extract_ty, pack_dto) = match item_struct.fields.clone() {
        Fields::Unit => {
            return syn::Error::new_spanned(
                item_struct.into_token_stream(),
                format!("`{ident}`: RequestBody should not be an empty struct. Try making it an empty named struct: `struct {ident} {{}}`.")
            ).into_compile_error();
        }
        Fields::Named(_) => {
            let schema = quote! {#ident::schema().1};
            let extract_ty = quote! {#ident};
            let pack_dto = quote! { dto };

            (schema, extract_ty, pack_dto)
        }
        Fields::Unnamed(f) => {
            if f.unnamed.len() == 0 {
                return syn::Error::new_spanned(
                    item_struct.into_token_stream(),
                    format!("`{ident}`: RequestBody should not be an empty unnamed struct. Try making it an empty named struct: `struct {ident} {{}}`.")
                ).into_compile_error();
            }

            if f.unnamed.len() > 1 {
                return syn::Error::new_spanned(
                    item_struct.into_token_stream(),
                    format!("`{ident}`: RequestBody should either have named fields or have a single unnamed field. Try making it a named struct: `struct {ident} {{...}}`.")
                ).into_compile_error();
            }

            let first_field = f.unnamed.first().unwrap();
            let ty = first_field.ty.clone();

            type_assertions.push(quote!{
                assert_impl_all!(#ty: ::groom::DTO);
            });

            let schema = quote! {#ty::schema().1};
            let extract_ty = quote! {#ty};
            let pack_dto = quote! { #ident(dto) };

            (schema, extract_ty, pack_dto)
        }
    };

    if args.format.json {
        body_extractors.push(quote! {
            Some(::groom::content_negotiation::BodyContentType::Json) => {
                let dto = ::axum::extract::Json::<#extract_ty>::from_request(req, state)
                    .await
                    .map_err(|e| #rejection_ident::JsonRejection(e))?
                    .0
                ;

                Ok(#pack_dto)
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
                    .schema(#schema)
                    .build()
            )
        });
    }

    if args.format.url_encoded {
        body_extractors.push(quote! {
            Some(::groom::content_negotiation::BodyContentType::FormUrlEncoded) => {
                let dto = ::axum::extract::Form::<#extract_ty>::from_request(req, state)
                    .await
                    .map_err(|e| #rejection_ident::FormRejection(e))?
                    .0
                ;

                Ok(#pack_dto)
            },
        });

        rejection_types.push(quote! {
            FormRejection(::axum::extract::rejection::FormRejection),
        });

        rejections_into_response.push(quote! {
            #rejection_ident::FormRejection(r) => r.into_response(),
        });

        openapi_generators.push(quote! {
            .content(
                ::mime::APPLICATION_WWW_FORM_URLENCODED.as_ref(),
                ::utoipa::openapi::ContentBuilder::new()
                    .schema(#schema)
                    .build()
            )
        });
    }

    let description_tk = match get_description(&item_struct.attrs).unwrap_or_default() {
        Some(s) => quote! { .description(Some(#s)) },
        None           => quote! {  },
    };

    quote! {
        #[derive(::serde::Deserialize)]
        #[derive(::utoipa::ToSchema)]
        #item_struct

        impl ::groom::extract::GroomExtractor for #ident {
            fn __openapi_modify_operation(op: ::utoipa::openapi::path::OperationBuilder) -> ::utoipa::openapi::path::OperationBuilder {
                op.request_body(Some(
                    ::utoipa::openapi::request_body::RequestBodyBuilder::new()
                        #(#openapi_generators)*
                        #description_tk
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
                let content_type = ::groom::content_negotiation::parse_content_type(req.headers());

                match ::groom::content_negotiation::get_body_content_type(content_type) {
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

        #(#type_assertions)*
    }
}

// region: RequestBodyTypesList ------------------------------------------------------------
//

#[derive(FromMeta, Default)]
pub(crate) struct RequestBodyTypesList {
    #[darling(default)]
    pub(crate) url_encoded: bool,

    #[darling(default)]
    pub(crate) json: bool,
}

impl RequestBodyTypesList {
    pub(crate) fn is_any(&self) -> bool {
        return self.url_encoded || self.json;
    }
}

//
// endregion: RequestBodyTypesList ---------------------------------------------------------
