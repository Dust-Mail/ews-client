//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.1.6
//!

            #![allow(dead_code)]           
            #![allow(unused_imports)]
            use yaserde_derive::{YaSerialize, YaDeserialize};
            use std::io::{Read, Write};
            use log::{warn, debug};
            
            pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
            #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
pub struct Header {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Fault",
	namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
	prefix = "soapenv",
)]
pub struct SoapFault {
	#[yaserde(rename = "faultcode", default)]
	pub fault_code: Option<String>, 
	#[yaserde(rename = "faultstring", default)]
	pub fault_string: Option<String>, 
}
impl std::error::Error for SoapFault {}

impl std::fmt::Display for SoapFault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.fault_code, &self.fault_string) {
            (None, None) => Ok(()),
            (None, Some(fault_string)) => f.write_str(fault_string),
            (Some(fault_code), None) => f.write_str(fault_code),
            (Some(fault_code), Some(fault_string)) => {
                f.write_str(fault_code)?;
                f.write_str(": ")?;
                f.write_str(fault_string)
            }
        }
    }
}
pub type SoapResponse = Result<(reqwest::StatusCode, String), reqwest::Error>;

pub mod messages {
use yaserde::{YaSerialize, YaDeserialize};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;
            }

pub mod types {
use yaserde::{YaSerialize, YaDeserialize};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;
            #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetUserSettingsRequestMessage",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "tns",
)]
pub struct GetUserSettingsRequestMessage {
	#[yaserde(rename = "Request", prefix = "tns", default)]
	pub request: Option<GetUserSettingsRequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetUserSettingsRequest",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct GetUserSettingsRequest {
	#[yaserde(flatten, default)]
	pub autodiscover_request: AutodiscoverRequest, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Users", prefix = "tns", default)]
	pub users: Option<Users>, 
	#[yaserde(rename = "RequestedSettings", prefix = "tns", default)]
	pub requested_settings: Option<RequestedSettings>, 
	#[yaserde(rename = "RequestedVersion", prefix = "tns", default)]
	pub requested_version: Option<ExchangeVersion>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AutodiscoverRequest",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct AutodiscoverRequest {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Users",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct Users {
	#[yaserde(rename = "User", prefix = "tns", default)]
	pub user: Vec<User>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "User",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct User {
	#[yaserde(rename = "Mailbox", prefix = "tns", default)]
	pub mailbox: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RequestedSettings",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct RequestedSettings {
	#[yaserde(rename = "Setting", prefix = "tns", default)]
	pub setting: Vec<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ExchangeVersion",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct ExchangeVersion {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetUserSettingsResponseMessage",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "tns",
)]
pub struct GetUserSettingsResponseMessage {
	#[yaserde(rename = "Response", prefix = "tns", default)]
	pub response: Option<GetUserSettingsResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetUserSettingsResponse",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct GetUserSettingsResponse {
	#[yaserde(flatten, default)]
	pub autodiscover_response: AutodiscoverResponse, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "UserResponses", prefix = "tns", default)]
	pub user_responses: Option<ArrayOfUserResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AutodiscoverResponse",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct AutodiscoverResponse {
	#[yaserde(rename = "ErrorCode", prefix = "tns", default)]
	pub error_code: Option<ErrorCode>, 
	#[yaserde(rename = "ErrorMessage", prefix = "tns", default)]
	pub error_message: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ErrorCode",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct ErrorCode {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfUserResponse",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct ArrayOfUserResponse {
	#[yaserde(rename = "UserResponse", prefix = "tns", default)]
	pub user_response: Vec<UserResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UserResponse",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct UserResponse {
	#[yaserde(flatten, default)]
	pub autodiscover_response: AutodiscoverResponse, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "RedirectTarget", prefix = "tns", default)]
	pub redirect_target: Option<String>, 
	#[yaserde(rename = "UserSettingErrors", prefix = "tns", default)]
	pub user_setting_errors: Option<UserSettingErrors>, 
	#[yaserde(rename = "UserSettings", prefix = "tns", default)]
	pub user_settings: Option<UserSettings>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UserSettingErrors",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct UserSettingErrors {
	#[yaserde(rename = "UserSettingError", prefix = "tns", default)]
	pub user_setting_error: Vec<UserSettingError>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UserSettingError",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct UserSettingError {
	#[yaserde(rename = "ErrorCode", prefix = "tns", default)]
	pub error_code: ErrorCode, 
	#[yaserde(rename = "ErrorMessage", prefix = "tns", default)]
	pub error_message: Option<String>, 
	#[yaserde(rename = "SettingName", prefix = "tns", default)]
	pub setting_name: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UserSettings",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct UserSettings {
	#[yaserde(rename = "UserSetting", prefix = "tns", default)]
	pub user_setting: Vec<UserSetting>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UserSetting",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct UserSetting {
	#[yaserde(rename = "Name", prefix = "tns", default)]
	pub name: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ProtocolConnectionCollectionSetting",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct ProtocolConnectionCollectionSetting {
	#[yaserde(flatten, default)]
	pub user_setting: UserSetting, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ProtocolConnections", prefix = "tns", default)]
	pub protocol_connections: Option<ProtocolConnections>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ProtocolConnections",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct ProtocolConnections {
	#[yaserde(rename = "ProtocolConnection", prefix = "tns", default)]
	pub protocol_connection: Vec<ProtocolConnection>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ProtocolConnection",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct ProtocolConnection {
	#[yaserde(rename = "Hostname", prefix = "tns", default)]
	pub hostname: Option<String>, 
	#[yaserde(rename = "Port", prefix = "tns", default)]
	pub port: i32, 
	#[yaserde(rename = "EncryptionMethod", prefix = "tns", default)]
	pub encryption_method: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "StringSetting",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct StringSetting {
	#[yaserde(flatten, default)]
	pub user_setting: UserSetting, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Value", prefix = "tns", default)]
	pub value: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "WebClientUrlCollectionSetting",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct WebClientUrlCollectionSetting {
	#[yaserde(flatten, default)]
	pub user_setting: UserSetting, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "WebClientUrls", prefix = "tns", default)]
	pub web_client_urls: Option<WebClientUrls>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "WebClientUrls",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct WebClientUrls {
	#[yaserde(rename = "WebClientUrl", prefix = "tns", default)]
	pub web_client_url: Vec<WebClientUrl>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "WebClientUrl",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct WebClientUrl {
	#[yaserde(rename = "AuthenticationMethods", prefix = "tns", default)]
	pub authentication_methods: Option<String>, 
	#[yaserde(rename = "Url", prefix = "tns", default)]
	pub url: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AlternateMailboxCollectionSetting",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct AlternateMailboxCollectionSetting {
	#[yaserde(flatten, default)]
	pub user_setting: UserSetting, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "AlternateMailboxes", prefix = "tns", default)]
	pub alternate_mailboxes: Option<AlternateMailboxes>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AlternateMailboxes",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct AlternateMailboxes {
	#[yaserde(rename = "AlternateMailbox", prefix = "tns", default)]
	pub alternate_mailbox: Vec<AlternateMailbox>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AlternateMailbox",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct AlternateMailbox {
	#[yaserde(rename = "Type", prefix = "tns", default)]
	pub rs_type: Option<String>, 
	#[yaserde(rename = "DisplayName", prefix = "tns", default)]
	pub display_name: Option<String>, 
	#[yaserde(rename = "LegacyDN", prefix = "tns", default)]
	pub legacy_dn: Option<String>, 
	#[yaserde(rename = "Server", prefix = "tns", default)]
	pub server: Option<String>, 
	#[yaserde(rename = "SmtpAddress", prefix = "tns", default)]
	pub smtp_address: Option<String>, 
	#[yaserde(rename = "OwnerSmtpAddress", prefix = "tns", default)]
	pub owner_smtp_address: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DocumentSharingLocationCollectionSetting",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct DocumentSharingLocationCollectionSetting {
	#[yaserde(flatten, default)]
	pub user_setting: UserSetting, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "DocumentSharingLocations", prefix = "tns", default)]
	pub document_sharing_locations: DocumentSharingLocations, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DocumentSharingLocations",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct DocumentSharingLocations {
	#[yaserde(rename = "DocumentSharingLocation", prefix = "tns", default)]
	pub document_sharing_location: Vec<DocumentSharingLocation>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DocumentSharingLocation",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct DocumentSharingLocation {
	#[yaserde(rename = "ServiceUrl", prefix = "tns", default)]
	pub service_url: String, 
	#[yaserde(rename = "LocationUrl", prefix = "tns", default)]
	pub location_url: String, 
	#[yaserde(rename = "DisplayName", prefix = "tns", default)]
	pub display_name: String, 
	#[yaserde(rename = "SupportedFileExtensions", prefix = "tns", default)]
	pub supported_file_extensions: ArrayOfFileExtension, 
	#[yaserde(rename = "ExternalAccessAllowed", prefix = "tns", default)]
	pub external_access_allowed: bool, 
	#[yaserde(rename = "AnonymousAccessAllowed", prefix = "tns", default)]
	pub anonymous_access_allowed: bool, 
	#[yaserde(rename = "CanModifyPermissions", prefix = "tns", default)]
	pub can_modify_permissions: bool, 
	#[yaserde(rename = "IsDefault", prefix = "tns", default)]
	pub is_default: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ServerVersionInfo",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct ServerVersionInfo {
	#[yaserde(rename = "MajorVersion", prefix = "tns", default)]
	pub major_version: Option<i32>, 
	#[yaserde(rename = "MinorVersion", prefix = "tns", default)]
	pub minor_version: Option<i32>, 
	#[yaserde(rename = "MajorBuildNumber", prefix = "tns", default)]
	pub major_build_number: Option<i32>, 
	#[yaserde(rename = "MinorBuildNumber", prefix = "tns", default)]
	pub minor_build_number: Option<i32>, 
	#[yaserde(rename = "Version", prefix = "tns", default)]
	pub version: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfFileExtension",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct ArrayOfFileExtension {
	#[yaserde(rename = "FileExtension", prefix = "tns", default)]
	pub file_extension: Vec<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetDomainSettingsRequestMessage",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "tns",
)]
pub struct GetDomainSettingsRequestMessage {
	#[yaserde(rename = "Request", prefix = "tns", default)]
	pub request: Option<GetDomainSettingsRequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetDomainSettingsRequest",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct GetDomainSettingsRequest {
	#[yaserde(flatten, default)]
	pub autodiscover_request: AutodiscoverRequest, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Domains", prefix = "tns", default)]
	pub domains: Option<Domains>, 
	#[yaserde(rename = "RequestedSettings", prefix = "tns", default)]
	pub requested_settings: Option<RequestedSettings>, 
	#[yaserde(rename = "RequestedVersion", prefix = "tns", default)]
	pub requested_version: Option<ExchangeVersion>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Domains",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct Domains {
	#[yaserde(rename = "Domain", prefix = "tns", default)]
	pub domain: Vec<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetDomainSettingsResponseMessage",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "tns",
)]
pub struct GetDomainSettingsResponseMessage {
	#[yaserde(rename = "Response", prefix = "tns", default)]
	pub response: Option<GetDomainSettingsResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetDomainSettingsResponse",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct GetDomainSettingsResponse {
	#[yaserde(flatten, default)]
	pub autodiscover_response: AutodiscoverResponse, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "DomainResponses", prefix = "tns", default)]
	pub domain_responses: Option<ArrayOfDomainResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfDomainResponse",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct ArrayOfDomainResponse {
	#[yaserde(rename = "DomainResponse", prefix = "tns", default)]
	pub domain_response: Vec<DomainResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DomainResponse",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct DomainResponse {
	#[yaserde(flatten, default)]
	pub autodiscover_response: AutodiscoverResponse, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "DomainSettingErrors", prefix = "tns", default)]
	pub domain_setting_errors: Option<DomainSettingErrors>, 
	#[yaserde(rename = "DomainSettings", prefix = "tns", default)]
	pub domain_settings: Option<DomainSettings>, 
	#[yaserde(rename = "RedirectTarget", prefix = "tns", default)]
	pub redirect_target: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DomainSettingErrors",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct DomainSettingErrors {
	#[yaserde(rename = "DomainSettingError", prefix = "tns", default)]
	pub domain_setting_error: Vec<DomainSettingError>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DomainSettingError",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct DomainSettingError {
	#[yaserde(rename = "ErrorCode", prefix = "tns", default)]
	pub error_code: ErrorCode, 
	#[yaserde(rename = "ErrorMessage", prefix = "tns", default)]
	pub error_message: Option<String>, 
	#[yaserde(rename = "SettingName", prefix = "tns", default)]
	pub setting_name: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DomainSettings",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct DomainSettings {
	#[yaserde(rename = "DomainSetting", prefix = "tns", default)]
	pub domain_setting: Vec<DomainSetting>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DomainSetting",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct DomainSetting {
	#[yaserde(rename = "Name", prefix = "tns", default)]
	pub name: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DomainStringSetting",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct DomainStringSetting {
	#[yaserde(flatten, default)]
	pub domain_setting: DomainSetting, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Value", prefix = "tns", default)]
	pub value: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetFederationInformationRequestMessage",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "tns",
)]
pub struct GetFederationInformationRequestMessage {
	#[yaserde(rename = "Request", prefix = "tns", default)]
	pub request: Option<GetFederationInformationRequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetFederationInformationRequest",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct GetFederationInformationRequest {
	#[yaserde(flatten, default)]
	pub autodiscover_request: AutodiscoverRequest, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Domain", prefix = "tns", default)]
	pub domain: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetFederationInformationResponseMessage",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "tns",
)]
pub struct GetFederationInformationResponseMessage {
	#[yaserde(rename = "Response", prefix = "tns", default)]
	pub response: Option<GetFederationInformationResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetFederationInformationResponse",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct GetFederationInformationResponse {
	#[yaserde(flatten, default)]
	pub autodiscover_response: AutodiscoverResponse, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ApplicationUri", prefix = "tns", default)]
	pub application_uri: Option<String>, 
	#[yaserde(rename = "TokenIssuers", prefix = "tns", default)]
	pub token_issuers: Option<TokenIssuers>, 
	#[yaserde(rename = "Domains", prefix = "tns", default)]
	pub domains: Option<Domains>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "TokenIssuers",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct TokenIssuers {
	#[yaserde(rename = "TokenIssuer", prefix = "tns", default)]
	pub token_issuer: Vec<TokenIssuer>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "TokenIssuer",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct TokenIssuer {
	#[yaserde(rename = "Uri", prefix = "tns", default)]
	pub uri: Option<String>, 
	#[yaserde(rename = "Endpoint", prefix = "tns", default)]
	pub endpoint: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetOrganizationRelationshipSettingsRequestMessage",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "tns",
)]
pub struct GetOrganizationRelationshipSettingsRequestMessage {
	#[yaserde(rename = "Request", prefix = "tns", default)]
	pub request: Option<GetOrganizationRelationshipSettingsRequest>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetOrganizationRelationshipSettingsRequest",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct GetOrganizationRelationshipSettingsRequest {
	#[yaserde(flatten, default)]
	pub autodiscover_request: AutodiscoverRequest, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Domains", prefix = "tns", default)]
	pub domains: Option<Domains>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetOrganizationRelationshipSettingsResponseMessage",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "tns",
)]
pub struct GetOrganizationRelationshipSettingsResponseMessage {
	#[yaserde(rename = "Response", prefix = "tns", default)]
	pub response: Option<GetOrganizationRelationshipSettingsResponse>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetOrganizationRelationshipSettingsResponse",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct GetOrganizationRelationshipSettingsResponse {
	#[yaserde(flatten, default)]
	pub autodiscover_response: AutodiscoverResponse, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "OrganizationRelationshipSettingsCollection", prefix = "tns", default)]
	pub organization_relationship_settings_collection: Option<OrganizationRelationshipSettingsCollection>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "OrganizationRelationshipSettingsCollection",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct OrganizationRelationshipSettingsCollection {
	#[yaserde(rename = "OrganizationRelationshipSettings", prefix = "tns", default)]
	pub organization_relationship_settings: Vec<OrganizationRelationshipSettings>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "OrganizationRelationshipSettings",
	namespace = "tns: http://schemas.microsoft.com/exchange/2010/Autodiscover",
	prefix = "tns",
)]
pub struct OrganizationRelationshipSettings {
	#[yaserde(rename = "DeliveryReportEnabled", prefix = "tns", default)]
	pub delivery_report_enabled: bool, 
	#[yaserde(rename = "DomainNames", prefix = "tns", default)]
	pub domain_names: Option<Domains>, 
	#[yaserde(rename = "FreeBusyAccessEnabled", prefix = "tns", default)]
	pub free_busy_access_enabled: bool, 
	#[yaserde(rename = "FreeBusyAccessLevel", prefix = "tns", default)]
	pub free_busy_access_level: Option<String>, 
	#[yaserde(rename = "MailTipsAccessEnabled", prefix = "tns", default)]
	pub mail_tips_access_enabled: bool, 
	#[yaserde(rename = "MailTipsAccessLevel", prefix = "tns", default)]
	pub mail_tips_access_level: Option<String>, 
	#[yaserde(rename = "MailboxMoveEnabled", prefix = "tns", default)]
	pub mailbox_move_enabled: bool, 
	#[yaserde(rename = "Name", prefix = "tns", default)]
	pub name: Option<String>, 
	#[yaserde(rename = "TargetApplicationUri", prefix = "tns", default)]
	pub target_application_uri: Option<String>, 
	#[yaserde(rename = "TargetAutodiscoverEpr", prefix = "tns", default)]
	pub target_autodiscover_epr: Option<String>, 
	#[yaserde(rename = "TargetSharingEpr", prefix = "tns", default)]
	pub target_sharing_epr: Option<String>, 
}
}

pub mod ports {
use yaserde::{YaSerialize, YaDeserialize};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;
            }

pub mod bindings {
use yaserde::{YaSerialize, YaDeserialize};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;
            }

pub mod services {
use yaserde::{YaSerialize, YaDeserialize};
            use yaserde::de::from_str;
            use async_trait::async_trait;
            use yaserde::ser::to_string;
            use super::*;
            }

