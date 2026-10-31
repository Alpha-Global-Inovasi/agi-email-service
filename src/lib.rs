pub mod client;
pub mod config;
pub mod dto;
pub mod error;
pub mod provider;
pub mod template;

pub use client::EmailClient;
pub use config::EmailConfig;
pub use dto::request::{EmailContent, EmailRequest};
pub use dto::response::EmailResponse;
pub use error::EmailError;
