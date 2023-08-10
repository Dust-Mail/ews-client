use reqwest::Method;

use async_recursion::async_recursion;

use crate::{
    candidate::CandidateType,
    config::{Config, ConfigResult, RedirectType},
    dns::Dns,
    error::{ErrorKind, Result},
    failed,
    http::{BasicCredentials, Http},
};

pub struct Client {
    creds: BasicCredentials,
    http: Http,
    dns: Dns,
}

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
}

impl Client {
    pub fn new<B: Into<BasicCredentials>>(creds: B) -> Result<Self> {
        let http = Http::new()?;
        let dns = Dns::new()?;

        let client = Self {
            dns,
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
                        if email_addr == request.email {
                            failed!(ErrorKind::InvalidConfig, "The returned config redirects us to the same email address that we are already requesting");
                        }

                        request.set_email(email_addr);
                    }
                    RedirectType::Url(url) => {
                        if url == request.url {
                            failed!(ErrorKind::InvalidConfig, "The returned config redirects us to the same url that we are already requesting");
                        }

                        request.set_url(url);
                    }
                }

                self.send_request(request).await
            }
        }
    }

    #[async_recursion]
    /// Send an autodiscover request to an Exchange server.
    pub async fn send_request<R: Into<AutodiscoverRequest> + Send>(
        &self,
        request: R,
    ) -> Result<Config> {
        let request: AutodiscoverRequest = request.into();

        let url = &request.url;
        let email = &request.email;

        if let Some(candidate_type) = CandidateType::from_url(url) {
            let body = candidate_type.create_request_body(email)?;

            let method = if request.use_auth {
                Method::POST
            } else {
                Method::GET
            };

            let creds = if request.use_auth {
                Some(&self.creds)
            } else {
                None
            };

            match self.http.fetch_xml(url, method, body, creds).await {
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

    pub async fn dns_query<D: AsRef<str>>(&self, domain: D) -> Result<(String, u16)> {
        let (fqdn, port) = self.dns.srv_lookup(domain).await?;

        let domain_name = fqdn.trim_end_matches('.').to_string();

        Ok((domain_name, port))
    }
}
