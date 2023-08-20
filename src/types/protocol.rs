use std::{fmt::Display, io, path::Path};

use bytes::Bytes;
use log::debug;
use surf::Url;

#[cfg(feature = "pox")]
use super::pox::Autodiscover as PoxAutodiscover;

use super::response::AutodiscoverResult;
use crate::{
    error::{ErrorKind, Result},
    failed,
};

pub enum Protocol {
    // SOAP,
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
            Protocol::POX => "xml",
            // CandidateType::SOAP => "svc",
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
            // "svc" => Some(Self::SOAP),
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

                let config_string = serde_xml_rs::to_string(&request_config)?;

                debug!("Request configuration: {}", config_string);

                Ok(config_string.into())
            }
            _ => Ok(Bytes::new()),
        }
    }

    pub fn parse_response<B: AsRef<[u8]>>(&self, bytes: B) -> Result<AutodiscoverResult> {
        let reader = io::Cursor::new(bytes);

        match &self {
            #[cfg(feature = "pox")]
            Protocol::POX => {
                let config = PoxAutodiscover::from_xml(reader)?;

                Ok(config.into())
            } // CandidateType::SOAP => {
            //     let config: SoapConfig = serde_xml_rs::from_reader(xml)?;

            //     Ok(config.into())
            // }
            _ => failed!(
                ErrorKind::InvalidProtocol,
                "There is valid protocol to handle the response"
            ),
        }
    }
}
