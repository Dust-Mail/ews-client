use bytes::Bytes;
use log::debug;
use reqwest::{Client, IntoUrl, Method};

use crate::error::{Error, ErrorKind, Result};

pub struct BasicCredentials {
    username: String,
    password: Option<String>,
}

impl From<(String, Option<String>)> for BasicCredentials {
    fn from((username, password): (String, Option<String>)) -> Self {
        Self { password, username }
    }
}

pub struct Http {
    client: Client,
}

impl Http {
    pub fn new() -> Result<Self> {
        let client = Client::builder().build()?;

        let http = Self { client };

        Ok(http)
    }

    const XML_CONTENT_TYPE: (&str, &str) = ("application/xml", "text/xml");

    pub async fn fetch_xml<U: IntoUrl, C: Into<BasicCredentials>>(
        &self,
        url: U,
        method: Method,
        body: Bytes,
        basic_creds: Option<C>,
    ) -> Result<Bytes> {
        let mut request = self
            .client
            .request(method, url)
            .body(body)
            .header("Content-Type", Self::XML_CONTENT_TYPE.1);

        if let Some(creds) = basic_creds {
            let creds: BasicCredentials = creds.into();

            request = request.basic_auth(creds.username, creds.password)
        }

        let response = request.send().await?;

        debug!("Status: {}", response.status());

        if !response.status().is_success() {
            return Err(Error::new(
                ErrorKind::InvalidHttpResponse,
                format!("Http request returned status {}", response.status()),
            ));
        };

        // Get the Content-Type header, error if it doesn't exist
        let content_type = match response.headers().get("content-type") {
            Some(header) => header.to_str().map_err(|_| {
                Error::new(
                    ErrorKind::InvalidHttpResponse,
                    "Content-Type header does not contain valid characters",
                )
            })?,
            None => {
                return Err(Error::new(
                    ErrorKind::InvalidHttpResponse,
                    "Server did not include a content-type header in response",
                ))
            }
        };

        let content_type = content_type.to_string();

        // Get the http message body
        let bytes = response.bytes().await?;

        debug!("Response string: {}", std::str::from_utf8(&bytes).unwrap());

        // Ensure the content type is XML
        if !(content_type.starts_with(Self::XML_CONTENT_TYPE.0)
            || content_type.starts_with(Self::XML_CONTENT_TYPE.1))
        {
            return Err(Error::new(
                ErrorKind::InvalidHttpResponse,
                "Server did not respond with XML content",
            ));
        }

        Ok(bytes)
    }
}
