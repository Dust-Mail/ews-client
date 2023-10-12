use std::time::Duration;

use base64::{display::Base64Display, engine::general_purpose::STANDARD};
use bytes::Bytes;
use log::debug;
use surf::{http::Method, Client as HttpClient, Config, Url};

use crate::error::{err, ErrorKind, Result};

#[derive(Clone)]
/// Credentials used for HTTP Basic auth
pub struct BasicCredentials {
    username: String,
    password: Option<String>,
}

impl BasicCredentials {
    pub fn header(&self) -> String {
        let creds = match self.password.as_ref() {
            Some(password) => {
                format!("{}:{}", self.username, password)
            }
            None => {
                format!("{}", self.username)
            }
        };

        let creds_base64 = Base64Display::new(creds.as_bytes(), &STANDARD);

        format!("Basic {}", creds_base64)
    }
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
    client: HttpClient,
}

impl Http {
    const XML_CONTENT_TYPE: (&str, &str) = ("application/xml", "text/xml");
    const TIMEOUT: Duration = Duration::from_secs(10);

    pub fn new() -> Result<Self> {
        let client: surf::Client = match Config::new().set_timeout(Some(Self::TIMEOUT)).try_into() {
            Ok(client) => client,
            Err(err) => err!(
                ErrorKind::BuildHttpClient,
                "Failed to create http client: {}",
                err,
            ),
        };

        let http = Self {
            client: client.with(surf::middleware::Redirect::new(5)),
        };

        Ok(http)
    }

    /// Fetch XML data over HTTP given the url, method, body and optionally basic auth credentials.
    pub async fn request_xml<U: AsRef<str>, C: AsRef<BasicCredentials>>(
        &self,
        url: U,
        method: Method,
        body: Bytes,
        basic_creds: Option<C>,
    ) -> Result<Bytes> {
        match Url::parse(url.as_ref()) {
            Ok(_) => {}
            Err(err) => {
                err!(
                    ErrorKind::InvalidRequestUrl,
                    "The provided request url is not valid: {}",
                    err,
                )
            }
        }

        let mut request = self
            .client
            .request(method, url)
            .body(body.as_ref())
            .header("Content-Type", Self::XML_CONTENT_TYPE.1);

        if let Some(creds) = basic_creds {
            let creds = creds.as_ref();

            let header_value = creds.header();

            request = request.header("Authorization", header_value);
        }

        let mut response = request.send().await?;

        debug!("Status: {}", response.status(),);

        if !response.status().is_success() {
            err!(
                ErrorKind::HttpRequest,
                "Http request returned status {}",
                response.status()
            )
        };

        // Get the http message body
        let bytes = response.body_bytes().await?;

        debug!("Response string: {}", std::str::from_utf8(&bytes).unwrap());

        Ok(bytes.into())
    }
}
