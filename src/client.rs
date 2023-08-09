use reqwest::Method;

use async_recursion::async_recursion;

use crate::{
    candidate::CandidateType,
    config::{Config, ConfigResult, RedirectType},
    error::{Error, ErrorKind, Result},
    failed,
    http::{BasicCredentials, Http},
};

pub struct Client {
    creds: BasicCredentials,
    http: Http,
}

pub struct AutodiscoverRequest {
    url: String,
    email: String,
}

impl<U: Into<String>, E: Into<String>> From<(U, E)> for AutodiscoverRequest {
    fn from((url, email): (U, E)) -> Self {
        Self::new(url, email)
    }
}

impl AutodiscoverRequest {
    pub fn new<U: Into<String>, E: Into<String>>(url: U, email: E) -> Self {
        Self {
            url: url.into(),
            email: email.into(),
        }
    }

    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }
}

impl Client {
    pub fn new<B: Into<BasicCredentials>>(creds: B) -> Result<Self> {
        let http = Http::new()?;

        let client = Self {
            creds: creds.into(),
            http,
        };

        Ok(client)
    }

    async fn handle_config_result(
        &self,
        result: ConfigResult,
        mut request: AutodiscoverRequest,
    ) -> Result<Config> {
        match result {
            ConfigResult::Ok(config) => Ok(config),
            ConfigResult::Error(error) => failed!(
                ErrorKind::InvalidConfig,
                "The received config is invalid: {}",
                error.message(),
            ),
            ConfigResult::Redirect(redirect_type) => {
                match redirect_type {
                    RedirectType::Email(email_addr) => {
                        request.set_email(email_addr);
                    }
                    RedirectType::Url(url) => {
                        request.set_url(url);
                    }
                }

                self.send_authenticated(request).await
            }
        }
    }

    #[async_recursion]
    pub async fn send_authenticated<R: Into<AutodiscoverRequest> + Send>(
        &self,
        request: R,
    ) -> Result<Config> {
        let request: AutodiscoverRequest = request.into();

        let url = &request.url;
        let email = &request.email;

        if let Some(candidate_type) = CandidateType::from_url(url) {
            let body = candidate_type.create_request_body(email)?;

            match self
                .http
                .fetch_xml(url, Method::POST, body, Some(&self.creds))
                .await
            {
                Ok(bytes) => {
                    let config_result = candidate_type.parse_config(bytes)?;

                    let config = self.handle_config_result(config_result, request).await?;

                    return Ok(config);
                }
                Err(err) => {
                    failed!(ErrorKind::HttpRequest, "Error fetching {}: {:?}", url, err)
                }
            }
        } else {
            failed!(
                ErrorKind::InvalidRequestUrl,
                "The request url does not have a valid extension"
            )
        }
    }
}
