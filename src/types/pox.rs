use std::io::Read;

use serde::{Deserialize, Serialize};

use super::response::{AutodiscoverResponse, AutodiscoverResult, RedirectType};

use crate::{
    constants::{DEFAULT_MICROSOFT_REQUEST_SCHEMA, DEFAULT_MICROSOFT_RESPONSE_SCHEMA},
    error::Result,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Error {
    error_code: usize,
    message: String,
    debug_data: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// Config follows Microsofts spec: https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/autodiscover-pox
pub struct Autodiscover {
    #[serde(rename(serialize = "@xmlns"), skip_deserializing)]
    xmlns: String,
    #[serde(rename = "$value")]
    properties: Vec<ConfigProperty>,
}

impl Autodiscover {
    pub fn create_request<E: Into<String>>(email_address: E) -> Self {
        let req = Request::new(email_address.into());

        let properties = vec![ConfigProperty::Request(req)];

        Self {
            properties,
            xmlns: DEFAULT_MICROSOFT_REQUEST_SCHEMA.to_string(),
        }
    }

    pub fn from_xml<R: Read>(xml: R) -> Result<Self> {
        let config: Self = serde_xml_rs::from_reader(xml)?;

        // info!("{:?}", config);

        Ok(config)
    }

    fn into_response(self) -> Option<Response> {
        for property in self.properties {
            match property {
                ConfigProperty::Response(response) => return Some(response),
                _ => {}
            }
        }

        None
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum ConfigProperty {
    Response(Response),
    Request(Request),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Request {
    #[serde(rename = "$value")]
    properties: Vec<RequestProperty>,
}

impl Request {
    pub fn new(email_address: String) -> Self {
        let properties = vec![
            RequestProperty::EMailAddress(email_address),
            RequestProperty::AcceptableResponseSchema(
                DEFAULT_MICROSOFT_RESPONSE_SCHEMA.to_string(),
            ),
        ];

        Self { properties }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum RequestProperty {
    EMailAddress(String),
    AcceptableResponseSchema(String),
    LegacyDN(LegacyDN),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    #[serde(rename = "$value")]
    properties: Vec<ResponseProperty>,
}

impl Response {
    pub fn account(&self) -> Option<&Account> {
        for property in &self.properties {
            match property {
                ResponseProperty::Account(account) => return Some(account),
                _ => {}
            }
        }

        None
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum ResponseProperty {
    User(User),
    Account(Account),
    Error(Error),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct User {
    #[serde(rename = "$value")]
    properties: Vec<UserProperty>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum UserProperty {
    DisplayName(String),
    LegacyDN(LegacyDN),
    DeploymentId(String),
    AutoDiscoverSMTPAddress(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LegacyDN(String);

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Account {
    #[serde(rename = "$value")]
    properties: Vec<AccountProperty>,
}

impl Account {
    pub fn action_type(&self) -> Option<&Action> {
        for property in &self.properties {
            match property {
                AccountProperty::Action(action) => return Some(action),
                _ => {}
            }
        }

        None
    }

    pub fn redirect_addr(&self) -> Option<&str> {
        for property in &self.properties {
            match property {
                AccountProperty::RedirectAddr(addr) => return Some(addr),
                _ => {}
            }
        }

        None
    }

    pub fn redirect_url(&self) -> Option<&str> {
        for property in &self.properties {
            match property {
                AccountProperty::RedirectUrl(url) => return Some(url),
                _ => {}
            }
        }

        None
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum AccountProperty {
    AccountType(AccountType),
    Action(Action),
    MicrosoftOnline(bool),
    RedirectUrl(String),
    RedirectAddr(String),
    Image(String),
    ServiceHome(String),
    Protocol(Protocol),
    PublicFolderInformation(PublicFolderInformation),
    Error(Error),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AccountType {
    Email,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    RedirectUrl,
    RedirectAddr,
    Settings,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Protocol {
    #[serde(rename = "$value")]
    properties: Vec<ProtocolProperty>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum ProtocolProperty {
    Type(Type),
    Internal(Internal),
    External(External),
    #[serde(rename = "TTL")]
    Ttl(usize),
    Server(String),
    ServerDN(String),
    ServerVersion(String),
    MdbDN(String),
    PublicFolderServer(String),
    Port(u16),
    DirectoryPort(u16),
    ReferralPort(u16),
    ASUrl(String),
    EwsUrl(String),
    SharingUrl(String),
    EmwsUrl(String),
    OOFUrl(String),
    OABUrl(String),
    UMUrl(String),
    EwsPartnerUrl(String),
    LoginName(String),
    DomainRequired(OnOff),
    DomainName(String),
    #[serde(rename = "SPA")]
    Spa(OnOff),
    AuthPackage(AuthPackage),
    CertPrincipalName(String),
    #[serde(rename = "SSL")]
    Ssl(OnOff),
    AuthRequired(OnOff),
    UsePOPAuth(OnOff),
    SMTPLast(OnOff),
    NetworkRequirements(NetworkRequirements),
    AddressBook(AddressBook),
    MailStore(MailStore),
    Encryption(Encryption),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum Type {
    Smtp,
    Imap,
    Exch,
    ExHttp,
    Expr,
    Web,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OnOff {
    On,
    Off,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Internal {
    #[serde(rename = "OWAUrl")]
    owa_url: String,
    protocol: Protocol,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct External {
    #[serde(rename = "OWAUrl")]
    owa_url: String,
    protocol: Protocol,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Encryption {
    #[serde(rename = "TLS")]
    Tls,
    #[serde(rename = "SSL")]
    Ssl,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum AuthPackage {
    Basic,
    Kerb,
    KerbNtlm,
    Ntlm,
    Certificate,
    Negotiate,
    Nego2,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PublicFolderInformation {
    smtp_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AddressBook {
    external_url: String,
    interal_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MailStore {
    external_url: String,
    interal_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NetworkRequirements {
    #[serde(rename = "Ipv4Start")]
    ipv4_start: String,
    #[serde(rename = "Ipv4End")]
    ipv4_end: String,
    #[serde(rename = "Ipv6Start")]
    ipv6_start: String,
    #[serde(rename = "Ipv6end")]
    ipv6_end: String,
}

impl Into<AutodiscoverResult> for Autodiscover {
    fn into(self) -> AutodiscoverResult {
        if let Some(response) = self.into_response() {
            if let Some(account) = response.account() {
                if let Some(action_type) = account.action_type() {
                    let redirect_type = match action_type {
                        Action::RedirectAddr => match account.redirect_addr() {
                            Some(addr) => Some(RedirectType::Email(addr.to_string())),
                            None => {
                                return AutodiscoverResult::error(
                                    "Missing redirect address when it should be available",
                                )
                            }
                        },
                        Action::RedirectUrl => match account.redirect_url() {
                            Some(url) => Some(RedirectType::Url(url.to_string())),
                            None => {
                                return AutodiscoverResult::error(
                                    "Missing redirect url when it should be available",
                                )
                            }
                        },
                        _ => None,
                    };

                    if let Some(redirect) = redirect_type {
                        return AutodiscoverResult::Redirect(redirect);
                    }
                }
            }

            return AutodiscoverResult::Ok(AutodiscoverResponse::Pox(response));
        }

        AutodiscoverResult::error("Config did not contain a response value")
    }
}
