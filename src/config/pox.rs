use std::io::Read;

use log::info;
use serde::{Deserialize, Serialize};

use super::Config;

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
/// The config following Microsofts spec: https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/autodiscover-pox
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

        info!("{:?}", config);

        Ok(config)
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
    LegacyDN(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    #[serde(rename = "$value")]
    properties: Vec<ResponseProperty>,
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
    display_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Account {
    #[serde(rename = "$value")]
    properties: Vec<AccountProperty>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum AccountProperty {
    AccountType(AccountType),
    Action(Action),
    MicrosoftOnline,
    RedirectUrl(String),
    RedirectAddr(String),
    Image(String),
    ServiceHome(String),
    Protocol(Protocol),
    // PublicFolderInformation(PublicFolderInformation),
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
    Port(usize),
    DirectoryPort(usize),
    ReferralPort(usize),
    ASUrl(String),
    EwsUrl(String),
    SharingUrl(String),
    EmwsUrl(String),
    LoginName(String),
    DomainRequired(OnOff),
    DomainName(String),
    CertPrincipalName(String),
    #[serde(rename = "SPA")]
    Spa(OnOff),
    AuthRequired(OnOff),
    #[serde(rename = "SSL")]
    Ssl(OnOff),
    UsePOPAuth(OnOff),
    SMTPLast(OnOff),
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

impl Into<Config> for Autodiscover {
    fn into(self) -> Config {
        super::Config {}
    }
}
