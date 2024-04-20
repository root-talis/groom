use utoipa::ToSchema;

pub mod extract;
pub mod response;

pub trait DTO: ToSchema<'static> {}
