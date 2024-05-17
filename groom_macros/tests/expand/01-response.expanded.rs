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
                    (::axum::http::StatusCode::from_u16(202u16).unwrap()).into_response()
                }
                Self::NotFound => {
                    (::axum::http::StatusCode::from_u16(404u16).unwrap()).into_response()
                }
            }
        }
    }
    impl ::groom::response::Response for RespJsonResponse {
        fn __groom_into_response(
            self,
            accept: Option<::accept_header::Accept>,
        ) -> ::axum::response::Response {
            self.into_response_any_content_type()
        }
        fn __openapi_modify_operation(
            op: ::utoipa::openapi::path::OperationBuilder,
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
                        ::axum::http::StatusCode::from_u16(200u16).unwrap(),
                        Into::<String>::into(body),
                    )
                        .into_response()
                }
                Self::NotFound => {
                    (::axum::http::StatusCode::from_u16(404u16).unwrap()).into_response()
                }
            }
        }
    }
    impl ::groom::response::Response for RespPlaintextResponse {
        fn __groom_into_response(
            self,
            accept: Option<::accept_header::Accept>,
        ) -> ::axum::response::Response {
            match accept {
                None => self.into_response_text_plain(),
                Some(accept) => {
                    match accept
                        .negotiate(
                            &__GROOM_RESPONSE_SUPPORTED_MIMES_RespPlaintextResponse,
                        )
                    {
                        Err(_) => {
                            (
                                ::axum::http::StatusCode::BAD_REQUEST,
                                "Requested Content-Type is not supported.",
                            )
                                .into_response()
                        }
                        Ok(negotiated) => {
                            match (negotiated.type_(), negotiated.subtype()) {
                                (::mime::TEXT, mime::PLAIN) => {
                                    self.into_response_text_plain()
                                }
                                _ => {
                                    (
                                        ::axum::http::StatusCode::BAD_REQUEST,
                                        "Content-Type negotiation produced an unexpected type/subtype pair.",
                                    )
                                        .into_response()
                                }
                            }
                        }
                    }
                }
            }
        }
        fn __openapi_modify_operation(
            op: ::utoipa::openapi::path::OperationBuilder,
        ) -> ::utoipa::openapi::path::OperationBuilder {
            let op = op
                .response(
                    "200",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("")
                        .content(
                            ::mime::TEXT_PLAIN_UTF_8.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(<String as utoipa::PartialSchema>::schema())
                                .build(),
                        )
                        .build(),
                );
            let op = op
                .response(
                    "404",
                    ::utoipa::openapi::ResponseBuilder::new().description("").build(),
                );
            op
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
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Struct {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
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
                        ::axum::http::StatusCode::from_u16(200u16).unwrap(),
                        <Struct as ::groom::response::HtmlFormat>::render(body),
                    )
                        .into_response()
                }
                Self::NotFound => {
                    (::axum::http::StatusCode::from_u16(404u16).unwrap()).into_response()
                }
            }
        }
    }
    impl ::groom::response::Response for RespHtmlResponse {
        fn __groom_into_response(
            self,
            accept: Option<::accept_header::Accept>,
        ) -> ::axum::response::Response {
            match accept {
                None => self.into_response_text_html(),
                Some(accept) => {
                    match accept
                        .negotiate(&__GROOM_RESPONSE_SUPPORTED_MIMES_RespHtmlResponse)
                    {
                        Err(_) => {
                            (
                                ::axum::http::StatusCode::BAD_REQUEST,
                                "Requested Content-Type is not supported.",
                            )
                                .into_response()
                        }
                        Ok(negotiated) => {
                            match (negotiated.type_(), negotiated.subtype()) {
                                (::mime::TEXT, mime::HTML) => self.into_response_text_html(),
                                _ => {
                                    (
                                        ::axum::http::StatusCode::BAD_REQUEST,
                                        "Content-Type negotiation produced an unexpected type/subtype pair.",
                                    )
                                        .into_response()
                                }
                            }
                        }
                    }
                }
            }
        }
        fn __openapi_modify_operation(
            op: ::utoipa::openapi::path::OperationBuilder,
        ) -> ::utoipa::openapi::path::OperationBuilder {
            let op = op
                .response(
                    "200",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("")
                        .content(
                            ::mime::TEXT_HTML_UTF_8.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(<String as utoipa::PartialSchema>::schema())
                                .build(),
                        )
                        .build(),
                );
            let op = op
                .response(
                    "404",
                    ::utoipa::openapi::ResponseBuilder::new().description("").build(),
                );
            op
        }
    }
}
mod json_only {
    pub struct StructJson {
        success: bool,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for StructJson {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
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
                        ::axum::http::StatusCode::from_u16(200u16).unwrap(),
                        ::axum::Json(body),
                    )
                        .into_response()
                }
                Self::NotFound => {
                    (::axum::http::StatusCode::from_u16(404u16).unwrap()).into_response()
                }
            }
        }
    }
    impl ::groom::response::Response for RespJsonResponse {
        fn __groom_into_response(
            self,
            accept: Option<::accept_header::Accept>,
        ) -> ::axum::response::Response {
            match accept {
                None => self.into_response_application_json(),
                Some(accept) => {
                    match accept
                        .negotiate(&__GROOM_RESPONSE_SUPPORTED_MIMES_RespJsonResponse)
                    {
                        Err(_) => {
                            (
                                ::axum::http::StatusCode::BAD_REQUEST,
                                "Requested Content-Type is not supported.",
                            )
                                .into_response()
                        }
                        Ok(negotiated) => {
                            match (negotiated.type_(), negotiated.subtype()) {
                                (::mime::APPLICATION, mime::JSON) => {
                                    self.into_response_application_json()
                                }
                                _ => {
                                    (
                                        ::axum::http::StatusCode::BAD_REQUEST,
                                        "Content-Type negotiation produced an unexpected type/subtype pair.",
                                    )
                                        .into_response()
                                }
                            }
                        }
                    }
                }
            }
        }
        fn __openapi_modify_operation(
            op: ::utoipa::openapi::path::OperationBuilder,
        ) -> ::utoipa::openapi::path::OperationBuilder {
            let op = op
                .response(
                    "200",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("")
                        .content(
                            ::mime::APPLICATION_JSON.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(StructJson::schema().1)
                                .build(),
                        )
                        .build(),
                );
            let op = op
                .response(
                    "404",
                    ::utoipa::openapi::ResponseBuilder::new().description("").build(),
                );
            op
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
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Struct {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
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
        ::mime::APPLICATION_JSON,
        ::mime::TEXT_HTML,
        ::mime::TEXT_PLAIN,
    ];
    impl RespMultipleTypesResponse {
        fn into_response_text_plain(self) -> ::axum::response::Response {
            match self {
                Self::Ok(body) => {
                    (
                        ::axum::http::StatusCode::from_u16(200u16).unwrap(),
                        Into::<String>::into(body),
                    )
                        .into_response()
                }
                Self::NotFound => {
                    (::axum::http::StatusCode::from_u16(404u16).unwrap()).into_response()
                }
            }
        }
        fn into_response_text_html(self) -> ::axum::response::Response {
            match self {
                Self::Ok(body) => {
                    (
                        ::axum::http::StatusCode::from_u16(200u16).unwrap(),
                        <Struct as ::groom::response::HtmlFormat>::render(body),
                    )
                        .into_response()
                }
                Self::NotFound => {
                    (::axum::http::StatusCode::from_u16(404u16).unwrap()).into_response()
                }
            }
        }
        fn into_response_application_json(self) -> ::axum::response::Response {
            match self {
                Self::Ok(body) => {
                    (
                        ::axum::http::StatusCode::from_u16(200u16).unwrap(),
                        ::axum::Json(body),
                    )
                        .into_response()
                }
                Self::NotFound => {
                    (::axum::http::StatusCode::from_u16(404u16).unwrap()).into_response()
                }
            }
        }
    }
    impl ::groom::response::Response for RespMultipleTypesResponse {
        fn __groom_into_response(
            self,
            accept: Option<::accept_header::Accept>,
        ) -> ::axum::response::Response {
            match accept {
                None => self.into_response_application_json(),
                Some(accept) => {
                    match accept
                        .negotiate(
                            &__GROOM_RESPONSE_SUPPORTED_MIMES_RespMultipleTypesResponse,
                        )
                    {
                        Err(_) => {
                            (
                                ::axum::http::StatusCode::BAD_REQUEST,
                                "Requested Content-Type is not supported.",
                            )
                                .into_response()
                        }
                        Ok(negotiated) => {
                            match (negotiated.type_(), negotiated.subtype()) {
                                (::mime::TEXT, mime::PLAIN) => {
                                    self.into_response_text_plain()
                                }
                                (::mime::TEXT, mime::HTML) => self.into_response_text_html(),
                                (::mime::APPLICATION, mime::JSON) => {
                                    self.into_response_application_json()
                                }
                                _ => {
                                    (
                                        ::axum::http::StatusCode::BAD_REQUEST,
                                        "Content-Type negotiation produced an unexpected type/subtype pair.",
                                    )
                                        .into_response()
                                }
                            }
                        }
                    }
                }
            }
        }
        fn __openapi_modify_operation(
            op: ::utoipa::openapi::path::OperationBuilder,
        ) -> ::utoipa::openapi::path::OperationBuilder {
            let op = op
                .response(
                    "200",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("")
                        .content(
                            ::mime::TEXT_PLAIN_UTF_8.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(<String as utoipa::PartialSchema>::schema())
                                .build(),
                        )
                        .content(
                            ::mime::TEXT_HTML_UTF_8.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(<String as utoipa::PartialSchema>::schema())
                                .build(),
                        )
                        .content(
                            ::mime::APPLICATION_JSON.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(Struct::schema().1)
                                .build(),
                        )
                        .build(),
                );
            let op = op
                .response(
                    "404",
                    ::utoipa::openapi::ResponseBuilder::new().description("").build(),
                );
            op
        }
    }
}
