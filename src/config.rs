use crate::error::EmailError;
use std::env;

#[derive(Debug, Clone)]
pub struct EmailConfig {
    pub provider: EmailProviderType,
    pub from_email: String,
    pub from_name: String,
    pub smtp_config: Option<SmtpConfig>,
}

#[derive(Debug, Clone)]
pub enum EmailProviderType {
    Smtp,
    Mock,
}

#[derive(Debug, Clone)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub use_tls: bool,
}

impl EmailConfig {
    pub fn from_env() -> Result<Self, EmailError> {
        let provider_str = env::var("EMAIL_DRIVER").unwrap_or_else(|_| "smtp".to_string());
        let provider = match provider_str.as_str() {
            "mock" => EmailProviderType::Mock,
            _ => EmailProviderType::Smtp,
        };

        let from_email = env::var("EMAIL_FROM_ADDRESS")
            .map_err(|_| EmailError::ConfigError("EMAIL_FROM_ADDRESS not set".to_string()))?;

        let from_name = env::var("EMAIL_FROM_NAME")
            .map_err(|_| EmailError::ConfigError("EMAIL_FROM_NAME not set".to_string()))?;

        let smtp_config = if let EmailProviderType::Smtp = provider {
            Some(SmtpConfig {
                host: env::var("SMTP_HOST")
                    .map_err(|_| EmailError::ConfigError("SMTP_HOST not set".to_string()))?,
                port: env::var("SMTP_PORT")
                    .map_err(|_| EmailError::ConfigError("SMTP_PORT not set".to_string()))?
                    .parse()
                    .map_err(|_| EmailError::ConfigError("SMTP_PORT invalid".to_string()))?,
                username: env::var("SMTP_USERNAME").unwrap_or_default(),
                password: env::var("SMTP_PASSWORD").unwrap_or_default(),
                use_tls: env::var("SMTP_USE_TLS")
                    .map(|v| v.to_lowercase() == "true")
                    .unwrap_or(true), // Default to true for safety
            })
        } else {
            None
        };

        Ok(Self {
            provider,
            from_email,
            from_name,
            smtp_config,
        })
    }
}
