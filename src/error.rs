use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmailError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("SMTP error: {0}")]
    SmtpError(String),

    #[error("Template error: {0}")]
    TemplateError(String),

    #[error("Build error: {0}")]
    BuildError(String),

    #[error("Provider error: {0}")]
    ProviderError(String),
}
