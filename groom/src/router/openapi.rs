use std::collections::HashSet;

use utoipa::openapi::OpenApi;

use super::core::GroomRouter;
use super::traits::SpecLayerModifier;
use super::Validated;

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

        // Per-operation modification: apply each path's spec layers to its own operations
        for (path_str, path_item) in &mut api.paths.paths {
            let methods: [(
                utoipa::openapi::path::HttpMethod,
                &mut Option<utoipa::openapi::path::Operation>,
            ); 8] = [
                (utoipa::openapi::path::HttpMethod::Get,     &mut path_item.get),
                (utoipa::openapi::path::HttpMethod::Put,     &mut path_item.put),
                (utoipa::openapi::path::HttpMethod::Post,    &mut path_item.post),
                (utoipa::openapi::path::HttpMethod::Delete,  &mut path_item.delete),
                (utoipa::openapi::path::HttpMethod::Options, &mut path_item.options),
                (utoipa::openapi::path::HttpMethod::Head,    &mut path_item.head),
                (utoipa::openapi::path::HttpMethod::Patch,   &mut path_item.patch),
                (utoipa::openapi::path::HttpMethod::Trace,   &mut path_item.trace),
            ];

            if let Some(layers) = self.path_spec_layers.get(path_str.as_str()) {
                for (method, operation_opt) in methods {
                    if let Some(operation) = operation_opt {
                        for binding in layers {
                            if !binding.methods.contains(&method) {
                                continue;
                            }
                            binding.layer.modify_operation(
                                path_str.as_str(),
                                &method,
                                operation,
                            );
                        }
                    }
                }
            }
        }

        // Whole-spec modification: invoke all unique spec layers across all paths
        // (pointer dedup remains until P003 introduces whole_spec_layers).
        let mut seen_ptrs: HashSet<*const dyn SpecLayerModifier> = HashSet::new();
        for layers in self.path_spec_layers.values() {
            for binding in layers {
                let ptr = binding.layer.as_ref() as *const dyn SpecLayerModifier;
                if !seen_ptrs.contains(&ptr) {
                    seen_ptrs.insert(ptr);
                    binding.layer.modify_openapi(&mut api);
                }
            }
        }

        api
    }

    pub fn to_axum_router(self) -> axum::Router<S> {
        self.router
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extract::ComponentsRegistry;

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
}
