use axum::http::StatusCode;
use darling::FromMeta;
use syn::{Attribute, Error, Item};
use syn::parse2;
use strum_macros::Display;
use derive_more::{Deref, DerefMut};

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};

use crate::{annotation_attrs::{parse_attr, remove_attrs}, http::HTTPStatusCode};

// region: Annotations -----------------------------------------------------------------------------
//

/// `#[Response(...)]` annotation args
#[derive(FromMeta, Default, Clone)]
pub(crate) struct ResponseArgsBase {
    #[darling(default)]
    pub(crate) format: ResponseFormatsList,

    #[darling(default)]
    pub(crate) default_format: Option<ResponseFormat>,
}

/// `#[Response(...)]` annotation args for `enum`
#[derive(FromMeta)]
pub(crate) struct ResponseArgsEnum {
    #[darling(default, flatten)]
    pub(crate) base_args: ResponseArgsBase,
}

/// `#[Response(...)]` annotation args for `struct`
#[derive(FromMeta)]
pub(crate) struct ResponseArgsStruct {
    #[darling(default, flatten)]
    pub(crate) base_args: ResponseArgsBase,

    #[darling(default)]
    pub(crate) code: HTTPStatusCode,
}

/// `default_format` part from `#[Response(default_format="...")]`
#[derive(Debug, Copy, Clone, FromMeta, Eq, PartialEq, Hash, Display)]
#[darling(rename_all = "snake_case")]
#[strum(serialize_all="snake_case")]
pub(crate) enum ResponseFormat {
    PlainText,
    Html,
    Json,
}

/// `format` part from `#[Response(format(...))]`
#[derive(FromMeta, Default, Clone)]
pub(crate) struct ResponseFormatsList {
    #[darling(default)]
    pub(crate) plain_text: bool,

    #[darling(default)]
    pub(crate) html: bool,

    #[darling(default)]
    pub(crate) json: bool,
}


impl ResponseFormatsList {
    pub(crate) fn is_any(&self) -> bool {
        return self.plain_text || self.html || self.json;
    }

    pub(crate) fn count(&self) -> usize {
        let mut result = 0;

        if self.plain_text {
            result += 1;
        }

        if self.html {
            result += 1;
        }

        if self.json {
            result += 1;
        }

        result
    }

    pub(crate) fn get_single_value(&self) -> Option<ResponseFormat> {
        if self.count() != 1 {
            None
        } else if self.plain_text {
            Some(ResponseFormat::PlainText)
        } else if self.html {
            Some(ResponseFormat::Html)
        } else if self.json {
            Some(ResponseFormat::Json)
        } else {
            panic!("bug in ResponseFormatsList::count() or ResponseFormatsList::get_single_value()")
        }
    }

    pub(crate) fn has(&self, t: ResponseFormat) -> bool {
        match t {
            ResponseFormat::PlainText => self.plain_text,
            ResponseFormat::Html => self.html,
            ResponseFormat::Json => self.json,
        }
    }
}

/// `#[Response]` annotation for enum variants
#[derive(FromMeta)]
pub(crate) struct ResponseVariantAnnotation {
    #[darling(default)]
    pub(crate) code: HTTPStatusCode,
}

impl ResponseVariantAnnotation {
    pub(crate) fn parse_from_attrs(attrs: &[Attribute]) -> Result<Option<Self>, darling::Error> {
        parse_attr("Response", attrs)
    }

    pub(crate) fn remove_from_attrs(attrs: &mut Vec<Attribute>) {
        remove_attrs("Response", attrs)
    }
}


//
// endregion: Annotations --------------------------------------------------------------------------

// region: AST fragments ---------------------------------------------------------------------------
//

/// Any `#[Response]` is deconstructed into these fragments and then recreated.
struct NewAstFragments {
    item_ident: Ident,

    /// List of available content-types that can be produced from this Response (generated as a constant array):
    supported_mimes: SupportedMimesTokenStreams,
    supported_mimes_ident: Ident,

    formatter_functions: Vec<TokenStream>,

    into_response_any_content_type_ident: Ident,
    into_response_text_plain_ident: Ident,
    into_response_text_html_ident: Ident,
    into_response_application_json_ident: Ident,

    /// compile-time checks of trait implementation (for better error messages)
    type_assertions: Vec<TokenStream>,

    openapi_impls: Vec<TokenStream>,
    new_item_code: TokenStream,

    response_args: ResponseArgsBase,
    response_args_t: TokenStream,
}

impl NewAstFragments {
    fn new(ident: &Ident, response_args: ResponseArgsBase, response_args_t: TokenStream) -> Self {
        Self {
            item_ident: ident.clone(),

            supported_mimes: Default::default(),
            supported_mimes_ident: format_ident!("__GROOM_RESPONSE_SUPPORTED_MIMES_{ident}"),

            formatter_functions: Default::default(),

            // todo: find a nicer way to do this?
            into_response_any_content_type_ident: format_ident!("into_response_any_content_type"),
            into_response_text_plain_ident: format_ident!("into_response_text_plain"),
            into_response_text_html_ident: format_ident!("into_response_text_html"),
            into_response_application_json_ident: format_ident!("into_response_application_json"),

            type_assertions: Default::default(),
            openapi_impls: Default::default(),
            new_item_code: Default::default(),

            response_args,
            response_args_t,
        }
    }
}

/// These token streams are used to build global const vectors of supported MIME types
/// for each `#[Response]`.
///
/// The resulting global const vectors of MIME types are then used in runtime
/// to configure parsing of `Accept` header for handlers that return this `#[Response]`.
#[derive(Deref, DerefMut, Default)]
struct SupportedMimesTokenStreams(Vec<TokenStream>);

//
// endregion: AST fragments ------------------------------------------------------------------------

// region: AST parsing and generation --------------------------------------------------------------
//

/// Entry point for `#[Response]` macro.
pub(crate) fn generate(args_t: TokenStream, input: TokenStream) -> TokenStream {
    generate_implementation(args_t, input).unwrap_or_else(|t| t)
}

/// Generates appropriate implementation (based on input type).
fn generate_implementation(args_t: TokenStream, input: TokenStream)
    -> Result<TokenStream, TokenStream>
{
    let item = parse2::<Item>(input).map_err(
        |e| e.to_compile_error()
    )?;

    let fragments = match item {
        Item::Enum(item_enum) =>
            enum_impl::make_fragments_for_enum(item_enum, args_t),

        Item::Struct(struct_enum) =>
            struct_impl::make_fragments_for_struct(struct_enum, args_t),

        _ =>
            Err(Error::new_spanned(item, "Response should be an enum or a struct.").to_compile_error()),
    }?;

    make_new_ast(fragments)
}

/// Populates supported mimes for specified content types.
fn populate_supported_mimes(
    content_types: &ResponseFormatsList,
    supported_mimes: &mut SupportedMimesTokenStreams,
)
{
    if content_types.plain_text {
        supported_mimes.push(quote! {
            ::mime::TEXT_PLAIN,
        });
    }

    if content_types.html {
        supported_mimes.push(quote! {
            ::mime::TEXT_HTML,
        });
    }

    if content_types.json {
        supported_mimes.push(quote! {
            ::mime::APPLICATION_JSON,
        });
    }
}

/// Extracts response HTTP code from `#[Response(code = ...)]` annotation.
fn extract_response_code<T: ToTokens>(response_code: HTTPStatusCode, span: T) -> Result<(u16, TokenStream), TokenStream> {
    let response_code_u16 = response_code.0;
    let response_code_ts = match StatusCode::from_u16(response_code_u16) {
        Ok(code) => {
            let code = code.as_u16();
            quote! {
                    ::axum::http::StatusCode::from_u16(#code).unwrap()
                }
        },
        Err(e) => {
            return Err(
                syn::Error::new_spanned(
                    &span,
                    format!("error in `#[Response]` annotation: cannot parse response code `{}`: {e}", response_code_u16)
                ).to_compile_error()
            )
        },
    };

    Ok((response_code_u16, response_code_ts))
}


fn make_openapi_fragments_for_type(ty: TokenStream, description_tk: TokenStream, response_code_str: String, fragments: &mut NewAstFragments) {
    let mut response_impls: Vec<TokenStream> = Vec::new();
    let content_types = &fragments.response_args.format;

    if content_types.plain_text {
        response_impls.push(quote! {
            .content(
                ::mime::TEXT_PLAIN_UTF_8.as_ref(),
                ::utoipa::openapi::ContentBuilder::new()
                    .schema(<String as utoipa::PartialSchema>::schema())
                    //.example(Some("Hello, world!".into()))
                    .build()
            )
        });
    }
    if content_types.html {
        response_impls.push(quote! {
            .content(
                ::mime::TEXT_HTML_UTF_8.as_ref(),
                ::utoipa::openapi::ContentBuilder::new()
                    .schema(<String as utoipa::PartialSchema>::schema())
                    //.example(Some("<h1>Hello, world!</h1>".into()))
                    .build()
            )
        });
    }
    if content_types.json {
        response_impls.push(quote! {
            .content(
                ::mime::APPLICATION_JSON.as_ref(),
                ::utoipa::openapi::ContentBuilder::new()
                    .schema(#ty::schema().extract_schema())
                    .build()
            )
        });
    }

    fragments.openapi_impls.push(quote! {
        let op = op.response(
            #response_code_str,
            ::utoipa::openapi::ResponseBuilder::new()
                .description(#description_tk)
                #(#response_impls)*
                .build()
        );
    })
}

/// Assembles final AST
fn make_new_ast(fragments: NewAstFragments)
    -> Result<TokenStream, TokenStream>
{
    let resp_args = &fragments.response_args;
    let resp_args_span = &fragments.response_args_t;

    let supported_mimes_ident = &fragments.supported_mimes_ident;
    let supported_mimes = &fragments.supported_mimes;

    let groom_into_response_function =
        make_groom_into_response_function(
            &fragments,
            &resp_args,
            &resp_args_span
        )?;

    let formatter_functions = &fragments.formatter_functions;
    let type_assertions = &fragments.type_assertions;
    let openapi_impls = &fragments.openapi_impls;
    let new_item_code = &fragments.new_item_code;
    let item_ident = &fragments.item_ident;

    Ok(
        quote! {
            #new_item_code

            #[allow(non_upper_case_globals)]
            const #supported_mimes_ident: &[::mime::Mime] = &[
                #(#supported_mimes)*
            ];

            impl #item_ident {
                #(#formatter_functions)*
            }

            impl ::groom::response::Response for #item_ident {
                #groom_into_response_function

                fn __openapi_modify_operation(op: ::utoipa::openapi::path::OperationBuilder)
                    -> ::utoipa::openapi::path::OperationBuilder
                {
                    #(#openapi_impls)*
                    op
                }

                // todo: __groom_content_type_supported
            }

            #(#type_assertions)*
        }
    )
}

/// Makes `::groom::response::Response::__groom_into_response()` - the main function that  performs
/// content negotiation and converts this response's data into appropriate response headers & body.
fn make_groom_into_response_function(
    fragments: &NewAstFragments,
    resp_args: &ResponseArgsBase,
    resp_args_span: &TokenStream
) -> Result<TokenStream, TokenStream>
{
    let item_ident = &fragments.item_ident;
    let supported_mimes_ident = &fragments.supported_mimes_ident;

    let fn_ident_for_any_content = &fragments.into_response_any_content_type_ident;
    let fn_ident_for_text_plain = &fragments.into_response_text_plain_ident;
    let fn_ident_for_text_html = &fragments.into_response_text_html_ident;
    let fn_ident_for_application_json = &fragments.into_response_application_json_ident;

    let default_format =
        detect_default_format(&fragments.item_ident, &resp_args, &resp_args_span)?;

    let content_type_negotiation = if !resp_args.format.is_any() {
        quote! {
            self.#fn_ident_for_any_content()
        }
    } else {
        let default_content_response = if let Some(default_format) = default_format {
            match default_format {
                ResponseFormat::PlainText => quote! { self.#fn_ident_for_text_plain() },
                ResponseFormat::Html => quote! { self.#fn_ident_for_text_html() },
                ResponseFormat::Json => quote! { self.#fn_ident_for_application_json() },
                // todo: option to force BadRequest response if client hasn't specified any format in Accept header
            }
        } else {
            return Err(syn::Error::new_spanned(
                resp_args_span,
                format!("cannot infer default_format for `{item_ident}` - this is a bug in groom, please report it; thank you~")
            ).into_compile_error());
        };

        let mime_type_matches = make_mime_types_matches_for_content_negotiation(
            &resp_args,
            &fragments
        );

        quote! {
            match accept {
                None => {
                    // no Accept header found
                    #default_content_response
                },
                Some(accept) => {
                    // some Accept header found
                    // #available_mimes_ident is a const array with a list of supported mime types for this enum
                    match accept.negotiate(&#supported_mimes_ident) {
                        Err(_) => {
                            // todo: is this response body good enough?
                            (::axum::http::StatusCode::BAD_REQUEST, "Requested Content-Type is not supported.").into_response()
                        },

                        Ok(negotiated) => {
                            // todo: this match's arms should not include formats that aren't valid for this enum
                            match (negotiated.type_(), negotiated.subtype()) {
                                #(#mime_type_matches)*

                                _ => {
                                    // todo: somehow log this error?
                                    (::axum::http::StatusCode::BAD_REQUEST, "Content-Type negotiation produced an unexpected type/subtype pair.")
                                        .into_response()
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    Ok(quote!{
        fn __groom_into_response(self, accept: Option<::accept_header::Accept>) -> ::axum::response::Response {
            #content_type_negotiation
        }
    })
}

/// Attempts to infer `default_format` value of `#[Response]` annotation if it is applicable.
fn detect_default_format(
    ident: &Ident,
    resp_args: &ResponseArgsBase,
    resp_args_span: &TokenStream
) -> Result<Option<ResponseFormat>, TokenStream> {
    Ok(
        if !resp_args.format.is_any() {
            None
        } else {
            let default_format = resp_args.default_format.map_or_else(
                || resp_args.format.get_single_value(),
                |val| Some(val),
            );

            if default_format.is_none() {
                return Err(syn::Error::new_spanned(
                    resp_args_span,
                    format!("error in `#[Response]` annotation: specify default_format for `{ident}` (e.g. #[Response(default_format=\"json\")])")
                ).into_compile_error());
            } else if !resp_args.format.has(default_format.unwrap()) {
                return Err(syn::Error::new_spanned(
                    resp_args_span,
                    format!("error in `#[Response]` annotation: default_format `{}` of `{ident}` is not mentioned in it's formats list", default_format.unwrap())
                ).into_compile_error());
            }

            default_format
        }
    )
}

/// Makes fragments of MIME type matching for calling appropriate `into_response_*` functions.
fn make_mime_types_matches_for_content_negotiation(
    resp_args: &ResponseArgsBase,
    fragments: &NewAstFragments
) -> Vec<TokenStream>
{
    let mut result = Vec::with_capacity(3);

    if resp_args.format.plain_text {
        let formatter = &fragments.into_response_text_plain_ident;
        result.push(quote! {
            (::mime::TEXT, mime::PLAIN) => self.#formatter(),
        });
    }

    if resp_args.format.html {
        let formatter = &fragments.into_response_text_html_ident;
        result.push(quote! {
            (::mime::TEXT, mime::HTML) => self.#formatter(),
        });
    }

    if resp_args.format.json {
        let formatter = &fragments.into_response_application_json_ident;
        result.push(quote! {
            (::mime::APPLICATION, mime::JSON) => self.#formatter(),
        });
    }

    result
}

/// `#[Response]` generation for `enum`.
mod enum_impl {
    use darling::FromMeta;
    use proc_macro2::{Ident, TokenStream};
    use syn::{Attribute, Field, ItemEnum, Variant};
    use quote::{quote, ToTokens};

    use crate::comments::get_docblock;
    use crate::parse_nested_meta;
    use crate::response;
    use crate::response::{extract_response_code, make_openapi_fragments_for_type, NewAstFragments, populate_supported_mimes, ResponseFormatsList, ResponseVariantAnnotation};

    /// Each enum variant produces a list of matchers for each supported content type.
    /// Each matcher calls an appropriate `into_response_*` function
    /// (see `make_formatter_functions()`).
    #[derive(Default)]
    struct EnumMatchers {
        match_enum_when_no_accept_header_found: Vec<TokenStream>,
        match_enum_for_text_plain: Vec<TokenStream>,
        match_enum_for_text_html: Vec<TokenStream>,
        match_enum_for_application_json: Vec<TokenStream>,
    }

    /// Entry point for generation of `#[Response]` code for `enum`.
    pub(crate) fn make_fragments_for_enum(enum_impl: ItemEnum, args: TokenStream) -> Result<NewAstFragments, TokenStream> {
        let resp_args = parse_nested_meta!(response::ResponseArgsEnum, &args)?;

        let ident = &enum_impl.ident;

        let mut variants_ts: Vec<TokenStream> = Vec::new();   // variants of output enum

        let mut fragments = NewAstFragments::new(ident, resp_args.base_args, args);
        let mut matchers = EnumMatchers::default();

        populate_supported_mimes(
            &fragments.response_args.format,
            &mut fragments.supported_mimes,
        );

        for mut variant in enum_impl.variants {
            let variant_annotation = extract_variant_annotation(&mut variant)?;

            let (response_code_u16, response_code_ts) =
                extract_response_code(variant_annotation.code, &variant)?;

            let response_body_field = extract_response_body_field(&variant)?;

            populate_matchers(
                &variant.ident,
                &response_body_field,
                &response_code_ts,
                &fragments.response_args.format,
                &mut matchers,
            )?;

            populate_openapi_impls(
                &response_body_field,
                response_code_u16,
                &variant.attrs,
                &mut fragments,
            );

            variants_ts.push(quote! { #variant, });
        }

        make_formatter_functions(&matchers, &mut fragments);

        let vis = &enum_impl.vis;
        fragments.new_item_code = quote! {
            #vis enum #ident {
                #(#variants_ts)*
            }
        };

        Ok(fragments)
    }

    /// Extracts `#[Response()]` annotation from enum variant.
    fn extract_variant_annotation(variant: &mut Variant) -> Result<ResponseVariantAnnotation, TokenStream> {
        match ResponseVariantAnnotation::parse_from_attrs(&variant.attrs) {
            Ok(Some(annotation)) => {
                ResponseVariantAnnotation::remove_from_attrs(&mut variant.attrs);
                Ok(annotation)
            },

            Ok(None) => Err(syn::Error::new_spanned(
                variant.into_token_stream(),
                "error in `#[Response]` annotation: each enum variant should be annotated with `#[Response()]`"
            ).to_compile_error()),

            Err(error) => Err(error.write_errors()),
        }
    }

    /// If `variant` has a field that can be used as a response body, extracts this field AST.
    fn extract_response_body_field(variant: &Variant) -> Result<Option<&Field>, TokenStream> {
        match &variant.fields {
            syn::Fields::Named(fields) => {
                return Err(syn::Error::new_spanned(fields, "error in `#[Response]` annotation: named fields are not supported").into_compile_error());
                // todo: support something like http::response::Parts
            },

            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    return Err(
                        syn::Error::new_spanned(fields, "error in `#[Response]` annotation: only exactly one unnamed field is supported").into_compile_error()
                    );
                }

                Ok(Some(fields.unnamed.first().expect("length is checked right above")))
            },

            syn::Fields::Unit =>
                Ok(None),
        }
    }

    /// Makes matchers for content type negotiation.
    fn populate_matchers(
        variant_ident: &Ident,
        response_body_field: &Option<&Field>,
        response_code_ts: &TokenStream,
        content_types: &ResponseFormatsList,
        matchers: &mut EnumMatchers,
    ) -> Result<(), TokenStream>
    {
        // Let's make enum variants conversion for when no Accept header was supplied by the client.
        if !content_types.is_any() {
            // If no formats were specified for this Response, we need to ensure that we:
            //  - either have only variants without fields in this enum and respond only with HTTP codes,
            //  - or raise a compile error telling the dev that they need to specify some response format.
            matchers.match_enum_when_no_accept_header_found.push(match &response_body_field {
                None =>
                    quote! {
                            Self::#variant_ident => (#response_code_ts).into_response(),
                        },

                Some(single_field) => {
                    return Err(
                        syn::Error::new_spanned(
                            single_field,
                            format!("error in `#[Response]` annotation: specify at least one response format for enum `{variant_ident}` to be able to return data through it's variant `{variant_ident}` (e.g. #[Response(format(json))])")
                        ).into_compile_error()
                    );
                }
            });
        }

        if content_types.plain_text {
            matchers.match_enum_for_text_plain.push(match &response_body_field {
                None =>
                    quote! {
                        Self::#variant_ident => (#response_code_ts).into_response(),
                    },

                Some(_single_field) =>
                    quote! {
                        Self::#variant_ident(body) => (
                            #response_code_ts,
                            Into::<String>::into(body)
                        ).into_response(),
                    },
            });
        }

        if content_types.html {
            matchers.match_enum_for_text_html.push(match &response_body_field {
                None =>
                    quote! {
                        Self::#variant_ident => (#response_code_ts).into_response(),
                    },

                Some(single_field) => {
                    let ty = &single_field.ty;

                    quote! {
                        Self::#variant_ident(body) => (
                            #response_code_ts,
                            <#ty as ::groom::response::HtmlFormat>::render(body)
                        ).into_response(),
                    }
                },
            });
        }

        if content_types.json {
            matchers.match_enum_for_application_json.push(match &response_body_field {
                None =>
                    quote! {
                        Self::#variant_ident => (#response_code_ts).into_response(),
                    },

                Some(_single_field) =>
                    quote! {
                        Self::#variant_ident(body) => (
                            #response_code_ts,
                            ::axum::Json(body)
                        ).into_response(),
                    },
            });
        }

        Ok(())
    }

    /// Makes code to set up OpenAPI spec generation for this enum field.
    fn populate_openapi_impls(
        response_body_field: &Option<&Field>,
        response_code_u16: u16,
        variant_attributes: &Vec<Attribute>,
        fragments: &mut NewAstFragments,
    )
    {
        let description_tk = match get_docblock(&variant_attributes).unwrap_or_default() {
            Some(s) => quote! { #s },
            None => quote! { "" },
        };

        let response_code_str = format!("{response_code_u16}");

        match &response_body_field {
            None => {
                fragments.openapi_impls.push(quote! {
                        let op = op.response(
                            #response_code_str,
                            ::utoipa::openapi::ResponseBuilder::new()
                                .description(#description_tk)
                                .build()
                        );
                    });
            },

            Some(single_field) => {
                let ty = &single_field.ty;

                fragments.type_assertions.push(quote! {
                    assert_impl_any!(#ty: ::utoipa::PartialSchema, ::groom::DTO_Response);
                });

                make_openapi_fragments_for_type(quote!{#ty}, description_tk, response_code_str, fragments);
            },
        };
    }


    /// Makes implementations of `into_response_*` formatters for all required content-types.
    fn make_formatter_functions(matchers: &EnumMatchers, fragments: &mut NewAstFragments) {
        let resp_args = &fragments.response_args;

        if !resp_args.format.is_any() {
            let formatter = &fragments.into_response_any_content_type_ident;
            let matcher = &matchers.match_enum_when_no_accept_header_found;
            fragments.formatter_functions.push(quote! {
                fn #formatter(self) -> ::axum::response::Response {
                    match self {
                        #(#matcher)*
                    }
                }
            });
        };

        if resp_args.format.plain_text {
            let formatter = &fragments.into_response_text_plain_ident;
            let matcher = &matchers.match_enum_for_text_plain;
            fragments.formatter_functions.push(quote! {
                fn #formatter(self) -> ::axum::response::Response {
                    match self {
                        #(#matcher)*
                    }
                }
            });
        }

        if resp_args.format.html {
            let formatter = &fragments.into_response_text_html_ident;
            let matcher = &matchers.match_enum_for_text_html;
            fragments.formatter_functions.push(quote! {
                fn #formatter(self) -> ::axum::response::Response {
                    match self {
                        #(#matcher)*
                    }
                }
            });
        }

        if resp_args.format.json {
            let formatter = &fragments.into_response_application_json_ident;
            let matcher = &matchers.match_enum_for_application_json;
            fragments.formatter_functions.push(quote! {
                fn #formatter(self) -> ::axum::response::Response {
                    match self {
                        #(#matcher)*
                    }
                }
            });
        }
    }
}


/// `#[Response]` generation for `struct`.
mod struct_impl {
    use darling::FromMeta;
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{Fields, ItemStruct};
    use crate::{parse_nested_meta, response};
    use crate::comments::get_docblock;
    use crate::response::{extract_response_code, make_openapi_fragments_for_type, NewAstFragments, populate_supported_mimes, ResponseArgsStruct};

    pub(crate) fn make_fragments_for_struct(struct_impl: ItemStruct, args: TokenStream) -> Result<NewAstFragments, TokenStream> {
        let resp_args = parse_nested_meta!(response::ResponseArgsStruct, &args)?;
        let mut fragments = NewAstFragments::new(&struct_impl.ident, resp_args.base_args.clone(), args);

        fragments.new_item_code = quote! {
            #[DTO(response)]
            #struct_impl
        };

        populate_supported_mimes(
            &resp_args.base_args.format,
            &mut fragments.supported_mimes,
        );

        make_formatter_functions(&resp_args, &struct_impl, &mut fragments)?;

        populate_openapi_impls(
            &struct_impl,
            resp_args.code.0,
            &mut fragments
        );

        Ok(fragments)
    }

    fn make_formatter_functions(resp_args: &ResponseArgsStruct, struct_impl: &ItemStruct, fragments: &mut NewAstFragments) -> Result<(), TokenStream> {
        let (_, response_code_ts) = extract_response_code(resp_args.code, &struct_impl)?;

        let ident = &struct_impl.ident;
        let base_args = &resp_args.base_args;

        if let Fields::Unit = struct_impl.fields {
            if base_args.format.is_any() {
                return Err(
                    syn::Error::new_spanned(
                        struct_impl,
                        format!("error in `#[Response]` annotation: no response formats are allowed for struct `{ident}` because it has no fields to output in any format; remove format definition (e.g. #[Response(code = 418)])")
                    ).into_compile_error()
                );
            }
        }

        if !base_args.format.is_any() {
            let formatter = &fragments.into_response_any_content_type_ident;

            let resp = match struct_impl.fields {
                Fields::Named(_) | Fields::Unnamed(_) => {
                    return Err(
                        syn::Error::new_spanned(
                            struct_impl,
                            format!("error in `#[Response]` annotation: specify at least one response format for struct `{ident}` to be able to return data through it (e.g. #[Response(format(json))])")
                        ).into_compile_error()
                    );
                }
                Fields::Unit => {
                    quote! {(#response_code_ts).into_response()}
                }
            };

            fragments.formatter_functions.push(quote! {
                fn #formatter(self) -> ::axum::response::Response {
                    #resp
                }
            });
        };

        if base_args.format.plain_text {
            let formatter = &fragments.into_response_text_plain_ident;

            let resp = match &struct_impl.fields {
                Fields::Named(_) => {
                    quote!{
                        (
                            #response_code_ts,
                            Into::<String>::into(self)
                        ).into_response()
                    }
                },
                Fields::Unnamed(f) => {
                    if f.unnamed.is_empty() {
                        return Err(
                            syn::Error::new_spanned(
                                struct_impl,
                                format!("error in `#[Response]` annotation: no fields are specified for struct `{ident}`. Please remove the parentheses (e.g. `struct {ident};`)")
                            ).into_compile_error()
                        );
                    }

                    if f.unnamed.len() > 1 {
                        return Err(
                            syn::Error::new_spanned(
                                struct_impl,
                                format!("error in `#[Response]` annotation: more then one unnamed field is specified for struct `{ident}`. Please remove the extra fields or give them names (e.g. `struct {ident} {{ field_0: String, ... }};`)")
                            ).into_compile_error()
                        );
                    }

                    quote! {
                        (
                            #response_code_ts,
                            Into::<String>::into(self.0)
                        ).into_response()
                    }
                },
                Fields::Unit => {
                    quote! {(#response_code_ts).into_response()}
                }
            };

            fragments.formatter_functions.push(quote! {
                fn #formatter(self) -> ::axum::response::Response {
                    #resp
                }
            });
        }

        if base_args.format.html {
            let formatter = &fragments.into_response_text_html_ident;
            fragments.formatter_functions.push(quote! {
                fn #formatter(self) -> ::axum::response::Response {
                    (
                        #response_code_ts,
                        <#ident as ::groom::response::HtmlFormat>::render(self)
                    ).into_response()
                }
            });
        }

        if base_args.format.json {
            let formatter = &fragments.into_response_application_json_ident;
            fragments.formatter_functions.push(quote! {
                fn #formatter(self) -> ::axum::response::Response {
                    (
                        #response_code_ts,
                        ::axum::Json(self)
                    ).into_response()
                }
            });
        }

        Ok(())
    }

    /// Makes code to set up OpenAPI spec generation for this struct.
    fn populate_openapi_impls(
        struct_impl: &ItemStruct,
        response_code_u16: u16,
        fragments: &mut NewAstFragments,
    )
    {
        let description_tk = match get_docblock(&struct_impl.attrs).unwrap_or_default() {
            Some(s) => quote! { #s },
            None => quote! { "" },
        };

        let response_code_str = format!("{response_code_u16}");

        match &struct_impl.fields {
            Fields::Unit => {
                fragments.openapi_impls.push(quote! {
                    let op = op.response(
                        #response_code_str,
                        ::utoipa::openapi::ResponseBuilder::new()
                            .description(#description_tk)
                            .build()
                    );
                });
            }

            Fields::Unnamed(f) => {
                let single_field = f.unnamed.first().expect("fields count is checked in make_formatter_functions()");

                let ty = &single_field.ty;
                fragments.type_assertions.push(quote! {
                    assert_impl_any!(#ty: ::utoipa::PartialSchema, ::groom::DTO_Response);
                });

                make_openapi_fragments_for_type(quote!{#ty}, description_tk, response_code_str, fragments);
            }

            Fields::Named(_) => {
                let ty = &struct_impl.ident;
                fragments.type_assertions.push(quote! {
                    assert_impl_any!(#ty: ::utoipa::PartialSchema, ::groom::DTO_Response);
                });

                make_openapi_fragments_for_type(quote!{#ty}, description_tk, response_code_str, fragments);
            }
        }
    }
}

//
// endregion: AST parsing and generation -----------------------------------------------------------
