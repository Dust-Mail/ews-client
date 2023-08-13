#[cfg(feature = "pox")]
use super::pox::{self, Response as PoxResponse};

pub enum AutodiscoverResult {
    Ok(AutodiscoverResponse),
    Redirect(RedirectType),
    Error(Error),
}

impl AutodiscoverResult {
    pub fn error<M: Into<String>>(message: M) -> Self {
        AutodiscoverResult::Error(Error::new(message))
    }
}

pub enum RedirectType {
    Url(String),
    Email(String),
}

pub struct Error {
    message: String,
}

#[cfg(feature = "pox")]
impl From<&pox::Error> for Error {
    fn from(error: &pox::Error) -> Self {
        Self::new(format!("Code {}, {}", error.code(), error.message()))
    }
}

impl Error {
    pub fn new<M: Into<String>>(message: M) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        self.message.as_ref()
    }
}

#[derive(Debug)]
pub enum AutodiscoverResponse {
    #[cfg(feature = "pox")]
    Pox(PoxResponse),
}
