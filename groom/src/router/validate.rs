use std::collections::HashMap;

use utoipa::openapi::path::HttpMethod;

use crate::router::error::RouterValidationError;

use super::core::{GroomRouter, MethodFlags};
use super::NotValidated;
use super::Validated;

/// OpenAPI PathItem methods (eight; no CONNECT) with matching `http::Method` for errors.
const OPENAPI_METHODS: [(HttpMethod, ::http::Method); 8] = [
    (HttpMethod::Get, ::http::Method::GET),
    (HttpMethod::Post, ::http::Method::POST),
    (HttpMethod::Put, ::http::Method::PUT),
    (HttpMethod::Delete, ::http::Method::DELETE),
    (HttpMethod::Options, ::http::Method::OPTIONS),
    (HttpMethod::Head, ::http::Method::HEAD),
    (HttpMethod::Patch, ::http::Method::PATCH),
    (HttpMethod::Trace, ::http::Method::TRACE),
];

fn first_shadowed_method(existing: MethodFlags, incoming: MethodFlags) -> Option<::http::Method> {
    for (utoipa_method, http_method) in &OPENAPI_METHODS {
        if existing.contains(utoipa_method) && incoming.contains(utoipa_method) {
            return Some(http_method.clone());
        }
    }
    None
}

impl<S: Clone + Send + Sync + 'static> GroomRouter<S, NotValidated> {
    pub fn validate(self) -> Result<GroomRouter<S, Validated>, RouterValidationError> {
        // One-pass path → method-flags insert; duplicate method bit → RouteShadow (P008).
        let mut seen: HashMap<&str, MethodFlags> = HashMap::new();
        for (path, item) in &self.openapi_paths {
            let incoming = MethodFlags::from_path_item(item);
            let entry = seen.entry(path.as_str()).or_insert_with(MethodFlags::empty);
            if let Some(method) = first_shadowed_method(*entry, incoming) {
                return Err(RouterValidationError::RouteShadow {
                    path: path.clone(),
                    method,
                });
            }
            *entry = entry.union(incoming);
        }

        Ok(GroomRouter {
            router: self.router,
            registry: self.registry,
            openapi_paths: self.openapi_paths,
            path_spec_layers: self.path_spec_layers,
            whole_spec_layers: self.whole_spec_layers,
            _marker: std::marker::PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extract::ComponentsRegistry;

    #[test]
    fn test_validate_passes_with_unique_paths() {
        use utoipa::openapi::path::{PathItemBuilder, HttpMethod, OperationBuilder};
        let op = OperationBuilder::new().operation_id(Some("a")).build();
        let pi = PathItemBuilder::new().operation(HttpMethod::Get, op).build();
        let r1: GroomRouter<()> = GroomRouter::from_controller_parts(
            axum::Router::new(), ComponentsRegistry::new(),
            vec![("/a".to_string(), pi.clone())],
        );
        let r2: GroomRouter<()> = GroomRouter::from_controller_parts(
            axum::Router::new(), ComponentsRegistry::new(),
            vec![("/b".to_string(), pi)],
        );
        let merged = r1.merge(r2).unwrap();
        let validated = merged.validate();
        assert!(validated.is_ok(), "different paths should validate ok");
    }

    #[test]
    fn test_validate_detects_same_path_same_method() {
        use utoipa::openapi::path::{PathItemBuilder, HttpMethod, OperationBuilder};
        let op = OperationBuilder::new().operation_id(Some("a")).build();
        let pi = PathItemBuilder::new().operation(HttpMethod::Get, op).build();
        let r1: GroomRouter<()> = GroomRouter::from_controller_parts(
            axum::Router::new(), ComponentsRegistry::new(),
            vec![("/foo".to_string(), pi.clone())],
        );
        let r2: GroomRouter<()> = GroomRouter::from_controller_parts(
            axum::Router::new(), ComponentsRegistry::new(),
            vec![("/foo".to_string(), pi)],
        );
        let merged = r1.merge(r2).unwrap();
        let result = merged.validate();
        assert!(result.is_err(), "same path+method should fail");
        match result.err().unwrap() {
            RouterValidationError::RouteShadow { path, method } => {
                assert_eq!(path, "/foo");
                assert_eq!(method, ::http::Method::GET);
            }
            other => panic!("expected RouterValidationError::RouteShadow, got {:?}", other),
        }
    }

    #[test]
    fn test_validate_allows_same_path_different_methods() {
        use utoipa::openapi::path::{PathItemBuilder, HttpMethod, OperationBuilder};
        let get_op = OperationBuilder::new().operation_id(Some("get")).build();
        let post_op = OperationBuilder::new().operation_id(Some("post")).build();
        let get_pi = PathItemBuilder::new().operation(HttpMethod::Get, get_op).build();
        let post_pi = PathItemBuilder::new().operation(HttpMethod::Post, post_op).build();
        let r1: GroomRouter<()> = GroomRouter::from_controller_parts(
            axum::Router::new(), ComponentsRegistry::new(),
            vec![("/foo".to_string(), get_pi)],
        );
        let r2: GroomRouter<()> = GroomRouter::from_controller_parts(
            axum::Router::new(), ComponentsRegistry::new(),
            vec![("/foo".to_string(), post_pi)],
        );
        let merged = r1.merge(r2).unwrap();
        let result = merged.validate();
        assert!(result.is_ok(), "different methods on same path should be allowed (GET + POST on /foo)");
    }

    #[test]
    fn test_validate_detects_route_shadow_in_nested_routes() {
        use utoipa::openapi::path::{PathItemBuilder, HttpMethod, OperationBuilder};
        let op = OperationBuilder::new().operation_id(Some("x")).build();
        let pi = PathItemBuilder::new().operation(HttpMethod::Put, op).build();
        let r1: GroomRouter<()> = GroomRouter::from_controller_parts(
            axum::Router::new(), ComponentsRegistry::new(),
            vec![("/api/v1/foo".to_string(), pi.clone())],
        );
        let r2: GroomRouter<()> = GroomRouter::from_controller_parts(
            axum::Router::new(), ComponentsRegistry::new(),
            vec![("/api/v1/foo".to_string(), pi)],
        );
        let merged = r1.merge(r2).unwrap();
        let result = merged.validate();
        assert!(result.is_err(), "same path+method after nesting should be detected");
    }

    /// Eight-method coverage checklist (OpenAPI PathItem; no CONNECT):
    /// GET, POST, PUT, DELETE, OPTIONS, HEAD, PATCH, TRACE.
    #[test]
    fn test_validate_detects_shadow_for_all_openapi_methods() {
        use utoipa::openapi::path::{PathItemBuilder, HttpMethod, OperationBuilder};

        let cases: &[(HttpMethod, ::http::Method)] = &[
            (HttpMethod::Get, ::http::Method::GET),
            (HttpMethod::Post, ::http::Method::POST),
            (HttpMethod::Put, ::http::Method::PUT),
            (HttpMethod::Delete, ::http::Method::DELETE),
            (HttpMethod::Options, ::http::Method::OPTIONS),
            (HttpMethod::Head, ::http::Method::HEAD),
            (HttpMethod::Patch, ::http::Method::PATCH),
            (HttpMethod::Trace, ::http::Method::TRACE),
        ];

        for (utoipa_method, expected_http) in cases {
            let op = OperationBuilder::new().operation_id(Some("op")).build();
            let pi = PathItemBuilder::new()
                .operation(utoipa_method.clone(), op)
                .build();
            let r1: GroomRouter<()> = GroomRouter::from_controller_parts(
                axum::Router::new(),
                ComponentsRegistry::new(),
                vec![("/shadow".to_string(), pi.clone())],
            );
            let r2: GroomRouter<()> = GroomRouter::from_controller_parts(
                axum::Router::new(),
                ComponentsRegistry::new(),
                vec![("/shadow".to_string(), pi)],
            );
            let merged = r1.merge(r2).unwrap();
            let result = merged.validate();
            assert!(
                result.is_err(),
                "same path+{:?} should fail validation",
                expected_http
            );
            match result.err().unwrap() {
                RouterValidationError::RouteShadow { path, method } => {
                    assert_eq!(path, "/shadow");
                    assert_eq!(method, *expected_http);
                }
                other => panic!(
                    "expected RouteShadow for {:?}, got {:?}",
                    expected_http, other
                ),
            }
        }
    }
}
