use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum MergeError {
    #[error("schema `{name}` conflicts: defined with different types")]
    SchemaConflict {
        name: String,
    },

    #[error("schema reference `{path}` not found in registry `{registry}`")]
    SchemaNotFound {
        path: String,
        registry: String,
    },
}

#[derive(Debug, Clone, Error)]
pub enum RouterValidationError {
    #[error("route shadowing detected: `{method} {path}` is registered by more then one controller")]
    RouteShadow {
        path: String,
        method: ::http::Method,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_shadow_can_be_constructed() {
        let err = RouterValidationError::RouteShadow {
            path: "/foo".into(),
            method: ::http::Method::GET,
        };
        let _ = err;
    }

    #[test]
    fn test_route_shadow_display() {
        let err = RouterValidationError::RouteShadow {
            path: "/foo".into(),
            method: ::http::Method::GET,
        };
        let msg = err.to_string();
        assert!(msg.contains("/foo"), "Display should include path: got {}", msg);
        assert!(msg.contains("GET"), "Display should include method: got {}", msg);
    }

    #[test]
    fn test_route_shadow_debug() {
        let err = RouterValidationError::RouteShadow {
            path: "/foo".into(),
            method: ::http::Method::POST,
        };
        let _ = format!("{:?}", err);
    }
}
