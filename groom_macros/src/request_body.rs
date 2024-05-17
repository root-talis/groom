use darling::FromMeta;
use proc_macro2::TokenStream;
use syn::{Error, Item, parse2};

// region: RequestBody annotation args -------------------------------------------------------------
//

/// `#[RequestBody]` arguments
#[derive(FromMeta)]
pub(crate) struct RequestBodyArgs {
    #[darling(default)]
    pub(crate) format: RequestBodyTypesList,
}

/// `#[RequestBody(format(...))]` values
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
// endregion: RequestBodyArgs annotation args ------------------------------------------------------


// region: AST parsing and generation --------------------------------------------------------------
//

/// Parses AST that has been annotated with `#[RequestBody]` and generates new implementation
pub(crate) fn generate(args_t: TokenStream, args: RequestBodyArgs, input: TokenStream) -> TokenStream {
    match parse2::<Item>(input) {
        Err(error) =>
            error.to_compile_error(),

        Ok(item) => match item {
            Item::Struct(item_struct) =>
                struct_impl::generate(args_t, args, item_struct),

            _ =>
                Error::new_spanned(item, "RequestBody should be a struct.").to_compile_error(),
        }
    }
}

/// This module generates implementation of `#[RequestBody]` for `struct`.
mod struct_impl {
    use proc_macro2::{Ident, TokenStream};
    use quote::{format_ident, quote, ToTokens};
    use syn::{Fields, ItemStruct};
    use crate::request_body::RequestBodyArgs;
    use crate::utils::get_description;

    /// All AST fragments for implementation generation
    struct AllFragments {
        /// Typename for Rejections (when parsing request bodies)
        rejection_ident: Ident,

        description_tk: TokenStream,

        dto_fragments: DtoFragments,

        body_extractors: Vec<TokenStream>,
        rejection_types: Vec<TokenStream>,
        rejections_into_response: Vec<TokenStream>,
        openapi_generators: Vec<TokenStream>,

        /// compile-time checks of trait implementation (for better error messages)
        type_assertions: Vec<TokenStream>
    }

    /// AST fragments to work with output DTO
    #[derive(Default)]
    struct DtoFragments {
        /// Typename into which request body should be extracted
        /// (either #ident for named struct or typename of the DTO that it's wrapping for unnamed struct)
        extract_ty: TokenStream,

        /// Code to pack the extracted DTO into #ident (if #ident is an unnamed struct that wraps DTO)
        pack_dto: TokenStream,

        /// Schema of #ident for OpenAPI spec
        schema: TokenStream,
    }

    /// Generates RequestBody implementation for `struct`
    pub(crate) fn generate(args_t: TokenStream, args: RequestBodyArgs, item_struct: ItemStruct) -> TokenStream {
        let ident = &item_struct.ident;

        if !args.format.is_any() {
            return syn::Error::new_spanned(
                args_t,
                format!("specify at least one request format for struct `{ident}` to be able to extract data from request body (e.g. #[RequestBody(format(json))])")
            ).into_compile_error();
        }

        let mut context = AllFragments {
            rejection_ident: format_ident!("{ident}Rejection"),
            description_tk: match get_description(&item_struct.attrs).unwrap_or_default() {
                Some(s) => quote! { .description(Some(#s)) },
                None => quote! {  },
            },

            dto_fragments: Default::default(),
            body_extractors: Vec::new(),
            rejection_types: Vec::new(),
            rejections_into_response: Vec::new(),
            openapi_generators: Vec::new(),
            type_assertions: Vec::new(),
        };

        let dto_fragments = match make_dto_fragments(&item_struct, &mut context) {
            Ok(value) => value,
            Err(value) => return value,
        };

        context.dto_fragments = dto_fragments;

        if args.format.json {
            make_fragments_for_format_json(&mut context);
        }

        if args.format.url_encoded {
            make_fragments_for_format_url_encoded(&mut context);
        }

        make_new_struct_ast(item_struct, &context)
    }

    /// Makes AST fragments to work with target DTO
    fn make_dto_fragments(item_struct: &ItemStruct, context: &mut AllFragments) -> Result<DtoFragments, TokenStream> {
        let ident = &item_struct.ident;

        match &item_struct.fields {
            Fields::Unit => {
                Err(syn::Error::new_spanned(
                    item_struct.into_token_stream(),
                    format!("`{ident}`: RequestBody should not be an empty struct. Try making it an empty named struct: `struct {ident} {{}}`.")
                ).into_compile_error())
            }

            Fields::Named(_) => {
                Ok(DtoFragments {
                    schema: quote! { #ident::schema().1},
                    extract_ty: quote! { #ident},
                    pack_dto: quote! { dto },
                })
            },

            Fields::Unnamed(f) => {
                if f.unnamed.len() == 0 {
                    return Err(syn::Error::new_spanned(
                        item_struct.into_token_stream(),
                        format!("`{ident}`: RequestBody should not be an empty unnamed struct. Try making it an empty named struct: `struct {ident} {{}}`.")
                    ).into_compile_error());
                }

                if f.unnamed.len() > 1 {
                    return Err(syn::Error::new_spanned(
                        item_struct.into_token_stream(),
                        format!("`{ident}`: RequestBody should either have named fields or have a single unnamed field. Try making it a named struct: `struct {ident} {{...}}`.")
                    ).into_compile_error());
                }

                let first_field = f.unnamed.first().unwrap();
                let ty = &first_field.ty;

                context.type_assertions.push(quote! {
                    assert_impl_all!(#ty: ::groom::DTO);
                });

                Ok(DtoFragments {
                    schema: quote! { #ty::schema().1 },
                    extract_ty: quote! { #ty },
                    pack_dto: quote! { #ident(dto) },
                })
            }
        }
    }

    /// Generates final AST for RequestBody struct
    fn make_new_struct_ast(item_struct: ItemStruct, context: &AllFragments) -> TokenStream {
        let ident = &item_struct.ident;
        let vis = &item_struct.vis;

        let rejection_ident = &context.rejection_ident;
        let openapi_generators = &context.openapi_generators;
        let body_extractors = &context.body_extractors;
        let rejection_types = &context.rejection_types;
        let rejections_into_response = &context.rejections_into_response;
        let type_assertions = &context.type_assertions;
        let description_tk = &context.description_tk;

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
                    let content_type = ::groom::content_negotiation::parse_content_type_header(req.headers());

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

    /// Makes AST fragment to support application/x-www-form-urlencoded
    fn make_fragments_for_format_url_encoded(context: &mut AllFragments) {
        let rejection_ident = &context.rejection_ident;
        let schema = &context.dto_fragments.schema;
        let extract_ty = &context.dto_fragments.extract_ty;
        let pack_dto = &context.dto_fragments.pack_dto;

        context.body_extractors.push(quote! {
                Some(::groom::content_negotiation::BodyContentType::FormUrlEncoded) => {
                    let dto = ::axum::extract::Form::<#extract_ty>::from_request(req, state)
                        .await
                        .map_err(|e| #rejection_ident::FormRejection(e))?
                        .0
                    ;

                    Ok(#pack_dto)
                },
            });

        context.rejection_types.push(quote! {
                FormRejection(::axum::extract::rejection::FormRejection),
            });

        context.rejections_into_response.push(quote! {
                #rejection_ident::FormRejection(r) => r.into_response(),
            });

        context.openapi_generators.push(quote! {
                .content(
                    ::mime::APPLICATION_WWW_FORM_URLENCODED.as_ref(),
                    ::utoipa::openapi::ContentBuilder::new()
                        .schema(#schema)
                        .build()
                )
            });
    }

    /// Makes AST fragment to support application/json
    fn make_fragments_for_format_json(context: &mut AllFragments) {
        let rejection_ident = &context.rejection_ident;
        let schema = &context.dto_fragments.schema;
        let extract_ty = &context.dto_fragments.extract_ty;
        let pack_dto = &context.dto_fragments.pack_dto;

        context.body_extractors.push(quote! {
                Some(::groom::content_negotiation::BodyContentType::Json) => {
                    let dto = ::axum::extract::Json::<#extract_ty>::from_request(req, state)
                        .await
                        .map_err(|e| #rejection_ident::JsonRejection(e))?
                        .0
                    ;

                    Ok(#pack_dto)
                },
            });

        context.rejection_types.push(quote! {
                JsonRejection(::axum::extract::rejection::JsonRejection),
            });

        context.rejections_into_response.push(quote! {
                #rejection_ident::JsonRejection(r) => r.into_response(),
            });

        context.openapi_generators.push(quote! {
                .content(
                    ::mime::APPLICATION_JSON.as_ref(),
                    ::utoipa::openapi::ContentBuilder::new()
                        .schema(#schema)
                        .build()
                )
            });
    }
}

//
// endregion: AST parsing and generation -----------------------------------------------------------
