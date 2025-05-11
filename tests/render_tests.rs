// tests/render_tests.rs

// dependencies
use pavex_tera_template::{TemplateConfig, TemplateEngine};
use std::borrow::Cow;
use std::path::PathBuf;
use tera::Context;

#[test]
fn test_rendering_index_template_with_includes_and_extends() {
    // Arrange
    let template_dir: PathBuf = std::env::current_dir().unwrap().join("tests/templates");

    assert!(template_dir.exists(), "Template directory does not exist");

    // Act
    let config = TemplateConfig {
        dir: Cow::Owned(template_dir.to_string_lossy().into_owned()),
        pattern: "*.html".into(),
        extra_templates: vec![],
    };

    let engine = TemplateEngine::from_config(&config).expect("Failed to compile templates");

    let mut context = Context::new();
    context.insert("name", "Rustacean");

    let rendered = engine.render("index.html", &context).unwrap();

    // Assert
    assert!(
        rendered.contains("Hello, Rustacean!"),
        "Missing greeting in rendered output"
    );

    assert!(
        rendered.contains("This is a partial."),
        "Partial template did not render"
    );

    assert!(
        rendered.contains("<title>Homepage</title>"),
        "Title block did not override correctly"
    );
}
