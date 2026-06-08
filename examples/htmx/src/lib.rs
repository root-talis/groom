pub mod controller;
pub mod state;
pub mod template;

pub use controller::make_router;

#[cfg(test)]
mod tests;
