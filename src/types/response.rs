pub use pox::Autodiscover as PoxAutodiscover;
use pox::Response as PoxResponse;

use super::pox;

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
    Pox(PoxResponse),
}
