use utoipa::ToSchema;

pub mod extract;

pub trait DTO: ToSchema<'static> {}
