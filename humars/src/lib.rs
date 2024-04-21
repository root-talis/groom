use utoipa::ToSchema;

pub mod extract;
pub mod response;

/// Indicates that type is annotated with `#[DTO(...)]`.
/// 
/// Do not implement this manually.
pub trait DTO: ToSchema<'static> {}

/// Indicates that type is annotated with `#[DTO(response(...))]`.
/// 
/// Do not implement this manually.
#[allow(non_camel_case_types)]
pub trait DTO_Response {}
