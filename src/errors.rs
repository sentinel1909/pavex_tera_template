// src/error.rs

// dependencies
use std::fmt;

#[derive(Debug)]
pub enum TemplateError {
    Tera(tera::Error),
}

impl From<tera::Error> for TemplateError {
    fn from(err: tera::Error) -> Self {
        TemplateError::Tera(err)
    }
}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateError::Tera(err) => write!(f, "Tera template error: {}", err),
        }
    }
}

impl std::error::Error for TemplateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TemplateError::Tera(err) => Some(err),
        }
    }
}
