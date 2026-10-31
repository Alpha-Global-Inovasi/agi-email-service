use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EmailResponse<T> {
    pub status: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T> EmailResponse<T> {
    pub fn success(message: impl Into<String>, data: T) -> Self {
        Self {
            status: true,
            message: message.into(),
            data: Some(data),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            status: false,
            message: message.into(),
            data: None,
        }
    }
}
