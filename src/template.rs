// src/template.rs

// dependencies
use crate::template_engine::compile_templates;
use serde::Serialize;
use std::borrow::Cow;
use tera::{Context, Error, Tera};

// struct type to represent the template storage configuration
#[derive(Clone, Debug)]
pub struct TemplateConfig {
    pub dir: Cow<'static, str>,
    pub pattern: String,
    pub extra_templates: Vec<TemplateFile>,
}

// Represents a single template file with an optional name override
#[derive(Clone, Debug)]
pub struct TemplateFile {
    pub path: String,
    pub name: Option<String>,
}

// struct type to represent a Tera Template Engine
#[derive(Clone, Debug)}
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

// helper method for constructing context from any type that implements serde::Serialze
pub fn context_from_struct<T: Serialize>(data: &T) -> Context {
    Context::from_serialize(data).unwrap_or_default()
}
