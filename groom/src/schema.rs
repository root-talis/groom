use utoipa::openapi::{RefOr, Schema};

pub trait GroomSchema {
    fn extract_schema(self) -> RefOr<Schema>;
}

impl GroomSchema for RefOr<Schema> {
    fn extract_schema(self) -> RefOr<Schema> {
        self
    }
}
impl GroomSchema for (&str, RefOr<Schema>) {
    fn extract_schema(self) -> RefOr<Schema> {
        self.1
    }
}
