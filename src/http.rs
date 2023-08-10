use bytes::Bytes;
use log::debug;
use reqwest::{Client, IntoUrl, Method};

use crate::{
    error::{ErrorKind, Result},
    failed,
};

#[derive(Clone)]
/// Credentials used for HTTP Basic auth
pub struct BasicCredentials {
    username: String,
    password: Option<String>,
}

impl AsRef<BasicCredentials> for BasicCredentials {
    fn as_ref(&self) -> &BasicCredentials {
        &self
    }
}

impl<U: Into<String>, P: Into<String>> From<(U, Option<P>)> for BasicCredentials {
    fn from((username, password): (U, Option<P>)) -> Self {
        Self {
            password: password.map(|pass| pass.into()),
            username: username.into(),
        }
    }
}

/// A simple HTTP client to fetch XML content
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

    /// Fetch XML data over HTTP given the url, method, body and optionally basic auth credentials.
    ///
    /// The server must respond with a Content-Type that corresponds with XML data.
    pub async fn fetch_xml<U: IntoUrl, C: AsRef<BasicCredentials>>(
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
            let creds = creds.as_ref();

            request = request.basic_auth(&creds.username, creds.password.as_ref())
        }

        let response = request.send().await?;

        debug!("Status: {}", response.status(),);

        if !response.status().is_success() {
            failed!(
                ErrorKind::HttpRequest,
                "Http request returned status {}",
                response.status()
            )
        };

        // Get the Content-Type header, error if it doesn't exist
        let content_type = match response.headers().get("content-type") {
            Some(header) => match header.to_str() {
                Ok(header) => header,
                Err(_) => {
                    failed!(
                        ErrorKind::HttpRequest,
                        "Content-Type header does not contain valid characters",
                    )
                }
            },
            None => {
                failed!(
                    ErrorKind::HttpRequest,
                    "Server did not include a content-type header in response",
                )
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
            failed!(
                ErrorKind::HttpRequest,
                "Server did not respond with XML content",
            );
        }

        Ok(bytes)
    }
}
