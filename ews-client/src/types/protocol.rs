use std::{fmt::Display, io, path::Path};

use bytes::Bytes;
use log::debug;
use surf::Url;

#[cfg(feature = "pox")]
use super::pox::Autodiscover as PoxAutodiscover;

use super::response::AutodiscoverResult;
use crate::error::Result;

pub enum Protocol {
    #[cfg(feature = "soap")]
    SOAP,
    #[cfg(feature = "pox")]
    POX,
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.file_extension())
    }
}

impl Protocol {
    /// Create the corresponding file extension for the current protocol.
    pub fn file_extension(&self) -> String {
        let ext = match self {
            #[cfg(feature = "pox")]
            Self::POX => "xml",
            #[cfg(feature = "soap")]
            Self::SOAP => "svc",
            #[cfg(all(not(feature = "soap"), not(feature = "pox")))]
            _ => "",
        };

        ext.to_string()
    }

    /// Detects the protocol from a given url.
    pub fn from_url<U: AsRef<str>>(url: U) -> Option<Self> {
        let url = Url::parse(url.as_ref()).ok()?;

        let path = Path::new(url.path());

        if let Some(ext) = path.extension() {
            if let Some(ext_str) = ext.to_str() {
                return Self::from_ext(ext_str);
            }
        }

        None
    }

    pub fn from_ext<E: AsRef<str>>(ext: E) -> Option<Self> {
        match ext.as_ref() {
            #[cfg(feature = "soap")]
            "svc" => Some(Self::SOAP),
            #[cfg(feature = "pox")]
            "xml" => Some(Self::POX),

            _ => None,
        }
    }

    pub fn create_request_body<E: AsRef<str>>(&self, email_address: E) -> Result<Bytes> {
        match &self {
            #[cfg(feature = "pox")]
            Protocol::POX => {
                let request_config = PoxAutodiscover::create_request(email_address.as_ref());

                let mut buf = Vec::new();

                serde_xml_rs::to_writer(&mut buf, &request_config)?;

                debug!("Request configuration: {}", String::from_utf8_lossy(&buf));

                Ok(buf.into())
            }
            #[cfg(feature = "soap")]
            Protocol::SOAP => Ok(Bytes::new()),
            #[cfg(all(not(feature = "soap"), not(feature = "pox")))]
            _ => Ok(Bytes::new()),
        }
    }

    pub fn parse_response<B: AsRef<[u8]>>(&self, bytes: B) -> Result<AutodiscoverResult> {
        let reader = io::Cursor::new(bytes);

        match &self {
            #[cfg(feature = "pox")]
            Self::POX => {
                let config = PoxAutodiscover::from_xml(reader)?;

                Ok(config.into())
            }
            #[cfg(feature = "soap")]
            Self::SOAP => {
                unimplemented!()
            }
            #[cfg(all(not(feature = "soap"), not(feature = "pox")))]
            _ => {
                use crate::error::{err, ErrorKind};

                err!(
                    ErrorKind::InvalidProtocol,
                    "There is valid protocol to handle the response"
                )
            }
        }
    }
}
