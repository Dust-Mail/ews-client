use std::{fmt::Display, io::Error as IoError, result};

use ldap3::LdapError;
use reqwest::Error as ReqwestError;
use serde_xml_rs::Error as ParseXmlError;
use trust_dns_resolver::error::ResolveError;

#[derive(Debug)]
pub enum ErrorKind {
    InvalidConfig,
    InvalidRequestUrl,
    InvalidEmailAddress,
    HttpRequest,
    NotFound,
    Resolve(ResolveError),
    ParseXml(ParseXmlError),
    Reqwest(ReqwestError),
    Io(IoError),
    Ldap(LdapError),
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl From<LdapError> for Error {
    fn from(error: LdapError) -> Self {
        Error::new(
            ErrorKind::Ldap(error),
            "An error with occurred in the LDAP communication",
        )
    }
}

impl From<ReqwestError> for Error {
    fn from(error: ReqwestError) -> Self {
        Error::new(
            ErrorKind::Reqwest(error),
            "An error while creating a http request",
        )
    }
}

impl From<ParseXmlError> for Error {
    fn from(error: ParseXmlError) -> Self {
        Error::new(
            ErrorKind::ParseXml(error),
            "An error while parsing the XML response",
        )
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Self {
        Error::new(ErrorKind::Io(error), "An error occurred in an io process")
    }
}

impl From<ResolveError> for Error {
    fn from(error: ResolveError) -> Self {
        Error::new(
            ErrorKind::Resolve(error),
            "An error occurred while resolving a dns query",
        )
    }
}

impl Error {
    pub fn new<M: Into<String>>(kind: ErrorKind, message: M) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
}

#[macro_export]
macro_rules! failed {
    ($kind:expr, $($arg:tt)*) => {{
		use crate::error::Error;

        let kind = $kind;
        let message = format!($($arg)*);
        return Err(Error::new( kind, message ));
    }};
}

pub type Result<T> = result::Result<T, Error>;
