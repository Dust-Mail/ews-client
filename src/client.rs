use async_recursion::async_recursion;
use surf::http::Method;

use crate::{
    dns::Dns,
    error::{err, ErrorKind, Result},
    http::{BasicCredentials, Http},
    types::{
        request::AutodiscoverRequest,
        response::{AutodiscoverResponse, AutodiscoverResult, RedirectType},
    },
};

pub struct Client {
    creds: BasicCredentials,
    http: Http,
    dns: Dns,
}

impl Client {
    pub async fn new<B: Into<BasicCredentials>>(creds: B) -> Result<Self> {
        let http = Http::new()?;
        let dns = Dns::new().await?;

        let client = Self {
            dns,
            creds: creds.into(),
            http,
        };

        Ok(client)
    }

    async fn handle_config_result<'a>(
        &self,
        result: AutodiscoverResult,
        mut request: AutodiscoverRequest<'a>,
    ) -> Result<AutodiscoverResponse> {
        match result {
            AutodiscoverResult::Ok(config) => Ok(config),
            AutodiscoverResult::Error(error) => err!(
                ErrorKind::InvalidConfig,
                "The received config is invalid: {}",
                error.message(),
            ),
            AutodiscoverResult::Redirect(redirect_type) => {
                match redirect_type {
                    RedirectType::Email(email_addr) => {
                        if email_addr == request.email() {
                            err!(ErrorKind::InvalidConfig, "The returned config redirects us to the same email address that we are already requesting");
                        }

                        request.set_email(email_addr);
                    }
                    RedirectType::Url(url) => {
                        if url == request.url() {
                            err!(ErrorKind::InvalidConfig, "The returned config redirects us to the same url that we are already requesting");
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
    pub async fn send_request<'a, R: Into<AutodiscoverRequest<'a>> + Send>(
        &self,
        request: R,
    ) -> Result<AutodiscoverResponse> {
        let request: AutodiscoverRequest = request.into();

        let url = request.url();
        let payload = request.payload()?;

        let method = if request.auth_required() {
            Method::Post
        } else {
            Method::Get
        };

        let creds = if request.auth_required() {
            Some(&self.creds)
        } else {
            None
        };

        let bytes = self.http.request_xml(url, method, payload, creds).await?;

        let request_protocol = request.protocol()?;

        let config_result = request_protocol.parse_response(bytes)?;

        let config = self.handle_config_result(config_result, request).await?;

        return Ok(config);
    }

    pub async fn dns_query<D: AsRef<str>>(&self, domain: D) -> Result<Vec<(String, u16)>> {
        let results = self.dns.srv_lookup(domain).await?;

        Ok(results
            .into_iter()
            .map(|(fqdn, port)| (fqdn.trim_end_matches('.').to_string(), port))
            .collect())
    }
}
