//! This is expansion preview for #[Response] annotation.
//! Each case is put into its own `mod` to make it easier to inspect expansion result.
#[macro_use]
extern crate groom_macros;
mod no_content_type {
    pub enum RespJsonResponse {
        Accepted,
        NotFound,
    }
    #[allow(non_upper_case_globals)]
    const __GROOM_RESPONSE_SUPPORTED_MIMES_RespJsonResponse: &[::mime::Mime] = &[];
    impl RespJsonResponse {
        fn into_response_any_content_type(self) -> ::axum::response::Response {
            match self {
                Self::Accepted => {
                    (match 202u16 {
                        200u16 => ::axum::http::StatusCode::OK,
                        201u16 => ::axum::http::StatusCode::CREATED,
                        202u16 => ::axum::http::StatusCode::ACCEPTED,
                        203u16 => ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION,
                        204u16 => ::axum::http::StatusCode::NO_CONTENT,
                        205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                        206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                        207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                        208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                        226u16 => ::axum::http::StatusCode::IM_USED,
                        300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                        301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                        302u16 => ::axum::http::StatusCode::FOUND,
                        303u16 => ::axum::http::StatusCode::SEE_OTHER,
                        304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                        307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                        308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                        400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                        401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                        402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                        403u16 => ::axum::http::StatusCode::FORBIDDEN,
                        404u16 => ::axum::http::StatusCode::NOT_FOUND,
                        405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                        406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                        407u16 => ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED,
                        408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                        409u16 => ::axum::http::StatusCode::CONFLICT,
                        410u16 => ::axum::http::StatusCode::GONE,
                        411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                        412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                        413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                        414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                        415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                        416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                        417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                        418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                        421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                        422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                        423u16 => ::axum::http::StatusCode::LOCKED,
                        424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                        425u16 => ::axum::http::StatusCode::TOO_EARLY,
                        426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                        428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                        429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                        431u16 => {
                            ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE
                        }
                        451u16 => ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                        500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                        502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                        503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                        504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                        505u16 => ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED,
                        506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                        507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                        508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                        510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                        511u16 => {
                            ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED
                        }
                        _ => {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "internal error: entered unreachable code: {0}",
                                    format_args!("groom: status code {0} was validated at expand time",
                                    202u16,),
                                ),
                            );
                        }
                    })
                        .into_response()
                }
                Self::NotFound => {
                    (match 404u16 {
                        200u16 => ::axum::http::StatusCode::OK,
                        201u16 => ::axum::http::StatusCode::CREATED,
                        202u16 => ::axum::http::StatusCode::ACCEPTED,
                        203u16 => ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION,
                        204u16 => ::axum::http::StatusCode::NO_CONTENT,
                        205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                        206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                        207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                        208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                        226u16 => ::axum::http::StatusCode::IM_USED,
                        300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                        301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                        302u16 => ::axum::http::StatusCode::FOUND,
                        303u16 => ::axum::http::StatusCode::SEE_OTHER,
                        304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                        307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                        308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                        400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                        401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                        402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                        403u16 => ::axum::http::StatusCode::FORBIDDEN,
                        404u16 => ::axum::http::StatusCode::NOT_FOUND,
                        405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                        406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                        407u16 => ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED,
                        408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                        409u16 => ::axum::http::StatusCode::CONFLICT,
                        410u16 => ::axum::http::StatusCode::GONE,
                        411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                        412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                        413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                        414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                        415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                        416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                        417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                        418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                        421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                        422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                        423u16 => ::axum::http::StatusCode::LOCKED,
                        424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                        425u16 => ::axum::http::StatusCode::TOO_EARLY,
                        426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                        428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                        429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                        431u16 => {
                            ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE
                        }
                        451u16 => ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                        500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                        502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                        503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                        504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                        505u16 => ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED,
                        506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                        507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                        508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                        510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                        511u16 => {
                            ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED
                        }
                        _ => {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "internal error: entered unreachable code: {0}",
                                    format_args!("groom: status code {0} was validated at expand time",
                                    404u16,),
                                ),
                            );
                        }
                    })
                        .into_response()
                }
            }
        }
    }
    impl ::groom::response::Response for RespJsonResponse {
        fn __groom_into_response(
            self,
            negotiated: Option<&::mime::Mime>,
        ) -> ::axum::response::Response {
            self.into_response_any_content_type()
        }
        fn __openapi_modify_operation(
            op: ::utoipa::openapi::path::OperationBuilder,
            components: &mut ::groom::extract::ComponentsRegistry,
        ) -> ::utoipa::openapi::path::OperationBuilder {
            let op = op
                .response(
                    "202",
                    ::utoipa::openapi::ResponseBuilder::new().description("").build(),
                );
            let op = op
                .response(
                    "404",
                    ::utoipa::openapi::ResponseBuilder::new().description("").build(),
                );
            op
        }
        fn __groom_negotiate_content_type(
            _accept: &::accept_header::Accept,
        ) -> ::core::result::Result<Option<::mime::Mime>, ::axum::response::Response> {
            Ok(None)
        }
        fn __groom_check_response_codes(
            context: &str,
            codes: &mut ::groom::runtime_checks::HTTPCodeSet,
        ) {
            let context = ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0} / enum `RespJsonResponse`", context),
                )
            });
            codes
                .ensure_distinct(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("{0} / variant `Accepted`", context),
                        )
                    }),
                    202u16,
                );
            codes
                .ensure_distinct(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("{0} / variant `NotFound`", context),
                        )
                    }),
                    404u16,
                );
        }
        fn __groom_check_response_formats(
            context: &str,
            formats: &mut ::groom::runtime_checks::HTTPFormatsSet,
        ) {
            formats.record(context, &__GROOM_RESPONSE_SUPPORTED_MIMES_RespJsonResponse);
        }
    }
}
mod plaintext_only {
    pub enum RespPlaintextResponse {
        Ok(String),
        NotFound,
    }
    #[allow(non_upper_case_globals)]
    const __GROOM_RESPONSE_SUPPORTED_MIMES_RespPlaintextResponse: &[::mime::Mime] = &[
        ::mime::TEXT_PLAIN,
    ];
    impl RespPlaintextResponse {
        fn into_response_text_plain(self) -> ::axum::response::Response {
            match self {
                Self::Ok(body) => {
                    (
                        match 200u16 {
                            200u16 => ::axum::http::StatusCode::OK,
                            201u16 => ::axum::http::StatusCode::CREATED,
                            202u16 => ::axum::http::StatusCode::ACCEPTED,
                            203u16 => {
                                ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION
                            }
                            204u16 => ::axum::http::StatusCode::NO_CONTENT,
                            205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                            206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                            207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                            208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                            226u16 => ::axum::http::StatusCode::IM_USED,
                            300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                            301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                            302u16 => ::axum::http::StatusCode::FOUND,
                            303u16 => ::axum::http::StatusCode::SEE_OTHER,
                            304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                            307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                            308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                            400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                            401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                            402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                            403u16 => ::axum::http::StatusCode::FORBIDDEN,
                            404u16 => ::axum::http::StatusCode::NOT_FOUND,
                            405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                            406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                            407u16 => {
                                ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED
                            }
                            408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                            409u16 => ::axum::http::StatusCode::CONFLICT,
                            410u16 => ::axum::http::StatusCode::GONE,
                            411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                            412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                            413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                            414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                            415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                            416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                            417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                            418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                            421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                            422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                            423u16 => ::axum::http::StatusCode::LOCKED,
                            424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                            425u16 => ::axum::http::StatusCode::TOO_EARLY,
                            426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                            428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                            429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                            431u16 => {
                                ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE
                            }
                            451u16 => {
                                ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS
                            }
                            500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                            502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                            503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                            504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                            505u16 => {
                                ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED
                            }
                            506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                            507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                            508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                            510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                            511u16 => {
                                ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED
                            }
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("groom: status code {0} was validated at expand time",
                                        200u16,),
                                    ),
                                );
                            }
                        },
                        Into::<String>::into(body),
                    )
                        .into_response()
                }
                Self::NotFound => {
                    (match 404u16 {
                        200u16 => ::axum::http::StatusCode::OK,
                        201u16 => ::axum::http::StatusCode::CREATED,
                        202u16 => ::axum::http::StatusCode::ACCEPTED,
                        203u16 => ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION,
                        204u16 => ::axum::http::StatusCode::NO_CONTENT,
                        205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                        206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                        207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                        208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                        226u16 => ::axum::http::StatusCode::IM_USED,
                        300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                        301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                        302u16 => ::axum::http::StatusCode::FOUND,
                        303u16 => ::axum::http::StatusCode::SEE_OTHER,
                        304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                        307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                        308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                        400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                        401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                        402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                        403u16 => ::axum::http::StatusCode::FORBIDDEN,
                        404u16 => ::axum::http::StatusCode::NOT_FOUND,
                        405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                        406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                        407u16 => ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED,
                        408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                        409u16 => ::axum::http::StatusCode::CONFLICT,
                        410u16 => ::axum::http::StatusCode::GONE,
                        411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                        412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                        413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                        414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                        415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                        416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                        417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                        418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                        421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                        422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                        423u16 => ::axum::http::StatusCode::LOCKED,
                        424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                        425u16 => ::axum::http::StatusCode::TOO_EARLY,
                        426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                        428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                        429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                        431u16 => {
                            ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE
                        }
                        451u16 => ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                        500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                        502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                        503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                        504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                        505u16 => ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED,
                        506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                        507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                        508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                        510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                        511u16 => {
                            ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED
                        }
                        _ => {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "internal error: entered unreachable code: {0}",
                                    format_args!("groom: status code {0} was validated at expand time",
                                    404u16,),
                                ),
                            );
                        }
                    })
                        .into_response()
                }
            }
        }
    }
    impl ::groom::response::Response for RespPlaintextResponse {
        fn __groom_into_response(
            self,
            negotiated: Option<&::mime::Mime>,
        ) -> ::axum::response::Response {
            match negotiated {
                None => self.into_response_text_plain(),
                Some(negotiated) => {
                    match (negotiated.type_(), negotiated.subtype()) {
                        (::mime::TEXT, ::mime::PLAIN) => self.into_response_text_plain(),
                        _ => {
                            if true {
                                if !false {
                                    {
                                        ::core::panicking::panic_fmt(
                                            format_args!(
                                                "groom: negotiated mime not covered by response arms",
                                            ),
                                        );
                                    }
                                }
                            }
                            (
                                ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                "internal server error",
                            )
                                .into_response()
                        }
                    }
                }
            }
        }
        fn __openapi_modify_operation(
            op: ::utoipa::openapi::path::OperationBuilder,
            components: &mut ::groom::extract::ComponentsRegistry,
        ) -> ::utoipa::openapi::path::OperationBuilder {
            let op = op
                .response(
                    "200",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("")
                        .content(
                            ::mime::TEXT_PLAIN_UTF_8.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(
                                    Some(
                                        ::groom::extract::ComponentsRegistry::schema_or_ref::<
                                            String,
                                        >(components),
                                    ),
                                )
                                .build(),
                        )
                        .build(),
                );
            let op = op
                .response(
                    "404",
                    ::utoipa::openapi::ResponseBuilder::new().description("").build(),
                );
            let op = op
                .response(
                    "406",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("The requested content type is not supported")
                        .content(
                            ::mime::TEXT_PLAIN.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(
                                    Some(
                                        ::groom::extract::ComponentsRegistry::schema_or_ref::<
                                            String,
                                        >(components),
                                    ),
                                )
                                .build(),
                        )
                        .build(),
                );
            op
        }
        fn __groom_negotiate_content_type(
            accept: &::accept_header::Accept,
        ) -> ::core::result::Result<Option<::mime::Mime>, ::axum::response::Response> {
            match accept
                .negotiate(&__GROOM_RESPONSE_SUPPORTED_MIMES_RespPlaintextResponse)
            {
                Ok(negotiated) => Ok(Some(negotiated)),
                Err(_) => {
                    Err(
                        ::groom::response::not_acceptable(
                            __GROOM_RESPONSE_SUPPORTED_MIMES_RespPlaintextResponse,
                        ),
                    )
                }
            }
        }
        fn __groom_check_response_codes(
            context: &str,
            codes: &mut ::groom::runtime_checks::HTTPCodeSet,
        ) {
            let context = ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0} / enum `RespPlaintextResponse`", context),
                )
            });
            codes
                .ensure_distinct(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0} / variant `Ok`", context))
                    }),
                    200u16,
                );
            codes
                .ensure_distinct(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("{0} / variant `NotFound`", context),
                        )
                    }),
                    404u16,
                );
        }
        fn __groom_check_response_formats(
            context: &str,
            formats: &mut ::groom::runtime_checks::HTTPFormatsSet,
        ) {
            formats
                .record(
                    context,
                    &__GROOM_RESPONSE_SUPPORTED_MIMES_RespPlaintextResponse,
                );
        }
    }
}
mod html_only {
    use groom::response::html_format;
    use groom::response::HtmlFormat;
    pub struct Struct {
        success: bool,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Struct {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Struct",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "success",
                    &self.success,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    impl ::groom::DTO for Struct {}
    impl ::groom::DTO_Response for Struct {}
    impl ::groom::response::HtmlFormat for Struct {
        fn render(self) -> ::axum::response::Html<axum::body::Body> {
            ::axum::response::Html(
                if self.success {
                    "<span style=\"color: #a3be8c;\">success</span>"
                } else {
                    "<span style=\"color: #bf616a;\">error</span>"
                }
                    .into(),
            )
        }
    }
    pub enum RespHtmlResponse {
        Ok(Struct),
        NotFound,
    }
    #[allow(non_upper_case_globals)]
    const __GROOM_RESPONSE_SUPPORTED_MIMES_RespHtmlResponse: &[::mime::Mime] = &[
        ::mime::TEXT_HTML,
    ];
    impl RespHtmlResponse {
        fn into_response_text_html(self) -> ::axum::response::Response {
            match self {
                Self::Ok(body) => {
                    (
                        match 200u16 {
                            200u16 => ::axum::http::StatusCode::OK,
                            201u16 => ::axum::http::StatusCode::CREATED,
                            202u16 => ::axum::http::StatusCode::ACCEPTED,
                            203u16 => {
                                ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION
                            }
                            204u16 => ::axum::http::StatusCode::NO_CONTENT,
                            205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                            206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                            207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                            208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                            226u16 => ::axum::http::StatusCode::IM_USED,
                            300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                            301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                            302u16 => ::axum::http::StatusCode::FOUND,
                            303u16 => ::axum::http::StatusCode::SEE_OTHER,
                            304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                            307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                            308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                            400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                            401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                            402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                            403u16 => ::axum::http::StatusCode::FORBIDDEN,
                            404u16 => ::axum::http::StatusCode::NOT_FOUND,
                            405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                            406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                            407u16 => {
                                ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED
                            }
                            408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                            409u16 => ::axum::http::StatusCode::CONFLICT,
                            410u16 => ::axum::http::StatusCode::GONE,
                            411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                            412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                            413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                            414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                            415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                            416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                            417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                            418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                            421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                            422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                            423u16 => ::axum::http::StatusCode::LOCKED,
                            424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                            425u16 => ::axum::http::StatusCode::TOO_EARLY,
                            426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                            428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                            429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                            431u16 => {
                                ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE
                            }
                            451u16 => {
                                ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS
                            }
                            500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                            502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                            503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                            504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                            505u16 => {
                                ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED
                            }
                            506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                            507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                            508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                            510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                            511u16 => {
                                ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED
                            }
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("groom: status code {0} was validated at expand time",
                                        200u16,),
                                    ),
                                );
                            }
                        },
                        <Struct as ::groom::response::HtmlFormat>::render(body),
                    )
                        .into_response()
                }
                Self::NotFound => {
                    (match 404u16 {
                        200u16 => ::axum::http::StatusCode::OK,
                        201u16 => ::axum::http::StatusCode::CREATED,
                        202u16 => ::axum::http::StatusCode::ACCEPTED,
                        203u16 => ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION,
                        204u16 => ::axum::http::StatusCode::NO_CONTENT,
                        205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                        206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                        207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                        208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                        226u16 => ::axum::http::StatusCode::IM_USED,
                        300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                        301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                        302u16 => ::axum::http::StatusCode::FOUND,
                        303u16 => ::axum::http::StatusCode::SEE_OTHER,
                        304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                        307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                        308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                        400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                        401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                        402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                        403u16 => ::axum::http::StatusCode::FORBIDDEN,
                        404u16 => ::axum::http::StatusCode::NOT_FOUND,
                        405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                        406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                        407u16 => ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED,
                        408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                        409u16 => ::axum::http::StatusCode::CONFLICT,
                        410u16 => ::axum::http::StatusCode::GONE,
                        411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                        412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                        413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                        414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                        415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                        416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                        417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                        418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                        421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                        422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                        423u16 => ::axum::http::StatusCode::LOCKED,
                        424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                        425u16 => ::axum::http::StatusCode::TOO_EARLY,
                        426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                        428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                        429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                        431u16 => {
                            ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE
                        }
                        451u16 => ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                        500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                        502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                        503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                        504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                        505u16 => ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED,
                        506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                        507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                        508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                        510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                        511u16 => {
                            ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED
                        }
                        _ => {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "internal error: entered unreachable code: {0}",
                                    format_args!("groom: status code {0} was validated at expand time",
                                    404u16,),
                                ),
                            );
                        }
                    })
                        .into_response()
                }
            }
        }
    }
    impl ::groom::response::Response for RespHtmlResponse {
        fn __groom_into_response(
            self,
            negotiated: Option<&::mime::Mime>,
        ) -> ::axum::response::Response {
            match negotiated {
                None => self.into_response_text_html(),
                Some(negotiated) => {
                    match (negotiated.type_(), negotiated.subtype()) {
                        (::mime::TEXT, ::mime::HTML) => self.into_response_text_html(),
                        _ => {
                            if true {
                                if !false {
                                    {
                                        ::core::panicking::panic_fmt(
                                            format_args!(
                                                "groom: negotiated mime not covered by response arms",
                                            ),
                                        );
                                    }
                                }
                            }
                            (
                                ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                "internal server error",
                            )
                                .into_response()
                        }
                    }
                }
            }
        }
        fn __openapi_modify_operation(
            op: ::utoipa::openapi::path::OperationBuilder,
            components: &mut ::groom::extract::ComponentsRegistry,
        ) -> ::utoipa::openapi::path::OperationBuilder {
            let op = op
                .response(
                    "200",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("")
                        .content(
                            ::mime::TEXT_HTML_UTF_8.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(
                                    Some(
                                        ::groom::extract::ComponentsRegistry::schema_or_ref::<
                                            String,
                                        >(components),
                                    ),
                                )
                                .build(),
                        )
                        .build(),
                );
            let op = op
                .response(
                    "404",
                    ::utoipa::openapi::ResponseBuilder::new().description("").build(),
                );
            let op = op
                .response(
                    "406",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("The requested content type is not supported")
                        .content(
                            ::mime::TEXT_PLAIN.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(
                                    Some(
                                        ::groom::extract::ComponentsRegistry::schema_or_ref::<
                                            String,
                                        >(components),
                                    ),
                                )
                                .build(),
                        )
                        .build(),
                );
            op
        }
        fn __groom_negotiate_content_type(
            accept: &::accept_header::Accept,
        ) -> ::core::result::Result<Option<::mime::Mime>, ::axum::response::Response> {
            match accept.negotiate(&__GROOM_RESPONSE_SUPPORTED_MIMES_RespHtmlResponse) {
                Ok(negotiated) => Ok(Some(negotiated)),
                Err(_) => {
                    Err(
                        ::groom::response::not_acceptable(
                            __GROOM_RESPONSE_SUPPORTED_MIMES_RespHtmlResponse,
                        ),
                    )
                }
            }
        }
        fn __groom_check_response_codes(
            context: &str,
            codes: &mut ::groom::runtime_checks::HTTPCodeSet,
        ) {
            let context = ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0} / enum `RespHtmlResponse`", context),
                )
            });
            codes
                .ensure_distinct(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0} / variant `Ok`", context))
                    }),
                    200u16,
                );
            codes
                .ensure_distinct(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("{0} / variant `NotFound`", context),
                        )
                    }),
                    404u16,
                );
        }
        fn __groom_check_response_formats(
            context: &str,
            formats: &mut ::groom::runtime_checks::HTTPFormatsSet,
        ) {
            formats.record(context, &__GROOM_RESPONSE_SUPPORTED_MIMES_RespHtmlResponse);
        }
    }
}
mod json_only {
    pub struct StructJson {
        success: bool,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for StructJson {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "StructJson",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "success",
                    &self.success,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    impl ::groom::DTO for StructJson {}
    impl ::groom::DTO_Response for StructJson {}
    pub enum RespJsonResponse {
        Ok(StructJson),
        NotFound,
    }
    #[allow(non_upper_case_globals)]
    const __GROOM_RESPONSE_SUPPORTED_MIMES_RespJsonResponse: &[::mime::Mime] = &[
        ::mime::APPLICATION_JSON,
    ];
    impl RespJsonResponse {
        fn into_response_application_json(self) -> ::axum::response::Response {
            match self {
                Self::Ok(body) => {
                    (
                        match 200u16 {
                            200u16 => ::axum::http::StatusCode::OK,
                            201u16 => ::axum::http::StatusCode::CREATED,
                            202u16 => ::axum::http::StatusCode::ACCEPTED,
                            203u16 => {
                                ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION
                            }
                            204u16 => ::axum::http::StatusCode::NO_CONTENT,
                            205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                            206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                            207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                            208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                            226u16 => ::axum::http::StatusCode::IM_USED,
                            300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                            301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                            302u16 => ::axum::http::StatusCode::FOUND,
                            303u16 => ::axum::http::StatusCode::SEE_OTHER,
                            304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                            307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                            308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                            400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                            401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                            402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                            403u16 => ::axum::http::StatusCode::FORBIDDEN,
                            404u16 => ::axum::http::StatusCode::NOT_FOUND,
                            405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                            406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                            407u16 => {
                                ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED
                            }
                            408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                            409u16 => ::axum::http::StatusCode::CONFLICT,
                            410u16 => ::axum::http::StatusCode::GONE,
                            411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                            412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                            413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                            414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                            415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                            416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                            417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                            418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                            421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                            422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                            423u16 => ::axum::http::StatusCode::LOCKED,
                            424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                            425u16 => ::axum::http::StatusCode::TOO_EARLY,
                            426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                            428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                            429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                            431u16 => {
                                ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE
                            }
                            451u16 => {
                                ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS
                            }
                            500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                            502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                            503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                            504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                            505u16 => {
                                ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED
                            }
                            506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                            507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                            508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                            510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                            511u16 => {
                                ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED
                            }
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("groom: status code {0} was validated at expand time",
                                        200u16,),
                                    ),
                                );
                            }
                        },
                        ::axum::Json(body),
                    )
                        .into_response()
                }
                Self::NotFound => {
                    (match 404u16 {
                        200u16 => ::axum::http::StatusCode::OK,
                        201u16 => ::axum::http::StatusCode::CREATED,
                        202u16 => ::axum::http::StatusCode::ACCEPTED,
                        203u16 => ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION,
                        204u16 => ::axum::http::StatusCode::NO_CONTENT,
                        205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                        206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                        207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                        208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                        226u16 => ::axum::http::StatusCode::IM_USED,
                        300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                        301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                        302u16 => ::axum::http::StatusCode::FOUND,
                        303u16 => ::axum::http::StatusCode::SEE_OTHER,
                        304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                        307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                        308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                        400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                        401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                        402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                        403u16 => ::axum::http::StatusCode::FORBIDDEN,
                        404u16 => ::axum::http::StatusCode::NOT_FOUND,
                        405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                        406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                        407u16 => ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED,
                        408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                        409u16 => ::axum::http::StatusCode::CONFLICT,
                        410u16 => ::axum::http::StatusCode::GONE,
                        411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                        412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                        413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                        414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                        415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                        416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                        417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                        418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                        421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                        422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                        423u16 => ::axum::http::StatusCode::LOCKED,
                        424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                        425u16 => ::axum::http::StatusCode::TOO_EARLY,
                        426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                        428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                        429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                        431u16 => {
                            ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE
                        }
                        451u16 => ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                        500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                        502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                        503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                        504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                        505u16 => ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED,
                        506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                        507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                        508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                        510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                        511u16 => {
                            ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED
                        }
                        _ => {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "internal error: entered unreachable code: {0}",
                                    format_args!("groom: status code {0} was validated at expand time",
                                    404u16,),
                                ),
                            );
                        }
                    })
                        .into_response()
                }
            }
        }
    }
    impl ::groom::response::Response for RespJsonResponse {
        fn __groom_into_response(
            self,
            negotiated: Option<&::mime::Mime>,
        ) -> ::axum::response::Response {
            match negotiated {
                None => self.into_response_application_json(),
                Some(negotiated) => {
                    match (negotiated.type_(), negotiated.subtype()) {
                        (::mime::APPLICATION, ::mime::JSON) => {
                            self.into_response_application_json()
                        }
                        _ => {
                            if true {
                                if !false {
                                    {
                                        ::core::panicking::panic_fmt(
                                            format_args!(
                                                "groom: negotiated mime not covered by response arms",
                                            ),
                                        );
                                    }
                                }
                            }
                            (
                                ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                "internal server error",
                            )
                                .into_response()
                        }
                    }
                }
            }
        }
        fn __openapi_modify_operation(
            op: ::utoipa::openapi::path::OperationBuilder,
            components: &mut ::groom::extract::ComponentsRegistry,
        ) -> ::utoipa::openapi::path::OperationBuilder {
            let op = op
                .response(
                    "200",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("")
                        .content(
                            ::mime::APPLICATION_JSON.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(Some(components.add_components::<StructJson>()))
                                .build(),
                        )
                        .build(),
                );
            let op = op
                .response(
                    "404",
                    ::utoipa::openapi::ResponseBuilder::new().description("").build(),
                );
            let op = op
                .response(
                    "406",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("The requested content type is not supported")
                        .content(
                            ::mime::TEXT_PLAIN.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(
                                    Some(
                                        ::groom::extract::ComponentsRegistry::schema_or_ref::<
                                            String,
                                        >(components),
                                    ),
                                )
                                .build(),
                        )
                        .build(),
                );
            op
        }
        fn __groom_negotiate_content_type(
            accept: &::accept_header::Accept,
        ) -> ::core::result::Result<Option<::mime::Mime>, ::axum::response::Response> {
            match accept.negotiate(&__GROOM_RESPONSE_SUPPORTED_MIMES_RespJsonResponse) {
                Ok(negotiated) => Ok(Some(negotiated)),
                Err(_) => {
                    Err(
                        ::groom::response::not_acceptable(
                            __GROOM_RESPONSE_SUPPORTED_MIMES_RespJsonResponse,
                        ),
                    )
                }
            }
        }
        fn __groom_check_response_codes(
            context: &str,
            codes: &mut ::groom::runtime_checks::HTTPCodeSet,
        ) {
            let context = ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0} / enum `RespJsonResponse`", context),
                )
            });
            codes
                .ensure_distinct(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0} / variant `Ok`", context))
                    }),
                    200u16,
                );
            codes
                .ensure_distinct(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("{0} / variant `NotFound`", context),
                        )
                    }),
                    404u16,
                );
        }
        fn __groom_check_response_formats(
            context: &str,
            formats: &mut ::groom::runtime_checks::HTTPFormatsSet,
        ) {
            formats.record(context, &__GROOM_RESPONSE_SUPPORTED_MIMES_RespJsonResponse);
        }
    }
}
mod multiple_content_types {
    use groom::response::html_format;
    use groom::response::HtmlFormat;
    pub struct Struct {
        success: bool,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Struct {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Struct",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "success",
                    &self.success,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    impl ::groom::DTO for Struct {}
    impl ::groom::DTO_Response for Struct {}
    impl ::groom::response::HtmlFormat for Struct {
        fn render(self) -> ::axum::response::Html<axum::body::Body> {
            ::axum::response::Html(
                if self.success {
                    "<span style=\"color: #a3be8c;\">success</span>"
                } else {
                    "<span style=\"color: #bf616a;\">error</span>"
                }
                    .into(),
            )
        }
    }
    pub enum RespMultipleTypesResponse {
        Ok(Struct),
        NotFound,
    }
    #[allow(non_upper_case_globals)]
    const __GROOM_RESPONSE_SUPPORTED_MIMES_RespMultipleTypesResponse: &[::mime::Mime] = &[
        ::mime::TEXT_PLAIN,
        ::mime::TEXT_HTML,
        ::mime::APPLICATION_JSON,
    ];
    impl RespMultipleTypesResponse {
        fn into_response_text_plain(self) -> ::axum::response::Response {
            match self {
                Self::Ok(body) => {
                    (
                        match 200u16 {
                            200u16 => ::axum::http::StatusCode::OK,
                            201u16 => ::axum::http::StatusCode::CREATED,
                            202u16 => ::axum::http::StatusCode::ACCEPTED,
                            203u16 => {
                                ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION
                            }
                            204u16 => ::axum::http::StatusCode::NO_CONTENT,
                            205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                            206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                            207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                            208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                            226u16 => ::axum::http::StatusCode::IM_USED,
                            300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                            301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                            302u16 => ::axum::http::StatusCode::FOUND,
                            303u16 => ::axum::http::StatusCode::SEE_OTHER,
                            304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                            307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                            308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                            400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                            401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                            402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                            403u16 => ::axum::http::StatusCode::FORBIDDEN,
                            404u16 => ::axum::http::StatusCode::NOT_FOUND,
                            405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                            406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                            407u16 => {
                                ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED
                            }
                            408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                            409u16 => ::axum::http::StatusCode::CONFLICT,
                            410u16 => ::axum::http::StatusCode::GONE,
                            411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                            412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                            413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                            414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                            415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                            416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                            417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                            418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                            421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                            422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                            423u16 => ::axum::http::StatusCode::LOCKED,
                            424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                            425u16 => ::axum::http::StatusCode::TOO_EARLY,
                            426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                            428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                            429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                            431u16 => {
                                ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE
                            }
                            451u16 => {
                                ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS
                            }
                            500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                            502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                            503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                            504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                            505u16 => {
                                ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED
                            }
                            506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                            507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                            508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                            510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                            511u16 => {
                                ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED
                            }
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("groom: status code {0} was validated at expand time",
                                        200u16,),
                                    ),
                                );
                            }
                        },
                        Into::<String>::into(body),
                    )
                        .into_response()
                }
                Self::NotFound => {
                    (match 404u16 {
                        200u16 => ::axum::http::StatusCode::OK,
                        201u16 => ::axum::http::StatusCode::CREATED,
                        202u16 => ::axum::http::StatusCode::ACCEPTED,
                        203u16 => ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION,
                        204u16 => ::axum::http::StatusCode::NO_CONTENT,
                        205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                        206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                        207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                        208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                        226u16 => ::axum::http::StatusCode::IM_USED,
                        300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                        301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                        302u16 => ::axum::http::StatusCode::FOUND,
                        303u16 => ::axum::http::StatusCode::SEE_OTHER,
                        304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                        307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                        308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                        400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                        401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                        402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                        403u16 => ::axum::http::StatusCode::FORBIDDEN,
                        404u16 => ::axum::http::StatusCode::NOT_FOUND,
                        405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                        406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                        407u16 => ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED,
                        408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                        409u16 => ::axum::http::StatusCode::CONFLICT,
                        410u16 => ::axum::http::StatusCode::GONE,
                        411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                        412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                        413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                        414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                        415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                        416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                        417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                        418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                        421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                        422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                        423u16 => ::axum::http::StatusCode::LOCKED,
                        424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                        425u16 => ::axum::http::StatusCode::TOO_EARLY,
                        426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                        428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                        429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                        431u16 => {
                            ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE
                        }
                        451u16 => ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                        500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                        502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                        503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                        504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                        505u16 => ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED,
                        506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                        507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                        508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                        510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                        511u16 => {
                            ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED
                        }
                        _ => {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "internal error: entered unreachable code: {0}",
                                    format_args!("groom: status code {0} was validated at expand time",
                                    404u16,),
                                ),
                            );
                        }
                    })
                        .into_response()
                }
            }
        }
        fn into_response_text_html(self) -> ::axum::response::Response {
            match self {
                Self::Ok(body) => {
                    (
                        match 200u16 {
                            200u16 => ::axum::http::StatusCode::OK,
                            201u16 => ::axum::http::StatusCode::CREATED,
                            202u16 => ::axum::http::StatusCode::ACCEPTED,
                            203u16 => {
                                ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION
                            }
                            204u16 => ::axum::http::StatusCode::NO_CONTENT,
                            205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                            206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                            207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                            208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                            226u16 => ::axum::http::StatusCode::IM_USED,
                            300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                            301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                            302u16 => ::axum::http::StatusCode::FOUND,
                            303u16 => ::axum::http::StatusCode::SEE_OTHER,
                            304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                            307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                            308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                            400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                            401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                            402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                            403u16 => ::axum::http::StatusCode::FORBIDDEN,
                            404u16 => ::axum::http::StatusCode::NOT_FOUND,
                            405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                            406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                            407u16 => {
                                ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED
                            }
                            408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                            409u16 => ::axum::http::StatusCode::CONFLICT,
                            410u16 => ::axum::http::StatusCode::GONE,
                            411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                            412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                            413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                            414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                            415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                            416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                            417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                            418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                            421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                            422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                            423u16 => ::axum::http::StatusCode::LOCKED,
                            424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                            425u16 => ::axum::http::StatusCode::TOO_EARLY,
                            426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                            428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                            429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                            431u16 => {
                                ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE
                            }
                            451u16 => {
                                ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS
                            }
                            500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                            502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                            503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                            504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                            505u16 => {
                                ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED
                            }
                            506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                            507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                            508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                            510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                            511u16 => {
                                ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED
                            }
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("groom: status code {0} was validated at expand time",
                                        200u16,),
                                    ),
                                );
                            }
                        },
                        <Struct as ::groom::response::HtmlFormat>::render(body),
                    )
                        .into_response()
                }
                Self::NotFound => {
                    (match 404u16 {
                        200u16 => ::axum::http::StatusCode::OK,
                        201u16 => ::axum::http::StatusCode::CREATED,
                        202u16 => ::axum::http::StatusCode::ACCEPTED,
                        203u16 => ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION,
                        204u16 => ::axum::http::StatusCode::NO_CONTENT,
                        205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                        206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                        207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                        208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                        226u16 => ::axum::http::StatusCode::IM_USED,
                        300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                        301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                        302u16 => ::axum::http::StatusCode::FOUND,
                        303u16 => ::axum::http::StatusCode::SEE_OTHER,
                        304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                        307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                        308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                        400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                        401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                        402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                        403u16 => ::axum::http::StatusCode::FORBIDDEN,
                        404u16 => ::axum::http::StatusCode::NOT_FOUND,
                        405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                        406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                        407u16 => ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED,
                        408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                        409u16 => ::axum::http::StatusCode::CONFLICT,
                        410u16 => ::axum::http::StatusCode::GONE,
                        411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                        412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                        413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                        414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                        415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                        416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                        417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                        418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                        421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                        422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                        423u16 => ::axum::http::StatusCode::LOCKED,
                        424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                        425u16 => ::axum::http::StatusCode::TOO_EARLY,
                        426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                        428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                        429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                        431u16 => {
                            ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE
                        }
                        451u16 => ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                        500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                        502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                        503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                        504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                        505u16 => ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED,
                        506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                        507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                        508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                        510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                        511u16 => {
                            ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED
                        }
                        _ => {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "internal error: entered unreachable code: {0}",
                                    format_args!("groom: status code {0} was validated at expand time",
                                    404u16,),
                                ),
                            );
                        }
                    })
                        .into_response()
                }
            }
        }
        fn into_response_application_json(self) -> ::axum::response::Response {
            match self {
                Self::Ok(body) => {
                    (
                        match 200u16 {
                            200u16 => ::axum::http::StatusCode::OK,
                            201u16 => ::axum::http::StatusCode::CREATED,
                            202u16 => ::axum::http::StatusCode::ACCEPTED,
                            203u16 => {
                                ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION
                            }
                            204u16 => ::axum::http::StatusCode::NO_CONTENT,
                            205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                            206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                            207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                            208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                            226u16 => ::axum::http::StatusCode::IM_USED,
                            300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                            301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                            302u16 => ::axum::http::StatusCode::FOUND,
                            303u16 => ::axum::http::StatusCode::SEE_OTHER,
                            304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                            307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                            308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                            400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                            401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                            402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                            403u16 => ::axum::http::StatusCode::FORBIDDEN,
                            404u16 => ::axum::http::StatusCode::NOT_FOUND,
                            405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                            406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                            407u16 => {
                                ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED
                            }
                            408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                            409u16 => ::axum::http::StatusCode::CONFLICT,
                            410u16 => ::axum::http::StatusCode::GONE,
                            411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                            412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                            413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                            414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                            415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                            416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                            417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                            418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                            421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                            422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                            423u16 => ::axum::http::StatusCode::LOCKED,
                            424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                            425u16 => ::axum::http::StatusCode::TOO_EARLY,
                            426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                            428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                            429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                            431u16 => {
                                ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE
                            }
                            451u16 => {
                                ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS
                            }
                            500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                            502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                            503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                            504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                            505u16 => {
                                ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED
                            }
                            506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                            507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                            508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                            510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                            511u16 => {
                                ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED
                            }
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("groom: status code {0} was validated at expand time",
                                        200u16,),
                                    ),
                                );
                            }
                        },
                        ::axum::Json(body),
                    )
                        .into_response()
                }
                Self::NotFound => {
                    (match 404u16 {
                        200u16 => ::axum::http::StatusCode::OK,
                        201u16 => ::axum::http::StatusCode::CREATED,
                        202u16 => ::axum::http::StatusCode::ACCEPTED,
                        203u16 => ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION,
                        204u16 => ::axum::http::StatusCode::NO_CONTENT,
                        205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                        206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                        207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                        208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                        226u16 => ::axum::http::StatusCode::IM_USED,
                        300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                        301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                        302u16 => ::axum::http::StatusCode::FOUND,
                        303u16 => ::axum::http::StatusCode::SEE_OTHER,
                        304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                        307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                        308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                        400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                        401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                        402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                        403u16 => ::axum::http::StatusCode::FORBIDDEN,
                        404u16 => ::axum::http::StatusCode::NOT_FOUND,
                        405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                        406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                        407u16 => ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED,
                        408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                        409u16 => ::axum::http::StatusCode::CONFLICT,
                        410u16 => ::axum::http::StatusCode::GONE,
                        411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                        412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                        413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                        414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                        415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                        416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                        417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                        418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                        421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                        422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                        423u16 => ::axum::http::StatusCode::LOCKED,
                        424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                        425u16 => ::axum::http::StatusCode::TOO_EARLY,
                        426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                        428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                        429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                        431u16 => {
                            ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE
                        }
                        451u16 => ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                        500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                        502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                        503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                        504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                        505u16 => ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED,
                        506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                        507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                        508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                        510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                        511u16 => {
                            ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED
                        }
                        _ => {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "internal error: entered unreachable code: {0}",
                                    format_args!("groom: status code {0} was validated at expand time",
                                    404u16,),
                                ),
                            );
                        }
                    })
                        .into_response()
                }
            }
        }
    }
    impl ::groom::response::Response for RespMultipleTypesResponse {
        fn __groom_into_response(
            self,
            negotiated: Option<&::mime::Mime>,
        ) -> ::axum::response::Response {
            match negotiated {
                None => self.into_response_application_json(),
                Some(negotiated) => {
                    match (negotiated.type_(), negotiated.subtype()) {
                        (::mime::TEXT, ::mime::PLAIN) => self.into_response_text_plain(),
                        (::mime::TEXT, ::mime::HTML) => self.into_response_text_html(),
                        (::mime::APPLICATION, ::mime::JSON) => {
                            self.into_response_application_json()
                        }
                        _ => {
                            if true {
                                if !false {
                                    {
                                        ::core::panicking::panic_fmt(
                                            format_args!(
                                                "groom: negotiated mime not covered by response arms",
                                            ),
                                        );
                                    }
                                }
                            }
                            (
                                ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                "internal server error",
                            )
                                .into_response()
                        }
                    }
                }
            }
        }
        fn __openapi_modify_operation(
            op: ::utoipa::openapi::path::OperationBuilder,
            components: &mut ::groom::extract::ComponentsRegistry,
        ) -> ::utoipa::openapi::path::OperationBuilder {
            let op = op
                .response(
                    "200",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("")
                        .content(
                            ::mime::TEXT_PLAIN_UTF_8.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(
                                    Some(
                                        ::groom::extract::ComponentsRegistry::schema_or_ref::<
                                            String,
                                        >(components),
                                    ),
                                )
                                .build(),
                        )
                        .content(
                            ::mime::TEXT_HTML_UTF_8.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(
                                    Some(
                                        ::groom::extract::ComponentsRegistry::schema_or_ref::<
                                            String,
                                        >(components),
                                    ),
                                )
                                .build(),
                        )
                        .content(
                            ::mime::APPLICATION_JSON.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(Some(components.add_components::<Struct>()))
                                .build(),
                        )
                        .build(),
                );
            let op = op
                .response(
                    "404",
                    ::utoipa::openapi::ResponseBuilder::new().description("").build(),
                );
            let op = op
                .response(
                    "406",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("The requested content type is not supported")
                        .content(
                            ::mime::TEXT_PLAIN.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(
                                    Some(
                                        ::groom::extract::ComponentsRegistry::schema_or_ref::<
                                            String,
                                        >(components),
                                    ),
                                )
                                .build(),
                        )
                        .build(),
                );
            op
        }
        fn __groom_negotiate_content_type(
            accept: &::accept_header::Accept,
        ) -> ::core::result::Result<Option<::mime::Mime>, ::axum::response::Response> {
            match accept
                .negotiate(&__GROOM_RESPONSE_SUPPORTED_MIMES_RespMultipleTypesResponse)
            {
                Ok(negotiated) => Ok(Some(negotiated)),
                Err(_) => {
                    Err(
                        ::groom::response::not_acceptable(
                            __GROOM_RESPONSE_SUPPORTED_MIMES_RespMultipleTypesResponse,
                        ),
                    )
                }
            }
        }
        fn __groom_check_response_codes(
            context: &str,
            codes: &mut ::groom::runtime_checks::HTTPCodeSet,
        ) {
            let context = ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0} / enum `RespMultipleTypesResponse`", context),
                )
            });
            codes
                .ensure_distinct(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0} / variant `Ok`", context))
                    }),
                    200u16,
                );
            codes
                .ensure_distinct(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("{0} / variant `NotFound`", context),
                        )
                    }),
                    404u16,
                );
        }
        fn __groom_check_response_formats(
            context: &str,
            formats: &mut ::groom::runtime_checks::HTTPFormatsSet,
        ) {
            formats
                .record(
                    context,
                    &__GROOM_RESPONSE_SUPPORTED_MIMES_RespMultipleTypesResponse,
                );
        }
    }
}
mod named_struct_response {
    use groom::response::html_format;
    use groom::response::HtmlFormat;
    pub struct Named {
        success: bool,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Named {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Named",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "success",
                    &self.success,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    impl ::groom::DTO for Named {}
    impl ::groom::DTO_Response for Named {}
    #[allow(non_upper_case_globals)]
    const __GROOM_RESPONSE_SUPPORTED_MIMES_Named: &[::mime::Mime] = &[
        ::mime::TEXT_HTML,
        ::mime::APPLICATION_JSON,
    ];
    impl Named {
        fn into_response_text_html(self) -> ::axum::response::Response {
            (
                match 200u16 {
                    200u16 => ::axum::http::StatusCode::OK,
                    201u16 => ::axum::http::StatusCode::CREATED,
                    202u16 => ::axum::http::StatusCode::ACCEPTED,
                    203u16 => ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION,
                    204u16 => ::axum::http::StatusCode::NO_CONTENT,
                    205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                    206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                    207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                    208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                    226u16 => ::axum::http::StatusCode::IM_USED,
                    300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                    301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                    302u16 => ::axum::http::StatusCode::FOUND,
                    303u16 => ::axum::http::StatusCode::SEE_OTHER,
                    304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                    307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                    308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                    400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                    401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                    402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                    403u16 => ::axum::http::StatusCode::FORBIDDEN,
                    404u16 => ::axum::http::StatusCode::NOT_FOUND,
                    405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                    406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                    407u16 => ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED,
                    408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                    409u16 => ::axum::http::StatusCode::CONFLICT,
                    410u16 => ::axum::http::StatusCode::GONE,
                    411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                    412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                    413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                    414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                    415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                    417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                    418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                    421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                    422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                    423u16 => ::axum::http::StatusCode::LOCKED,
                    424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                    425u16 => ::axum::http::StatusCode::TOO_EARLY,
                    426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                    428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                    429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                    431u16 => ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE,
                    451u16 => ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                    500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                    502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                    503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                    504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                    505u16 => ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED,
                    506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                    507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                    508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                    510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                    511u16 => ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED,
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("groom: status code {0} was validated at expand time",
                                200u16,),
                            ),
                        );
                    }
                },
                <Named as ::groom::response::HtmlFormat>::render(self),
            )
                .into_response()
        }
        fn into_response_application_json(self) -> ::axum::response::Response {
            (
                match 200u16 {
                    200u16 => ::axum::http::StatusCode::OK,
                    201u16 => ::axum::http::StatusCode::CREATED,
                    202u16 => ::axum::http::StatusCode::ACCEPTED,
                    203u16 => ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION,
                    204u16 => ::axum::http::StatusCode::NO_CONTENT,
                    205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                    206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                    207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                    208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                    226u16 => ::axum::http::StatusCode::IM_USED,
                    300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                    301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                    302u16 => ::axum::http::StatusCode::FOUND,
                    303u16 => ::axum::http::StatusCode::SEE_OTHER,
                    304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                    307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                    308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                    400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                    401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                    402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                    403u16 => ::axum::http::StatusCode::FORBIDDEN,
                    404u16 => ::axum::http::StatusCode::NOT_FOUND,
                    405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                    406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                    407u16 => ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED,
                    408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                    409u16 => ::axum::http::StatusCode::CONFLICT,
                    410u16 => ::axum::http::StatusCode::GONE,
                    411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                    412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                    413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                    414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                    415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                    417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                    418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                    421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                    422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                    423u16 => ::axum::http::StatusCode::LOCKED,
                    424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                    425u16 => ::axum::http::StatusCode::TOO_EARLY,
                    426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                    428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                    429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                    431u16 => ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE,
                    451u16 => ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                    500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                    502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                    503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                    504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                    505u16 => ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED,
                    506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                    507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                    508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                    510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                    511u16 => ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED,
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("groom: status code {0} was validated at expand time",
                                200u16,),
                            ),
                        );
                    }
                },
                ::axum::Json(self),
            )
                .into_response()
        }
    }
    impl ::groom::response::Response for Named {
        fn __groom_into_response(
            self,
            negotiated: Option<&::mime::Mime>,
        ) -> ::axum::response::Response {
            match negotiated {
                None => self.into_response_application_json(),
                Some(negotiated) => {
                    match (negotiated.type_(), negotiated.subtype()) {
                        (::mime::TEXT, ::mime::HTML) => self.into_response_text_html(),
                        (::mime::APPLICATION, ::mime::JSON) => {
                            self.into_response_application_json()
                        }
                        _ => {
                            if true {
                                if !false {
                                    {
                                        ::core::panicking::panic_fmt(
                                            format_args!(
                                                "groom: negotiated mime not covered by response arms",
                                            ),
                                        );
                                    }
                                }
                            }
                            (
                                ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                "internal server error",
                            )
                                .into_response()
                        }
                    }
                }
            }
        }
        fn __openapi_modify_operation(
            op: ::utoipa::openapi::path::OperationBuilder,
            components: &mut ::groom::extract::ComponentsRegistry,
        ) -> ::utoipa::openapi::path::OperationBuilder {
            let op = op
                .response(
                    "200",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("")
                        .content(
                            ::mime::TEXT_HTML_UTF_8.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(
                                    Some(
                                        ::groom::extract::ComponentsRegistry::schema_or_ref::<
                                            String,
                                        >(components),
                                    ),
                                )
                                .build(),
                        )
                        .content(
                            ::mime::APPLICATION_JSON.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(Some(components.add_components::<Named>()))
                                .build(),
                        )
                        .build(),
                );
            let op = op
                .response(
                    "406",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("The requested content type is not supported")
                        .content(
                            ::mime::TEXT_PLAIN.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(
                                    Some(
                                        ::groom::extract::ComponentsRegistry::schema_or_ref::<
                                            String,
                                        >(components),
                                    ),
                                )
                                .build(),
                        )
                        .build(),
                );
            op
        }
        fn __groom_negotiate_content_type(
            accept: &::accept_header::Accept,
        ) -> ::core::result::Result<Option<::mime::Mime>, ::axum::response::Response> {
            match accept.negotiate(&__GROOM_RESPONSE_SUPPORTED_MIMES_Named) {
                Ok(negotiated) => Ok(Some(negotiated)),
                Err(_) => {
                    Err(
                        ::groom::response::not_acceptable(
                            __GROOM_RESPONSE_SUPPORTED_MIMES_Named,
                        ),
                    )
                }
            }
        }
        fn __groom_check_response_codes(
            context: &str,
            codes: &mut ::groom::runtime_checks::HTTPCodeSet,
        ) {
            codes
                .ensure_distinct(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("{0} / struct `Named`", context),
                        )
                    }),
                    200u16,
                )
        }
        fn __groom_check_response_formats(
            context: &str,
            formats: &mut ::groom::runtime_checks::HTTPFormatsSet,
        ) {
            formats.record(context, &__GROOM_RESPONSE_SUPPORTED_MIMES_Named);
        }
    }
    impl ::groom::response::HtmlFormat for Named {
        fn render(self) -> ::axum::response::Html<axum::body::Body> {
            ::axum::response::Html(
                if self.success {
                    "<span style=\"color: #a3be8c;\">success</span>"
                } else {
                    "<span style=\"color: #bf616a;\">error</span>"
                }
                    .into(),
            )
        }
    }
}
mod unnamed_struct_response {
    use groom::response::html_format;
    use groom::response::HtmlFormat;
    pub struct Unnamed(String);
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Unnamed {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(
                    __serializer,
                    "Unnamed",
                    &self.0,
                )
            }
        }
    };
    impl ::groom::DTO for Unnamed {}
    impl ::groom::DTO_Response for Unnamed {}
    #[allow(non_upper_case_globals)]
    const __GROOM_RESPONSE_SUPPORTED_MIMES_Unnamed: &[::mime::Mime] = &[
        ::mime::TEXT_HTML,
        ::mime::APPLICATION_JSON,
    ];
    impl Unnamed {
        fn into_response_text_html(self) -> ::axum::response::Response {
            (
                match 200u16 {
                    200u16 => ::axum::http::StatusCode::OK,
                    201u16 => ::axum::http::StatusCode::CREATED,
                    202u16 => ::axum::http::StatusCode::ACCEPTED,
                    203u16 => ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION,
                    204u16 => ::axum::http::StatusCode::NO_CONTENT,
                    205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                    206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                    207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                    208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                    226u16 => ::axum::http::StatusCode::IM_USED,
                    300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                    301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                    302u16 => ::axum::http::StatusCode::FOUND,
                    303u16 => ::axum::http::StatusCode::SEE_OTHER,
                    304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                    307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                    308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                    400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                    401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                    402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                    403u16 => ::axum::http::StatusCode::FORBIDDEN,
                    404u16 => ::axum::http::StatusCode::NOT_FOUND,
                    405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                    406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                    407u16 => ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED,
                    408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                    409u16 => ::axum::http::StatusCode::CONFLICT,
                    410u16 => ::axum::http::StatusCode::GONE,
                    411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                    412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                    413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                    414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                    415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                    417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                    418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                    421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                    422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                    423u16 => ::axum::http::StatusCode::LOCKED,
                    424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                    425u16 => ::axum::http::StatusCode::TOO_EARLY,
                    426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                    428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                    429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                    431u16 => ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE,
                    451u16 => ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                    500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                    502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                    503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                    504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                    505u16 => ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED,
                    506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                    507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                    508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                    510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                    511u16 => ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED,
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("groom: status code {0} was validated at expand time",
                                200u16,),
                            ),
                        );
                    }
                },
                <Unnamed as ::groom::response::HtmlFormat>::render(self),
            )
                .into_response()
        }
        fn into_response_application_json(self) -> ::axum::response::Response {
            (
                match 200u16 {
                    200u16 => ::axum::http::StatusCode::OK,
                    201u16 => ::axum::http::StatusCode::CREATED,
                    202u16 => ::axum::http::StatusCode::ACCEPTED,
                    203u16 => ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION,
                    204u16 => ::axum::http::StatusCode::NO_CONTENT,
                    205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                    206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                    207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                    208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                    226u16 => ::axum::http::StatusCode::IM_USED,
                    300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                    301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                    302u16 => ::axum::http::StatusCode::FOUND,
                    303u16 => ::axum::http::StatusCode::SEE_OTHER,
                    304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                    307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                    308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                    400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                    401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                    402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                    403u16 => ::axum::http::StatusCode::FORBIDDEN,
                    404u16 => ::axum::http::StatusCode::NOT_FOUND,
                    405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                    406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                    407u16 => ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED,
                    408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                    409u16 => ::axum::http::StatusCode::CONFLICT,
                    410u16 => ::axum::http::StatusCode::GONE,
                    411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                    412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                    413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                    414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                    415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                    417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                    418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                    421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                    422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                    423u16 => ::axum::http::StatusCode::LOCKED,
                    424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                    425u16 => ::axum::http::StatusCode::TOO_EARLY,
                    426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                    428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                    429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                    431u16 => ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE,
                    451u16 => ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                    500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                    502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                    503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                    504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                    505u16 => ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED,
                    506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                    507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                    508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                    510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                    511u16 => ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED,
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("groom: status code {0} was validated at expand time",
                                200u16,),
                            ),
                        );
                    }
                },
                ::axum::Json(self),
            )
                .into_response()
        }
    }
    impl ::groom::response::Response for Unnamed {
        fn __groom_into_response(
            self,
            negotiated: Option<&::mime::Mime>,
        ) -> ::axum::response::Response {
            match negotiated {
                None => self.into_response_application_json(),
                Some(negotiated) => {
                    match (negotiated.type_(), negotiated.subtype()) {
                        (::mime::TEXT, ::mime::HTML) => self.into_response_text_html(),
                        (::mime::APPLICATION, ::mime::JSON) => {
                            self.into_response_application_json()
                        }
                        _ => {
                            if true {
                                if !false {
                                    {
                                        ::core::panicking::panic_fmt(
                                            format_args!(
                                                "groom: negotiated mime not covered by response arms",
                                            ),
                                        );
                                    }
                                }
                            }
                            (
                                ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                "internal server error",
                            )
                                .into_response()
                        }
                    }
                }
            }
        }
        fn __openapi_modify_operation(
            op: ::utoipa::openapi::path::OperationBuilder,
            components: &mut ::groom::extract::ComponentsRegistry,
        ) -> ::utoipa::openapi::path::OperationBuilder {
            let op = op
                .response(
                    "200",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("")
                        .content(
                            ::mime::TEXT_HTML_UTF_8.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(
                                    Some(
                                        ::groom::extract::ComponentsRegistry::schema_or_ref::<
                                            String,
                                        >(components),
                                    ),
                                )
                                .build(),
                        )
                        .content(
                            ::mime::APPLICATION_JSON.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(Some(components.add_components::<String>()))
                                .build(),
                        )
                        .build(),
                );
            let op = op
                .response(
                    "406",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("The requested content type is not supported")
                        .content(
                            ::mime::TEXT_PLAIN.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(
                                    Some(
                                        ::groom::extract::ComponentsRegistry::schema_or_ref::<
                                            String,
                                        >(components),
                                    ),
                                )
                                .build(),
                        )
                        .build(),
                );
            op
        }
        fn __groom_negotiate_content_type(
            accept: &::accept_header::Accept,
        ) -> ::core::result::Result<Option<::mime::Mime>, ::axum::response::Response> {
            match accept.negotiate(&__GROOM_RESPONSE_SUPPORTED_MIMES_Unnamed) {
                Ok(negotiated) => Ok(Some(negotiated)),
                Err(_) => {
                    Err(
                        ::groom::response::not_acceptable(
                            __GROOM_RESPONSE_SUPPORTED_MIMES_Unnamed,
                        ),
                    )
                }
            }
        }
        fn __groom_check_response_codes(
            context: &str,
            codes: &mut ::groom::runtime_checks::HTTPCodeSet,
        ) {
            codes
                .ensure_distinct(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("{0} / struct `Unnamed`", context),
                        )
                    }),
                    200u16,
                )
        }
        fn __groom_check_response_formats(
            context: &str,
            formats: &mut ::groom::runtime_checks::HTTPFormatsSet,
        ) {
            formats.record(context, &__GROOM_RESPONSE_SUPPORTED_MIMES_Unnamed);
        }
    }
    impl ::groom::response::HtmlFormat for Unnamed {
        fn render(self) -> ::axum::response::Html<axum::body::Body> {
            ::axum::response::Html(
                ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "<span style=\"color: #a3be8c;\">{0}</span>", self.0,
                            ),
                        )
                    })
                    .into(),
            )
        }
    }
}
mod unit_struct_response {
    pub struct Unit;
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Unit {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_unit_struct(__serializer, "Unit")
            }
        }
    };
    impl ::groom::DTO for Unit {}
    impl ::groom::DTO_Response for Unit {}
    #[allow(non_upper_case_globals)]
    const __GROOM_RESPONSE_SUPPORTED_MIMES_Unit: &[::mime::Mime] = &[];
    impl Unit {
        fn into_response_any_content_type(self) -> ::axum::response::Response {
            (match 200u16 {
                200u16 => ::axum::http::StatusCode::OK,
                201u16 => ::axum::http::StatusCode::CREATED,
                202u16 => ::axum::http::StatusCode::ACCEPTED,
                203u16 => ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION,
                204u16 => ::axum::http::StatusCode::NO_CONTENT,
                205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                226u16 => ::axum::http::StatusCode::IM_USED,
                300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                302u16 => ::axum::http::StatusCode::FOUND,
                303u16 => ::axum::http::StatusCode::SEE_OTHER,
                304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                403u16 => ::axum::http::StatusCode::FORBIDDEN,
                404u16 => ::axum::http::StatusCode::NOT_FOUND,
                405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                407u16 => ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED,
                408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                409u16 => ::axum::http::StatusCode::CONFLICT,
                410u16 => ::axum::http::StatusCode::GONE,
                411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                423u16 => ::axum::http::StatusCode::LOCKED,
                424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                425u16 => ::axum::http::StatusCode::TOO_EARLY,
                426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                431u16 => ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE,
                451u16 => ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                505u16 => ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED,
                506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                511u16 => ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("groom: status code {0} was validated at expand time",
                            200u16,),
                        ),
                    );
                }
            })
                .into_response()
        }
    }
    impl ::groom::response::Response for Unit {
        fn __groom_into_response(
            self,
            negotiated: Option<&::mime::Mime>,
        ) -> ::axum::response::Response {
            self.into_response_any_content_type()
        }
        fn __openapi_modify_operation(
            op: ::utoipa::openapi::path::OperationBuilder,
            components: &mut ::groom::extract::ComponentsRegistry,
        ) -> ::utoipa::openapi::path::OperationBuilder {
            let op = op
                .response(
                    "200",
                    ::utoipa::openapi::ResponseBuilder::new().description("").build(),
                );
            op
        }
        fn __groom_negotiate_content_type(
            _accept: &::accept_header::Accept,
        ) -> ::core::result::Result<Option<::mime::Mime>, ::axum::response::Response> {
            Ok(None)
        }
        fn __groom_check_response_codes(
            context: &str,
            codes: &mut ::groom::runtime_checks::HTTPCodeSet,
        ) {
            codes
                .ensure_distinct(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("{0} / struct `Unit`", context),
                        )
                    }),
                    200u16,
                )
        }
        fn __groom_check_response_formats(
            context: &str,
            formats: &mut ::groom::runtime_checks::HTTPFormatsSet,
        ) {
            formats.record(context, &__GROOM_RESPONSE_SUPPORTED_MIMES_Unit);
        }
    }
}
mod result_struct_struct {
    use ::static_assertions::{assert_impl_all, assert_impl_any};
    pub struct Success;
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Success {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_unit_struct(__serializer, "Success")
            }
        }
    };
    impl ::groom::DTO for Success {}
    impl ::groom::DTO_Response for Success {}
    #[allow(non_upper_case_globals)]
    const __GROOM_RESPONSE_SUPPORTED_MIMES_Success: &[::mime::Mime] = &[];
    impl Success {
        fn into_response_any_content_type(self) -> ::axum::response::Response {
            (match 200u16 {
                200u16 => ::axum::http::StatusCode::OK,
                201u16 => ::axum::http::StatusCode::CREATED,
                202u16 => ::axum::http::StatusCode::ACCEPTED,
                203u16 => ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION,
                204u16 => ::axum::http::StatusCode::NO_CONTENT,
                205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                226u16 => ::axum::http::StatusCode::IM_USED,
                300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                302u16 => ::axum::http::StatusCode::FOUND,
                303u16 => ::axum::http::StatusCode::SEE_OTHER,
                304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                403u16 => ::axum::http::StatusCode::FORBIDDEN,
                404u16 => ::axum::http::StatusCode::NOT_FOUND,
                405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                407u16 => ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED,
                408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                409u16 => ::axum::http::StatusCode::CONFLICT,
                410u16 => ::axum::http::StatusCode::GONE,
                411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                423u16 => ::axum::http::StatusCode::LOCKED,
                424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                425u16 => ::axum::http::StatusCode::TOO_EARLY,
                426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                431u16 => ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE,
                451u16 => ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                505u16 => ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED,
                506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                511u16 => ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("groom: status code {0} was validated at expand time",
                            200u16,),
                        ),
                    );
                }
            })
                .into_response()
        }
    }
    impl ::groom::response::Response for Success {
        fn __groom_into_response(
            self,
            negotiated: Option<&::mime::Mime>,
        ) -> ::axum::response::Response {
            self.into_response_any_content_type()
        }
        fn __openapi_modify_operation(
            op: ::utoipa::openapi::path::OperationBuilder,
            components: &mut ::groom::extract::ComponentsRegistry,
        ) -> ::utoipa::openapi::path::OperationBuilder {
            let op = op
                .response(
                    "200",
                    ::utoipa::openapi::ResponseBuilder::new().description("").build(),
                );
            op
        }
        fn __groom_negotiate_content_type(
            _accept: &::accept_header::Accept,
        ) -> ::core::result::Result<Option<::mime::Mime>, ::axum::response::Response> {
            Ok(None)
        }
        fn __groom_check_response_codes(
            context: &str,
            codes: &mut ::groom::runtime_checks::HTTPCodeSet,
        ) {
            codes
                .ensure_distinct(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("{0} / struct `Success`", context),
                        )
                    }),
                    200u16,
                )
        }
        fn __groom_check_response_formats(
            context: &str,
            formats: &mut ::groom::runtime_checks::HTTPFormatsSet,
        ) {
            formats.record(context, &__GROOM_RESPONSE_SUPPORTED_MIMES_Success);
        }
    }
    pub struct Error;
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Error {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_unit_struct(__serializer, "Error")
            }
        }
    };
    impl ::groom::DTO for Error {}
    impl ::groom::DTO_Response for Error {}
    #[allow(non_upper_case_globals)]
    const __GROOM_RESPONSE_SUPPORTED_MIMES_Error: &[::mime::Mime] = &[];
    impl Error {
        fn into_response_any_content_type(self) -> ::axum::response::Response {
            (match 404u16 {
                200u16 => ::axum::http::StatusCode::OK,
                201u16 => ::axum::http::StatusCode::CREATED,
                202u16 => ::axum::http::StatusCode::ACCEPTED,
                203u16 => ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION,
                204u16 => ::axum::http::StatusCode::NO_CONTENT,
                205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                226u16 => ::axum::http::StatusCode::IM_USED,
                300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                302u16 => ::axum::http::StatusCode::FOUND,
                303u16 => ::axum::http::StatusCode::SEE_OTHER,
                304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                403u16 => ::axum::http::StatusCode::FORBIDDEN,
                404u16 => ::axum::http::StatusCode::NOT_FOUND,
                405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                407u16 => ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED,
                408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                409u16 => ::axum::http::StatusCode::CONFLICT,
                410u16 => ::axum::http::StatusCode::GONE,
                411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                423u16 => ::axum::http::StatusCode::LOCKED,
                424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                425u16 => ::axum::http::StatusCode::TOO_EARLY,
                426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                431u16 => ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE,
                451u16 => ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                505u16 => ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED,
                506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                511u16 => ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED,
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("groom: status code {0} was validated at expand time",
                            404u16,),
                        ),
                    );
                }
            })
                .into_response()
        }
    }
    impl ::groom::response::Response for Error {
        fn __groom_into_response(
            self,
            negotiated: Option<&::mime::Mime>,
        ) -> ::axum::response::Response {
            self.into_response_any_content_type()
        }
        fn __openapi_modify_operation(
            op: ::utoipa::openapi::path::OperationBuilder,
            components: &mut ::groom::extract::ComponentsRegistry,
        ) -> ::utoipa::openapi::path::OperationBuilder {
            let op = op
                .response(
                    "404",
                    ::utoipa::openapi::ResponseBuilder::new().description("").build(),
                );
            op
        }
        fn __groom_negotiate_content_type(
            _accept: &::accept_header::Accept,
        ) -> ::core::result::Result<Option<::mime::Mime>, ::axum::response::Response> {
            Ok(None)
        }
        fn __groom_check_response_codes(
            context: &str,
            codes: &mut ::groom::runtime_checks::HTTPCodeSet,
        ) {
            codes
                .ensure_distinct(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("{0} / struct `Error`", context),
                        )
                    }),
                    404u16,
                )
        }
        fn __groom_check_response_formats(
            context: &str,
            formats: &mut ::groom::runtime_checks::HTTPFormatsSet,
        ) {
            formats.record(context, &__GROOM_RESPONSE_SUPPORTED_MIMES_Error);
        }
    }
    /// HTTP handler: GET /
    async fn foo() -> Result<Success, Error> {
        Ok(Success)
    }
    async fn __groom_wrapper_foo(
        headers: ::axum::http::header::HeaderMap,
    ) -> impl ::axum::response::IntoResponse {
        let accept = match ::groom::content_negotiation::parse_accept_header(&headers) {
            Err(_) => return ::groom::response::bad_accept_header(),
            Ok(accept) => accept,
        };
        let negotiated = match accept {
            None => None,
            Some(accept) => {
                match <Result<Success, Error>>::__groom_negotiate_content_type(&accept) {
                    Err(response) => return response,
                    Ok(negotiated) => negotiated,
                }
            }
        };
        let result = foo().await;
        result.__groom_into_response(negotiated.as_ref())
    }
    fn __groom_runtime_checks() {
        let context = "Groom runtime check of mod `result_struct_struct`".to_string();
        let mut codes = ::groom::runtime_checks::HTTPCodeSet::new();
        <Result<
            Success,
            Error,
        >>::__groom_check_response_codes(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}: handler `foo`", context))
            }),
            &mut codes,
        );
        let mut formats = ::groom::runtime_checks::HTTPFormatsSet::new();
        <Result<
            Success,
            Error,
        >>::__groom_check_response_formats(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}: handler `foo`", context))
            }),
            &mut formats,
        );
    }
    pub fn into_router() -> ::groom::router::GroomRouter<()> {
        __groom_runtime_checks();
        let this_router: ::axum::Router<()> = ::axum::Router::new()
            .route("/", ::axum::routing::get(__groom_wrapper_foo));
        let mut components = ::groom::extract::ComponentsRegistry::new();
        let mut __groom_paths: ::std::vec::Vec<
            (::std::string::String, ::utoipa::openapi::path::PathItem),
        > = ::std::vec::Vec::new();
        __groom_paths
            .push((
                "/".to_string(),
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(None as Option<String>)
                        .description(None as Option<String>)
                        .operation_id(Some("foo"));
                    op_builder = <Result<
                        Success,
                        Error,
                    >>::__openapi_modify_operation(op_builder, &mut components);
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::path::HttpMethod::Get,
                            op_builder.build(),
                        )
                        .build()
                },
            ));
        ::groom::router::GroomRouter::from_controller_parts(
            this_router,
            components,
            __groom_paths,
        )
    }
    pub fn merge_into_router(
        other: impl Into<::groom::router::GroomRouter<()>>,
    ) -> ::groom::router::GroomRouter<()> {
        __groom_runtime_checks();
        let this_router: ::axum::Router<()> = ::axum::Router::new()
            .route("/", ::axum::routing::get(__groom_wrapper_foo));
        let mut components = ::groom::extract::ComponentsRegistry::new();
        let mut __groom_paths: ::std::vec::Vec<
            (::std::string::String, ::utoipa::openapi::path::PathItem),
        > = ::std::vec::Vec::new();
        __groom_paths
            .push((
                "/".to_string(),
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(None as Option<String>)
                        .description(None as Option<String>)
                        .operation_id(Some("foo"));
                    op_builder = <Result<
                        Success,
                        Error,
                    >>::__openapi_modify_operation(op_builder, &mut components);
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::path::HttpMethod::Get,
                            op_builder.build(),
                        )
                        .build()
                },
            ));
        let __groom_this = ::groom::router::GroomRouter::from_controller_parts(
            this_router,
            components,
            __groom_paths,
        );
        let __groom_other = other.into();
        match __groom_other.merge(__groom_this) {
            ::std::result::Result::Ok(r) => r,
            ::std::result::Result::Err(e) => {
                ::core::panicking::panic_fmt(
                    format_args!("GroomRouter merge failed: {0}", e),
                );
            }
        }
    }
    const _: fn() = || {
        fn assert_impl_all<T: ?Sized + ::groom::response::Response>() {}
        assert_impl_all::<Result<Success, Error>>();
    };
}
mod result_struct_enum {
    use ::static_assertions::{assert_impl_all, assert_impl_any};
    pub struct Success(String);
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Success {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(
                    __serializer,
                    "Success",
                    &self.0,
                )
            }
        }
    };
    impl ::groom::DTO for Success {}
    impl ::groom::DTO_Response for Success {}
    #[allow(non_upper_case_globals)]
    const __GROOM_RESPONSE_SUPPORTED_MIMES_Success: &[::mime::Mime] = &[
        ::mime::TEXT_PLAIN,
    ];
    impl Success {
        fn into_response_text_plain(self) -> ::axum::response::Response {
            (
                match 200u16 {
                    200u16 => ::axum::http::StatusCode::OK,
                    201u16 => ::axum::http::StatusCode::CREATED,
                    202u16 => ::axum::http::StatusCode::ACCEPTED,
                    203u16 => ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION,
                    204u16 => ::axum::http::StatusCode::NO_CONTENT,
                    205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                    206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                    207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                    208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                    226u16 => ::axum::http::StatusCode::IM_USED,
                    300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                    301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                    302u16 => ::axum::http::StatusCode::FOUND,
                    303u16 => ::axum::http::StatusCode::SEE_OTHER,
                    304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                    307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                    308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                    400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                    401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                    402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                    403u16 => ::axum::http::StatusCode::FORBIDDEN,
                    404u16 => ::axum::http::StatusCode::NOT_FOUND,
                    405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                    406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                    407u16 => ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED,
                    408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                    409u16 => ::axum::http::StatusCode::CONFLICT,
                    410u16 => ::axum::http::StatusCode::GONE,
                    411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                    412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                    413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                    414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                    415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                    417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                    418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                    421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                    422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                    423u16 => ::axum::http::StatusCode::LOCKED,
                    424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                    425u16 => ::axum::http::StatusCode::TOO_EARLY,
                    426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                    428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                    429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                    431u16 => ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE,
                    451u16 => ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                    500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                    502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                    503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                    504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                    505u16 => ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED,
                    506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                    507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                    508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                    510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                    511u16 => ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED,
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("groom: status code {0} was validated at expand time",
                                200u16,),
                            ),
                        );
                    }
                },
                Into::<String>::into(self.0),
            )
                .into_response()
        }
    }
    impl ::groom::response::Response for Success {
        fn __groom_into_response(
            self,
            negotiated: Option<&::mime::Mime>,
        ) -> ::axum::response::Response {
            match negotiated {
                None => self.into_response_text_plain(),
                Some(negotiated) => {
                    match (negotiated.type_(), negotiated.subtype()) {
                        (::mime::TEXT, ::mime::PLAIN) => self.into_response_text_plain(),
                        _ => {
                            if true {
                                if !false {
                                    {
                                        ::core::panicking::panic_fmt(
                                            format_args!(
                                                "groom: negotiated mime not covered by response arms",
                                            ),
                                        );
                                    }
                                }
                            }
                            (
                                ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                "internal server error",
                            )
                                .into_response()
                        }
                    }
                }
            }
        }
        fn __openapi_modify_operation(
            op: ::utoipa::openapi::path::OperationBuilder,
            components: &mut ::groom::extract::ComponentsRegistry,
        ) -> ::utoipa::openapi::path::OperationBuilder {
            let op = op
                .response(
                    "200",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("")
                        .content(
                            ::mime::TEXT_PLAIN_UTF_8.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(
                                    Some(
                                        ::groom::extract::ComponentsRegistry::schema_or_ref::<
                                            String,
                                        >(components),
                                    ),
                                )
                                .build(),
                        )
                        .build(),
                );
            let op = op
                .response(
                    "406",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("The requested content type is not supported")
                        .content(
                            ::mime::TEXT_PLAIN.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(
                                    Some(
                                        ::groom::extract::ComponentsRegistry::schema_or_ref::<
                                            String,
                                        >(components),
                                    ),
                                )
                                .build(),
                        )
                        .build(),
                );
            op
        }
        fn __groom_negotiate_content_type(
            accept: &::accept_header::Accept,
        ) -> ::core::result::Result<Option<::mime::Mime>, ::axum::response::Response> {
            match accept.negotiate(&__GROOM_RESPONSE_SUPPORTED_MIMES_Success) {
                Ok(negotiated) => Ok(Some(negotiated)),
                Err(_) => {
                    Err(
                        ::groom::response::not_acceptable(
                            __GROOM_RESPONSE_SUPPORTED_MIMES_Success,
                        ),
                    )
                }
            }
        }
        fn __groom_check_response_codes(
            context: &str,
            codes: &mut ::groom::runtime_checks::HTTPCodeSet,
        ) {
            codes
                .ensure_distinct(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("{0} / struct `Success`", context),
                        )
                    }),
                    200u16,
                )
        }
        fn __groom_check_response_formats(
            context: &str,
            formats: &mut ::groom::runtime_checks::HTTPFormatsSet,
        ) {
            formats.record(context, &__GROOM_RESPONSE_SUPPORTED_MIMES_Success);
        }
    }
    const _: fn() = || {
        use ::static_assertions::_core::marker::PhantomData;
        use ::static_assertions::_core::ops::Deref;
        let previous = AssertImplAnyFallback;
        struct AssertImplAnyFallback;
        struct ActualAssertImplAnyToken;
        trait AssertImplAnyToken {}
        impl AssertImplAnyToken for ActualAssertImplAnyToken {}
        fn assert_impl_any_token<T: AssertImplAnyToken>(_: T) {}
        let previous = {
            struct Wrapper<T, N>(PhantomData<T>, N);
            impl<T, N> Deref for Wrapper<T, N> {
                type Target = N;
                fn deref(&self) -> &Self::Target {
                    &self.1
                }
            }
            impl<T: ::utoipa::PartialSchema, N> Wrapper<T, N> {
                fn _static_assertions_impl_any(&self) -> ActualAssertImplAnyToken {
                    ActualAssertImplAnyToken
                }
            }
            Wrapper::<String, _>(PhantomData, previous)
        };
        let previous = {
            struct Wrapper<T, N>(PhantomData<T>, N);
            impl<T, N> Deref for Wrapper<T, N> {
                type Target = N;
                fn deref(&self) -> &Self::Target {
                    &self.1
                }
            }
            impl<T: ::groom::DTO_Response, N> Wrapper<T, N> {
                fn _static_assertions_impl_any(&self) -> ActualAssertImplAnyToken {
                    ActualAssertImplAnyToken
                }
            }
            Wrapper::<String, _>(PhantomData, previous)
        };
        assert_impl_any_token(previous._static_assertions_impl_any());
    };
    pub enum Error {
        NotFound,
        NoAccess(String),
    }
    #[allow(non_upper_case_globals)]
    const __GROOM_RESPONSE_SUPPORTED_MIMES_Error: &[::mime::Mime] = &[
        ::mime::TEXT_PLAIN,
    ];
    impl Error {
        fn into_response_text_plain(self) -> ::axum::response::Response {
            match self {
                Self::NotFound => {
                    (match 404u16 {
                        200u16 => ::axum::http::StatusCode::OK,
                        201u16 => ::axum::http::StatusCode::CREATED,
                        202u16 => ::axum::http::StatusCode::ACCEPTED,
                        203u16 => ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION,
                        204u16 => ::axum::http::StatusCode::NO_CONTENT,
                        205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                        206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                        207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                        208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                        226u16 => ::axum::http::StatusCode::IM_USED,
                        300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                        301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                        302u16 => ::axum::http::StatusCode::FOUND,
                        303u16 => ::axum::http::StatusCode::SEE_OTHER,
                        304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                        307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                        308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                        400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                        401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                        402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                        403u16 => ::axum::http::StatusCode::FORBIDDEN,
                        404u16 => ::axum::http::StatusCode::NOT_FOUND,
                        405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                        406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                        407u16 => ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED,
                        408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                        409u16 => ::axum::http::StatusCode::CONFLICT,
                        410u16 => ::axum::http::StatusCode::GONE,
                        411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                        412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                        413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                        414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                        415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                        416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                        417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                        418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                        421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                        422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                        423u16 => ::axum::http::StatusCode::LOCKED,
                        424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                        425u16 => ::axum::http::StatusCode::TOO_EARLY,
                        426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                        428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                        429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                        431u16 => {
                            ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE
                        }
                        451u16 => ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                        500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                        502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                        503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                        504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                        505u16 => ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED,
                        506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                        507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                        508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                        510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                        511u16 => {
                            ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED
                        }
                        _ => {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "internal error: entered unreachable code: {0}",
                                    format_args!("groom: status code {0} was validated at expand time",
                                    404u16,),
                                ),
                            );
                        }
                    })
                        .into_response()
                }
                Self::NoAccess(body) => {
                    (
                        match 400u16 {
                            200u16 => ::axum::http::StatusCode::OK,
                            201u16 => ::axum::http::StatusCode::CREATED,
                            202u16 => ::axum::http::StatusCode::ACCEPTED,
                            203u16 => {
                                ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION
                            }
                            204u16 => ::axum::http::StatusCode::NO_CONTENT,
                            205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                            206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                            207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                            208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                            226u16 => ::axum::http::StatusCode::IM_USED,
                            300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                            301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                            302u16 => ::axum::http::StatusCode::FOUND,
                            303u16 => ::axum::http::StatusCode::SEE_OTHER,
                            304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                            307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                            308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                            400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                            401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                            402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                            403u16 => ::axum::http::StatusCode::FORBIDDEN,
                            404u16 => ::axum::http::StatusCode::NOT_FOUND,
                            405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                            406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                            407u16 => {
                                ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED
                            }
                            408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                            409u16 => ::axum::http::StatusCode::CONFLICT,
                            410u16 => ::axum::http::StatusCode::GONE,
                            411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                            412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                            413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                            414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                            415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                            416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                            417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                            418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                            421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                            422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                            423u16 => ::axum::http::StatusCode::LOCKED,
                            424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                            425u16 => ::axum::http::StatusCode::TOO_EARLY,
                            426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                            428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                            429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                            431u16 => {
                                ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE
                            }
                            451u16 => {
                                ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS
                            }
                            500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                            502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                            503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                            504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                            505u16 => {
                                ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED
                            }
                            506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                            507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                            508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                            510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                            511u16 => {
                                ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED
                            }
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("groom: status code {0} was validated at expand time",
                                        400u16,),
                                    ),
                                );
                            }
                        },
                        Into::<String>::into(body),
                    )
                        .into_response()
                }
            }
        }
    }
    impl ::groom::response::Response for Error {
        fn __groom_into_response(
            self,
            negotiated: Option<&::mime::Mime>,
        ) -> ::axum::response::Response {
            match negotiated {
                None => self.into_response_text_plain(),
                Some(negotiated) => {
                    match (negotiated.type_(), negotiated.subtype()) {
                        (::mime::TEXT, ::mime::PLAIN) => self.into_response_text_plain(),
                        _ => {
                            if true {
                                if !false {
                                    {
                                        ::core::panicking::panic_fmt(
                                            format_args!(
                                                "groom: negotiated mime not covered by response arms",
                                            ),
                                        );
                                    }
                                }
                            }
                            (
                                ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                "internal server error",
                            )
                                .into_response()
                        }
                    }
                }
            }
        }
        fn __openapi_modify_operation(
            op: ::utoipa::openapi::path::OperationBuilder,
            components: &mut ::groom::extract::ComponentsRegistry,
        ) -> ::utoipa::openapi::path::OperationBuilder {
            let op = op
                .response(
                    "404",
                    ::utoipa::openapi::ResponseBuilder::new().description("").build(),
                );
            let op = op
                .response(
                    "400",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("")
                        .content(
                            ::mime::TEXT_PLAIN_UTF_8.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(
                                    Some(
                                        ::groom::extract::ComponentsRegistry::schema_or_ref::<
                                            String,
                                        >(components),
                                    ),
                                )
                                .build(),
                        )
                        .build(),
                );
            let op = op
                .response(
                    "406",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("The requested content type is not supported")
                        .content(
                            ::mime::TEXT_PLAIN.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(
                                    Some(
                                        ::groom::extract::ComponentsRegistry::schema_or_ref::<
                                            String,
                                        >(components),
                                    ),
                                )
                                .build(),
                        )
                        .build(),
                );
            op
        }
        fn __groom_negotiate_content_type(
            accept: &::accept_header::Accept,
        ) -> ::core::result::Result<Option<::mime::Mime>, ::axum::response::Response> {
            match accept.negotiate(&__GROOM_RESPONSE_SUPPORTED_MIMES_Error) {
                Ok(negotiated) => Ok(Some(negotiated)),
                Err(_) => {
                    Err(
                        ::groom::response::not_acceptable(
                            __GROOM_RESPONSE_SUPPORTED_MIMES_Error,
                        ),
                    )
                }
            }
        }
        fn __groom_check_response_codes(
            context: &str,
            codes: &mut ::groom::runtime_checks::HTTPCodeSet,
        ) {
            let context = ::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0} / enum `Error`", context))
            });
            codes
                .ensure_distinct(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("{0} / variant `NotFound`", context),
                        )
                    }),
                    404u16,
                );
            codes
                .ensure_distinct(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("{0} / variant `NoAccess`", context),
                        )
                    }),
                    400u16,
                );
        }
        fn __groom_check_response_formats(
            context: &str,
            formats: &mut ::groom::runtime_checks::HTTPFormatsSet,
        ) {
            formats.record(context, &__GROOM_RESPONSE_SUPPORTED_MIMES_Error);
        }
    }
    const _: fn() = || {
        use ::static_assertions::_core::marker::PhantomData;
        use ::static_assertions::_core::ops::Deref;
        let previous = AssertImplAnyFallback;
        struct AssertImplAnyFallback;
        struct ActualAssertImplAnyToken;
        trait AssertImplAnyToken {}
        impl AssertImplAnyToken for ActualAssertImplAnyToken {}
        fn assert_impl_any_token<T: AssertImplAnyToken>(_: T) {}
        let previous = {
            struct Wrapper<T, N>(PhantomData<T>, N);
            impl<T, N> Deref for Wrapper<T, N> {
                type Target = N;
                fn deref(&self) -> &Self::Target {
                    &self.1
                }
            }
            impl<T: ::utoipa::PartialSchema, N> Wrapper<T, N> {
                fn _static_assertions_impl_any(&self) -> ActualAssertImplAnyToken {
                    ActualAssertImplAnyToken
                }
            }
            Wrapper::<String, _>(PhantomData, previous)
        };
        let previous = {
            struct Wrapper<T, N>(PhantomData<T>, N);
            impl<T, N> Deref for Wrapper<T, N> {
                type Target = N;
                fn deref(&self) -> &Self::Target {
                    &self.1
                }
            }
            impl<T: ::groom::DTO_Response, N> Wrapper<T, N> {
                fn _static_assertions_impl_any(&self) -> ActualAssertImplAnyToken {
                    ActualAssertImplAnyToken
                }
            }
            Wrapper::<String, _>(PhantomData, previous)
        };
        assert_impl_any_token(previous._static_assertions_impl_any());
    };
    /// HTTP handler: GET /
    async fn foo() -> Result<Success, Error> {
        Ok(Success("ok".into()))
    }
    async fn __groom_wrapper_foo(
        headers: ::axum::http::header::HeaderMap,
    ) -> impl ::axum::response::IntoResponse {
        let accept = match ::groom::content_negotiation::parse_accept_header(&headers) {
            Err(_) => return ::groom::response::bad_accept_header(),
            Ok(accept) => accept,
        };
        let negotiated = match accept {
            None => None,
            Some(accept) => {
                match <Result<Success, Error>>::__groom_negotiate_content_type(&accept) {
                    Err(response) => return response,
                    Ok(negotiated) => negotiated,
                }
            }
        };
        let result = foo().await;
        result.__groom_into_response(negotiated.as_ref())
    }
    fn __groom_runtime_checks() {
        let context = "Groom runtime check of mod `result_struct_enum`".to_string();
        let mut codes = ::groom::runtime_checks::HTTPCodeSet::new();
        <Result<
            Success,
            Error,
        >>::__groom_check_response_codes(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}: handler `foo`", context))
            }),
            &mut codes,
        );
        let mut formats = ::groom::runtime_checks::HTTPFormatsSet::new();
        <Result<
            Success,
            Error,
        >>::__groom_check_response_formats(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}: handler `foo`", context))
            }),
            &mut formats,
        );
    }
    pub fn into_router() -> ::groom::router::GroomRouter<()> {
        __groom_runtime_checks();
        let this_router: ::axum::Router<()> = ::axum::Router::new()
            .route("/", ::axum::routing::get(__groom_wrapper_foo));
        let mut components = ::groom::extract::ComponentsRegistry::new();
        let mut __groom_paths: ::std::vec::Vec<
            (::std::string::String, ::utoipa::openapi::path::PathItem),
        > = ::std::vec::Vec::new();
        __groom_paths
            .push((
                "/".to_string(),
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(None as Option<String>)
                        .description(None as Option<String>)
                        .operation_id(Some("foo"));
                    op_builder = <Result<
                        Success,
                        Error,
                    >>::__openapi_modify_operation(op_builder, &mut components);
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::path::HttpMethod::Get,
                            op_builder.build(),
                        )
                        .build()
                },
            ));
        ::groom::router::GroomRouter::from_controller_parts(
            this_router,
            components,
            __groom_paths,
        )
    }
    pub fn merge_into_router(
        other: impl Into<::groom::router::GroomRouter<()>>,
    ) -> ::groom::router::GroomRouter<()> {
        __groom_runtime_checks();
        let this_router: ::axum::Router<()> = ::axum::Router::new()
            .route("/", ::axum::routing::get(__groom_wrapper_foo));
        let mut components = ::groom::extract::ComponentsRegistry::new();
        let mut __groom_paths: ::std::vec::Vec<
            (::std::string::String, ::utoipa::openapi::path::PathItem),
        > = ::std::vec::Vec::new();
        __groom_paths
            .push((
                "/".to_string(),
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(None as Option<String>)
                        .description(None as Option<String>)
                        .operation_id(Some("foo"));
                    op_builder = <Result<
                        Success,
                        Error,
                    >>::__openapi_modify_operation(op_builder, &mut components);
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::path::HttpMethod::Get,
                            op_builder.build(),
                        )
                        .build()
                },
            ));
        let __groom_this = ::groom::router::GroomRouter::from_controller_parts(
            this_router,
            components,
            __groom_paths,
        );
        let __groom_other = other.into();
        match __groom_other.merge(__groom_this) {
            ::std::result::Result::Ok(r) => r,
            ::std::result::Result::Err(e) => {
                ::core::panicking::panic_fmt(
                    format_args!("GroomRouter merge failed: {0}", e),
                );
            }
        }
    }
    const _: fn() = || {
        fn assert_impl_all<T: ?Sized + ::groom::response::Response>() {}
        assert_impl_all::<Result<Success, Error>>();
    };
}
mod wrapped_enum {
    use ::static_assertions::{assert_impl_all, assert_impl_any};
    pub enum EnumValueObject {
        UnitVariant,
        UnnamedStructVariant(String),
        NamedStructVariant { value: String },
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for EnumValueObject {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    EnumValueObject::UnitVariant => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "EnumValueObject",
                            0u32,
                            "UnitVariant",
                        )
                    }
                    EnumValueObject::UnnamedStructVariant(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "EnumValueObject",
                            1u32,
                            "UnnamedStructVariant",
                            __field0,
                        )
                    }
                    EnumValueObject::NamedStructVariant { ref value } => {
                        let mut __serde_state = _serde::Serializer::serialize_struct_variant(
                            __serializer,
                            "EnumValueObject",
                            2u32,
                            "NamedStructVariant",
                            0 + 1,
                        )?;
                        _serde::ser::SerializeStructVariant::serialize_field(
                            &mut __serde_state,
                            "value",
                            value,
                        )?;
                        _serde::ser::SerializeStructVariant::end(__serde_state)
                    }
                }
            }
        }
    };
    impl ::groom::DTO for EnumValueObject {}
    impl ::groom::DTO_Response for EnumValueObject {}
    pub struct WrapperStruct {
        pub v: EnumValueObject,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for WrapperStruct {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "WrapperStruct",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "v",
                    &self.v,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    impl ::groom::DTO for WrapperStruct {}
    impl ::groom::DTO_Response for WrapperStruct {}
    pub enum Resp {
        Enum(EnumValueObject),
        StructWithEnum(WrapperStruct),
    }
    #[allow(non_upper_case_globals)]
    const __GROOM_RESPONSE_SUPPORTED_MIMES_Resp: &[::mime::Mime] = &[
        ::mime::APPLICATION_JSON,
    ];
    impl Resp {
        fn into_response_application_json(self) -> ::axum::response::Response {
            match self {
                Self::Enum(body) => {
                    (
                        match 200u16 {
                            200u16 => ::axum::http::StatusCode::OK,
                            201u16 => ::axum::http::StatusCode::CREATED,
                            202u16 => ::axum::http::StatusCode::ACCEPTED,
                            203u16 => {
                                ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION
                            }
                            204u16 => ::axum::http::StatusCode::NO_CONTENT,
                            205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                            206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                            207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                            208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                            226u16 => ::axum::http::StatusCode::IM_USED,
                            300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                            301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                            302u16 => ::axum::http::StatusCode::FOUND,
                            303u16 => ::axum::http::StatusCode::SEE_OTHER,
                            304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                            307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                            308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                            400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                            401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                            402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                            403u16 => ::axum::http::StatusCode::FORBIDDEN,
                            404u16 => ::axum::http::StatusCode::NOT_FOUND,
                            405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                            406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                            407u16 => {
                                ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED
                            }
                            408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                            409u16 => ::axum::http::StatusCode::CONFLICT,
                            410u16 => ::axum::http::StatusCode::GONE,
                            411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                            412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                            413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                            414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                            415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                            416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                            417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                            418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                            421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                            422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                            423u16 => ::axum::http::StatusCode::LOCKED,
                            424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                            425u16 => ::axum::http::StatusCode::TOO_EARLY,
                            426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                            428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                            429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                            431u16 => {
                                ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE
                            }
                            451u16 => {
                                ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS
                            }
                            500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                            502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                            503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                            504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                            505u16 => {
                                ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED
                            }
                            506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                            507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                            508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                            510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                            511u16 => {
                                ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED
                            }
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("groom: status code {0} was validated at expand time",
                                        200u16,),
                                    ),
                                );
                            }
                        },
                        ::axum::Json(body),
                    )
                        .into_response()
                }
                Self::StructWithEnum(body) => {
                    (
                        match 202u16 {
                            200u16 => ::axum::http::StatusCode::OK,
                            201u16 => ::axum::http::StatusCode::CREATED,
                            202u16 => ::axum::http::StatusCode::ACCEPTED,
                            203u16 => {
                                ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION
                            }
                            204u16 => ::axum::http::StatusCode::NO_CONTENT,
                            205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                            206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                            207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                            208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                            226u16 => ::axum::http::StatusCode::IM_USED,
                            300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                            301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                            302u16 => ::axum::http::StatusCode::FOUND,
                            303u16 => ::axum::http::StatusCode::SEE_OTHER,
                            304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                            307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                            308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                            400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                            401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                            402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                            403u16 => ::axum::http::StatusCode::FORBIDDEN,
                            404u16 => ::axum::http::StatusCode::NOT_FOUND,
                            405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                            406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                            407u16 => {
                                ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED
                            }
                            408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                            409u16 => ::axum::http::StatusCode::CONFLICT,
                            410u16 => ::axum::http::StatusCode::GONE,
                            411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                            412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                            413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                            414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                            415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                            416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                            417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                            418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                            421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                            422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                            423u16 => ::axum::http::StatusCode::LOCKED,
                            424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                            425u16 => ::axum::http::StatusCode::TOO_EARLY,
                            426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                            428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                            429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                            431u16 => {
                                ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE
                            }
                            451u16 => {
                                ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS
                            }
                            500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                            502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                            503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                            504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                            505u16 => {
                                ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED
                            }
                            506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                            507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                            508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                            510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                            511u16 => {
                                ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED
                            }
                            _ => {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "internal error: entered unreachable code: {0}",
                                        format_args!("groom: status code {0} was validated at expand time",
                                        202u16,),
                                    ),
                                );
                            }
                        },
                        ::axum::Json(body),
                    )
                        .into_response()
                }
            }
        }
    }
    impl ::groom::response::Response for Resp {
        fn __groom_into_response(
            self,
            negotiated: Option<&::mime::Mime>,
        ) -> ::axum::response::Response {
            match negotiated {
                None => self.into_response_application_json(),
                Some(negotiated) => {
                    match (negotiated.type_(), negotiated.subtype()) {
                        (::mime::APPLICATION, ::mime::JSON) => {
                            self.into_response_application_json()
                        }
                        _ => {
                            if true {
                                if !false {
                                    {
                                        ::core::panicking::panic_fmt(
                                            format_args!(
                                                "groom: negotiated mime not covered by response arms",
                                            ),
                                        );
                                    }
                                }
                            }
                            (
                                ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                "internal server error",
                            )
                                .into_response()
                        }
                    }
                }
            }
        }
        fn __openapi_modify_operation(
            op: ::utoipa::openapi::path::OperationBuilder,
            components: &mut ::groom::extract::ComponentsRegistry,
        ) -> ::utoipa::openapi::path::OperationBuilder {
            let op = op
                .response(
                    "200",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("")
                        .content(
                            ::mime::APPLICATION_JSON.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(
                                    Some(components.add_components::<EnumValueObject>()),
                                )
                                .build(),
                        )
                        .build(),
                );
            let op = op
                .response(
                    "202",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("")
                        .content(
                            ::mime::APPLICATION_JSON.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(Some(components.add_components::<WrapperStruct>()))
                                .build(),
                        )
                        .build(),
                );
            let op = op
                .response(
                    "406",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("The requested content type is not supported")
                        .content(
                            ::mime::TEXT_PLAIN.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(
                                    Some(
                                        ::groom::extract::ComponentsRegistry::schema_or_ref::<
                                            String,
                                        >(components),
                                    ),
                                )
                                .build(),
                        )
                        .build(),
                );
            op
        }
        fn __groom_negotiate_content_type(
            accept: &::accept_header::Accept,
        ) -> ::core::result::Result<Option<::mime::Mime>, ::axum::response::Response> {
            match accept.negotiate(&__GROOM_RESPONSE_SUPPORTED_MIMES_Resp) {
                Ok(negotiated) => Ok(Some(negotiated)),
                Err(_) => {
                    Err(
                        ::groom::response::not_acceptable(
                            __GROOM_RESPONSE_SUPPORTED_MIMES_Resp,
                        ),
                    )
                }
            }
        }
        fn __groom_check_response_codes(
            context: &str,
            codes: &mut ::groom::runtime_checks::HTTPCodeSet,
        ) {
            let context = ::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0} / enum `Resp`", context))
            });
            codes
                .ensure_distinct(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("{0} / variant `Enum`", context),
                        )
                    }),
                    200u16,
                );
            codes
                .ensure_distinct(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("{0} / variant `StructWithEnum`", context),
                        )
                    }),
                    202u16,
                );
        }
        fn __groom_check_response_formats(
            context: &str,
            formats: &mut ::groom::runtime_checks::HTTPFormatsSet,
        ) {
            formats.record(context, &__GROOM_RESPONSE_SUPPORTED_MIMES_Resp);
        }
    }
    const _: fn() = || {
        use ::static_assertions::_core::marker::PhantomData;
        use ::static_assertions::_core::ops::Deref;
        let previous = AssertImplAnyFallback;
        struct AssertImplAnyFallback;
        struct ActualAssertImplAnyToken;
        trait AssertImplAnyToken {}
        impl AssertImplAnyToken for ActualAssertImplAnyToken {}
        fn assert_impl_any_token<T: AssertImplAnyToken>(_: T) {}
        let previous = {
            struct Wrapper<T, N>(PhantomData<T>, N);
            impl<T, N> Deref for Wrapper<T, N> {
                type Target = N;
                fn deref(&self) -> &Self::Target {
                    &self.1
                }
            }
            impl<T: ::utoipa::PartialSchema, N> Wrapper<T, N> {
                fn _static_assertions_impl_any(&self) -> ActualAssertImplAnyToken {
                    ActualAssertImplAnyToken
                }
            }
            Wrapper::<EnumValueObject, _>(PhantomData, previous)
        };
        let previous = {
            struct Wrapper<T, N>(PhantomData<T>, N);
            impl<T, N> Deref for Wrapper<T, N> {
                type Target = N;
                fn deref(&self) -> &Self::Target {
                    &self.1
                }
            }
            impl<T: ::groom::DTO_Response, N> Wrapper<T, N> {
                fn _static_assertions_impl_any(&self) -> ActualAssertImplAnyToken {
                    ActualAssertImplAnyToken
                }
            }
            Wrapper::<EnumValueObject, _>(PhantomData, previous)
        };
        assert_impl_any_token(previous._static_assertions_impl_any());
    };
    const _: fn() = || {
        use ::static_assertions::_core::marker::PhantomData;
        use ::static_assertions::_core::ops::Deref;
        let previous = AssertImplAnyFallback;
        struct AssertImplAnyFallback;
        struct ActualAssertImplAnyToken;
        trait AssertImplAnyToken {}
        impl AssertImplAnyToken for ActualAssertImplAnyToken {}
        fn assert_impl_any_token<T: AssertImplAnyToken>(_: T) {}
        let previous = {
            struct Wrapper<T, N>(PhantomData<T>, N);
            impl<T, N> Deref for Wrapper<T, N> {
                type Target = N;
                fn deref(&self) -> &Self::Target {
                    &self.1
                }
            }
            impl<T: ::utoipa::PartialSchema, N> Wrapper<T, N> {
                fn _static_assertions_impl_any(&self) -> ActualAssertImplAnyToken {
                    ActualAssertImplAnyToken
                }
            }
            Wrapper::<WrapperStruct, _>(PhantomData, previous)
        };
        let previous = {
            struct Wrapper<T, N>(PhantomData<T>, N);
            impl<T, N> Deref for Wrapper<T, N> {
                type Target = N;
                fn deref(&self) -> &Self::Target {
                    &self.1
                }
            }
            impl<T: ::groom::DTO_Response, N> Wrapper<T, N> {
                fn _static_assertions_impl_any(&self) -> ActualAssertImplAnyToken {
                    ActualAssertImplAnyToken
                }
            }
            Wrapper::<WrapperStruct, _>(PhantomData, previous)
        };
        assert_impl_any_token(previous._static_assertions_impl_any());
    };
    pub struct Error {
        pub message: &'static str,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Error {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private228::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Error",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "message",
                    &self.message,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    impl ::groom::DTO for Error {}
    impl ::groom::DTO_Response for Error {}
    #[allow(non_upper_case_globals)]
    const __GROOM_RESPONSE_SUPPORTED_MIMES_Error: &[::mime::Mime] = &[
        ::mime::APPLICATION_JSON,
    ];
    impl Error {
        fn into_response_application_json(self) -> ::axum::response::Response {
            (
                match 404u16 {
                    200u16 => ::axum::http::StatusCode::OK,
                    201u16 => ::axum::http::StatusCode::CREATED,
                    202u16 => ::axum::http::StatusCode::ACCEPTED,
                    203u16 => ::axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION,
                    204u16 => ::axum::http::StatusCode::NO_CONTENT,
                    205u16 => ::axum::http::StatusCode::RESET_CONTENT,
                    206u16 => ::axum::http::StatusCode::PARTIAL_CONTENT,
                    207u16 => ::axum::http::StatusCode::MULTI_STATUS,
                    208u16 => ::axum::http::StatusCode::ALREADY_REPORTED,
                    226u16 => ::axum::http::StatusCode::IM_USED,
                    300u16 => ::axum::http::StatusCode::MULTIPLE_CHOICES,
                    301u16 => ::axum::http::StatusCode::MOVED_PERMANENTLY,
                    302u16 => ::axum::http::StatusCode::FOUND,
                    303u16 => ::axum::http::StatusCode::SEE_OTHER,
                    304u16 => ::axum::http::StatusCode::NOT_MODIFIED,
                    307u16 => ::axum::http::StatusCode::TEMPORARY_REDIRECT,
                    308u16 => ::axum::http::StatusCode::PERMANENT_REDIRECT,
                    400u16 => ::axum::http::StatusCode::BAD_REQUEST,
                    401u16 => ::axum::http::StatusCode::UNAUTHORIZED,
                    402u16 => ::axum::http::StatusCode::PAYMENT_REQUIRED,
                    403u16 => ::axum::http::StatusCode::FORBIDDEN,
                    404u16 => ::axum::http::StatusCode::NOT_FOUND,
                    405u16 => ::axum::http::StatusCode::METHOD_NOT_ALLOWED,
                    406u16 => ::axum::http::StatusCode::NOT_ACCEPTABLE,
                    407u16 => ::axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED,
                    408u16 => ::axum::http::StatusCode::REQUEST_TIMEOUT,
                    409u16 => ::axum::http::StatusCode::CONFLICT,
                    410u16 => ::axum::http::StatusCode::GONE,
                    411u16 => ::axum::http::StatusCode::LENGTH_REQUIRED,
                    412u16 => ::axum::http::StatusCode::PRECONDITION_FAILED,
                    413u16 => ::axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                    414u16 => ::axum::http::StatusCode::URI_TOO_LONG,
                    415u16 => ::axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    416u16 => ::axum::http::StatusCode::RANGE_NOT_SATISFIABLE,
                    417u16 => ::axum::http::StatusCode::EXPECTATION_FAILED,
                    418u16 => ::axum::http::StatusCode::IM_A_TEAPOT,
                    421u16 => ::axum::http::StatusCode::MISDIRECTED_REQUEST,
                    422u16 => ::axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                    423u16 => ::axum::http::StatusCode::LOCKED,
                    424u16 => ::axum::http::StatusCode::FAILED_DEPENDENCY,
                    425u16 => ::axum::http::StatusCode::TOO_EARLY,
                    426u16 => ::axum::http::StatusCode::UPGRADE_REQUIRED,
                    428u16 => ::axum::http::StatusCode::PRECONDITION_REQUIRED,
                    429u16 => ::axum::http::StatusCode::TOO_MANY_REQUESTS,
                    431u16 => ::axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE,
                    451u16 => ::axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                    500u16 => ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    501u16 => ::axum::http::StatusCode::NOT_IMPLEMENTED,
                    502u16 => ::axum::http::StatusCode::BAD_GATEWAY,
                    503u16 => ::axum::http::StatusCode::SERVICE_UNAVAILABLE,
                    504u16 => ::axum::http::StatusCode::GATEWAY_TIMEOUT,
                    505u16 => ::axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED,
                    506u16 => ::axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES,
                    507u16 => ::axum::http::StatusCode::INSUFFICIENT_STORAGE,
                    508u16 => ::axum::http::StatusCode::LOOP_DETECTED,
                    510u16 => ::axum::http::StatusCode::NOT_EXTENDED,
                    511u16 => ::axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED,
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("groom: status code {0} was validated at expand time",
                                404u16,),
                            ),
                        );
                    }
                },
                ::axum::Json(self),
            )
                .into_response()
        }
    }
    impl ::groom::response::Response for Error {
        fn __groom_into_response(
            self,
            negotiated: Option<&::mime::Mime>,
        ) -> ::axum::response::Response {
            match negotiated {
                None => self.into_response_application_json(),
                Some(negotiated) => {
                    match (negotiated.type_(), negotiated.subtype()) {
                        (::mime::APPLICATION, ::mime::JSON) => {
                            self.into_response_application_json()
                        }
                        _ => {
                            if true {
                                if !false {
                                    {
                                        ::core::panicking::panic_fmt(
                                            format_args!(
                                                "groom: negotiated mime not covered by response arms",
                                            ),
                                        );
                                    }
                                }
                            }
                            (
                                ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                "internal server error",
                            )
                                .into_response()
                        }
                    }
                }
            }
        }
        fn __openapi_modify_operation(
            op: ::utoipa::openapi::path::OperationBuilder,
            components: &mut ::groom::extract::ComponentsRegistry,
        ) -> ::utoipa::openapi::path::OperationBuilder {
            let op = op
                .response(
                    "404",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("")
                        .content(
                            ::mime::APPLICATION_JSON.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(Some(components.add_components::<Error>()))
                                .build(),
                        )
                        .build(),
                );
            let op = op
                .response(
                    "406",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("The requested content type is not supported")
                        .content(
                            ::mime::TEXT_PLAIN.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(
                                    Some(
                                        ::groom::extract::ComponentsRegistry::schema_or_ref::<
                                            String,
                                        >(components),
                                    ),
                                )
                                .build(),
                        )
                        .build(),
                );
            op
        }
        fn __groom_negotiate_content_type(
            accept: &::accept_header::Accept,
        ) -> ::core::result::Result<Option<::mime::Mime>, ::axum::response::Response> {
            match accept.negotiate(&__GROOM_RESPONSE_SUPPORTED_MIMES_Error) {
                Ok(negotiated) => Ok(Some(negotiated)),
                Err(_) => {
                    Err(
                        ::groom::response::not_acceptable(
                            __GROOM_RESPONSE_SUPPORTED_MIMES_Error,
                        ),
                    )
                }
            }
        }
        fn __groom_check_response_codes(
            context: &str,
            codes: &mut ::groom::runtime_checks::HTTPCodeSet,
        ) {
            codes
                .ensure_distinct(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("{0} / struct `Error`", context),
                        )
                    }),
                    404u16,
                )
        }
        fn __groom_check_response_formats(
            context: &str,
            formats: &mut ::groom::runtime_checks::HTTPFormatsSet,
        ) {
            formats.record(context, &__GROOM_RESPONSE_SUPPORTED_MIMES_Error);
        }
    }
    const _: fn() = || {
        use ::static_assertions::_core::marker::PhantomData;
        use ::static_assertions::_core::ops::Deref;
        let previous = AssertImplAnyFallback;
        struct AssertImplAnyFallback;
        struct ActualAssertImplAnyToken;
        trait AssertImplAnyToken {}
        impl AssertImplAnyToken for ActualAssertImplAnyToken {}
        fn assert_impl_any_token<T: AssertImplAnyToken>(_: T) {}
        let previous = {
            struct Wrapper<T, N>(PhantomData<T>, N);
            impl<T, N> Deref for Wrapper<T, N> {
                type Target = N;
                fn deref(&self) -> &Self::Target {
                    &self.1
                }
            }
            impl<T: ::utoipa::PartialSchema, N> Wrapper<T, N> {
                fn _static_assertions_impl_any(&self) -> ActualAssertImplAnyToken {
                    ActualAssertImplAnyToken
                }
            }
            Wrapper::<Error, _>(PhantomData, previous)
        };
        let previous = {
            struct Wrapper<T, N>(PhantomData<T>, N);
            impl<T, N> Deref for Wrapper<T, N> {
                type Target = N;
                fn deref(&self) -> &Self::Target {
                    &self.1
                }
            }
            impl<T: ::groom::DTO_Response, N> Wrapper<T, N> {
                fn _static_assertions_impl_any(&self) -> ActualAssertImplAnyToken {
                    ActualAssertImplAnyToken
                }
            }
            Wrapper::<Error, _>(PhantomData, previous)
        };
        assert_impl_any_token(previous._static_assertions_impl_any());
    };
    /// HTTP handler: GET /
    async fn foo() -> Result<Resp, Error> {
        Ok(
            Resp::StructWithEnum(WrapperStruct {
                v: EnumValueObject::NamedStructVariant {
                    value: "hello, world",
                },
            }),
        )
    }
    async fn __groom_wrapper_foo(
        headers: ::axum::http::header::HeaderMap,
    ) -> impl ::axum::response::IntoResponse {
        let accept = match ::groom::content_negotiation::parse_accept_header(&headers) {
            Err(_) => return ::groom::response::bad_accept_header(),
            Ok(accept) => accept,
        };
        let negotiated = match accept {
            None => None,
            Some(accept) => {
                match <Result<Resp, Error>>::__groom_negotiate_content_type(&accept) {
                    Err(response) => return response,
                    Ok(negotiated) => negotiated,
                }
            }
        };
        let result = foo().await;
        result.__groom_into_response(negotiated.as_ref())
    }
    fn __groom_runtime_checks() {
        let context = "Groom runtime check of mod `wrapped_enum`".to_string();
        let mut codes = ::groom::runtime_checks::HTTPCodeSet::new();
        <Result<
            Resp,
            Error,
        >>::__groom_check_response_codes(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}: handler `foo`", context))
            }),
            &mut codes,
        );
        let mut formats = ::groom::runtime_checks::HTTPFormatsSet::new();
        <Result<
            Resp,
            Error,
        >>::__groom_check_response_formats(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}: handler `foo`", context))
            }),
            &mut formats,
        );
    }
    pub fn into_router() -> ::groom::router::GroomRouter<()> {
        __groom_runtime_checks();
        let this_router: ::axum::Router<()> = ::axum::Router::new()
            .route("/", ::axum::routing::get(__groom_wrapper_foo));
        let mut components = ::groom::extract::ComponentsRegistry::new();
        let mut __groom_paths: ::std::vec::Vec<
            (::std::string::String, ::utoipa::openapi::path::PathItem),
        > = ::std::vec::Vec::new();
        __groom_paths
            .push((
                "/".to_string(),
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(None as Option<String>)
                        .description(None as Option<String>)
                        .operation_id(Some("foo"));
                    op_builder = <Result<
                        Resp,
                        Error,
                    >>::__openapi_modify_operation(op_builder, &mut components);
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::path::HttpMethod::Get,
                            op_builder.build(),
                        )
                        .build()
                },
            ));
        ::groom::router::GroomRouter::from_controller_parts(
            this_router,
            components,
            __groom_paths,
        )
    }
    pub fn merge_into_router(
        other: impl Into<::groom::router::GroomRouter<()>>,
    ) -> ::groom::router::GroomRouter<()> {
        __groom_runtime_checks();
        let this_router: ::axum::Router<()> = ::axum::Router::new()
            .route("/", ::axum::routing::get(__groom_wrapper_foo));
        let mut components = ::groom::extract::ComponentsRegistry::new();
        let mut __groom_paths: ::std::vec::Vec<
            (::std::string::String, ::utoipa::openapi::path::PathItem),
        > = ::std::vec::Vec::new();
        __groom_paths
            .push((
                "/".to_string(),
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(None as Option<String>)
                        .description(None as Option<String>)
                        .operation_id(Some("foo"));
                    op_builder = <Result<
                        Resp,
                        Error,
                    >>::__openapi_modify_operation(op_builder, &mut components);
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::path::HttpMethod::Get,
                            op_builder.build(),
                        )
                        .build()
                },
            ));
        let __groom_this = ::groom::router::GroomRouter::from_controller_parts(
            this_router,
            components,
            __groom_paths,
        );
        let __groom_other = other.into();
        match __groom_other.merge(__groom_this) {
            ::std::result::Result::Ok(r) => r,
            ::std::result::Result::Err(e) => {
                ::core::panicking::panic_fmt(
                    format_args!("GroomRouter merge failed: {0}", e),
                );
            }
        }
    }
    const _: fn() = || {
        fn assert_impl_all<T: ?Sized + ::groom::response::Response>() {}
        assert_impl_all::<Result<Resp, Error>>();
    };
}
