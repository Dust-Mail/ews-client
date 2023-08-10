pub mod candidate;
pub mod pox;
// mod soap;

// pub use soap::SoapConfig;
pub use pox::Autodiscover as PoxAutodiscover;
use pox::Response as PoxResponse;

pub enum ConfigResult {
    Ok(AutodiscoverResponse),
    Redirect(RedirectType),
    Error(Error),
}

impl ConfigResult {
    pub fn error<M: Into<String>>(message: M) -> Self {
        ConfigResult::Error(Error {
            message: message.into(),
        })
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
    pub fn message(&self) -> &str {
        self.message.as_ref()
    }
}

#[derive(Debug)]
pub enum AutodiscoverResponse {
    Pox(PoxResponse),
}
