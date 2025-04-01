// src/config.rs

// dependencies
use std::borrow::Cow;

// optional serde dependency
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// struct type to represent the template storage configuration
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TemplateConfig {
    pub dir: Cow<'static, str>,
    pub pattern: String,
    pub extra_templates: Vec<TemplateFile>,
}

// Represents a single template file with an optional name override
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TemplateFile {
    pub path: String,
    pub name: Option<String>,
}
