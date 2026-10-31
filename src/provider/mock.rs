use super::EmailProvider;
use crate::dto::request::EmailRequest;
use crate::error::EmailError;
use async_trait::async_trait;
use tracing::info;

pub struct MockProvider;

#[async_trait]
impl EmailProvider for MockProvider {
    async fn send(&self, req: EmailRequest) -> Result<(), EmailError> {
        info!(
            "Mock Provider: Sending email to {:?} with subject '{}'",
            req.to, req.subject
        );
        Ok(())
    }
}
