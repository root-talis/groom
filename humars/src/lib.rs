use utoipa::ToSchema;

pub mod extract;
pub mod response;

/// Indicates that type is annotated with `#[DTO(...)]`.
/// 
/// Do not implement this manually.
pub trait DTO: ToSchema<'static> {}

pub use response::DTO_Response;
