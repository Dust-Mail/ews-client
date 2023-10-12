use std::{io::Read, time::Duration};

use serde::{Deserialize, Serialize};

use super::response::{AutodiscoverResponse, AutodiscoverResult, RedirectType};

use crate::{
    constants::{DEFAULT_MICROSOFT_REQUEST_SCHEMA, DEFAULT_MICROSOFT_RESPONSE_SCHEMA},
    error::Result,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Error {
    code: usize,
    message: String,
    debug_data: String,
}

impl Error {
    pub fn code(&self) -> usize {
        self.code
    }

    pub fn message(&self) -> &str {
        self.message.as_ref()
    }

    pub fn debug_data(&self) -> &str {
        self.debug_data.as_ref()
    }
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
    pub fn account(&self) -> Vec<&Account> {
        let mut accounts = Vec::new();

        for property in &self.properties {
            match property {
                ResponseProperty::Account(account) => accounts.push(account),
                _ => {}
            }
        }

        accounts
    }

    pub fn user(&self) -> Option<&User> {
        for property in &self.properties {
            match property {
                ResponseProperty::User(user) => return Some(user),
                _ => {}
            }
        }

        None
    }

    pub fn error(&self) -> Option<&Error> {
        for property in &self.properties {
            match property {
                ResponseProperty::Error(error) => return Some(error),
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

impl User {
    /// Represents the user's display name.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/displayname-string
    pub fn display_name(&self) -> Option<&str> {
        for property in &self.properties {
            match property {
                UserProperty::DisplayName(name) => return Some(name),
                _ => {}
            }
        }

        None
    }

    /// Identifies a user's mailbox by legacy distinguished name.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/legacydn-pox
    pub fn legacy_dn(&self) -> Option<&str> {
        for property in &self.properties {
            match property {
                UserProperty::LegacyDN(name) => return Some(&name.0),
                _ => {}
            }
        }

        None
    }

    /// Uniquely identifies the Exchange forest.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/deploymentid-pox
    pub fn deployment_id(&self) -> Option<&str> {
        for property in &self.properties {
            match property {
                UserProperty::DeploymentId(id) => return Some(id),
                _ => {}
            }
        }

        None
    }

    /// Contains the user's SMTP address that is used for the Autodiscover process.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/autodiscoversmtpaddress-pox
    pub fn autodiscover_smtp_address(&self) -> Option<&str> {
        for property in &self.properties {
            match property {
                UserProperty::AutoDiscoverSMTPAddress(addr) => return Some(addr),
                _ => {}
            }
        }

        None
    }
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
    /// Represents the account type.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/accounttype-pox
    pub fn account_type(&self) -> Option<&AccountType> {
        for property in &self.properties {
            match property {
                AccountProperty::AccountType(r#type) => return Some(r#type),
                _ => {}
            }
        }

        None
    }

    /// Provides information that is used to determine whether another Autodiscover request is required to return the user configuration information.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/action-pox
    pub fn action_type(&self) -> Option<&Action> {
        for property in &self.properties {
            match property {
                AccountProperty::Action(action) => return Some(action),
                _ => {}
            }
        }

        None
    }

    /// Indicates whether the user's mailbox is hosted in Exchange Online or Exchange Online as part of Office 365.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/microsoftonline-pox
    pub fn microsoft_online(&self) -> bool {
        for property in &self.properties {
            match property {
                AccountProperty::MicrosoftOnline(online) => return *online,
                _ => {}
            }
        }

        false
    }

    /// Specifies the email address that should be used for a subsequent Autodiscover request.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/redirectaddr-pox
    pub fn redirect_addr(&self) -> Option<&str> {
        for property in &self.properties {
            match property {
                AccountProperty::RedirectAddr(addr) => return Some(addr),
                _ => {}
            }
        }

        None
    }

    /// Contains the URL of the computer that is running Exchange Server that has the Client Access server role installed that should be used to obtain Autodiscover settings.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/redirecturl-pox
    pub fn redirect_url(&self) -> Option<&str> {
        for property in &self.properties {
            match property {
                AccountProperty::RedirectUrl(url) => return Some(url),
                _ => {}
            }
        }

        None
    }

    /// Contains the path of an image that is used to brand the configuration experience.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/image-pox
    pub fn image(&self) -> Option<&str> {
        for property in &self.properties {
            match property {
                AccountProperty::Image(path) => return Some(path),
                _ => {}
            }
        }

        None
    }

    /// Contains the URL of the home page of the Internet service provider (ISP).
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/servicehome-pox
    pub fn service_home(&self) -> Option<&str> {
        for property in &self.properties {
            match property {
                AccountProperty::ServiceHome(url) => return Some(url),
                _ => {}
            }
        }

        None
    }

    /// Contains the specifications for connecting a client to the Client Access server
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/protocol-pox
    pub fn protocol(&self) -> Option<&Protocol> {
        for property in &self.properties {
            match property {
                AccountProperty::Protocol(proto) => return Some(proto),
                _ => {}
            }
        }

        None
    }

    /// Contains information that clients can use to send an Autodiscover request to discover public folder information for the user.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/publicfolderinformation-pox
    pub fn public_folder_information(&self) -> Option<&PublicFolderInformation> {
        for property in &self.properties {
            match property {
                AccountProperty::PublicFolderInformation(info) => return Some(info),
                _ => {}
            }
        }

        None
    }

    /// Contains an Autodiscover error response.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/error-pox
    pub fn error(&self) -> Option<&Error> {
        for property in &self.properties {
            match property {
                AccountProperty::Error(error) => return Some(error),
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

impl Protocol {
    /// Identifies the type of the configured mail account.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/type-pox
    pub fn r#type(&self) -> Option<&Type> {
        for property in &self.properties {
            match property {
                ProtocolProperty::Type(r#type) => return Some(r#type),
                _ => {}
            }
        }

        None
    }

    /// Contains a collection of URLs that a client can use to connect to Exchange from inside the organization's network.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/internal-pox
    pub fn internal(&self) -> Option<&Connectivity> {
        for property in &self.properties {
            match property {
                ProtocolProperty::Internal(internal) => return Some(internal),
                _ => {}
            }
        }

        None
    }

    /// Contains a collection of URLs that a client can use to connect to Exchange from outside of the organization's network.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/external-pox
    pub fn external(&self) -> Option<&Connectivity> {
        for property in &self.properties {
            match property {
                ProtocolProperty::External(external) => return Some(external),
                _ => {}
            }
        }

        None
    }

    /// Specifies the Time to Live, during which the settings remain valid.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/ttl-pox
    pub fn ttl(&self) -> Duration {
        for property in &self.properties {
            match property {
                ProtocolProperty::Ttl(ttl) => return Duration::from_secs(ttl * 60 * 60),
                _ => {}
            }
        }

        Duration::from_secs(60 * 60)
    }

    /// Specifies the name of the mail server.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/server-pox
    pub fn server(&self) -> Option<&str> {
        for property in &self.properties {
            match property {
                ProtocolProperty::Server(server) => return Some(server),
                _ => {}
            }
        }

        None
    }

    /// Specifies the Exchange Server distinguished name.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/serverdn-pox
    pub fn server_dn(&self) -> Option<&str> {
        for property in &self.properties {
            match property {
                ProtocolProperty::Server(server) => return Some(server),
                _ => {}
            }
        }

        None
    }

    /// Represents the Exchange Server version number.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/serverversion-pox
    pub fn server_version(&self) -> Option<&str> {
        for property in &self.properties {
            match property {
                ProtocolProperty::ServerVersion(version) => return Some(version),
                _ => {}
            }
        }

        None
    }

    /// Contains the fully-qualified domain name (FQDN) of the public folder server for the user.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/publicfolderserver-pox
    pub fn public_folder_server(&self) -> Option<&str> {
        for property in &self.properties {
            match property {
                ProtocolProperty::PublicFolderServer(server) => return Some(server),
                _ => {}
            }
        }

        None
    }

    /// Specifies the port that is used to connect to the store.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/port-pox
    pub fn port(&self) -> Option<&u16> {
        for property in &self.properties {
            match property {
                ProtocolProperty::Port(port) => return Some(port),
                _ => {}
            }
        }

        None
    }

    /// Specifies the URL of the best endpoint instance for Exchange Web Services (EWS) for a mail-enabled user.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/ewsurl-pox
    pub fn ews_url(&self) -> Option<&str> {
        for property in &self.properties {
            match property {
                ProtocolProperty::EwsUrl(url) => return Some(url),
                _ => {}
            }
        }

        None
    }

    /// Specifies the user's logon name.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/loginname-pox
    pub fn login_name(&self) -> Option<&str> {
        for property in &self.properties {
            match property {
                ProtocolProperty::LoginName(name) => return Some(name),
                _ => {}
            }
        }

        None
    }

    /// Indicates whether the domain is required for authentication.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/domainrequired-pox
    pub fn domain_required(&self) -> Option<bool> {
        for property in &self.properties {
            match property {
                ProtocolProperty::DomainRequired(required) => return Some(required.bool()),
                _ => {}
            }
        }

        None
    }

    /// Specifies the user's domain.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/domainname-pox
    pub fn domain_name(&self) -> Option<&str> {
        for property in &self.properties {
            match property {
                ProtocolProperty::DomainName(name) => return Some(name),
                _ => {}
            }
        }

        None
    }

    /// Indicates whether secure password authentication is required.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/spa-pox
    pub fn spa(&self) -> bool {
        for property in &self.properties {
            match property {
                ProtocolProperty::Spa(spa) => return spa.bool(),
                _ => {}
            }
        }

        true
    }

    /// Specifies the authentication scheme that is used when authenticating against the Exchange 2007 computer that has the Mailbox server role installed.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/authpackage-pox
    pub fn auth_package(&self) -> Option<&AuthPackage> {
        for property in &self.properties {
            match property {
                ProtocolProperty::AuthPackage(package) => return Some(package),
                _ => {}
            }
        }

        None
    }

    /// Specifies the Secure Sockets Layer (SSL) certificate principal name that is required to connect to the Microsoft Exchange organization by using SSL.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/certprincipalname-pox
    pub fn cert_principal_name(&self) -> &str {
        for property in &self.properties {
            match property {
                ProtocolProperty::CertPrincipalName(name) => return name,
                _ => {}
            }
        }

        "msstd:SERVER"
    }

    /// Specifies whether secure logon is needed.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/ssl-pox
    pub fn ssl(&self) -> bool {
        for property in &self.properties {
            match property {
                ProtocolProperty::Ssl(ssl) => return ssl.bool(),
                _ => {}
            }
        }

        true
    }

    /// Indicates whether the authentication information that is provided for a POP3 type of account is also used for Simple Mail Transfer Protocol (SMTP).
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/usepopauth-pox
    pub fn use_pop_auth(&self) -> Option<bool> {
        for property in &self.properties {
            match property {
                ProtocolProperty::UsePOPAuth(use_pop) => return Some(use_pop.bool()),
                _ => {}
            }
        }

        None
    }

    /// Specifies whether the SMTP server requires that e-mail be downloaded before it sends e-mail by using the SMTP server.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/smtplast-pox
    pub fn smtp_last(&self) -> bool {
        for property in &self.properties {
            match property {
                ProtocolProperty::SMTPLast(last) => return last.bool(),
                _ => {}
            }
        }

        false
    }

    /// Contains the criteria that are used to determine whether the client computer is on a network that meets the Internet Service Provider's requirements to connect to the server.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/networkrequirements-pox
    pub fn network_requirements(&self) -> Option<&NetworkRequirements> {
        for property in &self.properties {
            match property {
                ProtocolProperty::NetworkRequirements(requirements) => return Some(requirements),
                _ => {}
            }
        }

        None
    }

    /// Contains the specifications for connecting a client to the address book server by using the MAPI/HTTP protocol. This element is only present if the Type attribute on the Protocol element is present and set to "mapiHttp". The AddressBook element is applicable to clients that implement the MAPI/HTTP protocol and target Exchange Online and versions of Exchange starting with 15.00.0847.032.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/addressbook-pox
    pub fn address_book(&self) -> Option<&AddressBook> {
        for property in &self.properties {
            match property {
                ProtocolProperty::AddressBook(address_book) => return Some(address_book),
                _ => {}
            }
        }

        None
    }

    /// Contains the specifications for connecting a client to the user's mailbox by using the MAPI/HTTP protocol. This element is only present if the Type attribute on the Protocol element is present and set to "mapiHttp". The MailStore element is applicable to clients that implement the MAPI/HTTP protocol and target Exchange Online and versions of Exchange starting with 15.00.0847.032.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/mailstore-pox
    pub fn mail_store(&self) -> Option<&MailStore> {
        for property in &self.properties {
            match property {
                ProtocolProperty::MailStore(store) => return Some(store),
                _ => {}
            }
        }

        None
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum ProtocolProperty {
    Type(Type),
    Internal(Connectivity),
    External(Connectivity),
    #[serde(rename = "TTL")]
    Ttl(u64),
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

impl OnOff {
    pub fn bool(&self) -> bool {
        match self {
            OnOff::On => true,
            OnOff::Off => false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Connectivity {
    #[serde(rename = "OWAUrl")]
    owa_url: String,
    protocol: Protocol,
}

impl Connectivity {
    /// Describes the URL and authentication schema that is used to access a particular computer that is running Microsoft Exchange Server that has the Client Access server role installed that hosts Outlook Web Access.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/owaurl-pox
    pub fn owa_url(&self) -> &str {
        self.owa_url.as_ref()
    }

    /// Contains the specifications for connecting a client to the computer that is running Microsoft Exchange Server that has the Client Access server role installed. This Protocol element has only two child elements: a Type (POX) element specifying the connection protocol, and an ASUrl (POX) element, specifying the EWS endpoint for the Availability web service.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/protocol-pox
    pub fn protocol(&self) -> &Protocol {
        &self.protocol
    }
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

impl PublicFolderInformation {
    /// Contains the SMTP address assigned to the public folder message store configured for the user. This SMTP address can be used in the EMailAddress (POX) element of an Autodiscover request to discover public folder settings.
    ///
    /// https://learn.microsoft.com/en-us/exchange/client-developer/web-service-reference/smtpaddress-pox
    pub fn smtp_address(&self) -> &str {
        self.smtp_address.as_ref()
    }
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
    pub ipv4_start: String,
    #[serde(rename = "Ipv4End")]
    pub ipv4_end: String,
    #[serde(rename = "Ipv6Start")]
    pub ipv6_start: String,
    #[serde(rename = "Ipv6end")]
    pub ipv6_end: String,
}

impl Into<AutodiscoverResult> for Autodiscover {
    fn into(self) -> AutodiscoverResult {
        if let Some(response) = self.into_response() {
            for account in response.account() {
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

                if let Some(error) = account.error() {
                    return AutodiscoverResult::Error(error.into());
                }
            }

            if let Some(error) = response.error() {
                return AutodiscoverResult::Error(error.into());
            }

            return AutodiscoverResult::Ok(AutodiscoverResponse::Pox(response));
        }

        AutodiscoverResult::error("Config did not contain a response value")
    }
}
