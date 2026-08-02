//! This is an old expansion preview that is too messy and should be broken down into separate
//! files and then removed.
#[macro_use]
extern crate groom_macros;
pub mod api_root {
    use ::static_assertions::{assert_impl_all, assert_impl_any};
    use axum::extract::{Path, Query};
    /// Summary
    ///
    /// HTTP handler: GET /
    ///
    /// Description
    pub async fn get_root() -> GetRootResponse {
        let a = 1;
    }
    async fn __groom_wrapper_get_root(
        headers: ::axum::http::header::HeaderMap,
    ) -> impl ::axum::response::IntoResponse {
        let accept = match ::groom::content_negotiation::parse_accept_header(&headers) {
            Err(_) => return ::groom::response::bad_accept_header(),
            Ok(accept) => accept,
        };
        let negotiated = match accept {
            None => None,
            Some(accept) => {
                match <GetRootResponse>::__groom_negotiate_content_type(&accept) {
                    Err(response) => return response,
                    Ok(negotiated) => negotiated,
                }
            }
        };
        let result = get_root().await;
        result.__groom_into_response(negotiated.as_ref())
    }
    /// HTTP handler: POST /
    pub async fn post_root() -> GetRootResponse {
        let a = 2;
    }
    async fn __groom_wrapper_post_root(
        headers: ::axum::http::header::HeaderMap,
    ) -> impl ::axum::response::IntoResponse {
        let accept = match ::groom::content_negotiation::parse_accept_header(&headers) {
            Err(_) => return ::groom::response::bad_accept_header(),
            Ok(accept) => accept,
        };
        let negotiated = match accept {
            None => None,
            Some(accept) => {
                match <GetRootResponse>::__groom_negotiate_content_type(&accept) {
                    Err(response) => return response,
                    Ok(negotiated) => negotiated,
                }
            }
        };
        let result = post_root().await;
        result.__groom_into_response(negotiated.as_ref())
    }
    fn sync_util_fn(s: String) -> String {
        s
    }
    /// Query<struct>
    ///
    /// HTTP handler: GET /greet
    pub async fn rq_cons_query_struct(
        query: Query<RqConsQueryStruct>,
    ) -> RqConsQueryResponse {
        if query.name.is_empty() {
            RqConsQueryResponse::BadRequest("Empty string".into())
        } else {
            let mut result = "Hello, ".to_owned();
            result.push_str(query.name);
            RqConsQueryResponse::Ok(sync_util_fn(result))
        }
    }
    async fn __groom_wrapper_rq_cons_query_struct(
        headers: ::axum::http::header::HeaderMap,
        input0: Query<RqConsQueryStruct>,
    ) -> impl ::axum::response::IntoResponse {
        let accept = match ::groom::content_negotiation::parse_accept_header(&headers) {
            Err(_) => return ::groom::response::bad_accept_header(),
            Ok(accept) => accept,
        };
        let negotiated = match accept {
            None => None,
            Some(accept) => {
                match <RqConsQueryResponse>::__groom_negotiate_content_type(&accept) {
                    Err(response) => return response,
                    Ok(negotiated) => negotiated,
                }
            }
        };
        let result = rq_cons_query_struct(input0).await;
        result.__groom_into_response(negotiated.as_ref())
    }
    /// Path<struct>
    ///
    /// HTTP handler: GET /team/:team_id/user/:user_id
    pub async fn rq_cons_path_struct(
        Path(team): Path<RqConsPathStruct>,
    ) -> RqConsPathResponse {
        RqConsPathResponse::Ok("ok".into())
    }
    async fn __groom_wrapper_rq_cons_path_struct(
        headers: ::axum::http::header::HeaderMap,
        input0: Path<RqConsPathStruct>,
    ) -> impl ::axum::response::IntoResponse {
        let accept = match ::groom::content_negotiation::parse_accept_header(&headers) {
            Err(_) => return ::groom::response::bad_accept_header(),
            Ok(accept) => accept,
        };
        let negotiated = match accept {
            None => None,
            Some(accept) => {
                match <RqConsPathResponse>::__groom_negotiate_content_type(&accept) {
                    Err(response) => return response,
                    Ok(negotiated) => negotiated,
                }
            }
        };
        let result = rq_cons_path_struct(input0).await;
        result.__groom_into_response(negotiated.as_ref())
    }
    /// HTTP handler: GET /json
    pub async fn resp_json() -> RespJsonResponse {
        RespJsonResponse::Ok(StructJson { success: true })
    }
    async fn __groom_wrapper_resp_json(
        headers: ::axum::http::header::HeaderMap,
    ) -> impl ::axum::response::IntoResponse {
        let accept = match ::groom::content_negotiation::parse_accept_header(&headers) {
            Err(_) => return ::groom::response::bad_accept_header(),
            Ok(accept) => accept,
        };
        let negotiated = match accept {
            None => None,
            Some(accept) => {
                match <RespJsonResponse>::__groom_negotiate_content_type(&accept) {
                    Err(response) => return response,
                    Ok(negotiated) => negotiated,
                }
            }
        };
        let result = resp_json().await;
        result.__groom_into_response(negotiated.as_ref())
    }
    async fn not_a_handler() {
        let a = 1;
    }
    pub enum GetRootResponse {
        /// There you go mate.
        Ok(String),
        /// Are you insane?
        ///
        /// Bad request.
        BadRequest(String),
        /// You shall not pass!
        Forbidden,
    }
    #[allow(non_upper_case_globals)]
    const __GROOM_RESPONSE_SUPPORTED_MIMES_GetRootResponse: &[::mime::Mime] = &[
        ::mime::TEXT_PLAIN,
    ];
    impl GetRootResponse {
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
                Self::BadRequest(body) => {
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
                Self::Forbidden => {
                    (match 401u16 {
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
                                    401u16,),
                                ),
                            );
                        }
                    })
                        .into_response()
                }
            }
        }
    }
    impl ::groom::response::Response for GetRootResponse {
        fn __groom_into_response(
            self,
            negotiated: Option<&::mime::Mime>,
        ) -> ::axum::response::Response {
            match negotiated {
                None => self.into_response_text_plain(),
                Some(negotiated) => {
                    match (negotiated.type_(), negotiated.subtype()) {
                        (::mime::TEXT, mime::PLAIN) => self.into_response_text_plain(),
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
                        .description("There you go mate.")
                        .content(
                            ::mime::TEXT_PLAIN_UTF_8.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema({
                                    match <String as utoipa::PartialSchema>::schema() {
                                        ::utoipa::openapi::RefOr::T(s) => Some(s),
                                        ::utoipa::openapi::RefOr::Ref(_) => {
                                            ::core::panicking::panic_fmt(
                                                format_args!("String schema for plain_text is ref"),
                                            );
                                        }
                                    }
                                })
                                .build(),
                        )
                        .build(),
                );
            components.add_components::<String>();
            let op = op
                .response(
                    "400",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("Are you insane?\n\nBad request.")
                        .content(
                            ::mime::TEXT_PLAIN_UTF_8.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema({
                                    match <String as utoipa::PartialSchema>::schema() {
                                        ::utoipa::openapi::RefOr::T(s) => Some(s),
                                        ::utoipa::openapi::RefOr::Ref(_) => {
                                            ::core::panicking::panic_fmt(
                                                format_args!("String schema for plain_text is ref"),
                                            );
                                        }
                                    }
                                })
                                .build(),
                        )
                        .build(),
                );
            components.add_components::<String>();
            let op = op
                .response(
                    "401",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("You shall not pass!")
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
                                .schema({
                                    match <String as utoipa::PartialSchema>::schema() {
                                        ::utoipa::openapi::RefOr::T(s) => Some(s),
                                        ::utoipa::openapi::RefOr::Ref(_) => {
                                            ::core::panicking::panic_fmt(
                                                format_args!("String schema for plain_text is ref"),
                                            );
                                        }
                                    }
                                })
                                .build(),
                        )
                        .build(),
                );
            op
        }
        fn __groom_negotiate_content_type(
            accept: &::accept_header::Accept,
        ) -> ::core::result::Result<Option<::mime::Mime>, ::axum::response::Response> {
            match accept.negotiate(&__GROOM_RESPONSE_SUPPORTED_MIMES_GetRootResponse) {
                Ok(negotiated) => Ok(Some(negotiated)),
                Err(_) => {
                    Err(
                        ::groom::response::not_acceptable(
                            __GROOM_RESPONSE_SUPPORTED_MIMES_GetRootResponse,
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
                    format_args!("{0} / enum `GetRootResponse`", context),
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
                            format_args!("{0} / variant `BadRequest`", context),
                        )
                    }),
                    400u16,
                );
            codes
                .ensure_distinct(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!("{0} / variant `Forbidden`", context),
                        )
                    }),
                    401u16,
                );
        }
        fn __groom_check_response_formats(
            context: &str,
            formats: &mut ::groom::runtime_checks::HTTPFormatsSet,
        ) {
            formats.record(context, &__GROOM_RESPONSE_SUPPORTED_MIMES_GetRootResponse);
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
    pub struct RqConsQueryStruct {
        name: String,
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
        impl<'de> _serde::Deserialize<'de> for RqConsQueryStruct {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private228::Ok(__Field::__field0),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private228::Ok(__Field::__field0),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<RqConsQueryStruct>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = RqConsQueryStruct;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct RqConsQueryStruct",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct RqConsQueryStruct with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private228::Ok(RqConsQueryStruct {
                            name: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private228::Option<String> = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private228::Some(__field0) => __field0,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("name")?
                            }
                        };
                        _serde::__private228::Ok(RqConsQueryStruct {
                            name: __field0,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["name"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "RqConsQueryStruct",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<RqConsQueryStruct>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    impl ::groom::DTO for RqConsQueryStruct {}
    impl ::groom::DTO_Request for RqConsQueryStruct {}
    pub enum RqConsQueryResponse {
        Ok(String),
        BadRequest(String),
    }
    #[allow(non_upper_case_globals)]
    const __GROOM_RESPONSE_SUPPORTED_MIMES_RqConsQueryResponse: &[::mime::Mime] = &[
        ::mime::TEXT_PLAIN,
    ];
    impl RqConsQueryResponse {
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
                Self::BadRequest(body) => {
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
    impl ::groom::response::Response for RqConsQueryResponse {
        fn __groom_into_response(
            self,
            negotiated: Option<&::mime::Mime>,
        ) -> ::axum::response::Response {
            match negotiated {
                None => self.into_response_text_plain(),
                Some(negotiated) => {
                    match (negotiated.type_(), negotiated.subtype()) {
                        (::mime::TEXT, mime::PLAIN) => self.into_response_text_plain(),
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
                                .schema({
                                    match <String as utoipa::PartialSchema>::schema() {
                                        ::utoipa::openapi::RefOr::T(s) => Some(s),
                                        ::utoipa::openapi::RefOr::Ref(_) => {
                                            ::core::panicking::panic_fmt(
                                                format_args!("String schema for plain_text is ref"),
                                            );
                                        }
                                    }
                                })
                                .build(),
                        )
                        .build(),
                );
            components.add_components::<String>();
            let op = op
                .response(
                    "400",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("")
                        .content(
                            ::mime::TEXT_PLAIN_UTF_8.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema({
                                    match <String as utoipa::PartialSchema>::schema() {
                                        ::utoipa::openapi::RefOr::T(s) => Some(s),
                                        ::utoipa::openapi::RefOr::Ref(_) => {
                                            ::core::panicking::panic_fmt(
                                                format_args!("String schema for plain_text is ref"),
                                            );
                                        }
                                    }
                                })
                                .build(),
                        )
                        .build(),
                );
            components.add_components::<String>();
            let op = op
                .response(
                    "406",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("The requested content type is not supported")
                        .content(
                            ::mime::TEXT_PLAIN.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema({
                                    match <String as utoipa::PartialSchema>::schema() {
                                        ::utoipa::openapi::RefOr::T(s) => Some(s),
                                        ::utoipa::openapi::RefOr::Ref(_) => {
                                            ::core::panicking::panic_fmt(
                                                format_args!("String schema for plain_text is ref"),
                                            );
                                        }
                                    }
                                })
                                .build(),
                        )
                        .build(),
                );
            op
        }
        fn __groom_negotiate_content_type(
            accept: &::accept_header::Accept,
        ) -> ::core::result::Result<Option<::mime::Mime>, ::axum::response::Response> {
            match accept.negotiate(&__GROOM_RESPONSE_SUPPORTED_MIMES_RqConsQueryResponse)
            {
                Ok(negotiated) => Ok(Some(negotiated)),
                Err(_) => {
                    Err(
                        ::groom::response::not_acceptable(
                            __GROOM_RESPONSE_SUPPORTED_MIMES_RqConsQueryResponse,
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
                    format_args!("{0} / enum `RqConsQueryResponse`", context),
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
                            format_args!("{0} / variant `BadRequest`", context),
                        )
                    }),
                    400u16,
                );
        }
        fn __groom_check_response_formats(
            context: &str,
            formats: &mut ::groom::runtime_checks::HTTPFormatsSet,
        ) {
            formats
                .record(context, &__GROOM_RESPONSE_SUPPORTED_MIMES_RqConsQueryResponse);
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
    pub struct RqConsPathStruct {
        user_id: String,
        team_id: i32,
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
        impl<'de> _serde::Deserialize<'de> for RqConsPathStruct {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private228::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private228::Ok(__Field::__field0),
                            1u64 => _serde::__private228::Ok(__Field::__field1),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "user_id" => _serde::__private228::Ok(__Field::__field0),
                            "team_id" => _serde::__private228::Ok(__Field::__field1),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private228::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"user_id" => _serde::__private228::Ok(__Field::__field0),
                            b"team_id" => _serde::__private228::Ok(__Field::__field1),
                            _ => _serde::__private228::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private228::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private228::PhantomData<RqConsPathStruct>,
                    lifetime: _serde::__private228::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = RqConsPathStruct;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private228::Formatter,
                    ) -> _serde::__private228::fmt::Result {
                        _serde::__private228::Formatter::write_str(
                            __formatter,
                            "struct RqConsPathStruct",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct RqConsPathStruct with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            i32,
                        >(&mut __seq)? {
                            _serde::__private228::Some(__value) => __value,
                            _serde::__private228::None => {
                                return _serde::__private228::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct RqConsPathStruct with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private228::Ok(RqConsPathStruct {
                            user_id: __field0,
                            team_id: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private228::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private228::Option<String> = _serde::__private228::None;
                        let mut __field1: _serde::__private228::Option<i32> = _serde::__private228::None;
                        while let _serde::__private228::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private228::Option::is_some(&__field0) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "user_id",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private228::Option::is_some(&__field1) {
                                        return _serde::__private228::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "team_id",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private228::Some(
                                        _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private228::Some(__field0) => __field0,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("user_id")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private228::Some(__field1) => __field1,
                            _serde::__private228::None => {
                                _serde::__private228::de::missing_field("team_id")?
                            }
                        };
                        _serde::__private228::Ok(RqConsPathStruct {
                            user_id: __field0,
                            team_id: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["user_id", "team_id"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "RqConsPathStruct",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private228::PhantomData::<RqConsPathStruct>,
                        lifetime: _serde::__private228::PhantomData,
                    },
                )
            }
        }
    };
    impl ::groom::DTO for RqConsPathStruct {}
    impl ::groom::DTO_Request for RqConsPathStruct {}
    pub enum RqConsPathResponse {
        Ok(String),
    }
    #[allow(non_upper_case_globals)]
    const __GROOM_RESPONSE_SUPPORTED_MIMES_RqConsPathResponse: &[::mime::Mime] = &[
        ::mime::TEXT_PLAIN,
    ];
    impl RqConsPathResponse {
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
            }
        }
    }
    impl ::groom::response::Response for RqConsPathResponse {
        fn __groom_into_response(
            self,
            negotiated: Option<&::mime::Mime>,
        ) -> ::axum::response::Response {
            match negotiated {
                None => self.into_response_text_plain(),
                Some(negotiated) => {
                    match (negotiated.type_(), negotiated.subtype()) {
                        (::mime::TEXT, mime::PLAIN) => self.into_response_text_plain(),
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
                                .schema({
                                    match <String as utoipa::PartialSchema>::schema() {
                                        ::utoipa::openapi::RefOr::T(s) => Some(s),
                                        ::utoipa::openapi::RefOr::Ref(_) => {
                                            ::core::panicking::panic_fmt(
                                                format_args!("String schema for plain_text is ref"),
                                            );
                                        }
                                    }
                                })
                                .build(),
                        )
                        .build(),
                );
            components.add_components::<String>();
            let op = op
                .response(
                    "406",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("The requested content type is not supported")
                        .content(
                            ::mime::TEXT_PLAIN.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema({
                                    match <String as utoipa::PartialSchema>::schema() {
                                        ::utoipa::openapi::RefOr::T(s) => Some(s),
                                        ::utoipa::openapi::RefOr::Ref(_) => {
                                            ::core::panicking::panic_fmt(
                                                format_args!("String schema for plain_text is ref"),
                                            );
                                        }
                                    }
                                })
                                .build(),
                        )
                        .build(),
                );
            op
        }
        fn __groom_negotiate_content_type(
            accept: &::accept_header::Accept,
        ) -> ::core::result::Result<Option<::mime::Mime>, ::axum::response::Response> {
            match accept.negotiate(&__GROOM_RESPONSE_SUPPORTED_MIMES_RqConsPathResponse)
            {
                Ok(negotiated) => Ok(Some(negotiated)),
                Err(_) => {
                    Err(
                        ::groom::response::not_acceptable(
                            __GROOM_RESPONSE_SUPPORTED_MIMES_RqConsPathResponse,
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
                    format_args!("{0} / enum `RqConsPathResponse`", context),
                )
            });
            codes
                .ensure_distinct(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0} / variant `Ok`", context))
                    }),
                    200u16,
                );
        }
        fn __groom_check_response_formats(
            context: &str,
            formats: &mut ::groom::runtime_checks::HTTPFormatsSet,
        ) {
            formats
                .record(context, &__GROOM_RESPONSE_SUPPORTED_MIMES_RqConsPathResponse);
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
                        (::mime::APPLICATION, mime::JSON) => {
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
                                    match <StructJson as ::utoipa::PartialSchema>::schema() {
                                        ::utoipa::openapi::RefOr::T(s) => {
                                            Some(components.add_components::<StructJson>())
                                        }
                                        ::utoipa::openapi::RefOr::Ref(_) => {
                                            ::core::panicking::panic_fmt(
                                                format_args!(
                                                    "Type `{0}` schema for application/json is ref",
                                                    "StructJson",
                                                ),
                                            );
                                        }
                                    },
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
                                .schema({
                                    match <String as utoipa::PartialSchema>::schema() {
                                        ::utoipa::openapi::RefOr::T(s) => Some(s),
                                        ::utoipa::openapi::RefOr::Ref(_) => {
                                            ::core::panicking::panic_fmt(
                                                format_args!("String schema for plain_text is ref"),
                                            );
                                        }
                                    }
                                })
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
        }
        fn __groom_check_response_formats(
            context: &str,
            formats: &mut ::groom::runtime_checks::HTTPFormatsSet,
        ) {
            formats.record(context, &__GROOM_RESPONSE_SUPPORTED_MIMES_RespJsonResponse);
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
            Wrapper::<StructJson, _>(PhantomData, previous)
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
            Wrapper::<StructJson, _>(PhantomData, previous)
        };
        assert_impl_any_token(previous._static_assertions_impl_any());
    };
    fn __groom_runtime_checks() {
        let context = "Groom runtime check of mod `api_root`".to_string();
        let mut codes = ::groom::runtime_checks::HTTPCodeSet::new();
        <GetRootResponse>::__groom_check_response_codes(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}: handler `get_root`", context))
            }),
            &mut codes,
        );
        let mut formats = ::groom::runtime_checks::HTTPFormatsSet::new();
        <GetRootResponse>::__groom_check_response_formats(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}: handler `get_root`", context))
            }),
            &mut formats,
        );
        let mut codes = ::groom::runtime_checks::HTTPCodeSet::new();
        <GetRootResponse>::__groom_check_response_codes(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}: handler `post_root`", context))
            }),
            &mut codes,
        );
        let mut formats = ::groom::runtime_checks::HTTPFormatsSet::new();
        <GetRootResponse>::__groom_check_response_formats(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}: handler `post_root`", context))
            }),
            &mut formats,
        );
        let mut codes = ::groom::runtime_checks::HTTPCodeSet::new();
        <RqConsQueryResponse>::__groom_check_response_codes(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0}: handler `rq_cons_query_struct`", context),
                )
            }),
            &mut codes,
        );
        let mut formats = ::groom::runtime_checks::HTTPFormatsSet::new();
        <RqConsQueryResponse>::__groom_check_response_formats(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0}: handler `rq_cons_query_struct`", context),
                )
            }),
            &mut formats,
        );
        let mut codes = ::groom::runtime_checks::HTTPCodeSet::new();
        <RqConsPathResponse>::__groom_check_response_codes(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0}: handler `rq_cons_path_struct`", context),
                )
            }),
            &mut codes,
        );
        let mut formats = ::groom::runtime_checks::HTTPFormatsSet::new();
        <RqConsPathResponse>::__groom_check_response_formats(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0}: handler `rq_cons_path_struct`", context),
                )
            }),
            &mut formats,
        );
        let mut codes = ::groom::runtime_checks::HTTPCodeSet::new();
        <RespJsonResponse>::__groom_check_response_codes(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}: handler `resp_json`", context))
            }),
            &mut codes,
        );
        let mut formats = ::groom::runtime_checks::HTTPFormatsSet::new();
        <RespJsonResponse>::__groom_check_response_formats(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}: handler `resp_json`", context))
            }),
            &mut formats,
        );
    }
    pub fn into_router() -> ::groom::router::GroomRouter<()> {
        __groom_runtime_checks();
        let this_router: ::axum::Router<()> = ::axum::Router::new()
            .route("/", ::axum::routing::get(__groom_wrapper_get_root))
            .route("/", ::axum::routing::post(__groom_wrapper_post_root))
            .route("/greet", ::axum::routing::get(__groom_wrapper_rq_cons_query_struct))
            .route(
                "/team/:team_id/user/:user_id",
                ::axum::routing::get(__groom_wrapper_rq_cons_path_struct),
            )
            .route("/json", ::axum::routing::get(__groom_wrapper_resp_json));
        let mut components = ::groom::extract::ComponentsRegistry::new();
        let mut __groom_paths: ::std::vec::Vec<
            (::std::string::String, ::utoipa::openapi::path::PathItem),
        > = ::std::vec::Vec::new();
        __groom_paths
            .push((
                "/".to_string(),
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(Some("Summary"))
                        .description(Some("Description"))
                        .operation_id(Some("getRoot"));
                    op_builder = <GetRootResponse>::__openapi_modify_operation(
                        op_builder,
                        &mut components,
                    );
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::path::HttpMethod::Get,
                            op_builder.build(),
                        )
                        .build()
                },
            ));
        __groom_paths
            .push((
                "/".to_string(),
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(None as Option<String>)
                        .description(None as Option<String>)
                        .operation_id(Some("postRoot"));
                    op_builder = <GetRootResponse>::__openapi_modify_operation(
                        op_builder,
                        &mut components,
                    );
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::path::HttpMethod::Post,
                            op_builder.build(),
                        )
                        .build()
                },
            ));
        __groom_paths
            .push((
                "/greet".to_string(),
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(Some("Query<struct>"))
                        .description(None as Option<String>)
                        .operation_id(Some("rqConsQueryStruct"));
                    op_builder = <Query<
                        RqConsQueryStruct,
                    >>::__openapi_modify_operation(op_builder, &mut components);
                    op_builder = <RqConsQueryResponse>::__openapi_modify_operation(
                        op_builder,
                        &mut components,
                    );
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::path::HttpMethod::Get,
                            op_builder.build(),
                        )
                        .build()
                },
            ));
        __groom_paths
            .push((
                "/team/:team_id/user/:user_id".to_string(),
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(Some("Path<struct>"))
                        .description(None as Option<String>)
                        .operation_id(Some("rqConsPathStruct"));
                    op_builder = <Path<
                        RqConsPathStruct,
                    >>::__openapi_modify_operation(op_builder, &mut components);
                    op_builder = <RqConsPathResponse>::__openapi_modify_operation(
                        op_builder,
                        &mut components,
                    );
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::path::HttpMethod::Get,
                            op_builder.build(),
                        )
                        .build()
                },
            ));
        __groom_paths
            .push((
                "/json".to_string(),
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(None as Option<String>)
                        .description(None as Option<String>)
                        .operation_id(Some("respJson"));
                    op_builder = <RespJsonResponse>::__openapi_modify_operation(
                        op_builder,
                        &mut components,
                    );
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
            .route("/", ::axum::routing::get(__groom_wrapper_get_root))
            .route("/", ::axum::routing::post(__groom_wrapper_post_root))
            .route("/greet", ::axum::routing::get(__groom_wrapper_rq_cons_query_struct))
            .route(
                "/team/:team_id/user/:user_id",
                ::axum::routing::get(__groom_wrapper_rq_cons_path_struct),
            )
            .route("/json", ::axum::routing::get(__groom_wrapper_resp_json));
        let mut components = ::groom::extract::ComponentsRegistry::new();
        let mut __groom_paths: ::std::vec::Vec<
            (::std::string::String, ::utoipa::openapi::path::PathItem),
        > = ::std::vec::Vec::new();
        __groom_paths
            .push((
                "/".to_string(),
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(Some("Summary"))
                        .description(Some("Description"))
                        .operation_id(Some("getRoot"));
                    op_builder = <GetRootResponse>::__openapi_modify_operation(
                        op_builder,
                        &mut components,
                    );
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::path::HttpMethod::Get,
                            op_builder.build(),
                        )
                        .build()
                },
            ));
        __groom_paths
            .push((
                "/".to_string(),
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(None as Option<String>)
                        .description(None as Option<String>)
                        .operation_id(Some("postRoot"));
                    op_builder = <GetRootResponse>::__openapi_modify_operation(
                        op_builder,
                        &mut components,
                    );
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::path::HttpMethod::Post,
                            op_builder.build(),
                        )
                        .build()
                },
            ));
        __groom_paths
            .push((
                "/greet".to_string(),
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(Some("Query<struct>"))
                        .description(None as Option<String>)
                        .operation_id(Some("rqConsQueryStruct"));
                    op_builder = <Query<
                        RqConsQueryStruct,
                    >>::__openapi_modify_operation(op_builder, &mut components);
                    op_builder = <RqConsQueryResponse>::__openapi_modify_operation(
                        op_builder,
                        &mut components,
                    );
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::path::HttpMethod::Get,
                            op_builder.build(),
                        )
                        .build()
                },
            ));
        __groom_paths
            .push((
                "/team/:team_id/user/:user_id".to_string(),
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(Some("Path<struct>"))
                        .description(None as Option<String>)
                        .operation_id(Some("rqConsPathStruct"));
                    op_builder = <Path<
                        RqConsPathStruct,
                    >>::__openapi_modify_operation(op_builder, &mut components);
                    op_builder = <RqConsPathResponse>::__openapi_modify_operation(
                        op_builder,
                        &mut components,
                    );
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::path::HttpMethod::Get,
                            op_builder.build(),
                        )
                        .build()
                },
            ));
        __groom_paths
            .push((
                "/json".to_string(),
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(None as Option<String>)
                        .description(None as Option<String>)
                        .operation_id(Some("respJson"));
                    op_builder = <RespJsonResponse>::__openapi_modify_operation(
                        op_builder,
                        &mut components,
                    );
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
        assert_impl_all::<GetRootResponse>();
    };
    const _: fn() = || {
        fn assert_impl_all<T: ?Sized + ::groom::response::Response>() {}
        assert_impl_all::<GetRootResponse>();
    };
    const _: fn() = || {
        fn assert_impl_all<T: ?Sized + ::groom::extract::GroomExtractor>() {}
        assert_impl_all::<Query<RqConsQueryStruct>>();
    };
    const _: fn() = || {
        fn assert_impl_all<T: ?Sized + ::groom::response::Response>() {}
        assert_impl_all::<RqConsQueryResponse>();
    };
    const _: fn() = || {
        fn assert_impl_all<T: ?Sized + ::groom::extract::GroomExtractor>() {}
        assert_impl_all::<Path<RqConsPathStruct>>();
    };
    const _: fn() = || {
        fn assert_impl_all<T: ?Sized + ::groom::response::Response>() {}
        assert_impl_all::<RqConsPathResponse>();
    };
    const _: fn() = || {
        fn assert_impl_all<T: ?Sized + ::groom::response::Response>() {}
        assert_impl_all::<RespJsonResponse>();
    };
}
mod options_connect {
    use ::static_assertions::{assert_impl_all, assert_impl_any};
    pub enum OptionsResult {
        Ok,
    }
    #[allow(non_upper_case_globals)]
    const __GROOM_RESPONSE_SUPPORTED_MIMES_OptionsResult: &[::mime::Mime] = &[];
    impl OptionsResult {
        fn into_response_any_content_type(self) -> ::axum::response::Response {
            match self {
                Self::Ok => {
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
                                    200u16,),
                                ),
                            );
                        }
                    })
                        .into_response()
                }
            }
        }
    }
    impl ::groom::response::Response for OptionsResult {
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
            let context = ::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0} / enum `OptionsResult`", context))
            });
            codes
                .ensure_distinct(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(format_args!("{0} / variant `Ok`", context))
                    }),
                    200u16,
                );
        }
        fn __groom_check_response_formats(
            context: &str,
            formats: &mut ::groom::runtime_checks::HTTPFormatsSet,
        ) {
            formats.record(context, &__GROOM_RESPONSE_SUPPORTED_MIMES_OptionsResult);
        }
    }
    /// HTTP handler: OPTIONS /options-check
    pub async fn options_route() -> OptionsResult {
        OptionsResult::Ok
    }
    async fn __groom_wrapper_options_route(
        headers: ::axum::http::header::HeaderMap,
    ) -> impl ::axum::response::IntoResponse {
        let accept = match ::groom::content_negotiation::parse_accept_header(&headers) {
            Err(_) => return ::groom::response::bad_accept_header(),
            Ok(accept) => accept,
        };
        let negotiated = match accept {
            None => None,
            Some(accept) => {
                match <OptionsResult>::__groom_negotiate_content_type(&accept) {
                    Err(response) => return response,
                    Ok(negotiated) => negotiated,
                }
            }
        };
        let result = options_route().await;
        result.__groom_into_response(negotiated.as_ref())
    }
    /// HTTP handler: CONNECT /tunnel
    pub async fn connect_route() -> OptionsResult {
        OptionsResult::Ok
    }
    async fn __groom_wrapper_connect_route(
        headers: ::axum::http::header::HeaderMap,
    ) -> impl ::axum::response::IntoResponse {
        let accept = match ::groom::content_negotiation::parse_accept_header(&headers) {
            Err(_) => return ::groom::response::bad_accept_header(),
            Ok(accept) => accept,
        };
        let negotiated = match accept {
            None => None,
            Some(accept) => {
                match <OptionsResult>::__groom_negotiate_content_type(&accept) {
                    Err(response) => return response,
                    Ok(negotiated) => negotiated,
                }
            }
        };
        let result = connect_route().await;
        result.__groom_into_response(negotiated.as_ref())
    }
    fn __groom_runtime_checks() {
        let context = "Groom runtime check of mod `options_connect`".to_string();
        let mut codes = ::groom::runtime_checks::HTTPCodeSet::new();
        <OptionsResult>::__groom_check_response_codes(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0}: handler `options_route`", context),
                )
            }),
            &mut codes,
        );
        let mut formats = ::groom::runtime_checks::HTTPFormatsSet::new();
        <OptionsResult>::__groom_check_response_formats(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0}: handler `options_route`", context),
                )
            }),
            &mut formats,
        );
        let mut codes = ::groom::runtime_checks::HTTPCodeSet::new();
        <OptionsResult>::__groom_check_response_codes(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0}: handler `connect_route`", context),
                )
            }),
            &mut codes,
        );
        let mut formats = ::groom::runtime_checks::HTTPFormatsSet::new();
        <OptionsResult>::__groom_check_response_formats(
            &::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("{0}: handler `connect_route`", context),
                )
            }),
            &mut formats,
        );
    }
    pub fn into_router() -> ::groom::router::GroomRouter<()> {
        __groom_runtime_checks();
        let this_router: ::axum::Router<()> = ::axum::Router::new()
            .route(
                "/options-check",
                ::axum::routing::options(__groom_wrapper_options_route),
            )
            .route("/tunnel", ::axum::routing::connect(__groom_wrapper_connect_route));
        let mut components = ::groom::extract::ComponentsRegistry::new();
        let mut __groom_paths: ::std::vec::Vec<
            (::std::string::String, ::utoipa::openapi::path::PathItem),
        > = ::std::vec::Vec::new();
        __groom_paths
            .push((
                "/options-check".to_string(),
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(None as Option<String>)
                        .description(None as Option<String>)
                        .operation_id(Some("optionsRoute"));
                    op_builder = <OptionsResult>::__openapi_modify_operation(
                        op_builder,
                        &mut components,
                    );
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::path::HttpMethod::Options,
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
            .route(
                "/options-check",
                ::axum::routing::options(__groom_wrapper_options_route),
            )
            .route("/tunnel", ::axum::routing::connect(__groom_wrapper_connect_route));
        let mut components = ::groom::extract::ComponentsRegistry::new();
        let mut __groom_paths: ::std::vec::Vec<
            (::std::string::String, ::utoipa::openapi::path::PathItem),
        > = ::std::vec::Vec::new();
        __groom_paths
            .push((
                "/options-check".to_string(),
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(None as Option<String>)
                        .description(None as Option<String>)
                        .operation_id(Some("optionsRoute"));
                    op_builder = <OptionsResult>::__openapi_modify_operation(
                        op_builder,
                        &mut components,
                    );
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::path::HttpMethod::Options,
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
        assert_impl_all::<OptionsResult>();
    };
    const _: fn() = || {
        fn assert_impl_all<T: ?Sized + ::groom::response::Response>() {}
        assert_impl_all::<OptionsResult>();
    };
}
