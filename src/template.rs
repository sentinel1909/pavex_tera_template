// src/template.rs

// dependencies
use serde::Serialize;
use std::borrow::Cow;
use tera::{Context, Error, Tera};

// struct type to represent a Tera Template Engine
pub struct TemplateEngine {
    tera: Tera,
}

// struct type to represent the emplate storage configuration
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

// struct type to represent a Tera template
#[derive(Clone, Debug)]
pub struct TeraTemplate {
    pub config: TemplateConfig,
    pub context: Context,
}

// methods for the Tera Template Engine
impl TemplateEngine {
    pub fn compile_templates(config: &TemplateConfig) -> Result<Tera, Error> {
        let pattern = format!("{}/{}", config.dir, config.pattern);
        let mut tera = Tera::new(&pattern)?;

        // Add any extra templates with custom names
        let template_files: Vec<(String, Option<&str>)> = config
            .extra_templates
            .iter()
            .map(|f| {
                let path = format!("{}/{}", config.dir, f.path);
                (path, f.name.as_deref())
            })
            .collect();

        if !template_files.is_empty() {
            tera.add_template_files(template_files)?;
        }

        Ok(tera)
    }

    pub fn render(&self, template_name: &str, context: &Context) -> Result<String, Error> {
        self.tera.render(template_name, context)
    }
}

// helper method for constructing context from any type that implements serde::Serialze
pub fn context_from_struct<T: Serialize>(data: &T) -> Context {
    Context::from_serialize(data).unwrap_or_default()
}
