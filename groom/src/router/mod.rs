mod error;
pub use error::MergeError;
pub use error::RouterValidationError;

use std::marker::PhantomData;
use utoipa::openapi::OpenApi;
use crate::extract::ComponentsRegistry;

pub type MergeResult<T> = Result<T, MergeError>;

#[derive(Clone)]
pub struct NotValidated;
#[derive(Clone)]
pub struct Validated;

pub struct GroomRouter<S = (), V = NotValidated> {
    router: axum::Router<S>,
    registry: ComponentsRegistry,
    openapi_paths: Vec<(String, utoipa::openapi::path::PathItem)>,
    _marker: PhantomData<V>,
}

pub type GroomRouterValid<S = ()> = GroomRouter<S, Validated>;

/// Prepends `prefix` to `path`, producing the full mount path for a nested route's OpenAPI entry.
///
/// Mirrors axum's internal `path_for_nested_route` logic.
/// Both `prefix` and `path` must start with `/`.
///
/// # Panics
/// Panics if either argument does not start with `/`.
pub fn prepend_path(prefix: &str, path: &str) -> String {
    assert!(prefix.starts_with('/'), "prefix must start with '/'");
    assert!(path.starts_with('/'), "path must start with '/'");

    if prefix.ends_with('/') {
        format!("{}{}", prefix, &path[1..])
    } else if path == "/" {
        prefix.to_string()
    } else {
        format!("{}{}", prefix, path)
    }
}

impl<S: Clone + Send + Sync + 'static, V> GroomRouter<S, V> {
    /// macro-internal — not public API
    #[doc(hidden)]
    pub fn from_controller_parts(
        router: axum::Router<S>,
        registry: ComponentsRegistry,
        openapi_paths: Vec<(String, utoipa::openapi::path::PathItem)>,
    ) -> Self {
        Self { router, registry, openapi_paths, _marker: PhantomData }
    }

    pub fn fallback<H, T>(self, handler: H) -> Self
    where
        H: axum::handler::Handler<T, S>,
        T: 'static,
    {
        Self {
            router: self.router.fallback(handler),
            registry: self.registry,
            openapi_paths: self.openapi_paths,
            _marker: PhantomData,
        }
    }
}

impl<S: Clone + Send + Sync + 'static> GroomRouter<S, NotValidated> {
    pub fn new() -> Self {
        Self {
            router: axum::Router::new(),
            registry: ComponentsRegistry::new(),
            openapi_paths: Vec::new(),
            _marker: PhantomData,
        }
    }

    pub fn merge(self, other: GroomRouter<S, NotValidated>) -> MergeResult<Self> {
        let router = self.router.merge(other.router);
        let mut openapi_paths = self.openapi_paths;
        openapi_paths.extend(other.openapi_paths);
        let registry = self.registry
            .merge(other.registry)
            .map_err(|(name, _existing, _incoming)| MergeError::SchemaConflict {
                name,
                source_a: "self".into(),
                source_b: "other".into(),
            })?;
        Ok(Self { router, registry, openapi_paths, _marker: PhantomData })
    }

    pub fn nest(self, path: &str, other: GroomRouter<S, NotValidated>) -> MergeResult<Self> {
        let router = self.router.nest(path, other.router);
        let prefixed_paths: Vec<_> = other.openapi_paths
            .into_iter()
            .map(|(p, item)| (prepend_path(path, &p), item))
            .collect();
        let mut openapi_paths = self.openapi_paths;
        openapi_paths.extend(prefixed_paths);
        let registry = self.registry
            .merge(other.registry)
            .map_err(|(name, _existing, _incoming)| MergeError::SchemaConflict {
                name,
                source_a: "self".into(),
                source_b: "other".into(),
            })?;
        Ok(Self { router, registry, openapi_paths, _marker: PhantomData })
    }

    pub fn layer<L>(self, layer: L) -> Self
    where
        L: tower::layer::Layer<axum::routing::Route> + Clone + Send + Sync + 'static,
        L::Service: tower::Service<axum::extract::Request> + Clone + Send + Sync + 'static,
        <L::Service as tower::Service<axum::extract::Request>>::Response: axum::response::IntoResponse + 'static,
        <L::Service as tower::Service<axum::extract::Request>>::Error: Into<std::convert::Infallible> + 'static,
        <L::Service as tower::Service<axum::extract::Request>>::Future: Send + 'static,
    {
        Self {
            router: self.router.layer(layer),
            registry: self.registry,
            openapi_paths: self.openapi_paths,
            _marker: PhantomData,
        }
    }

    pub fn route_layer<L>(self, layer: L) -> Self
    where
        L: tower::layer::Layer<axum::routing::Route> + Clone + Send + Sync + 'static,
        L::Service: tower::Service<axum::extract::Request> + Clone + Send + Sync + 'static,
        <L::Service as tower::Service<axum::extract::Request>>::Response: axum::response::IntoResponse + 'static,
        <L::Service as tower::Service<axum::extract::Request>>::Error: Into<std::convert::Infallible> + 'static,
        <L::Service as tower::Service<axum::extract::Request>>::Future: Send + 'static,
    {
        Self {
            router: self.router.route_layer(layer),
            registry: self.registry,
            openapi_paths: self.openapi_paths,
            _marker: PhantomData,
        }
    }

    pub fn validate(self) -> Result<GroomRouter<S, Validated>, RouterValidationError> {
        for i in 0..self.openapi_paths.len() {
            for j in (i + 1)..self.openapi_paths.len() {
                let (path_a, item_a) = &self.openapi_paths[i];
                let (path_b, item_b) = &self.openapi_paths[j];

                if path_a != path_b {
                    continue;
                }

                if item_a.get.is_some() && item_b.get.is_some() {
                    return Err(RouterValidationError::RouteShadow { path: path_a.clone(), method: ::http::Method::GET });
                }
                if item_a.post.is_some() && item_b.post.is_some() {
                    return Err(RouterValidationError::RouteShadow { path: path_a.clone(), method: ::http::Method::POST });
                }
                if item_a.put.is_some() && item_b.put.is_some() {
                    return Err(RouterValidationError::RouteShadow { path: path_a.clone(), method: ::http::Method::PUT });
                }
                if item_a.delete.is_some() && item_b.delete.is_some() {
                    return Err(RouterValidationError::RouteShadow { path: path_a.clone(), method: ::http::Method::DELETE });
                }
                if item_a.options.is_some() && item_b.options.is_some() {
                    return Err(RouterValidationError::RouteShadow { path: path_a.clone(), method: ::http::Method::OPTIONS });
                }
                if item_a.head.is_some() && item_b.head.is_some() {
                    return Err(RouterValidationError::RouteShadow { path: path_a.clone(), method: ::http::Method::HEAD });
                }
                if item_a.patch.is_some() && item_b.patch.is_some() {
                    return Err(RouterValidationError::RouteShadow { path: path_a.clone(), method: ::http::Method::PATCH });
                }
                if item_a.trace.is_some() && item_b.trace.is_some() {
                    return Err(RouterValidationError::RouteShadow { path: path_a.clone(), method: ::http::Method::TRACE });
                }
            }
        }

        Ok(GroomRouter {
            router: self.router,
            registry: self.registry,
            openapi_paths: self.openapi_paths,
            _marker: PhantomData,
        })
    }
}

impl<S: Clone + Send + Sync + 'static> GroomRouter<S, Validated> {
    pub fn to_openapi(&self, mut api: OpenApi) -> OpenApi {
        let mut paths_builder = utoipa::openapi::path::PathsBuilder::new();
        for (path_str, path_item) in &self.openapi_paths {
            paths_builder = paths_builder.path(path_str.as_str(), path_item.clone());
        }
        let paths = paths_builder.build();

        let existing_components = api.components
            .take()
            .unwrap_or_else(utoipa::openapi::Components::new);
        let merged_components = self.registry.into_components(existing_components);

        let spec_part = utoipa::openapi::OpenApiBuilder::new()
            .paths(paths)
            .components(Some(merged_components))
            .build();

        api.merge(spec_part);
        api
    }

    pub fn to_axum_router(self) -> axum::Router<S> {
        self.router
    }
}

impl GroomRouter<(), NotValidated> {
    pub fn with_state<S2: Clone + Send + Sync + 'static>(self, state: ()) -> GroomRouter<S2, NotValidated> {
        GroomRouter {
            router: self.router.with_state(state),
            registry: self.registry,
            openapi_paths: self.openapi_paths,
            _marker: PhantomData,
        }
    }
}

impl<S: Clone + Send + Sync + 'static> Default for GroomRouter<S, NotValidated> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: Clone, V: Clone> Clone for GroomRouter<S, V> {
    fn clone(&self) -> Self {
        Self {
            router: self.router.clone(),
            registry: self.registry.clone(),
            openapi_paths: self.openapi_paths.clone(),
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod sub1 {
        #[derive(utoipa::ToSchema)]
        pub struct SchemaA {
            pub value: i32,
        }
    }

    mod sub2 {
        #[derive(utoipa::ToSchema)]
        pub struct SchemaA {
            pub value: String,
        }
    }

    fn router_with_schema_a() -> GroomRouter {
        let mut reg = ComponentsRegistry::new();
        reg.add_components::<sub1::SchemaA>();
        GroomRouter::from_controller_parts(
            axum::Router::new(),
            reg,
            Vec::new(),
        )
    }

    fn router_with_schema_a_string() -> GroomRouter {
        let mut reg = ComponentsRegistry::new();
        reg.add_components::<sub2::SchemaA>();
        GroomRouter::from_controller_parts(
            axum::Router::new(),
            reg,
            Vec::new(),
        )
    }

    fn router_with_schema_a_duplicate() -> GroomRouter {
        let mut reg = ComponentsRegistry::new();
        reg.add_components::<sub1::SchemaA>();
        GroomRouter::from_controller_parts(
            axum::Router::new(),
            reg,
            Vec::new(),
        )
    }

    #[test]
    fn test_new_creates_empty_router() {
        let r: GroomRouter<()> = GroomRouter::new();
        let _axum_router: axum::Router = r.validate().unwrap().to_axum_router();
    }

    #[test]
    fn test_merge_combines_empty_routers() {
        let r1: GroomRouter<()> = GroomRouter::new();
        let r2: GroomRouter<()> = GroomRouter::new();
        let result = r1.merge(r2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_merge_accepts_identical_schemas() {
        let r1 = router_with_schema_a();
        let r2 = router_with_schema_a_duplicate();
        let result = r1.merge(r2);
        assert!(result.is_ok(), "identical schemas with same name should merge ok");
    }

    #[test]
    fn test_merge_rejects_conflicting_schemas() {
        let r1 = router_with_schema_a();
        let r2 = router_with_schema_a_string();
        let result = r1.merge(r2);
        assert!(result.is_err(), "differing schemas with same name should conflict");
        match result.err().unwrap() {
            MergeError::SchemaConflict { name, .. } => {
                assert_eq!(name, "SchemaA");
            }
            _ => panic!("expected SchemaConflict"),
        }
    }

    #[test]
    fn test_nest_empty_routers() {
        let r1: GroomRouter<()> = GroomRouter::new();
        let r2: GroomRouter<()> = GroomRouter::new();
        let result = r1.nest("/api", r2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_nest_detects_conflicts() {
        let r1 = router_with_schema_a();
        let r2 = router_with_schema_a_string();
        let result = r1.nest("/api", r2);
        assert!(result.is_err(), "nest should detect schema conflicts like merge");
    }

    #[test]
    fn test_fallback_delegation() {
        use axum::response::IntoResponse;
        async fn fallback_handler() -> impl IntoResponse { "fallback" }
        let r: GroomRouter<()> = GroomRouter::new();
        let r = r.fallback(fallback_handler);
        let _axum: axum::Router = r.validate().unwrap().to_axum_router();
    }

    #[test]
    fn test_with_state_delegation() {
        let r: GroomRouter<()> = GroomRouter::new();
        let r = r.with_state(());
        let _axum: axum::Router<i32> = r.validate().unwrap().to_axum_router();
    }

    #[test]
    fn test_to_axum_router_converts_back() {
        let r: GroomRouter<()> = GroomRouter::new();
        let axum_r: axum::Router = r.validate().unwrap().to_axum_router();
        let _merged = axum::Router::new().merge(axum_r);
    }

    #[test]
    fn test_to_openapi_returns_valid_openapi() {
        use utoipa::OpenApi;
        #[derive(OpenApi)]
        #[openapi(info(title = "test", version = "0.1.0"))]
        struct ApiDoc;

        let r: GroomRouter<()> = GroomRouter::new();
        let api = r.validate().unwrap().to_openapi(ApiDoc::openapi());
        assert_eq!(api.info.title, "test");
    }

    #[test]
    fn test_to_openapi_includes_paths() {
        use utoipa::OpenApi;
        use utoipa::openapi::path::{PathItemBuilder, HttpMethod, OperationBuilder};

        #[derive(OpenApi)]
        #[openapi(info(title = "test", version = "0.1.0"))]
        struct ApiDoc;

        let operation = OperationBuilder::new()
            .operation_id(Some("hello"))
            .build();
        let path_item = PathItemBuilder::new()
            .operation(HttpMethod::Get, operation)
            .build();

        let r: GroomRouter<()> = GroomRouter::from_controller_parts(
            axum::Router::new(),
            ComponentsRegistry::new(),
            vec![("/hello".to_string(), path_item)],
        );

        let api = r.validate().unwrap().to_openapi(ApiDoc::openapi());
        let paths_json = serde_json::to_value(&api.paths).unwrap();
        assert!(
            paths_json.get("/hello").is_some(),
            "to_openapi should include stored paths: got {:?}",
            paths_json
        );
    }

    #[test]
    fn test_no_route_method_available() {
        let r: GroomRouter<()> = GroomRouter::new();
        let _ = r.validate().unwrap().to_axum_router();
    }

    #[test]
    fn test_merge_error_display() {
        let err = MergeError::SchemaConflict {
            name: "Foo".into(),
            source_a: "controller_a".into(),
            source_b: "controller_b".into(),
        };
        let msg = err.to_string();
        assert!(msg.contains("Foo"), "Display should include schema name: got {}", msg);
        assert!(msg.contains("controller_a"), "Display should include source_a: got {}", msg);
    }

    #[test]
    fn test_merge_error_debug() {
        let err = MergeError::SchemaNotFound {
            path: "#/components/schemas/Foo".into(),
            registry: "controller_b".into(),
        };
        let _ = format!("{:?}", err);
    }

    #[test]
    fn test_clone() {
        let r1: GroomRouter<()> = GroomRouter::new();
        let r2 = r1.clone();
        let _ = r2.validate().unwrap().to_axum_router();
    }

    #[test]
    fn test_default() {
        let r: GroomRouter<()> = Default::default();
        let _ = r.validate().unwrap().to_axum_router();
    }

    #[test]
    fn test_merge_combines_paths() {
        use utoipa::openapi::path::{PathItemBuilder, HttpMethod, OperationBuilder};

        let op = OperationBuilder::new().operation_id(Some("a")).build();
        let path_item = PathItemBuilder::new()
            .operation(HttpMethod::Get, op)
            .build();

        let r1: GroomRouter<()> = GroomRouter::from_controller_parts(
            axum::Router::new(),
            ComponentsRegistry::new(),
            vec![("/a".to_string(), path_item.clone())],
        );
        let r2: GroomRouter<()> = GroomRouter::from_controller_parts(
            axum::Router::new(),
            ComponentsRegistry::new(),
            vec![("/b".to_string(), path_item)],
        );

        let merged = r1.merge(r2).unwrap();
        assert_eq!(merged.openapi_paths.len(), 2);
    }

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

    #[test]
    fn test_prepend_path_trailing_slash_prefix() {
        assert_eq!(super::prepend_path("/api/v1/", "/foo"), "/api/v1/foo");
        assert_eq!(super::prepend_path("/api/", "/v1/foo"), "/api/v1/foo");
        assert_eq!(super::prepend_path("/", "/foo"), "/foo");
    }

    #[test]
    fn test_prepend_path_root_path() {
        assert_eq!(super::prepend_path("/api/v1", "/"), "/api/v1");
        assert_eq!(super::prepend_path("/api", "/"), "/api");
    }

    #[test]
    fn test_prepend_path_normal_join() {
        assert_eq!(super::prepend_path("/api/v1", "/foo"), "/api/v1/foo");
        assert_eq!(super::prepend_path("/api", "/v1/foo"), "/api/v1/foo");
        assert_eq!(super::prepend_path("/root", "/path"), "/root/path");
    }

    #[test]
    fn test_prepend_path_path_params() {
        assert_eq!(super::prepend_path("/api/v1/{version}", "/foo/{id}"), "/api/v1/{version}/foo/{id}");
        assert_eq!(super::prepend_path("/root", "/{param}"), "/root/{param}");
    }

    #[test]
    #[should_panic(expected = "prefix must start with '/'")]
    fn test_prepend_path_panics_on_bad_prefix() {
        super::prepend_path("api/v1", "/foo");
    }

    #[test]
    #[should_panic(expected = "path must start with '/'")]
    fn test_prepend_path_panics_on_bad_path() {
        super::prepend_path("/api/v1", "foo");
    }
}
