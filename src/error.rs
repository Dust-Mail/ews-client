use std::{error, fmt::Display, io::Error as IoError, result};

use serde_xml_rs::Error as ParseXmlError;
use surf::Error as SurfError;
use trust_dns_resolver::error::ResolveError;

macro_rules! impl_from_error {
    ($error_type:ty, $error_kind:expr, $error_msg:expr) => {
        impl From<$error_type> for Error {
            fn from(err: $error_type) -> Self {
                Error::new($error_kind(err), $error_msg)
            }
        }
    };
}

macro_rules! err {
    ($kind:expr, $($arg:tt)*) => {{
		use crate::error::Error;

        let kind = $kind;
        let message = format!($($arg)*);
        return Err(Error::new( kind, message ));
    }};
}

#[derive(Debug)]
pub enum ErrorKind {
    InvalidConfig,
    InvalidRequestUrl,
    InvalidEmailAddress,
    NotFound,
    InvalidProtocol,
    HttpRequest,
    BuildHttpClient,
    ConfigNotFound(Vec<Error>),
    Resolve(ResolveError),
    ParseXml(ParseXmlError),
    Surf(SurfError),
    Io(IoError),
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl_from_error!(
    SurfError,
    |err| ErrorKind::Surf(err),
    "Failed to create http request"
);
impl_from_error!(
    ParseXmlError,
    |err| ErrorKind::ParseXml(err),
    "An error while parsing the XML response"
);
impl_from_error!(
    IoError,
    |err| ErrorKind::Io(err),
    "An error occurred in an io process"
);
impl_from_error!(
    ResolveError,
    |err| ErrorKind::Resolve(err),
    "An error occurred while resolving a dns query"
);

impl Error {
    pub fn new<M: Into<String>>(kind: ErrorKind, message: M) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

pub(crate) use err;

pub type Result<T> = result::Result<T, Error>;
