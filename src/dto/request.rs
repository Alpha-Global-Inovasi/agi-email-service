use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EmailContent {
    Plain(String),
    Html(String),
    Template {
        name: String,
        data: serde_json::Value,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailAttachment {
    pub filename: String,
    pub content: Vec<u8>,
    pub content_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailRequest {
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub subject: String,
    pub content: EmailContent,
    pub attachments: Vec<EmailAttachment>,
}

impl EmailRequest {
    pub fn new(to: Vec<String>, subject: String, content: EmailContent) -> Self {
        Self {
            to,
            cc: vec![],
            bcc: vec![],
            subject,
            content,
            attachments: vec![],
        }
    }

    pub fn builder() -> EmailRequestBuilder {
        EmailRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct EmailRequestBuilder {
    to: Vec<String>,
    cc: Vec<String>,
    bcc: Vec<String>,
    subject: String,
    content: Option<EmailContent>,
    attachments: Vec<EmailAttachment>,
}

impl EmailRequestBuilder {
    pub fn to(mut self, to: impl Into<String>) -> Self {
        self.to.push(to.into());
        self
    }

    pub fn cc(mut self, cc: impl Into<String>) -> Self {
        self.cc.push(cc.into());
        self
    }

    pub fn bcc(mut self, bcc: impl Into<String>) -> Self {
        self.bcc.push(bcc.into());
        self
    }

    pub fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = subject.into();
        self
    }

    pub fn content(mut self, content: EmailContent) -> Self {
        self.content = Some(content);
        self
    }

    pub fn plain(mut self, text: impl Into<String>) -> Self {
        self.content = Some(EmailContent::Plain(text.into()));
        self
    }

    pub fn html(mut self, html: impl Into<String>) -> Self {
        self.content = Some(EmailContent::Html(html.into()));
        self
    }

    pub fn template(mut self, name: impl Into<String>, data: serde_json::Value) -> Self {
        self.content = Some(EmailContent::Template {
            name: name.into(),
            data,
        });
        self
    }

    pub fn attachment(mut self, attachment: EmailAttachment) -> Self {
        self.attachments.push(attachment);
        self
    }

    pub fn build(self) -> Result<EmailRequest, String> {
        Ok(EmailRequest {
            to: self.to,
            cc: self.cc,
            bcc: self.bcc,
            subject: self.subject,
            content: self.content.ok_or("Content is required")?,
            attachments: self.attachments,
        })
    }
}
