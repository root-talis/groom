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
                                .schema(StructJson::schema().extract_schema())
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
        ::mime::TEXT_PLAIN,
        ::mime::TEXT_HTML,
        ::mime::APPLICATION_JSON,
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
                                .schema(Struct::schema().extract_schema())
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
mod named_struct_response {
    use groom::response::html_format;
    use groom::response::HtmlFormat;
    pub struct Named {
        success: bool,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Named {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
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
                ::axum::http::StatusCode::from_u16(200u16).unwrap(),
                <Named as ::groom::response::HtmlFormat>::render(self),
            )
                .into_response()
        }
        fn into_response_application_json(self) -> ::axum::response::Response {
            (::axum::http::StatusCode::from_u16(200u16).unwrap(), ::axum::Json(self))
                .into_response()
        }
    }
    impl ::groom::response::Response for Named {
        fn __groom_into_response(
            self,
            accept: Option<::accept_header::Accept>,
        ) -> ::axum::response::Response {
            match accept {
                None => self.into_response_application_json(),
                Some(accept) => {
                    match accept.negotiate(&__GROOM_RESPONSE_SUPPORTED_MIMES_Named) {
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
                            ::mime::TEXT_HTML_UTF_8.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(<String as utoipa::PartialSchema>::schema())
                                .build(),
                        )
                        .content(
                            ::mime::APPLICATION_JSON.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(Named::schema().extract_schema())
                                .build(),
                        )
                        .build(),
                );
            op
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
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Unnamed {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
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
                ::axum::http::StatusCode::from_u16(200u16).unwrap(),
                <Unnamed as ::groom::response::HtmlFormat>::render(self),
            )
                .into_response()
        }
        fn into_response_application_json(self) -> ::axum::response::Response {
            (::axum::http::StatusCode::from_u16(200u16).unwrap(), ::axum::Json(self))
                .into_response()
        }
    }
    impl ::groom::response::Response for Unnamed {
        fn __groom_into_response(
            self,
            accept: Option<::accept_header::Accept>,
        ) -> ::axum::response::Response {
            match accept {
                None => self.into_response_application_json(),
                Some(accept) => {
                    match accept.negotiate(&__GROOM_RESPONSE_SUPPORTED_MIMES_Unnamed) {
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
                            ::mime::TEXT_HTML_UTF_8.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(<String as utoipa::PartialSchema>::schema())
                                .build(),
                        )
                        .content(
                            ::mime::APPLICATION_JSON.as_ref(),
                            ::utoipa::openapi::ContentBuilder::new()
                                .schema(String::schema().extract_schema())
                                .build(),
                        )
                        .build(),
                );
            op
        }
    }
    impl ::groom::response::HtmlFormat for Unnamed {
        fn render(self) -> ::axum::response::Html<axum::body::Body> {
            ::axum::response::Html(
                {
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "<span style=\"color: #a3be8c;\">{0}</span>", self.0,
                        ),
                    );
                    res
                }
                    .into(),
            )
        }
    }
}
mod unit_struct_response {
    pub struct Unit;
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Unit {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
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
            (::axum::http::StatusCode::from_u16(200u16).unwrap()).into_response()
        }
    }
    impl ::groom::response::Response for Unit {
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
                    "200",
                    ::utoipa::openapi::ResponseBuilder::new().description("").build(),
                );
            op
        }
    }
}
mod result_struct_struct {
    use ::static_assertions::{assert_impl_all, assert_impl_any};
    pub struct Success;
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Success {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
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
            (::axum::http::StatusCode::from_u16(200u16).unwrap()).into_response()
        }
    }
    impl ::groom::response::Response for Success {
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
                    "200",
                    ::utoipa::openapi::ResponseBuilder::new().description("").build(),
                );
            op
        }
    }
    pub struct Error;
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Error {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
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
            (::axum::http::StatusCode::from_u16(404u16).unwrap()).into_response()
        }
    }
    impl ::groom::response::Response for Error {
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
                    "404",
                    ::utoipa::openapi::ResponseBuilder::new().description("").build(),
                );
            op
        }
    }
    /// HTTP handler: GET /
    async fn foo() -> Result<Success, Error> {
        Ok(Success)
    }
    async fn __groom_wrapper_foo(
        headers: ::axum::http::header::HeaderMap,
    ) -> impl ::axum::response::IntoResponse {
        let accept = ::groom::content_negotiation::parse_accept_header(&headers);
        let result = foo().await;
        result.__groom_into_response(accept)
    }
    pub fn merge_into_router(other: ::axum::Router<()>) -> ::axum::Router<()> {
        let this_router = ::axum::Router::new()
            .route("/", ::axum::routing::get(__groom_wrapper_foo));
        other.merge(this_router)
    }
    pub fn merge_into_openapi_builder(
        other: ::utoipa::openapi::OpenApiBuilder,
    ) -> ::utoipa::openapi::OpenApiBuilder {
        let mut paths = ::utoipa::openapi::path::PathsBuilder::new();
        paths = paths
            .path(
                "/",
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(None as Option<String>)
                        .description(None as Option<String>);
                    op_builder = <Result<
                        Success,
                        Error,
                    >>::__openapi_modify_operation(op_builder);
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::PathItemType::Get,
                            op_builder.build(),
                        )
                        .build()
                },
            );
        other.paths(paths)
    }
    const _: fn() = || {
        fn assert_impl_all<T: ?Sized + ::groom::response::Response>() {}
        assert_impl_all::<Result<Success, Error>>();
    };
}
mod result_struct_enum {
    use ::static_assertions::{assert_impl_all, assert_impl_any};
    pub struct Success;
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Success {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
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
            (::axum::http::StatusCode::from_u16(200u16).unwrap()).into_response()
        }
    }
    impl ::groom::response::Response for Success {
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
                    "200",
                    ::utoipa::openapi::ResponseBuilder::new().description("").build(),
                );
            op
        }
    }
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
                    (::axum::http::StatusCode::from_u16(404u16).unwrap()).into_response()
                }
                Self::NoAccess(body) => {
                    (
                        ::axum::http::StatusCode::from_u16(400u16).unwrap(),
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
            accept: Option<::accept_header::Accept>,
        ) -> ::axum::response::Response {
            match accept {
                None => self.into_response_text_plain(),
                Some(accept) => {
                    match accept.negotiate(&__GROOM_RESPONSE_SUPPORTED_MIMES_Error) {
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
                                .schema(<String as utoipa::PartialSchema>::schema())
                                .build(),
                        )
                        .build(),
                );
            op
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
        Err(Error::NoAccess("ip blocked".into()))
    }
    async fn __groom_wrapper_foo(
        headers: ::axum::http::header::HeaderMap,
    ) -> impl ::axum::response::IntoResponse {
        let accept = ::groom::content_negotiation::parse_accept_header(&headers);
        let result = foo().await;
        result.__groom_into_response(accept)
    }
    pub fn merge_into_router(other: ::axum::Router<()>) -> ::axum::Router<()> {
        let this_router = ::axum::Router::new()
            .route("/", ::axum::routing::get(__groom_wrapper_foo));
        other.merge(this_router)
    }
    pub fn merge_into_openapi_builder(
        other: ::utoipa::openapi::OpenApiBuilder,
    ) -> ::utoipa::openapi::OpenApiBuilder {
        let mut paths = ::utoipa::openapi::path::PathsBuilder::new();
        paths = paths
            .path(
                "/",
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(None as Option<String>)
                        .description(None as Option<String>);
                    op_builder = <Result<
                        Success,
                        Error,
                    >>::__openapi_modify_operation(op_builder);
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::PathItemType::Get,
                            op_builder.build(),
                        )
                        .build()
                },
            );
        other.paths(paths)
    }
    const _: fn() = || {
        fn assert_impl_all<T: ?Sized + ::groom::response::Response>() {}
        assert_impl_all::<Result<Success, Error>>();
    };
}
