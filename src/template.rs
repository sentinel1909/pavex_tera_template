// src/template.rs

// dependencies
use crate::template_engine::compile_templates;
use std::borrow::Cow;
use tera::{Context, Error, Tera};

// features
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

// struct type to represent a Tera Template Engine
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct TemplateEngine {
    pub tera: Tera,
}

// methods for the Tera Template Engine
impl TemplateEngine {
    pub fn build(config: &TemplateConfig) -> Result<Self, Error> {
        let tera = compile_templates(config)?;
        Ok(Self { tera })
    }

    pub fn render(&self, template_name: &str, context: &Context) -> Result<String, Error> {
        self.tera.render(template_name, context)
    }
}
