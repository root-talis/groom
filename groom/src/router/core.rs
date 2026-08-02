use std::collections::HashMap;
use std::marker::PhantomData;

use utoipa::openapi::path::{HttpMethod, PathItem};

use crate::extract::ComponentsRegistry;

use super::error::MergeError;
use super::traits::{OpenApiSpecLayer, SpecLayerModifier};
use super::{MergeResult, NotValidated};

/// Bit mask of HTTP methods a per-path spec layer is tagged for (D-13 / D-14).
///
/// Mapped to utoipa's eight `HttpMethod` variants — no external bitflags crate.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct MethodFlags(u8);

impl MethodFlags {
    const GET: u8 = 1 << 0;
    const PUT: u8 = 1 << 1;
    const POST: u8 = 1 << 2;
    const DELETE: u8 = 1 << 3;
    const OPTIONS: u8 = 1 << 4;
    const HEAD: u8 = 1 << 5;
    const PATCH: u8 = 1 << 6;
    const TRACE: u8 = 1 << 7;

    pub(crate) fn empty() -> Self {
        Self(0)
    }

    pub(crate) fn from_path_item(item: &PathItem) -> Self {
        let mut flags = Self::empty();
        if item.get.is_some() {
            flags = flags.union_method(HttpMethod::Get);
        }
        if item.put.is_some() {
            flags = flags.union_method(HttpMethod::Put);
        }
        if item.post.is_some() {
            flags = flags.union_method(HttpMethod::Post);
        }
        if item.delete.is_some() {
            flags = flags.union_method(HttpMethod::Delete);
        }
        if item.options.is_some() {
            flags = flags.union_method(HttpMethod::Options);
        }
        if item.head.is_some() {
            flags = flags.union_method(HttpMethod::Head);
        }
        if item.patch.is_some() {
            flags = flags.union_method(HttpMethod::Patch);
        }
        if item.trace.is_some() {
            flags = flags.union_method(HttpMethod::Trace);
        }
        flags
    }

    pub(crate) fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub(crate) fn union_method(self, method: HttpMethod) -> Self {
        Self(self.0 | Self::bit(&method))
    }

    pub(crate) fn contains(self, method: &HttpMethod) -> bool {
        self.0 & Self::bit(method) != 0
    }

    fn bit(method: &HttpMethod) -> u8 {
        match method {
            HttpMethod::Get => Self::GET,
            HttpMethod::Put => Self::PUT,
            HttpMethod::Post => Self::POST,
            HttpMethod::Delete => Self::DELETE,
            HttpMethod::Options => Self::OPTIONS,
            HttpMethod::Head => Self::HEAD,
            HttpMethod::Patch => Self::PATCH,
            HttpMethod::Trace => Self::TRACE,
        }
    }
}

/// A per-path OpenAPI spec layer tagged with the methods present at attach time.
pub(crate) struct SpecLayerBinding {
    pub(crate) methods: MethodFlags,
    pub(crate) layer: Box<dyn SpecLayerModifier>,
}

impl SpecLayerBinding {
    pub(crate) fn clone_binding(&self) -> Self {
        Self {
            methods: self.methods,
            layer: self.layer.clone_box(),
        }
    }
}

pub struct GroomRouter<S = (), V = NotValidated> {
    pub(crate) router: axum::Router<S>,
    pub(crate) registry: ComponentsRegistry,
    pub(crate) openapi_paths: Vec<(String, utoipa::openapi::path::PathItem)>,
    /// Per-path spec layers, keyed by path string. Ensures that when controllers are
    /// merged, spec layers only apply to the operations they were attached to.
    pub(crate) path_spec_layers: HashMap<String, Vec<SpecLayerBinding>>,
    pub(crate) _marker: PhantomData<V>,
}

impl<S: Clone + Send + Sync + 'static, V> GroomRouter<S, V> {
    /// macro-internal — not public API
    #[doc(hidden)]
    pub fn from_controller_parts(
        router: axum::Router<S>,
        registry: ComponentsRegistry,
        openapi_paths: Vec<(String, utoipa::openapi::path::PathItem)>,
    ) -> Self {
        let path_spec_layers: HashMap<String, Vec<SpecLayerBinding>> = openapi_paths
            .iter()
            .map(|(path, _)| (path.clone(), Vec::new()))
            .collect();
        Self { router, registry, openapi_paths, path_spec_layers, _marker: PhantomData }
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
            path_spec_layers: self.path_spec_layers,
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
            path_spec_layers: HashMap::new(),
            _marker: PhantomData,
        }
    }

    pub fn merge(self, other: GroomRouter<S, NotValidated>) -> MergeResult<Self> {
        let router = self.router.merge(other.router);

        let mut openapi_paths = self.openapi_paths;
        openapi_paths.extend(other.openapi_paths);

        let mut path_spec_layers = self.path_spec_layers;
        for (path, other_layers) in other.path_spec_layers {
            path_spec_layers
                .entry(path)
                .or_default()
                .extend(other_layers);
        }

        let registry = self.registry
            .merge(other.registry)
            .map_err(|(name, _existing, _incoming)| MergeError::SchemaConflict {
                name,
                source_a: "self".into(),
                source_b: "other".into(),
            })?;

        Ok(Self { router, registry, openapi_paths, path_spec_layers, _marker: PhantomData })
    }

    pub fn nest(self, path: &str, other: GroomRouter<S, NotValidated>) -> MergeResult<Self> {
        let router = self.router.nest(path, other.router);
        let mut openapi_paths = self.openapi_paths;
        let mut path_spec_layers = self.path_spec_layers;

        for (p, item) in other.openapi_paths {
            let prefixed_path = super::prepend_path(path, &p);

            let spec_layers: Vec<SpecLayerBinding> = other
                .path_spec_layers
                .get(&p)
                .map(|layers| layers.iter().map(SpecLayerBinding::clone_binding).collect())
                .unwrap_or_default();

            openapi_paths.push((prefixed_path.clone(), item));
            path_spec_layers
                .entry(prefixed_path)
                .or_default()
                .extend(spec_layers);
        }

        let registry = self.registry
            .merge(other.registry)
            .map_err(|(name, _existing, _incoming)| MergeError::SchemaConflict {
                name,
                source_a: "self".into(),  // todo: replace "self" with reference to GroomRouter or remove it
                source_b: "other".into(), // todo: replace "self" with reference to GroomRouter or remove it
            })?;
        Ok(Self { router, registry, openapi_paths, path_spec_layers, _marker: PhantomData })
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
            path_spec_layers: self.path_spec_layers,
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
            path_spec_layers: self.path_spec_layers,
            _marker: PhantomData,
        }
    }

    /// Apply an [`OpenApiSpecLayer`] to the request pipeline and store it
    /// for OpenAPI spec contribution during [`to_openapi`](Self::to_openapi).
    ///
    /// The layer is applied to requests exactly like [`layer`](Self::layer).
    /// It is also stored and invoked during [`to_openapi`](Self::to_openapi),
    /// receiving `&mut OpenApi` to add security schemes, response codes, or other metadata.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let router = GroomRouter::new()
    ///     .layer_with_spec(AuthLayer);
    /// ```
    pub fn layer_with_spec<SL>(self, spec_layer: SL) -> Self
    where
        SL: OpenApiSpecLayer + Clone
    {
        let boxed = spec_layer.clone_box();

        // Methods present on each path at attach time (union across duplicate path keys).
        let mut methods_by_path: HashMap<&str, MethodFlags> = HashMap::new();
        for (path, item) in &self.openapi_paths {
            let entry = methods_by_path.entry(path.as_str()).or_default();
            *entry = entry.union(MethodFlags::from_path_item(item));
        }

        let mut path_spec_layers = self.path_spec_layers;
        for (path, layers) in path_spec_layers.iter_mut() {
            let methods = methods_by_path
                .get(path.as_str())
                .copied()
                .unwrap_or_else(MethodFlags::empty);
            layers.push(SpecLayerBinding {
                methods,
                layer: boxed.clone_box(),
            });
        }

        Self {
            router: spec_layer.mount(self.router),
            registry: self.registry,
            openapi_paths: self.openapi_paths,
            path_spec_layers,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extract::ComponentsRegistry;

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
}
