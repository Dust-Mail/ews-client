use std::{fmt::Display, result};

use ldap3::LdapError;
use reqwest::Error as ReqwestError;
use serde_xml_rs::Error as ParseXmlError;

#[derive(Debug)]
pub enum ErrorKind {
    InvalidEmailAddress,
    InvalidHttpResponse,
    NotFound,
    ParseXml(ParseXmlError),
    Reqwest(ReqwestError),
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
            "An error while parse the XML response",
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

pub type Result<T> = result::Result<T, Error>;
