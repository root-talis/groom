use crate::router::error::RouterValidationError;

use super::core::GroomRouter;
use super::NotValidated;
use super::Validated;

impl<S: Clone + Send + Sync + 'static> GroomRouter<S, NotValidated> {
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
            path_spec_layers: self.path_spec_layers,
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
}
