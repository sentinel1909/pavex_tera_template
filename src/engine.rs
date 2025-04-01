// src/engine.rs

// dependencies
use crate::config::TemplateConfig;
use crate::errors::TemplateError;
use tera::{Context, Tera};

// struct type to represent a Tera Template Engine
#[derive(Clone, Debug)]
pub struct TemplateEngine {
    pub tera: Tera,
}

// methods for the Tera Template Engine
impl TemplateEngine {
    pub fn from_config(config: &TemplateConfig) -> Result<Self, TemplateError> {
        let tera = compile_templates(config)?;
        Ok(Self { tera })
    }

    pub fn render(&self, template_name: &str, context: &Context) -> Result<String, TemplateError> {
        Ok(self.tera.render(template_name, context)?)
    }
}

// function which accepts a reference to a configuration type, builds an returns a template instance
pub fn compile_templates(config: &TemplateConfig) -> Result<Tera, TemplateError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use tera::Context;

    #[test]
    fn test_compile_and_render_minimal_template() {
        // Arrange - Part 1
        let tmp_dir = tempfile::tempdir().unwrap();
        let file_path = tmp_dir.path().join("hello.html");
        std::fs::write(&file_path, "Hello, {{ name }}!").unwrap();

        // Arrange - Part 2
        let config = TemplateConfig {
            dir: Cow::Owned(tmp_dir.path().to_string_lossy().into()),
            pattern: "*.html".to_string(),
            extra_templates: vec![],
        };

        // Act
        let engine = crate::TemplateEngine::from_config(&config).unwrap();

        let mut context = Context::new();
        context.insert("name", "Rustacean");

        // Assert
        let result = engine.render("hello.html", &context).unwrap();
        assert_eq!(result, "Hello, Rustacean!");
    }
}
