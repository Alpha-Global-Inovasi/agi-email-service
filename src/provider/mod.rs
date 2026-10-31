pub mod mock;
pub mod smtp;

use crate::dto::request::EmailRequest;
use crate::error::EmailError;
use async_trait::async_trait;

#[async_trait]
pub trait EmailProvider: Send + Sync {
    async fn send(&self, req: EmailRequest) -> Result<(), EmailError>;
}
