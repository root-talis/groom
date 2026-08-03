use utoipa::openapi::OpenApi;

/// Extension point for tower middleware to contribute to the generated OpenAPI spec.
pub trait OpenApiSpecLayer: Send + Sync + Clone + 'static {
    /// Modify the OpenAPI spec to document this middleware's behavior.
    ///
    /// Called once per spec layer during [`GroomRouter::to_openapi`].
    /// The method receives a mutable reference to the OpenAPI spec
    /// so it can add security schemes, response codes, or other metadata.
    fn modify_openapi(&self, _api: &mut utoipa::openapi::OpenApi) {
        // Default: do nothing
    }

    /// Modify individual OpenAPI operations for this middleware's behavior.
    ///
    /// Called once per (path, method) combination during [`GroomRouter::to_openapi`],
    /// before [`modify_openapi`](Self::modify_openapi). The method receives the path,
    /// HTTP method, and a mutable reference to the operation so it can add per-operation
    /// metadata like security requirements.
    ///
    /// The default implementation does nothing. Override this to modify operations
    /// (e.g., adding `security` requirements to each operation).
    fn modify_operation(
        &self,
        _path: &str,
        _method: &utoipa::openapi::path::HttpMethod,
        _operation: &mut utoipa::openapi::path::Operation,
    ) {
        // Default: do nothing
    }

    /// Clone this spec layer into a type-erased box for storage.
    ///
    /// Required so that `GroomRouter` can implement `Clone` when it
    /// contains spec layers. Implementors should delegate to their
    /// type's `Clone` impl.
    fn clone_box(&self) -> Box<dyn SpecLayerModifier> {
        Box::new(self.clone())
    }

    /// Mount this spec layer into axum::Router through layer
    fn mount<S>(&self, r: axum::Router<S>) -> axum::Router<S> where S: Clone + Send + Sync + 'static;
}

/// Internal trait for type-erased storage of spec layers.
///
/// This trait mirrors [`OpenApiSpecLayer`] but without the generic `mount<S>`` method,
/// so that the generic type does not need to be specified in the
/// trait object. A blanket impl delegates to [`OpenApiSpecLayer`].
pub trait SpecLayerModifier: Send + Sync + 'static {
    /// Modify the OpenAPI spec.
    fn modify_openapi(&self, api: &mut OpenApi);

    /// Modify individual OpenAPI operations.
    fn modify_operation(
        &self,
        path: &str,
        method: &utoipa::openapi::path::HttpMethod,
        operation: &mut utoipa::openapi::path::Operation,
    );

    /// Clone into a boxed trait object.
    fn clone_box(&self) -> Box<dyn SpecLayerModifier>;
}

impl<T: OpenApiSpecLayer + 'static> SpecLayerModifier for T {
    fn modify_openapi(&self, api: &mut OpenApi) {
        OpenApiSpecLayer::modify_openapi(self, api)
    }

    fn modify_operation(
        &self,
        path: &str,
        method: &utoipa::openapi::path::HttpMethod,
        operation: &mut utoipa::openapi::path::Operation,
    ) {
        OpenApiSpecLayer::modify_operation(self, path, method, operation)
    }

    fn clone_box(&self) -> Box<dyn SpecLayerModifier> {
        OpenApiSpecLayer::clone_box(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extract::ComponentsRegistry;
    use crate::router::core::GroomRouter;

    /// Simple spec layer that sets the info title
    #[derive(Clone)]
    struct TitleSpecLayer {
        title: String,
    }

    impl OpenApiSpecLayer for TitleSpecLayer {
        fn modify_openapi(&self, api: &mut OpenApi) {
            api.info.title = self.title.clone();
        }

        fn mount<S>(&self, r: axum::Router<S>) -> axum::Router<S> {
            r
        }
    }

    #[test]
    fn test_layer_with_spec_stores_spec_layer() {
        use utoipa::openapi::path::{PathItemBuilder, HttpMethod, OperationBuilder};
        let op = OperationBuilder::new().operation_id(Some("a")).build();
        let pi = PathItemBuilder::new().operation(HttpMethod::Get, op).build();
        let r: GroomRouter<()> = GroomRouter::from_controller_parts(
            axum::Router::new(),
            ComponentsRegistry::new(),
            vec![("/a".to_string(), pi)],
        )
        .layer_with_spec(TitleSpecLayer {
            title: "Custom Title".into(),
        });
        // Spec layer stored per-path in HashMap
        assert_eq!(r.path_spec_layers.len(), 1);
        assert_eq!(r.path_spec_layers["/a"].len(), 1);
    }

    #[test]
    fn test_to_openapi_invokes_spec_layers() {
        use utoipa::OpenApi;
        use utoipa::openapi::path::{PathItemBuilder, HttpMethod, OperationBuilder};
        #[derive(OpenApi)]
        #[openapi(info(title = "original", version = "0.1.0"))]
        struct ApiDoc;

        let op = OperationBuilder::new().operation_id(Some("a")).build();
        let pi = PathItemBuilder::new().operation(HttpMethod::Get, op).build();
        let r: GroomRouter<()> = GroomRouter::from_controller_parts(
            axum::Router::new(),
            ComponentsRegistry::new(),
            vec![("/a".to_string(), pi)],
        )
        .layer_with_spec(TitleSpecLayer {
            title: "Modified Title".into(),
        });
        let api = r.validate().unwrap().to_openapi(ApiDoc::openapi());
        assert_eq!(api.info.title, "Modified Title");
    }

    #[test]
    fn test_multiple_spec_layers_compose() {
        use utoipa::OpenApi;
        use utoipa::openapi::path::{PathItemBuilder, HttpMethod, OperationBuilder};
        #[derive(OpenApi)]
        #[openapi(info(title = "original", version = "0.1.0"))]
        struct ApiDoc;

        let op = OperationBuilder::new().operation_id(Some("a")).build();
        let pi = PathItemBuilder::new().operation(HttpMethod::Get, op).build();
        let r: GroomRouter<()> = GroomRouter::from_controller_parts(
            axum::Router::new(),
            ComponentsRegistry::new(),
            vec![("/a".to_string(), pi)],
        )
        .layer_with_spec(TitleSpecLayer {
            title: "First".into(),
        })
        .layer_with_spec(TitleSpecLayer {
            title: "Second".into(),
        });
        let api = r.validate().unwrap().to_openapi(ApiDoc::openapi());
        // Both layers run in insertion order; second overwrites first
        assert_eq!(api.info.title, "Second");
    }

    #[test]
    fn test_merge_combines_spec_layers() {
        use utoipa::openapi::path::{PathItemBuilder, HttpMethod, OperationBuilder};
        let op = OperationBuilder::new().operation_id(Some("a")).build();
        let pi = PathItemBuilder::new().operation(HttpMethod::Get, op).build();
        let r1: GroomRouter<()> = GroomRouter::from_controller_parts(
            axum::Router::new(), ComponentsRegistry::new(),
            vec![("/a".to_string(), pi.clone())],
        ).layer_with_spec(TitleSpecLayer { title: "A".into() });
        let r2: GroomRouter<()> = GroomRouter::from_controller_parts(
            axum::Router::new(), ComponentsRegistry::new(),
            vec![("/b".to_string(), pi)],
        ).layer_with_spec(TitleSpecLayer { title: "B".into() });
        let merged = r1.merge(r2).unwrap();
        // Each path carries its own spec layers in HashMap
        assert_eq!(merged.path_spec_layers.len(), 2);
        assert_eq!(merged.path_spec_layers["/a"].len(), 1);
        assert_eq!(merged.path_spec_layers["/b"].len(), 1);
    }

    #[test]
    fn test_nest_combines_spec_layers() {
        use utoipa::openapi::path::{PathItemBuilder, HttpMethod, OperationBuilder};
        let op = OperationBuilder::new().operation_id(Some("a")).build();
        let pi = PathItemBuilder::new().operation(HttpMethod::Get, op).build();
        let inner: GroomRouter<()> = GroomRouter::from_controller_parts(
            axum::Router::new(), ComponentsRegistry::new(),
            vec![("/inner".to_string(), pi)],
        ).layer_with_spec(TitleSpecLayer { title: "inner".into() });
        let outer: GroomRouter<()> = GroomRouter::new();
        let result = outer.nest("/api", inner);
        let nested = result.unwrap();
        // Inner spec layers propagate with prefixed path key
        assert_eq!(nested.path_spec_layers.len(), 1);
        assert_eq!(nested.path_spec_layers["/api/inner"].len(), 1);
    }

    /// Controllers emit one PathItem per method with the same path string.
    /// Nest must rewrite path_spec_layers once per path key (not once per PathItem).
    #[test]
    fn test_nest_multi_pathitem_same_path_keeps_one_spec_binding() {
        use utoipa::openapi::path::{PathItemBuilder, HttpMethod, OperationBuilder};
        let get_op = OperationBuilder::new().operation_id(Some("get_inner")).build();
        let post_op = OperationBuilder::new().operation_id(Some("post_inner")).build();
        let get_only = PathItemBuilder::new()
            .operation(HttpMethod::Get, get_op)
            .build();
        let post_only = PathItemBuilder::new()
            .operation(HttpMethod::Post, post_op)
            .build();
        let inner: GroomRouter<()> = GroomRouter::from_controller_parts(
            axum::Router::new(),
            ComponentsRegistry::new(),
            vec![
                ("/inner".to_string(), get_only),
                ("/inner".to_string(), post_only),
            ],
        )
        .layer_with_spec(TitleSpecLayer {
            title: "inner".into(),
        });
        let outer: GroomRouter<()> = GroomRouter::new();
        let nested = outer.nest("/api", inner).unwrap();
        assert_eq!(nested.path_spec_layers.len(), 1);
        assert_eq!(
            nested.path_spec_layers["/api/inner"].len(),
            1,
            "nest must keep one SpecLayerBinding per path key for multi-PathItem controllers"
        );
    }

    #[test]
    fn test_existing_layer_behavior_unchanged() {
        // Existing .layer() should still work without spec_layers changes breaking it
        let r: GroomRouter<()> = GroomRouter::new()
            .layer(tower::layer::util::Identity::new());
        let _ = r.validate().unwrap().to_axum_router();
    }

    /// Spec layer that adds a description to every operation via modify_operation
    #[derive(Clone)]
    struct OperationTagSpecLayer {
        tag: String,
    }

    impl OpenApiSpecLayer for OperationTagSpecLayer {
        fn modify_openapi(&self, _api: &mut OpenApi) {}

        fn modify_operation(
            &self,
            _path: &str,
            _method: &utoipa::openapi::path::HttpMethod,
            operation: &mut utoipa::openapi::path::Operation,
        ) {
            operation.description = Some(self.tag.clone());
        }

        fn mount<S>(&self, r: axum::Router<S>) -> axum::Router<S> {
            r
        }
    }

    impl tower::Layer<axum::routing::Route> for OperationTagSpecLayer {
        type Service = axum::routing::Route;
        fn layer(&self, inner: axum::routing::Route) -> Self::Service {
            inner
        }
    }

    #[test]
    fn test_modify_operation_called_per_operation() {
        use utoipa::OpenApi;
        use utoipa::openapi::path::{PathItemBuilder, HttpMethod, OperationBuilder};

        #[derive(OpenApi)]
        #[openapi(info(title = "test", version = "0.1.0"))]
        struct ApiDoc;

        let get_op = OperationBuilder::new().operation_id(Some("get_hello")).build();
        let post_op = OperationBuilder::new().operation_id(Some("post_hello")).build();
        let path_item = PathItemBuilder::new()
            .operation(HttpMethod::Get, get_op)
            .operation(HttpMethod::Post, post_op)
            .build();

        let r: GroomRouter<()> = GroomRouter::from_controller_parts(
            axum::Router::new(),
            ComponentsRegistry::new(),
            vec![("/hello".to_string(), path_item)],
        ).layer_with_spec(OperationTagSpecLayer {
            tag: "secured".into(),
        });

        let api = r.validate().unwrap().to_openapi(ApiDoc::openapi());
        let hello_path = api.paths.paths.get("/hello").expect("/hello should exist");

        // Both operations should have been modified
        assert_eq!(
            hello_path.get.as_ref().unwrap().description.as_deref(),
            Some("secured"),
            "modify_operation should set description on GET operation"
        );
        assert_eq!(
            hello_path.post.as_ref().unwrap().description.as_deref(),
            Some("secured"),
            "modify_operation should set description on POST operation"
        );
    }

    /// Spec layer that checks ordering: if operation.description is set,
    /// modify_operation ran first. Then modify_openapi sets info.title.
    #[derive(Clone)]
    struct OrderingSpecLayer;

    impl OpenApiSpecLayer for OrderingSpecLayer {
        fn modify_openapi(&self, api: &mut OpenApi) {
            // This runs AFTER modify_operation, so description should already be set
            let has_desc = api.paths.paths.values().any(|pi| {
                pi.get.as_ref().is_some_and(|op| op.description.is_some())
            });
            if has_desc {
                api.info.title = "ORDERED".into();
            } else {
                api.info.title = "WRONG_ORDER".into();
            }
        }

        fn modify_operation(
            &self,
            _path: &str,
            _method: &utoipa::openapi::path::HttpMethod,
            operation: &mut utoipa::openapi::path::Operation,
        ) {
            operation.description = Some("set-first".into());
        }

        fn mount<S>(&self, r: axum::Router<S>) -> axum::Router<S> { r }
    }

    impl tower::Layer<axum::routing::Route> for OrderingSpecLayer {
        type Service = axum::routing::Route;
        fn layer(&self, inner: axum::routing::Route) -> Self::Service { inner }
    }

    #[test]
    fn test_modify_operation_runs_before_modify_openapi() {
        use utoipa::OpenApi;
        use utoipa::openapi::path::{PathItemBuilder, HttpMethod, OperationBuilder};

        #[derive(OpenApi)]
        #[openapi(info(title = "test", version = "0.1.0"))]
        struct ApiDoc;

        let op = OperationBuilder::new().operation_id(Some("hello")).build();
        let path_item = PathItemBuilder::new()
            .operation(HttpMethod::Get, op)
            .build();

        let r: GroomRouter<()> = GroomRouter::from_controller_parts(
            axum::Router::new(),
            ComponentsRegistry::new(),
            vec![("/hello".to_string(), path_item)],
        ).layer_with_spec(OrderingSpecLayer);

        let api = r.validate().unwrap().to_openapi(ApiDoc::openapi());
        assert_eq!(api.info.title, "ORDERED",
            "modify_operation must run before modify_openapi");
    }

    /// Same-path merge must append per-path layers and apply modify_operation
    /// only to methods present when each layer was attached (P002 / D-10, D-13, D-14).
    #[test]
    fn test_same_path_merge_keeps_method_affinity_for_spec_layers() {
        use utoipa::OpenApi;
        use utoipa::openapi::path::{PathItemBuilder, HttpMethod, OperationBuilder};

        #[derive(OpenApi)]
        #[openapi(info(title = "test", version = "0.1.0"))]
        struct ApiDoc;

        let get_op = OperationBuilder::new().operation_id(Some("get_foo")).build();
        let get_item = PathItemBuilder::new()
            .operation(HttpMethod::Get, get_op)
            .build();
        let post_op = OperationBuilder::new().operation_id(Some("post_foo")).build();
        let post_item = PathItemBuilder::new()
            .operation(HttpMethod::Post, post_op)
            .build();

        let r1: GroomRouter<()> = GroomRouter::from_controller_parts(
            axum::Router::new(),
            ComponentsRegistry::new(),
            vec![("/foo".to_string(), get_item)],
        )
        .layer_with_spec(OperationTagSpecLayer {
            tag: "get-only".into(),
        });
        let r2: GroomRouter<()> = GroomRouter::from_controller_parts(
            axum::Router::new(),
            ComponentsRegistry::new(),
            vec![("/foo".to_string(), post_item)],
        )
        .layer_with_spec(OperationTagSpecLayer {
            tag: "post-only".into(),
        });

        let merged = r1.merge(r2).unwrap();
        assert_eq!(
            merged.path_spec_layers["/foo"].len(),
            2,
            "same-path merge must append both controllers' spec layers"
        );

        let api = merged.validate().unwrap().to_openapi(ApiDoc::openapi());
        let foo = api.paths.paths.get("/foo").expect("/foo should exist");

        assert_eq!(
            foo.get.as_ref().unwrap().description.as_deref(),
            Some("get-only"),
            "GET must receive only the get-only layer"
        );
        assert_eq!(
            foo.post.as_ref().unwrap().description.as_deref(),
            Some("post-only"),
            "POST must receive only the post-only layer"
        );
    }

    /// Counts `modify_openapi` invocations (P003: must run once per attach, not once per path).
    #[derive(Clone)]
    struct CountingSpecLayer {
        calls: std::sync::Arc<std::sync::atomic::AtomicUsize>,
    }

    impl OpenApiSpecLayer for CountingSpecLayer {
        fn modify_openapi(&self, _api: &mut OpenApi) {
            self.calls
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }

        fn mount<S>(&self, r: axum::Router<S>) -> axum::Router<S> {
            r
        }
    }

    #[test]
    fn test_modify_openapi_runs_once_for_multi_path_layer() {
        use std::sync::atomic::AtomicUsize;
        use std::sync::Arc;
        use utoipa::OpenApi;
        use utoipa::openapi::path::{HttpMethod, OperationBuilder, PathItemBuilder};

        #[derive(OpenApi)]
        #[openapi(info(title = "original", version = "0.1.0"))]
        struct ApiDoc;

        let calls = Arc::new(AtomicUsize::new(0));
        let op = OperationBuilder::new().operation_id(Some("op")).build();
        let pi = PathItemBuilder::new()
            .operation(HttpMethod::Get, op)
            .build();

        let r: GroomRouter<()> = GroomRouter::from_controller_parts(
            axum::Router::new(),
            ComponentsRegistry::new(),
            vec![
                ("/a".to_string(), pi.clone()),
                ("/b".to_string(), pi.clone()),
                ("/c".to_string(), pi),
            ],
        )
        .layer_with_spec(CountingSpecLayer {
            calls: Arc::clone(&calls),
        });

        let _api = r.validate().unwrap().to_openapi(ApiDoc::openapi());
        assert_eq!(
            calls.load(std::sync::atomic::Ordering::SeqCst),
            1,
            "modify_openapi must run once per layer attach, not once per path"
        );
    }
}
