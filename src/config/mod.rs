mod pox;
// mod soap;

use derive_getters::Getters;
pub use pox::Autodiscover as PoxAutodiscover;
// pub use soap::SoapConfig;

pub enum ConfigResult {
    Ok(Config),
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

#[derive(Debug, Getters)]
pub struct Config {}

#[derive(Debug, Getters)]
pub struct User {
    display_name: String,
}
