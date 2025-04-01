// src/lib/lib.rs

// module declarations
pub mod config;
pub mod engine;
pub mod errors;

// re-exports
pub use config::{TemplateConfig, TemplateFile};
pub use engine::TemplateEngine;
pub use errors::TemplateError;
