use super::EmailProvider;
use crate::config::SmtpConfig;
use crate::dto::request::{EmailContent, EmailRequest};
use crate::error::EmailError;
use async_trait::async_trait;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::{Attachment, MultiPart, SinglePart, header::ContentType},
    transport::smtp::authentication::{Credentials, Mechanism},
};

pub struct SmtpProvider {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from_email: String,
    from_name: String,
}

impl SmtpProvider {
    pub fn new(
        config: SmtpConfig,
        from_email: String,
        from_name: String,
    ) -> Result<Self, EmailError> {
        let creds = Credentials::new(config.username, config.password);

        let transport_builder = if config.use_tls && config.port == 465 {
            // Implicit TLS (SMTPS): TLS from the first byte, no STARTTLS (port 465)
            AsyncSmtpTransport::<Tokio1Executor>::relay(&config.host)
                .map_err(|e| EmailError::ConfigError(e.to_string()))?
                .port(config.port)
                .credentials(creds)
                .authentication(vec![Mechanism::Login, Mechanism::Plain])
        } else if config.use_tls {
            // STARTTLS: connect plain, then upgrade to TLS (port 587)
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.host)
                .map_err(|e| EmailError::ConfigError(e.to_string()))?
                .port(config.port)
                .credentials(creds)
                .authentication(vec![Mechanism::Login, Mechanism::Plain])
        } else {
            // No TLS: plain text connection
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.host)
                .port(config.port)
                .credentials(creds)
                .authentication(vec![Mechanism::Login, Mechanism::Plain])
        };

        let transport = transport_builder.build();

        Ok(Self {
            transport,
            from_email,
            from_name,
        })
    }
}

fn build_message(from: &str, req: EmailRequest) -> Result<Message, EmailError> {
    let mut builder = Message::builder()
        .from(
            from.parse()
                .map_err(|e| EmailError::BuildError(format!("Invalid from address: {}", e)))?,
        )
        .subject(req.subject);

    for to in req.to {
        builder = builder.to(to
            .parse()
            .map_err(|e| EmailError::BuildError(format!("Invalid to address: {}", e)))?);
    }

    for cc in req.cc {
        builder = builder.cc(cc
            .parse()
            .map_err(|e| EmailError::BuildError(format!("Invalid cc address: {}", e)))?);
    }

    for bcc in req.bcc {
        builder = builder.bcc(
            bcc.parse()
                .map_err(|e| EmailError::BuildError(format!("Invalid bcc address: {}", e)))?,
        );
    }

    let html_body = |html: String| {
        MultiPart::alternative()
            .singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_PLAIN)
                    .body("Please view this email in a modern client".to_string()),
            )
            .singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(html),
            )
    };

    let result = if req.attachments.is_empty() {
        match req.content {
            EmailContent::Plain(text) => builder.body(text),
            EmailContent::Html(html) => builder.multipart(html_body(html)),
            // Template should be rendered BEFORE calling provider.send
            EmailContent::Template { .. } => {
                return Err(EmailError::ProviderError(
                    "Template content should be rendered before sending".to_string(),
                ));
            }
        }
    } else {
        let mut mixed = match req.content {
            EmailContent::Plain(text) => MultiPart::mixed().singlepart(SinglePart::plain(text)),
            EmailContent::Html(html) => MultiPart::mixed().multipart(html_body(html)),
            // Template should be rendered BEFORE calling provider.send
            EmailContent::Template { .. } => {
                return Err(EmailError::ProviderError(
                    "Template content should be rendered before sending".to_string(),
                ));
            }
        };

        for attachment in req.attachments {
            let content_type = ContentType::parse(&attachment.content_type)
                .map_err(|e| EmailError::BuildError(format!("Invalid content type: {}", e)))?;
            mixed = mixed.singlepart(
                Attachment::new(attachment.filename).body(attachment.content, content_type),
            );
        }

        builder.multipart(mixed)
    };

    result.map_err(|e| EmailError::BuildError(e.to_string()))
}

#[async_trait]
impl EmailProvider for SmtpProvider {
    async fn send(&self, req: EmailRequest) -> Result<(), EmailError> {
        let from = format!("{} <{}>", self.from_name, self.from_email);
        let email = build_message(&from, req)?;

        self.transport
            .send(email)
            .await
            .map_err(|e| EmailError::SmtpError(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dto::request::EmailAttachment;

    fn request() -> crate::dto::request::EmailRequestBuilder {
        EmailRequest::builder()
            .to("to@example.com")
            .subject("Hello")
            .html("<p>Hello</p>")
    }

    fn build(req: EmailRequest) -> Message {
        build_message("Sender <from@example.com>", req).unwrap()
    }

    #[test]
    fn sends_to_cc_and_hides_bcc() {
        let message = build(
            request()
                .cc("cc@example.com")
                .bcc("bcc@example.com")
                .build()
                .unwrap(),
        );

        let recipients: Vec<String> = message
            .envelope()
            .to()
            .iter()
            .map(|a| a.to_string())
            .collect();
        assert!(recipients.contains(&"to@example.com".to_string()));
        assert!(recipients.contains(&"cc@example.com".to_string()));
        assert!(recipients.contains(&"bcc@example.com".to_string()));

        let raw = String::from_utf8(message.formatted()).unwrap();
        assert!(raw.contains("cc@example.com"));
        assert!(!raw.contains("bcc@example.com"));
    }

    #[test]
    fn includes_attachment_with_html_body() {
        let message = build(
            request()
                .attachment(EmailAttachment {
                    filename: "report.txt".to_string(),
                    content: b"attachment-content".to_vec(),
                    content_type: "text/plain".to_string(),
                })
                .build()
                .unwrap(),
        );

        let raw = String::from_utf8(message.formatted()).unwrap();
        assert!(raw.contains("multipart/mixed"));
        assert!(raw.contains("multipart/alternative"));
        assert!(raw.contains("report.txt"));
        assert!(raw.contains("attachment-content"));
        assert!(raw.contains("<p>Hello</p>"));
    }

    #[test]
    fn rejects_invalid_attachment_content_type() {
        let req = request()
            .attachment(EmailAttachment {
                filename: "report.txt".to_string(),
                content: vec![1],
                content_type: "not a content type".to_string(),
            })
            .build()
            .unwrap();

        assert!(matches!(
            build_message("Sender <from@example.com>", req),
            Err(EmailError::BuildError(_))
        ));
    }

    #[test]
    fn rejects_unrendered_template() {
        let req = EmailRequest::builder()
            .to("to@example.com")
            .subject("Hello")
            .template("hello", serde_json::json!({}))
            .build()
            .unwrap();

        assert!(matches!(
            build_message("Sender <from@example.com>", req),
            Err(EmailError::ProviderError(_))
        ));
    }
}
