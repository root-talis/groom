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
        let accept = ::groom::content_negotiation::parse_accept_header(&headers);
        let result = get_root().await;
        result.__groom_into_response(accept)
    }
    /// HTTP handler: POST /
    pub async fn post_root() -> GetRootResponse {
        let a = 2;
    }
    async fn __groom_wrapper_post_root(
        headers: ::axum::http::header::HeaderMap,
    ) -> impl ::axum::response::IntoResponse {
        let accept = ::groom::content_negotiation::parse_accept_header(&headers);
        let result = post_root().await;
        result.__groom_into_response(accept)
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
            RqConsQueryResponse::Ok(result)
        }
    }
    async fn __groom_wrapper_rq_cons_query_struct(
        headers: ::axum::http::header::HeaderMap,
        input0: Query<RqConsQueryStruct>,
    ) -> impl ::axum::response::IntoResponse {
        let accept = ::groom::content_negotiation::parse_accept_header(&headers);
        let result = rq_cons_query_struct(input0).await;
        result.__groom_into_response(accept)
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
        let accept = ::groom::content_negotiation::parse_accept_header(&headers);
        let result = rq_cons_path_struct(input0).await;
        result.__groom_into_response(accept)
    }
    /// HTTP handler: GET /json
    pub async fn resp_json() -> RespJsonResponse {
        RespJsonResponse::Ok(StructJson { success: true })
    }
    async fn __groom_wrapper_resp_json(
        headers: ::axum::http::header::HeaderMap,
    ) -> impl ::axum::response::IntoResponse {
        let accept = ::groom::content_negotiation::parse_accept_header(&headers);
        let result = resp_json().await;
        result.__groom_into_response(accept)
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
                        ::axum::http::StatusCode::from_u16(200u16).unwrap(),
                        Into::<String>::into(body),
                    )
                        .into_response()
                }
                Self::BadRequest(body) => {
                    (
                        ::axum::http::StatusCode::from_u16(400u16).unwrap(),
                        Into::<String>::into(body),
                    )
                        .into_response()
                }
                Self::Forbidden => {
                    (::axum::http::StatusCode::from_u16(401u16).unwrap()).into_response()
                }
            }
        }
    }
    impl ::groom::response::Response for GetRootResponse {
        fn __groom_into_response(
            self,
            accept: Option<::accept_header::Accept>,
        ) -> ::axum::response::Response {
            match accept {
                None => self.into_response_text_plain(),
                Some(accept) => {
                    match accept
                        .negotiate(&__GROOM_RESPONSE_SUPPORTED_MIMES_GetRootResponse)
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
                        .description("There you go mate.")
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
                    "400",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("Are you insane?\n\nBad request.")
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
                    "401",
                    ::utoipa::openapi::ResponseBuilder::new()
                        .description("You shall not pass!")
                        .build(),
                );
            op
        }
        fn __groom_check_response_codes(
            context: &String,
            codes: &mut ::groom::runtime_checks::HTTPCodeSet,
        ) {
            let context = {
                let res = ::alloc::fmt::format(
                    format_args!("{0} / enum `GetRootResponse`", context),
                );
                res
            };
            codes
                .ensure_distinct(
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("{0} / variant `Ok`", context),
                        );
                        res
                    },
                    200u16,
                );
            codes
                .ensure_distinct(
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("{0} / variant `BadRequest`", context),
                        );
                        res
                    },
                    400u16,
                );
            codes
                .ensure_distinct(
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("{0} / variant `Forbidden`", context),
                        );
                        res
                    },
                    401u16,
                );
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
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for RqConsQueryStruct {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
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
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
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
                    marker: _serde::__private::PhantomData<RqConsQueryStruct>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = RqConsQueryStruct;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct RqConsQueryStruct",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct RqConsQueryStruct with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(RqConsQueryStruct {
                            name: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
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
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("name")?
                            }
                        };
                        _serde::__private::Ok(RqConsQueryStruct {
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
                        marker: _serde::__private::PhantomData::<RqConsQueryStruct>,
                        lifetime: _serde::__private::PhantomData,
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
                        ::axum::http::StatusCode::from_u16(200u16).unwrap(),
                        Into::<String>::into(body),
                    )
                        .into_response()
                }
                Self::BadRequest(body) => {
                    (
                        ::axum::http::StatusCode::from_u16(400u16).unwrap(),
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
            accept: Option<::accept_header::Accept>,
        ) -> ::axum::response::Response {
            match accept {
                None => self.into_response_text_plain(),
                Some(accept) => {
                    match accept
                        .negotiate(&__GROOM_RESPONSE_SUPPORTED_MIMES_RqConsQueryResponse)
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
        fn __groom_check_response_codes(
            context: &String,
            codes: &mut ::groom::runtime_checks::HTTPCodeSet,
        ) {
            let context = {
                let res = ::alloc::fmt::format(
                    format_args!("{0} / enum `RqConsQueryResponse`", context),
                );
                res
            };
            codes
                .ensure_distinct(
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("{0} / variant `Ok`", context),
                        );
                        res
                    },
                    200u16,
                );
            codes
                .ensure_distinct(
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("{0} / variant `BadRequest`", context),
                        );
                        res
                    },
                    400u16,
                );
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
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for RqConsPathStruct {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
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
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "user_id" => _serde::__private::Ok(__Field::__field0),
                            "team_id" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"user_id" => _serde::__private::Ok(__Field::__field0),
                            b"team_id" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
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
                    marker: _serde::__private::PhantomData<RqConsPathStruct>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = RqConsPathStruct;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct RqConsPathStruct",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
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
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct RqConsPathStruct with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(RqConsPathStruct {
                            user_id: __field0,
                            team_id: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<i32> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "user_id",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "team_id",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
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
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("user_id")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("team_id")?
                            }
                        };
                        _serde::__private::Ok(RqConsPathStruct {
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
                        marker: _serde::__private::PhantomData::<RqConsPathStruct>,
                        lifetime: _serde::__private::PhantomData,
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
                        ::axum::http::StatusCode::from_u16(200u16).unwrap(),
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
            accept: Option<::accept_header::Accept>,
        ) -> ::axum::response::Response {
            match accept {
                None => self.into_response_text_plain(),
                Some(accept) => {
                    match accept
                        .negotiate(&__GROOM_RESPONSE_SUPPORTED_MIMES_RqConsPathResponse)
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
            op
        }
        fn __groom_check_response_codes(
            context: &String,
            codes: &mut ::groom::runtime_checks::HTTPCodeSet,
        ) {
            let context = {
                let res = ::alloc::fmt::format(
                    format_args!("{0} / enum `RqConsPathResponse`", context),
                );
                res
            };
            codes
                .ensure_distinct(
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("{0} / variant `Ok`", context),
                        );
                        res
                    },
                    200u16,
                );
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
            op
        }
        fn __groom_check_response_codes(
            context: &String,
            codes: &mut ::groom::runtime_checks::HTTPCodeSet,
        ) {
            let context = {
                let res = ::alloc::fmt::format(
                    format_args!("{0} / enum `RespJsonResponse`", context),
                );
                res
            };
            codes
                .ensure_distinct(
                    {
                        let res = ::alloc::fmt::format(
                            format_args!("{0} / variant `Ok`", context),
                        );
                        res
                    },
                    200u16,
                );
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
            &{
                let res = ::alloc::fmt::format(
                    format_args!("{0}: handler `get_root`", context),
                );
                res
            },
            &mut codes,
        );
        let mut codes = ::groom::runtime_checks::HTTPCodeSet::new();
        <GetRootResponse>::__groom_check_response_codes(
            &{
                let res = ::alloc::fmt::format(
                    format_args!("{0}: handler `post_root`", context),
                );
                res
            },
            &mut codes,
        );
        let mut codes = ::groom::runtime_checks::HTTPCodeSet::new();
        <RqConsQueryResponse>::__groom_check_response_codes(
            &{
                let res = ::alloc::fmt::format(
                    format_args!("{0}: handler `rq_cons_query_struct`", context),
                );
                res
            },
            &mut codes,
        );
        let mut codes = ::groom::runtime_checks::HTTPCodeSet::new();
        <RqConsPathResponse>::__groom_check_response_codes(
            &{
                let res = ::alloc::fmt::format(
                    format_args!("{0}: handler `rq_cons_path_struct`", context),
                );
                res
            },
            &mut codes,
        );
        let mut codes = ::groom::runtime_checks::HTTPCodeSet::new();
        <RespJsonResponse>::__groom_check_response_codes(
            &{
                let res = ::alloc::fmt::format(
                    format_args!("{0}: handler `resp_json`", context),
                );
                res
            },
            &mut codes,
        );
    }
    pub fn merge_into_router(other: ::axum::Router<()>) -> ::axum::Router<()> {
        __groom_runtime_checks();
        let this_router = ::axum::Router::new()
            .route("/", ::axum::routing::get(__groom_wrapper_get_root))
            .route("/", ::axum::routing::post(__groom_wrapper_post_root))
            .route("/greet", ::axum::routing::get(__groom_wrapper_rq_cons_query_struct))
            .route(
                "/team/:team_id/user/:user_id",
                ::axum::routing::get(__groom_wrapper_rq_cons_path_struct),
            )
            .route("/json", ::axum::routing::get(__groom_wrapper_resp_json));
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
                        .summary(Some("Summary"))
                        .description(Some("Description"));
                    op_builder = <GetRootResponse>::__openapi_modify_operation(
                        op_builder,
                    );
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::PathItemType::Get,
                            op_builder.build(),
                        )
                        .build()
                },
            );
        paths = paths
            .path(
                "/",
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(None as Option<String>)
                        .description(None as Option<String>);
                    op_builder = <GetRootResponse>::__openapi_modify_operation(
                        op_builder,
                    );
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::PathItemType::Post,
                            op_builder.build(),
                        )
                        .build()
                },
            );
        paths = paths
            .path(
                "/greet",
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(Some("Query<struct>"))
                        .description(None as Option<String>);
                    op_builder = <Query<
                        RqConsQueryStruct,
                    >>::__openapi_modify_operation(op_builder);
                    op_builder = <RqConsQueryResponse>::__openapi_modify_operation(
                        op_builder,
                    );
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::PathItemType::Get,
                            op_builder.build(),
                        )
                        .build()
                },
            );
        paths = paths
            .path(
                "/team/:team_id/user/:user_id",
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(Some("Path<struct>"))
                        .description(None as Option<String>);
                    op_builder = <Path<
                        RqConsPathStruct,
                    >>::__openapi_modify_operation(op_builder);
                    op_builder = <RqConsPathResponse>::__openapi_modify_operation(
                        op_builder,
                    );
                    ::utoipa::openapi::path::PathItemBuilder::new()
                        .operation(
                            ::utoipa::openapi::PathItemType::Get,
                            op_builder.build(),
                        )
                        .build()
                },
            );
        paths = paths
            .path(
                "/json",
                {
                    let mut op_builder = ::utoipa::openapi::path::OperationBuilder::new()
                        .summary(None as Option<String>)
                        .description(None as Option<String>);
                    op_builder = <RespJsonResponse>::__openapi_modify_operation(
                        op_builder,
                    );
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
