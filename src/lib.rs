// src/lib/lib.rs

// module declarations
pub mod template;
pub mod template_engine;

// re-exports
pub use template::*;
pub use template_engine::*;
pub use tera::{Context, Error};