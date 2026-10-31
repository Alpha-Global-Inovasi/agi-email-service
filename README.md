# cf-email

Email library for Rust services, with SMTP and mock providers and Handlebars templates.

## Installation

```toml
[dependencies]
cf-email = { git = "https://github.com/Crypto-Foundation-Dev/cf-email.git", tag = "v0.1.0" }
```

## Environment Variables

| Variable             | Required           | Description                                       |
| -------------------- | ------------------ | ------------------------------------------------- |
| `EMAIL_DRIVER`       | No                 | `mock` for the mock provider, otherwise SMTP      |
| `EMAIL_FROM_ADDRESS` | Yes                | Sender address                                    |
| `EMAIL_FROM_NAME`    | Yes                | Sender name                                       |
| `SMTP_HOST`          | When using SMTP    | SMTP host                                         |
| `SMTP_PORT`          | When using SMTP    | SMTP port                                         |
| `SMTP_USERNAME`      | No                 | SMTP username                                     |
| `SMTP_PASSWORD`      | No                 | SMTP password                                     |
| `SMTP_USE_TLS`       | No                 | `true` (default) for STARTTLS, otherwise plaintext |

## Usage

```rust,ignore
use cf_email::{EmailClient, EmailConfig, EmailRequest};
use serde_json::json;

let client = EmailClient::new(EmailConfig::from_env()?).await?;
client.register_template("welcome", "<p>Hi {{name}}</p>").await?;

let request = EmailRequest::builder()
    .to("user@example.com")
    .subject("Welcome")
    .template("welcome", json!({ "name": "Kanda" }))
    .build()?;

client.send(request).await?;
```

## License

MIT
