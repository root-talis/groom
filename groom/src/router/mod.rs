mod error;
pub use error::MergeError;
pub use error::RouterValidationError;

pub(crate) mod core;
pub(crate) mod traits;
mod validate;
mod openapi;

pub use core::GroomRouter;
pub use traits::{OpenApiSpecLayer, SpecLayerModifier};

pub(crate) type MergeResult<T> = Result<T, MergeError>;

#[derive(Clone)]
pub struct NotValidated;
#[derive(Clone)]
pub struct Validated;

pub type GroomRouterValid<S = ()> = GroomRouter<S, Validated>;

/// Prepends `prefix` to `path`, producing the full mount path for a nested route's OpenAPI entry.
///
/// Mirrors axum's internal `path_for_nested_route` logic.
/// Both `prefix` and `path` must start with `/`.
///
/// # Panics
/// Panics if either argument does not start with `/`.
fn prepend_path(prefix: &str, path: &str) -> String {
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

impl GroomRouter<(), NotValidated> {
    pub fn with_state<S2: Clone + Send + Sync + 'static>(self, state: ()) -> GroomRouter<S2, NotValidated> {
        GroomRouter {
            router: self.router.with_state(state),
            registry: self.registry,
            openapi_paths: self.openapi_paths,
            path_spec_layers: self.path_spec_layers,
            whole_spec_layers: self.whole_spec_layers,
            _marker: std::marker::PhantomData,
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
            path_spec_layers: self.path_spec_layers.iter().map(
                |(k, layers)| (k.clone(), layers.iter().map(core::SpecLayerBinding::clone_binding).collect())
            ).collect(),
            whole_spec_layers: self
                .whole_spec_layers
                .iter()
                .map(|layer| layer.clone_box())
                .collect(),
            _marker: std::marker::PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_prepend_path_trailing_slash_prefix() {
        assert_eq!(prepend_path("/api/v1/", "/foo"), "/api/v1/foo");
        assert_eq!(prepend_path("/api/", "/v1/foo"), "/api/v1/foo");
        assert_eq!(prepend_path("/", "/foo"), "/foo");
    }

    #[test]
    fn test_prepend_path_root_path() {
        assert_eq!(prepend_path("/api/v1", "/"), "/api/v1");
        assert_eq!(prepend_path("/api", "/"), "/api");
    }

    #[test]
    fn test_prepend_path_normal_join() {
        assert_eq!(prepend_path("/api/v1", "/foo"), "/api/v1/foo");
        assert_eq!(prepend_path("/api", "/v1/foo"), "/api/v1/foo");
        assert_eq!(prepend_path("/root", "/path"), "/root/path");
    }

    #[test]
    fn test_prepend_path_path_params() {
        assert_eq!(prepend_path("/api/v1/{version}", "/foo/{id}"), "/api/v1/{version}/foo/{id}");
        assert_eq!(prepend_path("/root", "/{param}"), "/root/{param}");
    }

    #[test]
    #[should_panic(expected = "prefix must start with '/'")]
    fn test_prepend_path_panics_on_bad_prefix() {
        prepend_path("api/v1", "/foo");
    }

    #[test]
    #[should_panic(expected = "path must start with '/'")]
    fn test_prepend_path_panics_on_bad_path() {
        prepend_path("/api/v1", "foo");
    }
}
