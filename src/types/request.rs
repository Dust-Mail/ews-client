use bytes::Bytes;

use crate::error::{err, ErrorKind, Result};

use super::protocol::Protocol;

pub struct AutodiscoverRequest {
    use_auth: bool,
    url: String,
    email: String,
}

impl AutodiscoverRequest {
    pub fn new<U: Into<String>, E: Into<String>>(url: U, email: E, use_auth: bool) -> Self {
        Self {
            url: url.into(),
            email: email.into(),
            use_auth,
        }
    }

    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }

    pub fn protocol(&self) -> Result<Protocol> {
        match Protocol::from_url(&self.url) {
            Some(url) => Ok(url),
            None => {
                err!(ErrorKind::InvalidRequestUrl, "The request url is not valid")
            }
        }
    }

    pub fn payload(&self) -> Result<Bytes> {
        let protocol = self.protocol()?;

        let payload = protocol.create_request_body(&self.email)?;

        Ok(payload)
    }

    pub fn url(&self) -> &str {
        self.url.as_ref()
    }

    pub fn email(&self) -> &str {
        self.email.as_ref()
    }

    pub fn auth_required(&self) -> bool {
        self.use_auth
    }
}
