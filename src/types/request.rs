use std::borrow::Cow;

use bytes::Bytes;

use crate::error::{err, ErrorKind, Result};

use super::protocol::Protocol;

pub struct AutodiscoverRequest<'a> {
    use_auth: bool,
    url: Cow<'a, str>,
    email: Cow<'a, str>,
}

impl<'a> AutodiscoverRequest<'a> {
    pub fn new<U: Into<Cow<'a, str>>, E: Into<Cow<'a, str>>>(
        url: U,
        email: E,
        use_auth: bool,
    ) -> Self {
        Self {
            url: url.into(),
            email: email.into(),
            use_auth,
        }
    }

    pub fn set_url<U: Into<Cow<'a, str>>>(&mut self, url: U) {
        self.url = url.into();
    }

    pub fn set_email<E: Into<Cow<'a, str>>>(&mut self, email: E) {
        self.email = email.into();
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
