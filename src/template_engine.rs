// serc/template_engine.rs

// dependencies
use crate::template::TemplateConfig;
use tera::{Error, Tera};

// function which accepts a reference to a configuration type, builds an returns a template instance
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
        let engine = crate::TemplateEngine::build(&config).unwrap();

        let mut context = Context::new();
        context.insert("name", "Rustacean");

        // Assert
        let result = engine.render("hello.html", &context).unwrap();
        assert_eq!(result, "Hello, Rustacean!");
    }
}
