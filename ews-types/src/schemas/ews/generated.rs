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
	rename = "MailboxCultureType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MailboxCultureType {
}
pub type MailboxCulture = MailboxCultureType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "InstalledAppType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct InstalledAppType {
	#[yaserde(rename = "Id", prefix = "nsi1", default)]
	pub id: Option<String>, 
	#[yaserde(rename = "MarketplaceAssetId", prefix = "nsi1", default)]
	pub marketplace_asset_id: Option<String>, 
	#[yaserde(rename = "Enabled", prefix = "nsi1", default)]
	pub enabled: Option<bool>, 
	#[yaserde(rename = "ConsentState", prefix = "nsi1", default)]
	pub consent_state: Option<String>, 
	#[yaserde(rename = "Type", prefix = "nsi1", default)]
	pub rs_type: Option<String>, 
	#[yaserde(rename = "LicenseStatus", prefix = "nsi1", default)]
	pub license_status: Option<String>, 
	#[yaserde(rename = "TrialExpirationDate", prefix = "nsi1", default)]
	pub trial_expiration_date: Option<String>, 
	#[yaserde(rename = "ProviderName", prefix = "nsi1", default)]
	pub provider_name: Option<String>, 
	#[yaserde(rename = "IconUrl", prefix = "nsi1", default)]
	pub icon_url: Option<String>, 
	#[yaserde(rename = "HighResolutionIconUrl", prefix = "nsi1", default)]
	pub high_resolution_icon_url: Option<String>, 
	#[yaserde(rename = "DisplayName", prefix = "nsi1", default)]
	pub display_name: Option<String>, 
	#[yaserde(rename = "Description", prefix = "nsi1", default)]
	pub description: Option<String>, 
	#[yaserde(rename = "Requirements", prefix = "nsi1", default)]
	pub requirements: Option<String>, 
	#[yaserde(rename = "Version", prefix = "nsi1", default)]
	pub version: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfRoleType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfRoleType {
	#[yaserde(rename = "Role", prefix = "nsi1", default)]
	pub role: Vec<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ManagementRoleType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ManagementRoleType {
	#[yaserde(rename = "UserRoles", prefix = "nsi1", default)]
	pub user_roles: Option<NonEmptyArrayOfRoleType>, 
	#[yaserde(rename = "ApplicationRoles", prefix = "nsi1", default)]
	pub application_roles: Option<NonEmptyArrayOfRoleType>, 
}
pub type ManagementRole = ManagementRoleType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SidAndAttributesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SidAndAttributesType {
#[yaserde(rename="Attributes", attribute)]
pub attributes: u32,
	#[yaserde(rename = "SecurityIdentifier", prefix = "nsi1", default)]
	pub security_identifier: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfGroupIdentifiersType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfGroupIdentifiersType {
	#[yaserde(rename = "GroupIdentifier", prefix = "nsi1", default)]
	pub group_identifier: Vec<SidAndAttributesType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfRestrictedGroupIdentifiersType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfRestrictedGroupIdentifiersType {
	#[yaserde(rename = "RestrictedGroupIdentifier", prefix = "nsi1", default)]
	pub restricted_group_identifier: Vec<SidAndAttributesType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SerializedSecurityContextType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SerializedSecurityContextType {
	#[yaserde(rename = "UserSid", prefix = "nsi1", default)]
	pub user_sid: String, 
	#[yaserde(rename = "GroupSids", prefix = "nsi1", default)]
	pub group_sids: Option<NonEmptyArrayOfGroupIdentifiersType>, 
	#[yaserde(rename = "RestrictedGroupSids", prefix = "nsi1", default)]
	pub restricted_group_sids: Option<NonEmptyArrayOfRestrictedGroupIdentifiersType>, 
	#[yaserde(rename = "PrimarySmtpAddress", prefix = "nsi1", default)]
	pub primary_smtp_address: Option<String>, 
}
pub type SerializedSecurityContext = SerializedSecurityContextType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ConnectingSIDType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ConnectingSIDType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PrincipalNameType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PrincipalNameType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SIDType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct Sidtype {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PrimarySmtpAddressType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PrimarySmtpAddressType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SmtpAddressType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SmtpAddressType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ExchangeImpersonationType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ExchangeImpersonationType {
	#[yaserde(rename = "ConnectingSID", prefix = "nsi1", default)]
	pub connecting_sid: ConnectingSIDType, 
}
pub type ExchangeImpersonation = ExchangeImpersonationType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SpecialLogonTypeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SpecialLogonTypeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "OpenAsAdminOrSystemServiceType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct OpenAsAdminOrSystemServiceType {
#[yaserde(rename="LogonType", attribute)]
pub logon_type: SpecialLogonTypeType,
#[yaserde(rename="BudgetType", attribute)]
pub budget_type: Option<i32>,
	#[yaserde(rename = "ConnectingSID", prefix = "nsi1", default)]
	pub connecting_sid: ConnectingSIDType, 
}
pub type OpenAsAdminOrSystemService = OpenAsAdminOrSystemServiceType;

pub type SharingSecurity = EncryptedDataContainerType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ExchangeVersionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ExchangeVersionType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ProxySecurityContextType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ProxySecurityContextType {
}
pub type ProxySecurityContext = ProxySecurityContextType;

pub type ProxySuggesterSid = ProxySecurityContextType;

pub type ProxyPartnerToken = ProxySecurityContextType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ServerVersionInfo",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct ServerVersionInfo {
#[yaserde(rename="MajorVersion", attribute)]
pub major_version: Option<i32>,
#[yaserde(rename="MinorVersion", attribute)]
pub minor_version: Option<i32>,
#[yaserde(rename="MajorBuildNumber", attribute)]
pub major_build_number: Option<i32>,
#[yaserde(rename="MinorBuildNumber", attribute)]
pub minor_build_number: Option<i32>,
#[yaserde(rename="Version", attribute)]
pub version: Option<String>,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RequestServerVersion",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
	prefix = "nsi1",
)]
pub struct RequestServerVersion {
#[yaserde(rename="Version", attribute)]
pub version: ExchangeVersionType,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DateTimePrecisionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DateTimePrecisionType {
	#[yaserde(text, default)]
	pub body: String, 
}
pub type DateTimePrecision = DateTimePrecisionType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyStringType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyStringType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BaseEmailAddressType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct BaseEmailAddressType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MailboxTypeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MailboxTypeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EmailAddressType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct EmailAddressType {
	#[yaserde(flatten, default)]
	pub base_email_address_type: BaseEmailAddressType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Name", prefix = "nsi1", default)]
	pub name: Option<String>, 
	#[yaserde(rename = "EmailAddress", prefix = "nsi1", default)]
	pub email_address: Option<NonEmptyStringType>, 
	#[yaserde(rename = "RoutingType", prefix = "nsi1", default)]
	pub routing_type: Option<NonEmptyStringType>, 
	#[yaserde(rename = "MailboxType", prefix = "nsi1", default)]
	pub mailbox_type: Option<MailboxTypeType>, 
	#[yaserde(rename = "ItemId", prefix = "nsi1", default)]
	pub item_id: Option<ItemIdType>, 
	#[yaserde(rename = "OriginalDisplayName", prefix = "nsi1", default)]
	pub original_display_name: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EmailAddressExtendedType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct EmailAddressExtendedType {
	#[yaserde(flatten, default)]
	pub email_address_type: EmailAddressType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ExternalObjectId", prefix = "nsi1", default)]
	pub external_object_id: Option<String>, 
	#[yaserde(rename = "PrimaryEmailAddress", prefix = "nsi1", default)]
	pub primary_email_address: Option<NonEmptyStringType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfEmailAddressesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfEmailAddressesType {
	#[yaserde(rename = "Address", prefix = "nsi1", default)]
	pub address: Vec<EmailAddressType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DirectoryEntryType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DirectoryEntryType {
	#[yaserde(rename = "Id", prefix = "nsi1", default)]
	pub id: Option<EmailAddressType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RoomType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RoomType {
	#[yaserde(flatten, default)]
	pub directory_entry_type: DirectoryEntryType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfRoomsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfRoomsType {
	#[yaserde(rename = "Room", prefix = "nsi1", default)]
	pub room: Vec<RoomType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "TimeSlot",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct TimeSlot {
	#[yaserde(rename = "StartTime", prefix = "nsi1", default)]
	pub start_time: String, 
	#[yaserde(rename = "DurationInMinutes", prefix = "nsi1", default)]
	pub duration_in_minutes: f64, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MeetingLocation",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MeetingLocation {
	#[yaserde(rename = "EmailAddress", prefix = "nsi1", default)]
	pub email_address: String, 
	#[yaserde(rename = "DisplayName", prefix = "nsi1", default)]
	pub display_name: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ActivityDomainType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ActivityDomainType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ReminderGroupType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ReminderGroupType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ReminderType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ReminderType {
	#[yaserde(rename = "Subject", prefix = "nsi1", default)]
	pub subject: String, 
	#[yaserde(rename = "Location", prefix = "nsi1", default)]
	pub location: Option<String>, 
	#[yaserde(rename = "ReminderTime", prefix = "nsi1", default)]
	pub reminder_time: String, 
	#[yaserde(rename = "StartDate", prefix = "nsi1", default)]
	pub start_date: String, 
	#[yaserde(rename = "EndDate", prefix = "nsi1", default)]
	pub end_date: String, 
	#[yaserde(rename = "ItemId", prefix = "nsi1", default)]
	pub item_id: ItemIdType, 
	#[yaserde(rename = "RecurringMasterItemId", prefix = "nsi1", default)]
	pub recurring_master_item_id: Option<ItemIdType>, 
	#[yaserde(rename = "ReminderGroup", prefix = "nsi1", default)]
	pub reminder_group: Option<ReminderGroupType>, 
	#[yaserde(rename = "UID", prefix = "nsi1", default)]
	pub uid: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfRemindersType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfRemindersType {
	#[yaserde(rename = "Reminder", prefix = "nsi1", default)]
	pub reminder: Vec<ReminderType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfRecipientsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfRecipientsType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SingleRecipientType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SingleRecipientType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UnindexedFieldURIType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct UnindexedFieldURIType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DictionaryURIType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DictionaryURIType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ExceptionPropertyURIType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ExceptionPropertyURIType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GuidType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct GuidType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DistinguishedPropertySetType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DistinguishedPropertySetType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MapiPropertyTypeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MapiPropertyTypeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BasePathToElementType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct BasePathToElementType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PathToUnindexedFieldType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PathToUnindexedFieldType {
	#[yaserde(flatten, default)]
	pub base_path_to_element_type: BasePathToElementType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PathToIndexedFieldType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PathToIndexedFieldType {
	#[yaserde(flatten, default)]
	pub base_path_to_element_type: BasePathToElementType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PathToExceptionFieldType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PathToExceptionFieldType {
	#[yaserde(flatten, default)]
	pub base_path_to_element_type: BasePathToElementType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PropertyTagType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PropertyTagType {
	#[yaserde(flatten, default)]
	pub body: Box<PropertyTagType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PathToExtendedFieldType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PathToExtendedFieldType {
	#[yaserde(flatten, default)]
	pub base_path_to_element_type: BasePathToElementType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type Path = BasePathToElementType;

pub type FieldURI = PathToUnindexedFieldType;

pub type IndexedFieldURI = PathToIndexedFieldType;

pub type ExtendedFieldURI = PathToExtendedFieldType;

pub type ExceptionFieldURI = PathToExceptionFieldType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfPathsToElementType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfPathsToElementType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfPropertyValuesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfPropertyValuesType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfExtendedPropertyType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfExtendedPropertyType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ExtendedPropertyType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ExtendedPropertyType {
	#[yaserde(rename = "ExtendedFieldURI", prefix = "nsi1", default)]
	pub extended_field_uri: PathToExtendedFieldType, 
	#[yaserde(rename = "Value", prefix = "nsi1", default)]
	pub value: String, 
	#[yaserde(rename = "Values", prefix = "nsi1", default)]
	pub values: NonEmptyArrayOfPropertyValuesType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FolderQueryTraversalType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FolderQueryTraversalType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchFolderTraversalType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SearchFolderTraversalType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ItemQueryTraversalType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ItemQueryTraversalType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ConversationQueryTraversalType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ConversationQueryTraversalType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DefaultShapeNamesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DefaultShapeNamesType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BodyTypeResponseType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct BodyTypeResponseType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FolderResponseShapeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FolderResponseShapeType {
	#[yaserde(rename = "BaseShape", prefix = "nsi1", default)]
	pub base_shape: DefaultShapeNamesType, 
	#[yaserde(rename = "AdditionalProperties", prefix = "nsi1", default)]
	pub additional_properties: Option<NonEmptyArrayOfPathsToElementType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ItemResponseShapeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ItemResponseShapeType {
	#[yaserde(rename = "BaseShape", prefix = "nsi1", default)]
	pub base_shape: DefaultShapeNamesType, 
	#[yaserde(rename = "IncludeMimeContent", prefix = "nsi1", default)]
	pub include_mime_content: Option<bool>, 
	#[yaserde(rename = "BodyType", prefix = "nsi1", default)]
	pub body_type: Option<BodyTypeResponseType>, 
	#[yaserde(rename = "UniqueBodyType", prefix = "nsi1", default)]
	pub unique_body_type: Option<BodyTypeResponseType>, 
	#[yaserde(rename = "NormalizedBodyType", prefix = "nsi1", default)]
	pub normalized_body_type: Option<BodyTypeResponseType>, 
	#[yaserde(rename = "FilterHtmlContent", prefix = "nsi1", default)]
	pub filter_html_content: Option<bool>, 
	#[yaserde(rename = "ConvertHtmlCodePageToUTF8", prefix = "nsi1", default)]
	pub convert_html_code_page_to_utf8: Option<bool>, 
	#[yaserde(rename = "InlineImageUrlTemplate", prefix = "nsi1", default)]
	pub inline_image_url_template: Option<String>, 
	#[yaserde(rename = "BlockExternalImages", prefix = "nsi1", default)]
	pub block_external_images: Option<bool>, 
	#[yaserde(rename = "AddBlankTargetToLinks", prefix = "nsi1", default)]
	pub add_blank_target_to_links: Option<bool>, 
	#[yaserde(rename = "MaximumBodySize", prefix = "nsi1", default)]
	pub maximum_body_size: Option<i32>, 
	#[yaserde(rename = "AdditionalProperties", prefix = "nsi1", default)]
	pub additional_properties: Option<NonEmptyArrayOfPathsToElementType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AttachmentResponseShapeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AttachmentResponseShapeType {
	#[yaserde(rename = "IncludeMimeContent", prefix = "nsi1", default)]
	pub include_mime_content: Option<bool>, 
	#[yaserde(rename = "BodyType", prefix = "nsi1", default)]
	pub body_type: Option<BodyTypeResponseType>, 
	#[yaserde(rename = "FilterHtmlContent", prefix = "nsi1", default)]
	pub filter_html_content: Option<bool>, 
	#[yaserde(rename = "AdditionalProperties", prefix = "nsi1", default)]
	pub additional_properties: Option<NonEmptyArrayOfPathsToElementType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ConversationResponseShapeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ConversationResponseShapeType {
	#[yaserde(rename = "BaseShape", prefix = "nsi1", default)]
	pub base_shape: DefaultShapeNamesType, 
	#[yaserde(rename = "AdditionalProperties", prefix = "nsi1", default)]
	pub additional_properties: Option<NonEmptyArrayOfPathsToElementType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PersonaResponseShapeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PersonaResponseShapeType {
	#[yaserde(rename = "BaseShape", prefix = "nsi1", default)]
	pub base_shape: DefaultShapeNamesType, 
	#[yaserde(rename = "AdditionalProperties", prefix = "nsi1", default)]
	pub additional_properties: Option<NonEmptyArrayOfPathsToElementType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DisposalType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DisposalType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ConflictResolutionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ConflictResolutionType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ClientAccessTokenTypeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ClientAccessTokenTypeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfClientAccessTokenRequestsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfClientAccessTokenRequestsType {
	#[yaserde(rename = "TokenRequest", prefix = "nsi1", default)]
	pub token_request: Vec<ClientAccessTokenRequestType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ClientAccessTokenRequestType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ClientAccessTokenRequestType {
	#[yaserde(rename = "Id", prefix = "nsi1", default)]
	pub id: String, 
	#[yaserde(rename = "TokenType", prefix = "nsi1", default)]
	pub token_type: ClientAccessTokenTypeType, 
	#[yaserde(rename = "Scope", prefix = "nsi1", default)]
	pub scope: Option<String>, 
	#[yaserde(rename = "ResourceUri", prefix = "nsi1", default)]
	pub resource_uri: Option<String>, 
	#[yaserde(rename = "IdentityAndEwsTokenId", prefix = "nsi1", default)]
	pub identity_and_ews_token_id: Option<String>, 
	#[yaserde(rename = "RequestedCapabilities", prefix = "nsi1", default)]
	pub requested_capabilities: Option<String>, 
	#[yaserde(rename = "SupportsSharedFolders", prefix = "nsi1", default)]
	pub supports_shared_folders: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ClientAccessTokenType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ClientAccessTokenType {
	#[yaserde(rename = "Id", prefix = "nsi1", default)]
	pub id: String, 
	#[yaserde(rename = "TokenType", prefix = "nsi1", default)]
	pub token_type: ClientAccessTokenTypeType, 
	#[yaserde(rename = "TokenValue", prefix = "nsi1", default)]
	pub token_value: String, 
	#[yaserde(rename = "TTL", prefix = "nsi1", default)]
	pub ttl: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ResponseClassType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ResponseClassType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ChangeDescriptionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ChangeDescriptionType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ItemChangeDescriptionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ItemChangeDescriptionType {
	#[yaserde(flatten, default)]
	pub change_description_type: ChangeDescriptionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FolderChangeDescriptionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FolderChangeDescriptionType {
	#[yaserde(flatten, default)]
	pub change_description_type: ChangeDescriptionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SetItemFieldType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SetItemFieldType {
	#[yaserde(flatten, default)]
	pub item_change_description_type: ItemChangeDescriptionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SetFolderFieldType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SetFolderFieldType {
	#[yaserde(flatten, default)]
	pub folder_change_description_type: FolderChangeDescriptionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeleteItemFieldType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DeleteItemFieldType {
	#[yaserde(flatten, default)]
	pub item_change_description_type: ItemChangeDescriptionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeleteFolderFieldType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DeleteFolderFieldType {
	#[yaserde(flatten, default)]
	pub folder_change_description_type: FolderChangeDescriptionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AppendToItemFieldType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AppendToItemFieldType {
	#[yaserde(flatten, default)]
	pub item_change_description_type: ItemChangeDescriptionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AppendToFolderFieldType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AppendToFolderFieldType {
	#[yaserde(flatten, default)]
	pub folder_change_description_type: FolderChangeDescriptionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Folder", prefix = "nsi1", default)]
	pub folder: FolderType, 
	#[yaserde(rename = "CalendarFolder", prefix = "nsi1", default)]
	pub calendar_folder: CalendarFolderType, 
	#[yaserde(rename = "ContactsFolder", prefix = "nsi1", default)]
	pub contacts_folder: ContactsFolderType, 
	#[yaserde(rename = "SearchFolder", prefix = "nsi1", default)]
	pub search_folder: SearchFolderType, 
	#[yaserde(rename = "TasksFolder", prefix = "nsi1", default)]
	pub tasks_folder: TasksFolderType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfItemChangeDescriptionsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfItemChangeDescriptionsType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfFolderChangeDescriptionsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfFolderChangeDescriptionsType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ItemChangeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ItemChangeType {
	#[yaserde(rename = "ItemId", prefix = "nsi1", default)]
	pub item_id: ItemIdType, 
	#[yaserde(rename = "OccurrenceItemId", prefix = "nsi1", default)]
	pub occurrence_item_id: OccurrenceItemIdType, 
	#[yaserde(rename = "RecurringMasterItemId", prefix = "nsi1", default)]
	pub recurring_master_item_id: RecurringMasterItemIdType, 
	#[yaserde(rename = "Updates", prefix = "nsi1", default)]
	pub updates: NonEmptyArrayOfItemChangeDescriptionsType, 
	#[yaserde(rename = "CalendarActivityData", prefix = "nsi1", default)]
	pub calendar_activity_data: Option<CalendarActivityDataType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfItemChangesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfItemChangesType {
	#[yaserde(rename = "ItemChange", prefix = "nsi1", default)]
	pub item_change: Vec<ItemChangeType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "InternetHeaderType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct InternetHeaderType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfInternetHeadersType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfInternetHeadersType {
	#[yaserde(rename = "InternetMessageHeader", prefix = "nsi1", default)]
	pub internet_message_header: Vec<InternetHeaderType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfPredictedActionReasonType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfPredictedActionReasonType {
	#[yaserde(rename = "PredictedActionReason", prefix = "nsi1", default)]
	pub predicted_action_reason: Vec<PredictedActionReasonType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RequestAttachmentIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RequestAttachmentIdType {
	#[yaserde(flatten, default)]
	pub base_item_id_type: BaseItemIdType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AttachmentIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AttachmentIdType {
	#[yaserde(flatten, default)]
	pub request_attachment_id_type: RequestAttachmentIdType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RootItemIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RootItemIdType {
	#[yaserde(flatten, default)]
	pub base_item_id_type: BaseItemIdType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfRequestAttachmentIdsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfRequestAttachmentIdsType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AttachmentType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AttachmentType {
	#[yaserde(rename = "AttachmentId", prefix = "nsi1", default)]
	pub attachment_id: Option<AttachmentIdType>, 
	#[yaserde(rename = "Name", prefix = "nsi1", default)]
	pub name: Option<String>, 
	#[yaserde(rename = "ContentType", prefix = "nsi1", default)]
	pub content_type: Option<String>, 
	#[yaserde(rename = "ContentId", prefix = "nsi1", default)]
	pub content_id: Option<String>, 
	#[yaserde(rename = "ContentLocation", prefix = "nsi1", default)]
	pub content_location: Option<String>, 
	#[yaserde(rename = "AttachmentOriginalUrl", prefix = "nsi1", default)]
	pub attachment_original_url: Option<String>, 
	#[yaserde(rename = "Size", prefix = "nsi1", default)]
	pub size: Option<i32>, 
	#[yaserde(rename = "LastModifiedTime", prefix = "nsi1", default)]
	pub last_modified_time: Option<String>, 
	#[yaserde(rename = "IsInline", prefix = "nsi1", default)]
	pub is_inline: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ItemAttachmentType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ItemAttachmentType {
	#[yaserde(flatten, default)]
	pub attachment_type: AttachmentType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SyncFolderItemsCreateOrUpdateType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SyncFolderItemsCreateOrUpdateType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FileAttachmentType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FileAttachmentType {
	#[yaserde(flatten, default)]
	pub attachment_type: AttachmentType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "IsContactPhoto", prefix = "nsi1", default)]
	pub is_contact_photo: Option<bool>, 
	#[yaserde(rename = "Content", prefix = "nsi1", default)]
	pub content: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ReferenceAttachmentType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ReferenceAttachmentType {
	#[yaserde(flatten, default)]
	pub attachment_type: AttachmentType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "AttachLongPathName", prefix = "nsi1", default)]
	pub attach_long_path_name: Option<String>, 
	#[yaserde(rename = "ProviderType", prefix = "nsi1", default)]
	pub provider_type: Option<String>, 
	#[yaserde(rename = "ProviderEndpointUrl", prefix = "nsi1", default)]
	pub provider_endpoint_url: Option<String>, 
	#[yaserde(rename = "AttachmentThumbnailUrl", prefix = "nsi1", default)]
	pub attachment_thumbnail_url: Option<String>, 
	#[yaserde(rename = "AttachmentPreviewUrl", prefix = "nsi1", default)]
	pub attachment_preview_url: Option<String>, 
	#[yaserde(rename = "PermissionType", prefix = "nsi1", default)]
	pub permission_type: Option<i32>, 
	#[yaserde(rename = "OriginalPermissionType", prefix = "nsi1", default)]
	pub original_permission_type: Option<i32>, 
	#[yaserde(rename = "AttachmentIsFolder", prefix = "nsi1", default)]
	pub attachment_is_folder: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfAttachmentsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfAttachmentsType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfAttachmentsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfAttachmentsType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SensitivityChoicesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SensitivityChoicesType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ImportanceChoicesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ImportanceChoicesType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BodyTypeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct BodyTypeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BodyType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct BodyType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UniqueBodyType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct UniqueBodyType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NormalizedBodyType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NormalizedBodyType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BaseFolderIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct BaseFolderIdType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FolderClassType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FolderClassType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DistinguishedFolderIdNameType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DistinguishedFolderIdNameType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DistinguishedFolderIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DistinguishedFolderIdType {
	#[yaserde(flatten, default)]
	pub base_folder_id_type: BaseFolderIdType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Mailbox", prefix = "nsi1", default)]
	pub mailbox: Option<EmailAddressType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FolderIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FolderIdType {
	#[yaserde(flatten, default)]
	pub base_folder_id_type: BaseFolderIdType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AddressListIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AddressListIdType {
	#[yaserde(flatten, default)]
	pub base_folder_id_type: BaseFolderIdType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfDistinguishedFolderIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfDistinguishedFolderIdType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfBaseFolderIdsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfBaseFolderIdsType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "TargetFolderIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct TargetFolderIdType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfUploadItemsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfUploadItemsType {
	#[yaserde(rename = "Item", prefix = "nsi1", default)]
	pub item: Vec<UploadItemType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UploadItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct UploadItemType {
#[yaserde(rename="CreateAction", attribute)]
pub create_action: CreateActionType,
#[yaserde(rename="IsAssociated", attribute)]
pub is_associated: Option<bool>,
	#[yaserde(rename = "ParentFolderId", prefix = "nsi1", default)]
	pub parent_folder_id: FolderIdType, 
	#[yaserde(rename = "ItemId", prefix = "nsi1", default)]
	pub item_id: Option<ItemIdType>, 
	#[yaserde(rename = "Data", prefix = "nsi1", default)]
	pub data: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CreateActionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct CreateActionType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CompleteActionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct CompleteActionType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ViewFilterType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ViewFilterType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindFolderParentType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FindFolderParentType {
	#[yaserde(rename = "Folders", prefix = "nsi1", default)]
	pub folders: Option<ArrayOfFoldersType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BaseFolderType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct BaseFolderType {
	#[yaserde(rename = "FolderId", prefix = "nsi1", default)]
	pub folder_id: Option<FolderIdType>, 
	#[yaserde(rename = "ParentFolderId", prefix = "nsi1", default)]
	pub parent_folder_id: Option<FolderIdType>, 
	#[yaserde(rename = "FolderClass", prefix = "nsi1", default)]
	pub folder_class: Option<String>, 
	#[yaserde(rename = "DisplayName", prefix = "nsi1", default)]
	pub display_name: Option<String>, 
	#[yaserde(rename = "TotalCount", prefix = "nsi1", default)]
	pub total_count: Option<i32>, 
	#[yaserde(rename = "ChildFolderCount", prefix = "nsi1", default)]
	pub child_folder_count: Option<i32>, 
	#[yaserde(rename = "ExtendedProperty", prefix = "nsi1", default)]
	pub extended_property: Vec<ExtendedPropertyType>, 
	#[yaserde(rename = "ManagedFolderInformation", prefix = "nsi1", default)]
	pub managed_folder_information: Option<ManagedFolderInformationType>, 
	#[yaserde(rename = "EffectiveRights", prefix = "nsi1", default)]
	pub effective_rights: Option<EffectiveRightsType>, 
	#[yaserde(rename = "DistinguishedFolderId", prefix = "nsi1", default)]
	pub distinguished_folder_id: Option<DistinguishedFolderIdNameType>, 
	#[yaserde(rename = "PolicyTag", prefix = "nsi1", default)]
	pub policy_tag: Option<RetentionTagType>, 
	#[yaserde(rename = "ArchiveTag", prefix = "nsi1", default)]
	pub archive_tag: Option<RetentionTagType>, 
	#[yaserde(rename = "ReplicaList", prefix = "nsi1", default)]
	pub replica_list: Option<ArrayOfStringsType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ManagedFolderInformationType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ManagedFolderInformationType {
	#[yaserde(rename = "CanDelete", prefix = "nsi1", default)]
	pub can_delete: Option<bool>, 
	#[yaserde(rename = "CanRenameOrMove", prefix = "nsi1", default)]
	pub can_rename_or_move: Option<bool>, 
	#[yaserde(rename = "MustDisplayComment", prefix = "nsi1", default)]
	pub must_display_comment: Option<bool>, 
	#[yaserde(rename = "HasQuota", prefix = "nsi1", default)]
	pub has_quota: Option<bool>, 
	#[yaserde(rename = "IsManagedFoldersRoot", prefix = "nsi1", default)]
	pub is_managed_folders_root: Option<bool>, 
	#[yaserde(rename = "ManagedFolderId", prefix = "nsi1", default)]
	pub managed_folder_id: Option<String>, 
	#[yaserde(rename = "Comment", prefix = "nsi1", default)]
	pub comment: Option<String>, 
	#[yaserde(rename = "StorageQuota", prefix = "nsi1", default)]
	pub storage_quota: Option<i32>, 
	#[yaserde(rename = "FolderSize", prefix = "nsi1", default)]
	pub folder_size: Option<i32>, 
	#[yaserde(rename = "HomePage", prefix = "nsi1", default)]
	pub home_page: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FolderType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FolderType {
	#[yaserde(flatten, default)]
	pub base_folder_type: BaseFolderType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "PermissionSet", prefix = "nsi1", default)]
	pub permission_set: Option<PermissionSetType>, 
	#[yaserde(rename = "UnreadCount", prefix = "nsi1", default)]
	pub unread_count: Option<i32>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CalendarFolderType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct CalendarFolderType {
	#[yaserde(flatten, default)]
	pub base_folder_type: BaseFolderType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SharingEffectiveRights", prefix = "nsi1", default)]
	pub sharing_effective_rights: Option<CalendarPermissionReadAccessType>, 
	#[yaserde(rename = "PermissionSet", prefix = "nsi1", default)]
	pub permission_set: Option<CalendarPermissionSetType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ReminderActionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ReminderActionType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ReminderItemActionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ReminderItemActionType {
	#[yaserde(rename = "ActionType", prefix = "nsi1", default)]
	pub action_type: ReminderActionType, 
	#[yaserde(rename = "ItemId", prefix = "nsi1", default)]
	pub item_id: ItemIdType, 
	#[yaserde(rename = "NewReminderTime", prefix = "nsi1", default)]
	pub new_reminder_time: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfReminderItemActionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfReminderItemActionType {
	#[yaserde(rename = "ReminderItemAction", prefix = "nsi1", default)]
	pub reminder_item_action: Vec<ReminderItemActionType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ContactsFolderType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ContactsFolderType {
	#[yaserde(flatten, default)]
	pub base_folder_type: BaseFolderType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SharingEffectiveRights", prefix = "nsi1", default)]
	pub sharing_effective_rights: Option<PermissionReadAccessType>, 
	#[yaserde(rename = "PermissionSet", prefix = "nsi1", default)]
	pub permission_set: Option<PermissionSetType>, 
	#[yaserde(rename = "SourceId", prefix = "nsi1", default)]
	pub source_id: Option<String>, 
	#[yaserde(rename = "AccountName", prefix = "nsi1", default)]
	pub account_name: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchFolderType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SearchFolderType {
	#[yaserde(flatten, default)]
	pub folder_type: FolderType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SearchParameters", prefix = "nsi1", default)]
	pub search_parameters: Option<SearchParametersType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "TasksFolderType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct TasksFolderType {
	#[yaserde(flatten, default)]
	pub folder_type: FolderType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfFoldersType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfFoldersType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfFoldersType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfFoldersType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BaseItemIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct BaseItemIdType {
}
pub type BaseItemId = BaseItemIdType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DerivedItemIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DerivedItemIdType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ItemIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ItemIdType {
	#[yaserde(flatten, default)]
	pub base_item_id_type: BaseItemIdType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfBaseItemIdsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfBaseItemIdsType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfBaseItemIdsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfBaseItemIdsType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfItemIdsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfItemIdsType {
	#[yaserde(rename = "ItemId", prefix = "nsi1", default)]
	pub item_id: Vec<ItemIdType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfItemIdsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfItemIdsType {
	#[yaserde(rename = "ItemId", prefix = "nsi1", default)]
	pub item_id: Vec<ItemIdType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ItemClassType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ItemClassType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ResponseObjectCoreType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ResponseObjectCoreType {
	#[yaserde(flatten, default)]
	pub message_type: MessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ReferenceItemId", prefix = "nsi1", default)]
	pub reference_item_id: Option<ItemIdType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ResponseObjectType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ResponseObjectType {
	#[yaserde(flatten, default)]
	pub response_object_core_type: ResponseObjectCoreType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfResponseObjectsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfResponseObjectsType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FolderChangeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FolderChangeType {
	#[yaserde(rename = "FolderId", prefix = "nsi1", default)]
	pub folder_id: FolderIdType, 
	#[yaserde(rename = "DistinguishedFolderId", prefix = "nsi1", default)]
	pub distinguished_folder_id: DistinguishedFolderIdType, 
	#[yaserde(rename = "Updates", prefix = "nsi1", default)]
	pub updates: NonEmptyArrayOfFolderChangeDescriptionsType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfFolderChangesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfFolderChangesType {
	#[yaserde(rename = "FolderChange", prefix = "nsi1", default)]
	pub folder_change: Vec<FolderChangeType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "WellKnownResponseObjectType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct WellKnownResponseObjectType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SmartResponseBaseType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SmartResponseBaseType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SmartResponseType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SmartResponseType {
	#[yaserde(flatten, default)]
	pub smart_response_base_type: SmartResponseBaseType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "NewBodyContent", prefix = "nsi1", default)]
	pub new_body_content: Option<BodyType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ReplyToItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ReplyToItemType {
	#[yaserde(flatten, default)]
	pub smart_response_type: SmartResponseType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ReplyAllToItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ReplyAllToItemType {
	#[yaserde(flatten, default)]
	pub smart_response_type: SmartResponseType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "IsSpecificMessageReply", prefix = "nsi1", default)]
	pub is_specific_message_reply: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ForwardItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ForwardItemType {
	#[yaserde(flatten, default)]
	pub smart_response_type: SmartResponseType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CancelCalendarItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct CancelCalendarItemType {
	#[yaserde(flatten, default)]
	pub smart_response_type: SmartResponseType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ReferenceItemResponseType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ReferenceItemResponseType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SuppressReadReceiptType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SuppressReadReceiptType {
	#[yaserde(flatten, default)]
	pub reference_item_response_type: ReferenceItemResponseType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindItemParentType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FindItemParentType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ItemType {
	#[yaserde(rename = "MimeContent", prefix = "nsi1", default)]
	pub mime_content: Option<MimeContentType>, 
	#[yaserde(rename = "ItemId", prefix = "nsi1", default)]
	pub item_id: Option<ItemIdType>, 
	#[yaserde(rename = "ParentFolderId", prefix = "nsi1", default)]
	pub parent_folder_id: Option<FolderIdType>, 
	#[yaserde(rename = "ItemClass", prefix = "nsi1", default)]
	pub item_class: Option<ItemClassType>, 
	#[yaserde(rename = "Subject", prefix = "nsi1", default)]
	pub subject: Option<String>, 
	#[yaserde(rename = "Sensitivity", prefix = "nsi1", default)]
	pub sensitivity: Option<SensitivityChoicesType>, 
	#[yaserde(rename = "Body", prefix = "nsi1", default)]
	pub body: Option<BodyType>, 
	#[yaserde(rename = "Attachments", prefix = "nsi1", default)]
	pub attachments: Option<NonEmptyArrayOfAttachmentsType>, 
	#[yaserde(rename = "DateTimeReceived", prefix = "nsi1", default)]
	pub date_time_received: Option<String>, 
	#[yaserde(rename = "Size", prefix = "nsi1", default)]
	pub size: Option<i32>, 
	#[yaserde(rename = "Categories", prefix = "nsi1", default)]
	pub categories: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "Importance", prefix = "nsi1", default)]
	pub importance: Option<ImportanceChoicesType>, 
	#[yaserde(rename = "InReplyTo", prefix = "nsi1", default)]
	pub in_reply_to: Option<String>, 
	#[yaserde(rename = "IsSubmitted", prefix = "nsi1", default)]
	pub is_submitted: Option<bool>, 
	#[yaserde(rename = "IsDraft", prefix = "nsi1", default)]
	pub is_draft: Option<bool>, 
	#[yaserde(rename = "IsFromMe", prefix = "nsi1", default)]
	pub is_from_me: Option<bool>, 
	#[yaserde(rename = "IsResend", prefix = "nsi1", default)]
	pub is_resend: Option<bool>, 
	#[yaserde(rename = "IsUnmodified", prefix = "nsi1", default)]
	pub is_unmodified: Option<bool>, 
	#[yaserde(rename = "InternetMessageHeaders", prefix = "nsi1", default)]
	pub internet_message_headers: Option<NonEmptyArrayOfInternetHeadersType>, 
	#[yaserde(rename = "DateTimeSent", prefix = "nsi1", default)]
	pub date_time_sent: Option<String>, 
	#[yaserde(rename = "DateTimeCreated", prefix = "nsi1", default)]
	pub date_time_created: Option<String>, 
	#[yaserde(rename = "ResponseObjects", prefix = "nsi1", default)]
	pub response_objects: Option<NonEmptyArrayOfResponseObjectsType>, 
	#[yaserde(rename = "ReminderDueBy", prefix = "nsi1", default)]
	pub reminder_due_by: Option<String>, 
	#[yaserde(rename = "ReminderIsSet", prefix = "nsi1", default)]
	pub reminder_is_set: Option<bool>, 
	#[yaserde(rename = "ReminderNextTime", prefix = "nsi1", default)]
	pub reminder_next_time: Option<String>, 
	#[yaserde(rename = "ReminderMinutesBeforeStart", prefix = "nsi1", default)]
	pub reminder_minutes_before_start: Option<ReminderMinutesBeforeStartType>, 
	#[yaserde(rename = "DisplayCc", prefix = "nsi1", default)]
	pub display_cc: Option<String>, 
	#[yaserde(rename = "DisplayTo", prefix = "nsi1", default)]
	pub display_to: Option<String>, 
	#[yaserde(rename = "DisplayBcc", prefix = "nsi1", default)]
	pub display_bcc: Option<String>, 
	#[yaserde(rename = "HasAttachments", prefix = "nsi1", default)]
	pub has_attachments: Option<bool>, 
	#[yaserde(rename = "ExtendedProperty", prefix = "nsi1", default)]
	pub extended_property: Vec<ExtendedPropertyType>, 
	#[yaserde(rename = "Culture", prefix = "nsi1", default)]
	pub culture: Option<String>, 
	#[yaserde(rename = "EffectiveRights", prefix = "nsi1", default)]
	pub effective_rights: Option<EffectiveRightsType>, 
	#[yaserde(rename = "LastModifiedName", prefix = "nsi1", default)]
	pub last_modified_name: Option<String>, 
	#[yaserde(rename = "LastModifiedTime", prefix = "nsi1", default)]
	pub last_modified_time: Option<String>, 
	#[yaserde(rename = "IsAssociated", prefix = "nsi1", default)]
	pub is_associated: Option<bool>, 
	#[yaserde(rename = "WebClientReadFormQueryString", prefix = "nsi1", default)]
	pub web_client_read_form_query_string: Option<String>, 
	#[yaserde(rename = "WebClientEditFormQueryString", prefix = "nsi1", default)]
	pub web_client_edit_form_query_string: Option<String>, 
	#[yaserde(rename = "ConversationId", prefix = "nsi1", default)]
	pub conversation_id: Option<ItemIdType>, 
	#[yaserde(rename = "UniqueBody", prefix = "nsi1", default)]
	pub unique_body: Option<BodyType>, 
	#[yaserde(rename = "Flag", prefix = "nsi1", default)]
	pub flag: Option<FlagType>, 
	#[yaserde(rename = "StoreEntryId", prefix = "nsi1", default)]
	pub store_entry_id: Option<String>, 
	#[yaserde(rename = "InstanceKey", prefix = "nsi1", default)]
	pub instance_key: Option<String>, 
	#[yaserde(rename = "NormalizedBody", prefix = "nsi1", default)]
	pub normalized_body: Option<BodyType>, 
	#[yaserde(rename = "EntityExtractionResult", prefix = "nsi1", default)]
	pub entity_extraction_result: Option<EntityExtractionResultType>, 
	#[yaserde(rename = "PolicyTag", prefix = "nsi1", default)]
	pub policy_tag: Option<RetentionTagType>, 
	#[yaserde(rename = "ArchiveTag", prefix = "nsi1", default)]
	pub archive_tag: Option<RetentionTagType>, 
	#[yaserde(rename = "RetentionDate", prefix = "nsi1", default)]
	pub retention_date: Option<String>, 
	#[yaserde(rename = "Preview", prefix = "nsi1", default)]
	pub preview: Option<String>, 
	#[yaserde(rename = "RightsManagementLicenseData", prefix = "nsi1", default)]
	pub rights_management_license_data: Option<RightsManagementLicenseDataType>, 
	#[yaserde(rename = "PredictedActionReasons", prefix = "nsi1", default)]
	pub predicted_action_reasons: Option<NonEmptyArrayOfPredictedActionReasonType>, 
	#[yaserde(rename = "IsClutter", prefix = "nsi1", default)]
	pub is_clutter: Option<bool>, 
	#[yaserde(rename = "BlockStatus", prefix = "nsi1", default)]
	pub block_status: Option<bool>, 
	#[yaserde(rename = "HasBlockedImages", prefix = "nsi1", default)]
	pub has_blocked_images: Option<bool>, 
	#[yaserde(rename = "TextBody", prefix = "nsi1", default)]
	pub text_body: Option<BodyType>, 
	#[yaserde(rename = "IconIndex", prefix = "nsi1", default)]
	pub icon_index: Option<IconIndexType>, 
	#[yaserde(rename = "SearchKey", prefix = "nsi1", default)]
	pub search_key: Option<String>, 
	#[yaserde(rename = "SortKey", prefix = "nsi1", default)]
	pub sort_key: Option<i64>, 
	#[yaserde(rename = "Hashtags", prefix = "nsi1", default)]
	pub hashtags: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "Mentions", prefix = "nsi1", default)]
	pub mentions: Option<ArrayOfRecipientsType>, 
	#[yaserde(rename = "MentionedMe", prefix = "nsi1", default)]
	pub mentioned_me: Option<bool>, 
	#[yaserde(rename = "MentionsPreview", prefix = "nsi1", default)]
	pub mentions_preview: Option<MentionsPreviewType>, 
	#[yaserde(rename = "MentionsEx", prefix = "nsi1", default)]
	pub mentions_ex: Option<NonEmptyArrayOfMentionActionsType>, 
	#[yaserde(rename = "AppliedHashtags", prefix = "nsi1", default)]
	pub applied_hashtags: Option<NonEmptyArrayOfAppliedHashtagType>, 
	#[yaserde(rename = "AppliedHashtagsPreview", prefix = "nsi1", default)]
	pub applied_hashtags_preview: Option<AppliedHashtagsPreviewType>, 
	#[yaserde(rename = "Likes", prefix = "nsi1", default)]
	pub likes: Option<NonEmptyArrayOfLikeType>, 
	#[yaserde(rename = "LikesPreview", prefix = "nsi1", default)]
	pub likes_preview: Option<LikesPreviewType>, 
	#[yaserde(rename = "PendingSocialActivityTagIds", prefix = "nsi1", default)]
	pub pending_social_activity_tag_ids: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "AtAllMention", prefix = "nsi1", default)]
	pub at_all_mention: Option<bool>, 
	#[yaserde(rename = "CanDelete", prefix = "nsi1", default)]
	pub can_delete: Option<bool>, 
	#[yaserde(rename = "InferenceClassification", prefix = "nsi1", default)]
	pub inference_classification: Option<InferenceClassificationType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfItemClassType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfItemClassType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FlagType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FlagType {
	#[yaserde(rename = "FlagStatus", prefix = "nsi1", default)]
	pub flag_status: FlagStatusType, 
	#[yaserde(rename = "StartDate", prefix = "nsi1", default)]
	pub start_date: Option<String>, 
	#[yaserde(rename = "DueDate", prefix = "nsi1", default)]
	pub due_date: Option<String>, 
	#[yaserde(rename = "CompleteDate", prefix = "nsi1", default)]
	pub complete_date: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FlagStatusType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FlagStatusType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PredictedActionReasonType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PredictedActionReasonType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EntityType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct EntityType {
	#[yaserde(rename = "Position", prefix = "nsi1", default)]
	pub position: Vec<EmailPositionType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfAddressesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfAddressesType {
	#[yaserde(rename = "Address", prefix = "nsi1", default)]
	pub address: Vec<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfAddressEntitiesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfAddressEntitiesType {
	#[yaserde(rename = "AddressEntity", prefix = "nsi1", default)]
	pub address_entity: Vec<AddressEntityType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AddressEntityType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AddressEntityType {
	#[yaserde(flatten, default)]
	pub entity_type: EntityType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Address", prefix = "nsi1", default)]
	pub address: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfEmailAddressEntitiesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfEmailAddressEntitiesType {
	#[yaserde(rename = "EmailAddressEntity", prefix = "nsi1", default)]
	pub email_address_entity: Vec<EmailAddressEntityType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EmailAddressEntityType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct EmailAddressEntityType {
	#[yaserde(flatten, default)]
	pub entity_type: EntityType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "EmailAddress", prefix = "nsi1", default)]
	pub email_address: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfUrlEntitiesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfUrlEntitiesType {
	#[yaserde(rename = "UrlEntity", prefix = "nsi1", default)]
	pub url_entity: Vec<UrlEntityType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UrlEntityType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct UrlEntityType {
	#[yaserde(flatten, default)]
	pub entity_type: EntityType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Url", prefix = "nsi1", default)]
	pub url: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfMeetingSuggestionsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfMeetingSuggestionsType {
	#[yaserde(rename = "MeetingSuggestion", prefix = "nsi1", default)]
	pub meeting_suggestion: Vec<MeetingSuggestionType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MeetingSuggestionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MeetingSuggestionType {
	#[yaserde(flatten, default)]
	pub entity_type: EntityType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Attendees", prefix = "nsi1", default)]
	pub attendees: Option<ArrayOfEmailUsersType>, 
	#[yaserde(rename = "Location", prefix = "nsi1", default)]
	pub location: Option<String>, 
	#[yaserde(rename = "Subject", prefix = "nsi1", default)]
	pub subject: Option<String>, 
	#[yaserde(rename = "MeetingString", prefix = "nsi1", default)]
	pub meeting_string: Option<String>, 
	#[yaserde(rename = "StartTime", prefix = "nsi1", default)]
	pub start_time: Option<String>, 
	#[yaserde(rename = "EndTime", prefix = "nsi1", default)]
	pub end_time: Option<String>, 
	#[yaserde(rename = "TimeStringBeginIndex", prefix = "nsi1", default)]
	pub time_string_begin_index: Option<i32>, 
	#[yaserde(rename = "TimeStringLength", prefix = "nsi1", default)]
	pub time_string_length: Option<i32>, 
	#[yaserde(rename = "EntityId", prefix = "nsi1", default)]
	pub entity_id: Option<String>, 
	#[yaserde(rename = "ExtractionId", prefix = "nsi1", default)]
	pub extraction_id: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfTaskSuggestionsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfTaskSuggestionsType {
	#[yaserde(rename = "TaskSuggestion", prefix = "nsi1", default)]
	pub task_suggestion: Vec<TaskSuggestionType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfContactsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfContactsType {
	#[yaserde(rename = "Contact", prefix = "nsi1", default)]
	pub contact: Vec<ContactType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ContactType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ContactType {
	#[yaserde(flatten, default)]
	pub entity_type: EntityType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "PersonName", prefix = "nsi1", default)]
	pub person_name: Option<String>, 
	#[yaserde(rename = "BusinessName", prefix = "nsi1", default)]
	pub business_name: Option<String>, 
	#[yaserde(rename = "PhoneNumbers", prefix = "nsi1", default)]
	pub phone_numbers: Option<ArrayOfPhonesType>, 
	#[yaserde(rename = "Urls", prefix = "nsi1", default)]
	pub urls: Option<ArrayOfUrlsType>, 
	#[yaserde(rename = "EmailAddresses", prefix = "nsi1", default)]
	pub email_addresses: Option<ArrayOfExtractedEmailAddresses>, 
	#[yaserde(rename = "Addresses", prefix = "nsi1", default)]
	pub addresses: Option<ArrayOfAddressesType>, 
	#[yaserde(rename = "ContactString", prefix = "nsi1", default)]
	pub contact_string: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfUrlsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfUrlsType {
	#[yaserde(rename = "Url", prefix = "nsi1", default)]
	pub url: Vec<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfPhonesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfPhonesType {
	#[yaserde(rename = "Phone", prefix = "nsi1", default)]
	pub phone: Vec<PhoneType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PhoneType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PhoneType {
	#[yaserde(rename = "OriginalPhoneString", prefix = "nsi1", default)]
	pub original_phone_string: Option<String>, 
	#[yaserde(rename = "PhoneString", prefix = "nsi1", default)]
	pub phone_string: Option<String>, 
	#[yaserde(rename = "Type", prefix = "nsi1", default)]
	pub rs_type: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfPhoneEntitiesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfPhoneEntitiesType {
	#[yaserde(rename = "Phone", prefix = "nsi1", default)]
	pub phone: Vec<PhoneEntityType>, 
}
#[derive(Debug, Default,  YaDeserialize, Clone)]
#[yaserde(
	rename = "PhoneEntityType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PhoneEntityType {
	#[yaserde(flatten, default)]
	pub entity_type: EntityType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "OriginalPhoneString", prefix = "nsi1", default)]
	pub original_phone_string: Option<String>, 
	#[yaserde(rename = "PhoneString", prefix = "nsi1", default)]
	pub phone_string: Option<String>, 
	#[yaserde(rename = "Type", prefix = "nsi1", default)]
	pub rs_type: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EmailPositionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct EmailPositionType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfEmailUsersType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfEmailUsersType {
	#[yaserde(rename = "EmailUser", prefix = "nsi1", default)]
	pub email_user: Vec<EmailUserType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EmailUserType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct EmailUserType {
	#[yaserde(rename = "Name", prefix = "nsi1", default)]
	pub name: Option<String>, 
	#[yaserde(rename = "UserId", prefix = "nsi1", default)]
	pub user_id: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "TaskSuggestionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct TaskSuggestionType {
	#[yaserde(flatten, default)]
	pub entity_type: EntityType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "TaskString", prefix = "nsi1", default)]
	pub task_string: Option<String>, 
	#[yaserde(rename = "Assignees", prefix = "nsi1", default)]
	pub assignees: Option<ArrayOfEmailUsersType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfExtractedEmailAddresses",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfExtractedEmailAddresses {
	#[yaserde(rename = "EmailAddress", prefix = "nsi1", default)]
	pub email_address: Vec<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ParcelDeliveryEntityType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ParcelDeliveryEntityType {
	#[yaserde(rename = "Carrier", prefix = "nsi1", default)]
	pub carrier: Option<String>, 
	#[yaserde(rename = "TrackingNumber", prefix = "nsi1", default)]
	pub tracking_number: Option<String>, 
	#[yaserde(rename = "TrackingUrl", prefix = "nsi1", default)]
	pub tracking_url: Option<String>, 
	#[yaserde(rename = "ExpectedArrivalFrom", prefix = "nsi1", default)]
	pub expected_arrival_from: Option<String>, 
	#[yaserde(rename = "ExpectedArrivalUntil", prefix = "nsi1", default)]
	pub expected_arrival_until: Option<String>, 
	#[yaserde(rename = "Product", prefix = "nsi1", default)]
	pub product: Option<String>, 
	#[yaserde(rename = "ProductUrl", prefix = "nsi1", default)]
	pub product_url: Option<String>, 
	#[yaserde(rename = "ProductImage", prefix = "nsi1", default)]
	pub product_image: Option<String>, 
	#[yaserde(rename = "ProductSku", prefix = "nsi1", default)]
	pub product_sku: Option<String>, 
	#[yaserde(rename = "ProductDescription", prefix = "nsi1", default)]
	pub product_description: Option<String>, 
	#[yaserde(rename = "ProductBrand", prefix = "nsi1", default)]
	pub product_brand: Option<String>, 
	#[yaserde(rename = "ProductColor", prefix = "nsi1", default)]
	pub product_color: Option<String>, 
	#[yaserde(rename = "OrderNumber", prefix = "nsi1", default)]
	pub order_number: Option<String>, 
	#[yaserde(rename = "Seller", prefix = "nsi1", default)]
	pub seller: Option<String>, 
	#[yaserde(rename = "OrderStatus", prefix = "nsi1", default)]
	pub order_status: Option<String>, 
	#[yaserde(rename = "AddressName", prefix = "nsi1", default)]
	pub address_name: Option<String>, 
	#[yaserde(rename = "StreetAddress", prefix = "nsi1", default)]
	pub street_address: Option<String>, 
	#[yaserde(rename = "AddressLocality", prefix = "nsi1", default)]
	pub address_locality: Option<String>, 
	#[yaserde(rename = "AddressRegion", prefix = "nsi1", default)]
	pub address_region: Option<String>, 
	#[yaserde(rename = "AddressCountry", prefix = "nsi1", default)]
	pub address_country: Option<String>, 
	#[yaserde(rename = "PostalCode", prefix = "nsi1", default)]
	pub postal_code: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "InferenceClassificationType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct InferenceClassificationType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "InferenceClassificationOverrideType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct InferenceClassificationOverrideType {
	#[yaserde(rename = "Id", prefix = "nsi1", default)]
	pub id: GuidType, 
	#[yaserde(rename = "SenderSmtpAddress", prefix = "nsi1", default)]
	pub sender_smtp_address: String, 
	#[yaserde(rename = "SenderDisplayName", prefix = "nsi1", default)]
	pub sender_display_name: String, 
	#[yaserde(rename = "AlwaysClassifyAs", prefix = "nsi1", default)]
	pub always_classify_as: InferenceClassificationType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfInferenceClassificationOverridesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfInferenceClassificationOverridesType {
	#[yaserde(rename = "Override", prefix = "nsi1", default)]
	pub r#override: Vec<InferenceClassificationOverrideType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfParcelDeliveryEntitiesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfParcelDeliveryEntitiesType {
	#[yaserde(rename = "ParcelDelivery", prefix = "nsi1", default)]
	pub parcel_delivery: Vec<ParcelDeliveryEntityType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FlightEntityType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FlightEntityType {
	#[yaserde(rename = "FlightNumber", prefix = "nsi1", default)]
	pub flight_number: Option<String>, 
	#[yaserde(rename = "AirlineIataCode", prefix = "nsi1", default)]
	pub airline_iata_code: Option<String>, 
	#[yaserde(rename = "DepartureTime", prefix = "nsi1", default)]
	pub departure_time: Option<String>, 
	#[yaserde(rename = "WindowsTimeZoneName", prefix = "nsi1", default)]
	pub windows_time_zone_name: Option<String>, 
	#[yaserde(rename = "DepartureAirportIataCode", prefix = "nsi1", default)]
	pub departure_airport_iata_code: Option<String>, 
	#[yaserde(rename = "ArrivalAirportIataCode", prefix = "nsi1", default)]
	pub arrival_airport_iata_code: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfFlightsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfFlightsType {
	#[yaserde(rename = "Flight", prefix = "nsi1", default)]
	pub flight: Vec<FlightEntityType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FlightReservationEntityType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FlightReservationEntityType {
	#[yaserde(rename = "ReservationId", prefix = "nsi1", default)]
	pub reservation_id: Option<String>, 
	#[yaserde(rename = "ReservationStatus", prefix = "nsi1", default)]
	pub reservation_status: Option<String>, 
	#[yaserde(rename = "UnderName", prefix = "nsi1", default)]
	pub under_name: Option<String>, 
	#[yaserde(rename = "BrokerName", prefix = "nsi1", default)]
	pub broker_name: Option<String>, 
	#[yaserde(rename = "BrokerPhone", prefix = "nsi1", default)]
	pub broker_phone: Option<String>, 
	#[yaserde(rename = "Flights", prefix = "nsi1", default)]
	pub flights: Option<ArrayOfFlightsType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfFlightReservationsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfFlightReservationsType {
	#[yaserde(rename = "FlightReservation", prefix = "nsi1", default)]
	pub flight_reservation: Vec<FlightReservationEntityType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SenderAddInEntityType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SenderAddInEntityType {
	#[yaserde(rename = "ExtensionId", prefix = "nsi1", default)]
	pub extension_id: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfSenderAddInsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfSenderAddInsType {
	#[yaserde(rename = "Microsoft.OutlookServices.SenderApp", prefix = "nsi1", default)]
	pub microsoft_outlook_services_sender_app: Vec<SenderAddInEntityType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EntityExtractionResultType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct EntityExtractionResultType {
	#[yaserde(rename = "Addresses", prefix = "nsi1", default)]
	pub addresses: Option<ArrayOfAddressEntitiesType>, 
	#[yaserde(rename = "MeetingSuggestions", prefix = "nsi1", default)]
	pub meeting_suggestions: Option<ArrayOfMeetingSuggestionsType>, 
	#[yaserde(rename = "TaskSuggestions", prefix = "nsi1", default)]
	pub task_suggestions: Option<ArrayOfTaskSuggestionsType>, 
	#[yaserde(rename = "EmailAddresses", prefix = "nsi1", default)]
	pub email_addresses: Option<ArrayOfEmailAddressEntitiesType>, 
	#[yaserde(rename = "Contacts", prefix = "nsi1", default)]
	pub contacts: Option<ArrayOfContactsType>, 
	#[yaserde(rename = "Urls", prefix = "nsi1", default)]
	pub urls: Option<ArrayOfUrlEntitiesType>, 
	#[yaserde(rename = "PhoneNumbers", prefix = "nsi1", default)]
	pub phone_numbers: Option<ArrayOfPhoneEntitiesType>, 
	#[yaserde(rename = "ParcelDeliveries", prefix = "nsi1", default)]
	pub parcel_deliveries: Option<ArrayOfParcelDeliveryEntitiesType>, 
	#[yaserde(rename = "FlightReservations", prefix = "nsi1", default)]
	pub flight_reservations: Option<ArrayOfFlightReservationsType>, 
	#[yaserde(rename = "SenderAddIns", prefix = "nsi1", default)]
	pub sender_add_ins: Option<ArrayOfSenderAddInsType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RightsManagementLicenseDataType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RightsManagementLicenseDataType {
	#[yaserde(rename = "RightsManagedMessageDecryptionStatus", prefix = "nsi1", default)]
	pub rights_managed_message_decryption_status: Option<i32>, 
	#[yaserde(rename = "RmsTemplateId", prefix = "nsi1", default)]
	pub rms_template_id: Option<String>, 
	#[yaserde(rename = "TemplateName", prefix = "nsi1", default)]
	pub template_name: Option<String>, 
	#[yaserde(rename = "TemplateDescription", prefix = "nsi1", default)]
	pub template_description: Option<String>, 
	#[yaserde(rename = "EditAllowed", prefix = "nsi1", default)]
	pub edit_allowed: Option<bool>, 
	#[yaserde(rename = "ReplyAllowed", prefix = "nsi1", default)]
	pub reply_allowed: Option<bool>, 
	#[yaserde(rename = "ReplyAllAllowed", prefix = "nsi1", default)]
	pub reply_all_allowed: Option<bool>, 
	#[yaserde(rename = "ForwardAllowed", prefix = "nsi1", default)]
	pub forward_allowed: Option<bool>, 
	#[yaserde(rename = "ModifyRecipientsAllowed", prefix = "nsi1", default)]
	pub modify_recipients_allowed: Option<bool>, 
	#[yaserde(rename = "ExtractAllowed", prefix = "nsi1", default)]
	pub extract_allowed: Option<bool>, 
	#[yaserde(rename = "PrintAllowed", prefix = "nsi1", default)]
	pub print_allowed: Option<bool>, 
	#[yaserde(rename = "ExportAllowed", prefix = "nsi1", default)]
	pub export_allowed: Option<bool>, 
	#[yaserde(rename = "ProgrammaticAccessAllowed", prefix = "nsi1", default)]
	pub programmatic_access_allowed: Option<bool>, 
	#[yaserde(rename = "IsOwner", prefix = "nsi1", default)]
	pub is_owner: Option<bool>, 
	#[yaserde(rename = "ContentOwner", prefix = "nsi1", default)]
	pub content_owner: Option<String>, 
	#[yaserde(rename = "ContentExpiryDate", prefix = "nsi1", default)]
	pub content_expiry_date: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ConversationActionTypeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ConversationActionTypeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ConversationActionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ConversationActionType {
	#[yaserde(rename = "Action", prefix = "nsi1", default)]
	pub action: ConversationActionTypeType, 
	#[yaserde(rename = "ConversationId", prefix = "nsi1", default)]
	pub conversation_id: ItemIdType, 
	#[yaserde(rename = "ContextFolderId", prefix = "nsi1", default)]
	pub context_folder_id: Option<TargetFolderIdType>, 
	#[yaserde(rename = "ConversationLastSyncTime", prefix = "nsi1", default)]
	pub conversation_last_sync_time: Option<String>, 
	#[yaserde(rename = "ProcessRightAway", prefix = "nsi1", default)]
	pub process_right_away: Option<bool>, 
	#[yaserde(rename = "DestinationFolderId", prefix = "nsi1", default)]
	pub destination_folder_id: Option<TargetFolderIdType>, 
	#[yaserde(rename = "Categories", prefix = "nsi1", default)]
	pub categories: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "EnableAlwaysDelete", prefix = "nsi1", default)]
	pub enable_always_delete: Option<bool>, 
	#[yaserde(rename = "IsRead", prefix = "nsi1", default)]
	pub is_read: Option<bool>, 
	#[yaserde(rename = "DeleteType", prefix = "nsi1", default)]
	pub delete_type: Option<DisposalType>, 
	#[yaserde(rename = "RetentionPolicyType", prefix = "nsi1", default)]
	pub retention_policy_type: Option<RetentionType>, 
	#[yaserde(rename = "RetentionPolicyTagId", prefix = "nsi1", default)]
	pub retention_policy_tag_id: Option<String>, 
	#[yaserde(rename = "Flag", prefix = "nsi1", default)]
	pub flag: Option<FlagType>, 
	#[yaserde(rename = "SuppressReadReceipts", prefix = "nsi1", default)]
	pub suppress_read_receipts: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfApplyConversationActionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfApplyConversationActionType {
	#[yaserde(rename = "ConversationAction", prefix = "nsi1", default)]
	pub conversation_action: Vec<ConversationActionType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MailboxGuids",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MailboxGuids {
	#[yaserde(rename = "MailboxGuid", prefix = "nsi1", default)]
	pub mailbox_guid: Vec<GuidType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ConversationType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ConversationType {
	#[yaserde(rename = "ConversationId", prefix = "nsi1", default)]
	pub conversation_id: Option<ItemIdType>, 
	#[yaserde(rename = "ConversationTopic", prefix = "nsi1", default)]
	pub conversation_topic: Option<String>, 
	#[yaserde(rename = "UniqueRecipients", prefix = "nsi1", default)]
	pub unique_recipients: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "GlobalUniqueRecipients", prefix = "nsi1", default)]
	pub global_unique_recipients: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "UniqueUnreadSenders", prefix = "nsi1", default)]
	pub unique_unread_senders: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "GlobalUniqueUnreadSenders", prefix = "nsi1", default)]
	pub global_unique_unread_senders: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "UniqueSenders", prefix = "nsi1", default)]
	pub unique_senders: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "GlobalUniqueSenders", prefix = "nsi1", default)]
	pub global_unique_senders: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "LastDeliveryTime", prefix = "nsi1", default)]
	pub last_delivery_time: Option<String>, 
	#[yaserde(rename = "GlobalLastDeliveryTime", prefix = "nsi1", default)]
	pub global_last_delivery_time: Option<String>, 
	#[yaserde(rename = "Categories", prefix = "nsi1", default)]
	pub categories: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "GlobalCategories", prefix = "nsi1", default)]
	pub global_categories: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "FlagStatus", prefix = "nsi1", default)]
	pub flag_status: Option<FlagStatusType>, 
	#[yaserde(rename = "GlobalFlagStatus", prefix = "nsi1", default)]
	pub global_flag_status: Option<FlagStatusType>, 
	#[yaserde(rename = "HasAttachments", prefix = "nsi1", default)]
	pub has_attachments: Option<bool>, 
	#[yaserde(rename = "GlobalHasAttachments", prefix = "nsi1", default)]
	pub global_has_attachments: Option<bool>, 
	#[yaserde(rename = "MessageCount", prefix = "nsi1", default)]
	pub message_count: Option<i32>, 
	#[yaserde(rename = "GlobalMessageCount", prefix = "nsi1", default)]
	pub global_message_count: Option<i32>, 
	#[yaserde(rename = "UnreadCount", prefix = "nsi1", default)]
	pub unread_count: Option<i32>, 
	#[yaserde(rename = "GlobalUnreadCount", prefix = "nsi1", default)]
	pub global_unread_count: Option<i32>, 
	#[yaserde(rename = "Size", prefix = "nsi1", default)]
	pub size: Option<i32>, 
	#[yaserde(rename = "GlobalSize", prefix = "nsi1", default)]
	pub global_size: Option<i32>, 
	#[yaserde(rename = "ItemClasses", prefix = "nsi1", default)]
	pub item_classes: Option<ArrayOfItemClassType>, 
	#[yaserde(rename = "GlobalItemClasses", prefix = "nsi1", default)]
	pub global_item_classes: Option<ArrayOfItemClassType>, 
	#[yaserde(rename = "Importance", prefix = "nsi1", default)]
	pub importance: Option<ImportanceChoicesType>, 
	#[yaserde(rename = "GlobalImportance", prefix = "nsi1", default)]
	pub global_importance: Option<ImportanceChoicesType>, 
	#[yaserde(rename = "ItemIds", prefix = "nsi1", default)]
	pub item_ids: Option<NonEmptyArrayOfBaseItemIdsType>, 
	#[yaserde(rename = "GlobalItemIds", prefix = "nsi1", default)]
	pub global_item_ids: Option<NonEmptyArrayOfBaseItemIdsType>, 
	#[yaserde(rename = "LastModifiedTime", prefix = "nsi1", default)]
	pub last_modified_time: Option<String>, 
	#[yaserde(rename = "InstanceKey", prefix = "nsi1", default)]
	pub instance_key: Option<String>, 
	#[yaserde(rename = "Preview", prefix = "nsi1", default)]
	pub preview: Option<String>, 
	#[yaserde(rename = "MailboxScope", prefix = "nsi1", default)]
	pub mailbox_scope: Option<MailboxSearchLocationType>, 
	#[yaserde(rename = "IconIndex", prefix = "nsi1", default)]
	pub icon_index: Option<IconIndexType>, 
	#[yaserde(rename = "GlobalIconIndex", prefix = "nsi1", default)]
	pub global_icon_index: Option<IconIndexType>, 
	#[yaserde(rename = "DraftItemIds", prefix = "nsi1", default)]
	pub draft_item_ids: Option<NonEmptyArrayOfBaseItemIdsType>, 
	#[yaserde(rename = "HasIrm", prefix = "nsi1", default)]
	pub has_irm: Option<bool>, 
	#[yaserde(rename = "GlobalHasIrm", prefix = "nsi1", default)]
	pub global_has_irm: Option<bool>, 
	#[yaserde(rename = "InferenceClassification", prefix = "nsi1", default)]
	pub inference_classification: Option<InferenceClassificationType>, 
	#[yaserde(rename = "SortKey", prefix = "nsi1", default)]
	pub sort_key: Option<i64>, 
	#[yaserde(rename = "MentionedMe", prefix = "nsi1", default)]
	pub mentioned_me: Option<bool>, 
	#[yaserde(rename = "GlobalMentionedMe", prefix = "nsi1", default)]
	pub global_mentioned_me: Option<bool>, 
	#[yaserde(rename = "SenderSMTPAddress", prefix = "nsi1", default)]
	pub sender_smtp_address: Option<SmtpAddressType>, 
	#[yaserde(rename = "MailboxGuids", prefix = "nsi1", default)]
	pub mailbox_guids: Option<MailboxGuids>, 
	#[yaserde(rename = "From", prefix = "nsi1", default)]
	pub from: Option<SingleRecipientType>, 
	#[yaserde(rename = "AtAllMention", prefix = "nsi1", default)]
	pub at_all_mention: Option<bool>, 
	#[yaserde(rename = "GlobalAtAllMention", prefix = "nsi1", default)]
	pub global_at_all_mention: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "HighlightTermType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct HighlightTermType {
	#[yaserde(rename = "Scope", prefix = "nsi1", default)]
	pub scope: String, 
	#[yaserde(rename = "Value", prefix = "nsi1", default)]
	pub value: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfConversationsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfConversationsType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ConversationRequestType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ConversationRequestType {
	#[yaserde(rename = "ConversationId", prefix = "nsi1", default)]
	pub conversation_id: ItemIdType, 
	#[yaserde(rename = "SyncState", prefix = "nsi1", default)]
	pub sync_state: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfConversationRequestsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfConversationRequestsType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ConversationNodeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ConversationNodeType {
	#[yaserde(rename = "InternetMessageId", prefix = "nsi1", default)]
	pub internet_message_id: Option<String>, 
	#[yaserde(rename = "ParentInternetMessageId", prefix = "nsi1", default)]
	pub parent_internet_message_id: Option<String>, 
	#[yaserde(rename = "Items", prefix = "nsi1", default)]
	pub items: Option<NonEmptyArrayOfAllItemsType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfConversationNodesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfConversationNodesType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ConversationResponseType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ConversationResponseType {
	#[yaserde(rename = "ConversationId", prefix = "nsi1", default)]
	pub conversation_id: ItemIdType, 
	#[yaserde(rename = "SyncState", prefix = "nsi1", default)]
	pub sync_state: Option<String>, 
	#[yaserde(rename = "ConversationNodes", prefix = "nsi1", default)]
	pub conversation_nodes: Option<ArrayOfConversationNodesType>, 
	#[yaserde(rename = "CanDelete", prefix = "nsi1", default)]
	pub can_delete: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ConversationNodeSortOrder",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ConversationNodeSortOrder {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfHighlightTermsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfHighlightTermsType {
	#[yaserde(rename = "Term", prefix = "nsi1", default)]
	pub term: Vec<HighlightTermType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PersonaAttributionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PersonaAttributionType {
	#[yaserde(rename = "Id", prefix = "nsi1", default)]
	pub id: String, 
	#[yaserde(rename = "SourceId", prefix = "nsi1", default)]
	pub source_id: ItemIdType, 
	#[yaserde(rename = "DisplayName", prefix = "nsi1", default)]
	pub display_name: String, 
	#[yaserde(rename = "IsWritable", prefix = "nsi1", default)]
	pub is_writable: Option<bool>, 
	#[yaserde(rename = "IsQuickContact", prefix = "nsi1", default)]
	pub is_quick_contact: Option<bool>, 
	#[yaserde(rename = "IsHidden", prefix = "nsi1", default)]
	pub is_hidden: Option<bool>, 
	#[yaserde(rename = "FolderId", prefix = "nsi1", default)]
	pub folder_id: Option<FolderIdType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfPersonaAttributionsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfPersonaAttributionsType {
	#[yaserde(rename = "Attribution", prefix = "nsi1", default)]
	pub attribution: Vec<PersonaAttributionType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfValueAttributionsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfValueAttributionsType {
	#[yaserde(rename = "Attribution", prefix = "nsi1", default)]
	pub attribution: Vec<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfStringValueType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfStringValueType {
	#[yaserde(rename = "Value", prefix = "nsi1", default)]
	pub value: Vec<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "StringAttributedValueType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct StringAttributedValueType {
	#[yaserde(rename = "Value", prefix = "nsi1", default)]
	pub value: String, 
	#[yaserde(rename = "Attributions", prefix = "nsi1", default)]
	pub attributions: ArrayOfValueAttributionsType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BodyContentType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct BodyContentType {
	#[yaserde(rename = "Value", prefix = "nsi1", default)]
	pub value: String, 
	#[yaserde(rename = "BodyType", prefix = "nsi1", default)]
	pub body_type: BodyTypeType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BodyContentAttributedValueType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct BodyContentAttributedValueType {
	#[yaserde(rename = "Value", prefix = "nsi1", default)]
	pub value: BodyContentType, 
	#[yaserde(rename = "Attributions", prefix = "nsi1", default)]
	pub attributions: ArrayOfValueAttributionsType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "StringArrayAttributedValueType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct StringArrayAttributedValueType {
	#[yaserde(rename = "Values", prefix = "nsi1", default)]
	pub values: ArrayOfStringValueType, 
	#[yaserde(rename = "Attributions", prefix = "nsi1", default)]
	pub attributions: ArrayOfValueAttributionsType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EmailAddressAttributedValueType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct EmailAddressAttributedValueType {
	#[yaserde(rename = "Value", prefix = "nsi1", default)]
	pub value: EmailAddressType, 
	#[yaserde(rename = "Attributions", prefix = "nsi1", default)]
	pub attributions: ArrayOfValueAttributionsType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PersonaPhoneNumberType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PersonaPhoneNumberType {
	#[yaserde(rename = "Number", prefix = "nsi1", default)]
	pub number: String, 
	#[yaserde(rename = "Type", prefix = "nsi1", default)]
	pub rs_type: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PhoneNumberAttributedValueType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PhoneNumberAttributedValueType {
	#[yaserde(rename = "Value", prefix = "nsi1", default)]
	pub value: PersonaPhoneNumberType, 
	#[yaserde(rename = "Attributions", prefix = "nsi1", default)]
	pub attributions: ArrayOfValueAttributionsType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PersonaPostalAddressType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PersonaPostalAddressType {
	#[yaserde(rename = "Street", prefix = "nsi1", default)]
	pub street: Option<String>, 
	#[yaserde(rename = "City", prefix = "nsi1", default)]
	pub city: Option<String>, 
	#[yaserde(rename = "State", prefix = "nsi1", default)]
	pub state: Option<String>, 
	#[yaserde(rename = "Country", prefix = "nsi1", default)]
	pub country: Option<String>, 
	#[yaserde(rename = "PostalCode", prefix = "nsi1", default)]
	pub postal_code: Option<String>, 
	#[yaserde(rename = "PostOfficeBox", prefix = "nsi1", default)]
	pub post_office_box: Option<String>, 
	#[yaserde(rename = "Type", prefix = "nsi1", default)]
	pub rs_type: Option<String>, 
	#[yaserde(rename = "Latitude", prefix = "nsi1", default)]
	pub latitude: Option<f64>, 
	#[yaserde(rename = "Longitude", prefix = "nsi1", default)]
	pub longitude: Option<f64>, 
	#[yaserde(rename = "Accuracy", prefix = "nsi1", default)]
	pub accuracy: Option<f64>, 
	#[yaserde(rename = "Altitude", prefix = "nsi1", default)]
	pub altitude: Option<f64>, 
	#[yaserde(rename = "AltitudeAccuracy", prefix = "nsi1", default)]
	pub altitude_accuracy: Option<f64>, 
	#[yaserde(rename = "FormattedAddress", prefix = "nsi1", default)]
	pub formatted_address: Option<String>, 
	#[yaserde(rename = "LocationUri", prefix = "nsi1", default)]
	pub location_uri: Option<String>, 
	#[yaserde(rename = "LocationSource", prefix = "nsi1", default)]
	pub location_source: Option<LocationSourceType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PostalAddressAttributedValueType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PostalAddressAttributedValueType {
	#[yaserde(rename = "Value", prefix = "nsi1", default)]
	pub value: PersonaPostalAddressType, 
	#[yaserde(rename = "Attributions", prefix = "nsi1", default)]
	pub attributions: ArrayOfValueAttributionsType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ExtendedPropertyAttributedValueType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ExtendedPropertyAttributedValueType {
	#[yaserde(rename = "Value", prefix = "nsi1", default)]
	pub value: ExtendedPropertyType, 
	#[yaserde(rename = "Attributions", prefix = "nsi1", default)]
	pub attributions: ArrayOfValueAttributionsType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfStringAttributedValuesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfStringAttributedValuesType {
	#[yaserde(rename = "StringAttributedValue", prefix = "nsi1", default)]
	pub string_attributed_value: Vec<StringAttributedValueType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfStringArrayAttributedValuesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfStringArrayAttributedValuesType {
	#[yaserde(rename = "StringArrayAttributedValue", prefix = "nsi1", default)]
	pub string_array_attributed_value: Vec<StringArrayAttributedValueType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfBodyContentAttributedValuesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfBodyContentAttributedValuesType {
	#[yaserde(rename = "BodyContentAttributedValue", prefix = "nsi1", default)]
	pub body_content_attributed_value: Vec<BodyContentAttributedValueType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfEmailAddressAttributedValuesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfEmailAddressAttributedValuesType {
	#[yaserde(rename = "EmailAddressAttributedValue", prefix = "nsi1", default)]
	pub email_address_attributed_value: Vec<EmailAddressAttributedValueType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfPhoneNumberAttributedValuesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfPhoneNumberAttributedValuesType {
	#[yaserde(rename = "PhoneNumberAttributedValue", prefix = "nsi1", default)]
	pub phone_number_attributed_value: Vec<PhoneNumberAttributedValueType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfPostalAddressAttributedValuesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfPostalAddressAttributedValuesType {
	#[yaserde(rename = "PostalAddressAttributedValue", prefix = "nsi1", default)]
	pub postal_address_attributed_value: Vec<PostalAddressAttributedValueType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfExtendedPropertyAttributedValueType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfExtendedPropertyAttributedValueType {
	#[yaserde(rename = "ExtendedPropertyAttributedValue", prefix = "nsi1", default)]
	pub extended_property_attributed_value: Vec<ExtendedPropertyAttributedValueType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "InsightFiltersType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct InsightFiltersType {
	#[yaserde(rename = "Count", prefix = "nsi1", default)]
	pub count: Option<i32>, 
	#[yaserde(rename = "Types", prefix = "nsi1", default)]
	pub types: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "KeyInsightsOnly", prefix = "nsi1", default)]
	pub key_insights_only: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "InsightValue",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct InsightValue {
	#[yaserde(rename = "InsightSource", prefix = "nsi1", default)]
	pub insight_source: Option<String>, 
	#[yaserde(rename = "UpdatedUtcTicks", prefix = "nsi1", default)]
	pub updated_utc_ticks: Option<i64>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "StringInsightValue",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct StringInsightValue {
	#[yaserde(flatten, default)]
	pub insight_value: InsightValue, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Data", prefix = "nsi1", default)]
	pub data: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DelveDocument",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DelveDocument {
	#[yaserde(flatten, default)]
	pub insight_value: InsightValue, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Rank", prefix = "nsi1", default)]
	pub rank: Option<f64>, 
	#[yaserde(rename = "Author", prefix = "nsi1", default)]
	pub author: Option<String>, 
	#[yaserde(rename = "Created", prefix = "nsi1", default)]
	pub created: Option<String>, 
	#[yaserde(rename = "LastModifiedTime", prefix = "nsi1", default)]
	pub last_modified_time: Option<String>, 
	#[yaserde(rename = "DefaultEncodingURL", prefix = "nsi1", default)]
	pub default_encoding_url: Option<String>, 
	#[yaserde(rename = "FileType", prefix = "nsi1", default)]
	pub file_type: Option<String>, 
	#[yaserde(rename = "Title", prefix = "nsi1", default)]
	pub title: Option<String>, 
	#[yaserde(rename = "DocumentId", prefix = "nsi1", default)]
	pub document_id: Option<String>, 
	#[yaserde(rename = "PreviewURL", prefix = "nsi1", default)]
	pub preview_url: Option<String>, 
	#[yaserde(rename = "LastEditor", prefix = "nsi1", default)]
	pub last_editor: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ProfileInsightValue",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ProfileInsightValue {
	#[yaserde(flatten, default)]
	pub insight_value: InsightValue, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "FullName", prefix = "nsi1", default)]
	pub full_name: Option<String>, 
	#[yaserde(rename = "FirstName", prefix = "nsi1", default)]
	pub first_name: Option<String>, 
	#[yaserde(rename = "LastName", prefix = "nsi1", default)]
	pub last_name: Option<String>, 
	#[yaserde(rename = "EmailAddress", prefix = "nsi1", default)]
	pub email_address: Option<String>, 
	#[yaserde(rename = "Avatar", prefix = "nsi1", default)]
	pub avatar: Option<String>, 
	#[yaserde(rename = "JoinedUtcTicks", prefix = "nsi1", default)]
	pub joined_utc_ticks: Option<i64>, 
	#[yaserde(rename = "ProfilePicture", prefix = "nsi1", default)]
	pub profile_picture: Option<UserProfilePicture>, 
	#[yaserde(rename = "Title", prefix = "nsi1", default)]
	pub title: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "OutOfOfficeInsightValue",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct OutOfOfficeInsightValue {
	#[yaserde(flatten, default)]
	pub insight_value: InsightValue, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "StartTime", prefix = "nsi1", default)]
	pub start_time: Option<String>, 
	#[yaserde(rename = "EndTime", prefix = "nsi1", default)]
	pub end_time: Option<String>, 
	#[yaserde(rename = "Message", prefix = "nsi1", default)]
	pub message: Option<String>, 
	#[yaserde(rename = "Culture", prefix = "nsi1", default)]
	pub culture: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "JobInsightValue",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct JobInsightValue {
	#[yaserde(flatten, default)]
	pub insight_value: InsightValue, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Company", prefix = "nsi1", default)]
	pub company: Option<String>, 
	#[yaserde(rename = "CompanyDescription", prefix = "nsi1", default)]
	pub company_description: Option<String>, 
	#[yaserde(rename = "CompanyTicker", prefix = "nsi1", default)]
	pub company_ticker: Option<String>, 
	#[yaserde(rename = "CompanyLogoUrl", prefix = "nsi1", default)]
	pub company_logo_url: Option<String>, 
	#[yaserde(rename = "CompanyWebsiteUrl", prefix = "nsi1", default)]
	pub company_website_url: Option<String>, 
	#[yaserde(rename = "CompanyLinkedInUrl", prefix = "nsi1", default)]
	pub company_linked_in_url: Option<String>, 
	#[yaserde(rename = "Title", prefix = "nsi1", default)]
	pub title: Option<String>, 
	#[yaserde(rename = "StartUtcTicks", prefix = "nsi1", default)]
	pub start_utc_ticks: Option<i64>, 
	#[yaserde(rename = "EndUtcTicks", prefix = "nsi1", default)]
	pub end_utc_ticks: Option<i64>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CompanyInsightValue",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct CompanyInsightValue {
	#[yaserde(flatten, default)]
	pub insight_value: InsightValue, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Name", prefix = "nsi1", default)]
	pub name: String, 
	#[yaserde(rename = "SatoriId", prefix = "nsi1", default)]
	pub satori_id: Option<String>, 
	#[yaserde(rename = "Description", prefix = "nsi1", default)]
	pub description: Option<String>, 
	#[yaserde(rename = "DescriptionAttribution", prefix = "nsi1", default)]
	pub description_attribution: Option<String>, 
	#[yaserde(rename = "ImageUrl", prefix = "nsi1", default)]
	pub image_url: Option<String>, 
	#[yaserde(rename = "ImageUrlAttribution", prefix = "nsi1", default)]
	pub image_url_attribution: Option<String>, 
	#[yaserde(rename = "YearFound", prefix = "nsi1", default)]
	pub year_found: Option<String>, 
	#[yaserde(rename = "FinanceSymbol", prefix = "nsi1", default)]
	pub finance_symbol: Option<String>, 
	#[yaserde(rename = "WebsiteUrl", prefix = "nsi1", default)]
	pub website_url: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UserProfilePicture",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct UserProfilePicture {
	#[yaserde(flatten, default)]
	pub insight_value: InsightValue, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Blob", prefix = "nsi1", default)]
	pub blob: Option<String>, 
	#[yaserde(rename = "PhotoSize", prefix = "nsi1", default)]
	pub photo_size: Option<String>, 
	#[yaserde(rename = "Url", prefix = "nsi1", default)]
	pub url: Option<String>, 
	#[yaserde(rename = "ImageType", prefix = "nsi1", default)]
	pub image_type: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EducationInsightValue",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct EducationInsightValue {
	#[yaserde(flatten, default)]
	pub insight_value: InsightValue, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Institute", prefix = "nsi1", default)]
	pub institute: Option<String>, 
	#[yaserde(rename = "Degree", prefix = "nsi1", default)]
	pub degree: Option<String>, 
	#[yaserde(rename = "StartUtcTicks", prefix = "nsi1", default)]
	pub start_utc_ticks: Option<i64>, 
	#[yaserde(rename = "EndUtcTicks", prefix = "nsi1", default)]
	pub end_utc_ticks: Option<i64>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SkillInsightValue",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SkillInsightValue {
	#[yaserde(flatten, default)]
	pub insight_value: InsightValue, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Name", prefix = "nsi1", default)]
	pub name: Option<String>, 
	#[yaserde(rename = "Strength", prefix = "nsi1", default)]
	pub strength: Option<i32>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MeetingInsightValue",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MeetingInsightValue {
	#[yaserde(flatten, default)]
	pub insight_value: InsightValue, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Id", prefix = "nsi1", default)]
	pub id: Option<String>, 
	#[yaserde(rename = "Subject", prefix = "nsi1", default)]
	pub subject: Option<String>, 
	#[yaserde(rename = "StartUtcTicks", prefix = "nsi1", default)]
	pub start_utc_ticks: Option<i64>, 
	#[yaserde(rename = "EndUtcTicks", prefix = "nsi1", default)]
	pub end_utc_ticks: Option<i64>, 
	#[yaserde(rename = "Location", prefix = "nsi1", default)]
	pub location: Option<String>, 
	#[yaserde(rename = "Organizer", prefix = "nsi1", default)]
	pub organizer: Option<ProfileInsightValue>, 
	#[yaserde(rename = "Attendees", prefix = "nsi1", default)]
	pub attendees: Option<ArrayOfProfileInsightValue>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EmailInsightValue",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct EmailInsightValue {
	#[yaserde(flatten, default)]
	pub insight_value: InsightValue, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Id", prefix = "nsi1", default)]
	pub id: Option<String>, 
	#[yaserde(rename = "ThreadId", prefix = "nsi1", default)]
	pub thread_id: Option<String>, 
	#[yaserde(rename = "Subject", prefix = "nsi1", default)]
	pub subject: Option<String>, 
	#[yaserde(rename = "LastEmailDateUtcTicks", prefix = "nsi1", default)]
	pub last_email_date_utc_ticks: Option<i64>, 
	#[yaserde(rename = "Body", prefix = "nsi1", default)]
	pub body: Option<String>, 
	#[yaserde(rename = "LastEmailSender", prefix = "nsi1", default)]
	pub last_email_sender: Option<ProfileInsightValue>, 
	#[yaserde(rename = "EmailsCount", prefix = "nsi1", default)]
	pub emails_count: Option<i32>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfInsightValue",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfInsightValue {
	#[yaserde(rename = "Item", prefix = "nsi1", default)]
	pub item: Vec<InsightValue>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfSkillInsightValue",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfSkillInsightValue {
	#[yaserde(rename = "Item", prefix = "nsi1", default)]
	pub item: Vec<SkillInsightValue>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfProfileInsightValue",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfProfileInsightValue {
	#[yaserde(rename = "Item", prefix = "nsi1", default)]
	pub item: Vec<ProfileInsightValue>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfJobInsightValue",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfJobInsightValue {
	#[yaserde(rename = "Item", prefix = "nsi1", default)]
	pub item: Vec<JobInsightValue>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfCompanyInsightValue",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfCompanyInsightValue {
	#[yaserde(rename = "Item", prefix = "nsi1", default)]
	pub item: Vec<CompanyInsightValue>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "InsightContent",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct InsightContent {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SingleValueInsightContent",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SingleValueInsightContent {
	#[yaserde(flatten, default)]
	pub insight_content: InsightContent, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Item", prefix = "nsi1", default)]
	pub item: Option<InsightValue>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MultiValueInsightContent",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MultiValueInsightContent {
	#[yaserde(flatten, default)]
	pub insight_content: InsightContent, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ItemList", prefix = "nsi1", default)]
	pub item_list: Option<ArrayOfInsightValue>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Insight",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct Insight {
	#[yaserde(rename = "InsightType", prefix = "nsi1", default)]
	pub insight_type: Option<String>, 
	#[yaserde(rename = "Rank", prefix = "nsi1", default)]
	pub rank: Option<f64>, 
	#[yaserde(rename = "Content", prefix = "nsi1", default)]
	pub content: Option<InsightContent>, 
	#[yaserde(rename = "Text", prefix = "nsi1", default)]
	pub text: Option<String>, 
	#[yaserde(rename = "ItemList", prefix = "nsi1", default)]
	pub item_list: Option<ArrayOfInsightValue>, 
	#[yaserde(rename = "RequiresToken", prefix = "nsi1", default)]
	pub requires_token: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ComputedInsightValueProperty",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ComputedInsightValueProperty {
	#[yaserde(rename = "Key", prefix = "nsi1", default)]
	pub key: String, 
	#[yaserde(rename = "Value", prefix = "nsi1", default)]
	pub value: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfComputedInsightValueProperty",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfComputedInsightValueProperty {
	#[yaserde(rename = "Property", prefix = "nsi1", default)]
	pub property: Vec<ComputedInsightValueProperty>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ComputedInsightValue",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ComputedInsightValue {
	#[yaserde(flatten, default)]
	pub insight_value: InsightValue, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Properties", prefix = "nsi1", default)]
	pub properties: Option<ArrayOfComputedInsightValueProperty>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfInsight",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfInsight {
	#[yaserde(rename = "Insight", prefix = "nsi1", default)]
	pub insight: Vec<Insight>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PersonResponseShapeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PersonResponseShapeType {
	#[yaserde(rename = "BaseShape", prefix = "nsi1", default)]
	pub base_shape: DefaultShapeNamesType, 
	#[yaserde(rename = "AdditionalProperties", prefix = "nsi1", default)]
	pub additional_properties: Option<NonEmptyArrayOfPathsToElementType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PersonType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PersonType {
	#[yaserde(rename = "FullName", prefix = "nsi1", default)]
	pub full_name: Option<String>, 
	#[yaserde(rename = "GivenName", prefix = "nsi1", default)]
	pub given_name: Option<String>, 
	#[yaserde(rename = "Surname", prefix = "nsi1", default)]
	pub surname: Option<String>, 
	#[yaserde(rename = "PhoneNumber", prefix = "nsi1", default)]
	pub phone_number: Option<String>, 
	#[yaserde(rename = "SMSNumber", prefix = "nsi1", default)]
	pub sms_number: Option<String>, 
	#[yaserde(rename = "EmailAddress", prefix = "nsi1", default)]
	pub email_address: Option<String>, 
	#[yaserde(rename = "Alias", prefix = "nsi1", default)]
	pub alias: Option<String>, 
	#[yaserde(rename = "Department", prefix = "nsi1", default)]
	pub department: Option<String>, 
	#[yaserde(rename = "LinkedInProfileLink", prefix = "nsi1", default)]
	pub linked_in_profile_link: Option<String>, 
	#[yaserde(rename = "Skills", prefix = "nsi1", default)]
	pub skills: Option<ArrayOfSkillInsightValue>, 
	#[yaserde(rename = "ProfessionalBiography", prefix = "nsi1", default)]
	pub professional_biography: Option<String>, 
	#[yaserde(rename = "ManagementChain", prefix = "nsi1", default)]
	pub management_chain: Option<ArrayOfProfileInsightValue>, 
	#[yaserde(rename = "DirectReports", prefix = "nsi1", default)]
	pub direct_reports: Option<ArrayOfProfileInsightValue>, 
	#[yaserde(rename = "Peers", prefix = "nsi1", default)]
	pub peers: Option<ArrayOfProfileInsightValue>, 
	#[yaserde(rename = "TeamSize", prefix = "nsi1", default)]
	pub team_size: Option<String>, 
	#[yaserde(rename = "CurrentJob", prefix = "nsi1", default)]
	pub current_job: Option<ArrayOfJobInsightValue>, 
	#[yaserde(rename = "Birthday", prefix = "nsi1", default)]
	pub birthday: Option<String>, 
	#[yaserde(rename = "Hometown", prefix = "nsi1", default)]
	pub hometown: Option<String>, 
	#[yaserde(rename = "CurrentLocation", prefix = "nsi1", default)]
	pub current_location: Option<String>, 
	#[yaserde(rename = "CompanyProfile", prefix = "nsi1", default)]
	pub company_profile: Option<ArrayOfCompanyInsightValue>, 
	#[yaserde(rename = "Office", prefix = "nsi1", default)]
	pub office: Option<String>, 
	#[yaserde(rename = "Headline", prefix = "nsi1", default)]
	pub headline: Option<String>, 
	#[yaserde(rename = "MutualConnections", prefix = "nsi1", default)]
	pub mutual_connections: Option<ArrayOfProfileInsightValue>, 
	#[yaserde(rename = "Title", prefix = "nsi1", default)]
	pub title: Option<String>, 
	#[yaserde(rename = "MutualManager", prefix = "nsi1", default)]
	pub mutual_manager: Option<ProfileInsightValue>, 
	#[yaserde(rename = "Insights", prefix = "nsi1", default)]
	pub insights: Option<ArrayOfInsight>, 
	#[yaserde(rename = "UserProfilePicture", prefix = "nsi1", default)]
	pub user_profile_picture: Option<UserProfilePicture>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfPersonType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfPersonType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "TokenSourceType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct TokenSourceType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PeopleTokenType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PeopleTokenType {
	#[yaserde(rename = "TokenSource", prefix = "nsi1", default)]
	pub token_source: TokenSourceType, 
	#[yaserde(rename = "ExpirationDateTime", prefix = "nsi1", default)]
	pub expiration_date_time: Option<String>, 
	#[yaserde(rename = "TokenValue", prefix = "nsi1", default)]
	pub token_value: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfPeopleTokenType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfPeopleTokenType {
	#[yaserde(rename = "PeopleToken", prefix = "nsi1", default)]
	pub people_token: Vec<PeopleTokenType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PersonaType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PersonaType {
	#[yaserde(rename = "PersonaId", prefix = "nsi1", default)]
	pub persona_id: ItemIdType, 
	#[yaserde(rename = "PersonaType", prefix = "nsi1", default)]
	pub persona_type: Option<String>, 
	#[yaserde(rename = "PersonaObjectStatus", prefix = "nsi1", default)]
	pub persona_object_status: Option<String>, 
	#[yaserde(rename = "CreationTime", prefix = "nsi1", default)]
	pub creation_time: Option<String>, 
	#[yaserde(rename = "Bodies", prefix = "nsi1", default)]
	pub bodies: Option<ArrayOfBodyContentAttributedValuesType>, 
	#[yaserde(rename = "DisplayNameFirstLastSortKey", prefix = "nsi1", default)]
	pub display_name_first_last_sort_key: Option<String>, 
	#[yaserde(rename = "DisplayNameLastFirstSortKey", prefix = "nsi1", default)]
	pub display_name_last_first_sort_key: Option<String>, 
	#[yaserde(rename = "CompanyNameSortKey", prefix = "nsi1", default)]
	pub company_name_sort_key: Option<String>, 
	#[yaserde(rename = "HomeCitySortKey", prefix = "nsi1", default)]
	pub home_city_sort_key: Option<String>, 
	#[yaserde(rename = "WorkCitySortKey", prefix = "nsi1", default)]
	pub work_city_sort_key: Option<String>, 
	#[yaserde(rename = "DisplayNameFirstLastHeader", prefix = "nsi1", default)]
	pub display_name_first_last_header: Option<String>, 
	#[yaserde(rename = "DisplayNameLastFirstHeader", prefix = "nsi1", default)]
	pub display_name_last_first_header: Option<String>, 
	#[yaserde(rename = "DisplayName", prefix = "nsi1", default)]
	pub display_name: Option<String>, 
	#[yaserde(rename = "DisplayNameFirstLast", prefix = "nsi1", default)]
	pub display_name_first_last: Option<String>, 
	#[yaserde(rename = "DisplayNameLastFirst", prefix = "nsi1", default)]
	pub display_name_last_first: Option<String>, 
	#[yaserde(rename = "FileAs", prefix = "nsi1", default)]
	pub file_as: Option<String>, 
	#[yaserde(rename = "FileAsId", prefix = "nsi1", default)]
	pub file_as_id: Option<String>, 
	#[yaserde(rename = "DisplayNamePrefix", prefix = "nsi1", default)]
	pub display_name_prefix: Option<String>, 
	#[yaserde(rename = "GivenName", prefix = "nsi1", default)]
	pub given_name: Option<String>, 
	#[yaserde(rename = "MiddleName", prefix = "nsi1", default)]
	pub middle_name: Option<String>, 
	#[yaserde(rename = "Surname", prefix = "nsi1", default)]
	pub surname: Option<String>, 
	#[yaserde(rename = "Generation", prefix = "nsi1", default)]
	pub generation: Option<String>, 
	#[yaserde(rename = "Nickname", prefix = "nsi1", default)]
	pub nickname: Option<String>, 
	#[yaserde(rename = "YomiCompanyName", prefix = "nsi1", default)]
	pub yomi_company_name: Option<String>, 
	#[yaserde(rename = "YomiFirstName", prefix = "nsi1", default)]
	pub yomi_first_name: Option<String>, 
	#[yaserde(rename = "YomiLastName", prefix = "nsi1", default)]
	pub yomi_last_name: Option<String>, 
	#[yaserde(rename = "Title", prefix = "nsi1", default)]
	pub title: Option<String>, 
	#[yaserde(rename = "Department", prefix = "nsi1", default)]
	pub department: Option<String>, 
	#[yaserde(rename = "CompanyName", prefix = "nsi1", default)]
	pub company_name: Option<String>, 
	#[yaserde(rename = "Location", prefix = "nsi1", default)]
	pub location: Option<String>, 
	#[yaserde(rename = "EmailAddress", prefix = "nsi1", default)]
	pub email_address: Option<EmailAddressType>, 
	#[yaserde(rename = "EmailAddresses", prefix = "nsi1", default)]
	pub email_addresses: Option<ArrayOfEmailAddressesType>, 
	#[yaserde(rename = "PhoneNumber", prefix = "nsi1", default)]
	pub phone_number: Option<PersonaPhoneNumberType>, 
	#[yaserde(rename = "ImAddress", prefix = "nsi1", default)]
	pub im_address: Option<String>, 
	#[yaserde(rename = "HomeCity", prefix = "nsi1", default)]
	pub home_city: Option<String>, 
	#[yaserde(rename = "WorkCity", prefix = "nsi1", default)]
	pub work_city: Option<String>, 
	#[yaserde(rename = "RelevanceScore", prefix = "nsi1", default)]
	pub relevance_score: Option<i32>, 
	#[yaserde(rename = "FolderIds", prefix = "nsi1", default)]
	pub folder_ids: Option<ArrayOfFolderIdType>, 
	#[yaserde(rename = "Attributions", prefix = "nsi1", default)]
	pub attributions: Option<ArrayOfPersonaAttributionsType>, 
	#[yaserde(rename = "DisplayNames", prefix = "nsi1", default)]
	pub display_names: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "FileAses", prefix = "nsi1", default)]
	pub file_ases: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "FileAsIds", prefix = "nsi1", default)]
	pub file_as_ids: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "DisplayNamePrefixes", prefix = "nsi1", default)]
	pub display_name_prefixes: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "GivenNames", prefix = "nsi1", default)]
	pub given_names: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "MiddleNames", prefix = "nsi1", default)]
	pub middle_names: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "Surnames", prefix = "nsi1", default)]
	pub surnames: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "Generations", prefix = "nsi1", default)]
	pub generations: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "Nicknames", prefix = "nsi1", default)]
	pub nicknames: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "Initials", prefix = "nsi1", default)]
	pub initials: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "YomiCompanyNames", prefix = "nsi1", default)]
	pub yomi_company_names: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "YomiFirstNames", prefix = "nsi1", default)]
	pub yomi_first_names: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "YomiLastNames", prefix = "nsi1", default)]
	pub yomi_last_names: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "BusinessPhoneNumbers", prefix = "nsi1", default)]
	pub business_phone_numbers: Option<ArrayOfPhoneNumberAttributedValuesType>, 
	#[yaserde(rename = "BusinessPhoneNumbers2", prefix = "nsi1", default)]
	pub business_phone_numbers_2: Option<ArrayOfPhoneNumberAttributedValuesType>, 
	#[yaserde(rename = "HomePhones", prefix = "nsi1", default)]
	pub home_phones: Option<ArrayOfPhoneNumberAttributedValuesType>, 
	#[yaserde(rename = "HomePhones2", prefix = "nsi1", default)]
	pub home_phones_2: Option<ArrayOfPhoneNumberAttributedValuesType>, 
	#[yaserde(rename = "MobilePhones", prefix = "nsi1", default)]
	pub mobile_phones: Option<ArrayOfPhoneNumberAttributedValuesType>, 
	#[yaserde(rename = "MobilePhones2", prefix = "nsi1", default)]
	pub mobile_phones_2: Option<ArrayOfPhoneNumberAttributedValuesType>, 
	#[yaserde(rename = "AssistantPhoneNumbers", prefix = "nsi1", default)]
	pub assistant_phone_numbers: Option<ArrayOfPhoneNumberAttributedValuesType>, 
	#[yaserde(rename = "CallbackPhones", prefix = "nsi1", default)]
	pub callback_phones: Option<ArrayOfPhoneNumberAttributedValuesType>, 
	#[yaserde(rename = "CarPhones", prefix = "nsi1", default)]
	pub car_phones: Option<ArrayOfPhoneNumberAttributedValuesType>, 
	#[yaserde(rename = "HomeFaxes", prefix = "nsi1", default)]
	pub home_faxes: Option<ArrayOfPhoneNumberAttributedValuesType>, 
	#[yaserde(rename = "OrganizationMainPhones", prefix = "nsi1", default)]
	pub organization_main_phones: Option<ArrayOfPhoneNumberAttributedValuesType>, 
	#[yaserde(rename = "OtherFaxes", prefix = "nsi1", default)]
	pub other_faxes: Option<ArrayOfPhoneNumberAttributedValuesType>, 
	#[yaserde(rename = "OtherTelephones", prefix = "nsi1", default)]
	pub other_telephones: Option<ArrayOfPhoneNumberAttributedValuesType>, 
	#[yaserde(rename = "OtherPhones2", prefix = "nsi1", default)]
	pub other_phones_2: Option<ArrayOfPhoneNumberAttributedValuesType>, 
	#[yaserde(rename = "Pagers", prefix = "nsi1", default)]
	pub pagers: Option<ArrayOfPhoneNumberAttributedValuesType>, 
	#[yaserde(rename = "RadioPhones", prefix = "nsi1", default)]
	pub radio_phones: Option<ArrayOfPhoneNumberAttributedValuesType>, 
	#[yaserde(rename = "TelexNumbers", prefix = "nsi1", default)]
	pub telex_numbers: Option<ArrayOfPhoneNumberAttributedValuesType>, 
	#[yaserde(rename = "TTYTDDPhoneNumbers", prefix = "nsi1", default)]
	pub ttytdd_phone_numbers: Option<ArrayOfPhoneNumberAttributedValuesType>, 
	#[yaserde(rename = "WorkFaxes", prefix = "nsi1", default)]
	pub work_faxes: Option<ArrayOfPhoneNumberAttributedValuesType>, 
	#[yaserde(rename = "Emails1", prefix = "nsi1", default)]
	pub emails_1: Option<ArrayOfEmailAddressAttributedValuesType>, 
	#[yaserde(rename = "Emails2", prefix = "nsi1", default)]
	pub emails_2: Option<ArrayOfEmailAddressAttributedValuesType>, 
	#[yaserde(rename = "Emails3", prefix = "nsi1", default)]
	pub emails_3: Option<ArrayOfEmailAddressAttributedValuesType>, 
	#[yaserde(rename = "BusinessHomePages", prefix = "nsi1", default)]
	pub business_home_pages: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "PersonalHomePages", prefix = "nsi1", default)]
	pub personal_home_pages: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "OfficeLocations", prefix = "nsi1", default)]
	pub office_locations: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "ImAddresses", prefix = "nsi1", default)]
	pub im_addresses: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "ImAddresses2", prefix = "nsi1", default)]
	pub im_addresses_2: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "ImAddresses3", prefix = "nsi1", default)]
	pub im_addresses_3: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "BusinessAddresses", prefix = "nsi1", default)]
	pub business_addresses: Option<ArrayOfPostalAddressAttributedValuesType>, 
	#[yaserde(rename = "HomeAddresses", prefix = "nsi1", default)]
	pub home_addresses: Option<ArrayOfPostalAddressAttributedValuesType>, 
	#[yaserde(rename = "OtherAddresses", prefix = "nsi1", default)]
	pub other_addresses: Option<ArrayOfPostalAddressAttributedValuesType>, 
	#[yaserde(rename = "Titles", prefix = "nsi1", default)]
	pub titles: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "Departments", prefix = "nsi1", default)]
	pub departments: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "CompanyNames", prefix = "nsi1", default)]
	pub company_names: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "Managers", prefix = "nsi1", default)]
	pub managers: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "AssistantNames", prefix = "nsi1", default)]
	pub assistant_names: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "Professions", prefix = "nsi1", default)]
	pub professions: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "SpouseNames", prefix = "nsi1", default)]
	pub spouse_names: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "Children", prefix = "nsi1", default)]
	pub children: Option<ArrayOfStringArrayAttributedValuesType>, 
	#[yaserde(rename = "Schools", prefix = "nsi1", default)]
	pub schools: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "Hobbies", prefix = "nsi1", default)]
	pub hobbies: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "WeddingAnniversaries", prefix = "nsi1", default)]
	pub wedding_anniversaries: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "Birthdays", prefix = "nsi1", default)]
	pub birthdays: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "Locations", prefix = "nsi1", default)]
	pub locations: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "InlineLinks", prefix = "nsi1", default)]
	pub inline_links: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "ItemLinkIds", prefix = "nsi1", default)]
	pub item_link_ids: Option<ArrayOfStringArrayAttributedValuesType>, 
	#[yaserde(rename = "HasActiveDeals", prefix = "nsi1", default)]
	pub has_active_deals: Option<String>, 
	#[yaserde(rename = "IsBusinessContact", prefix = "nsi1", default)]
	pub is_business_contact: Option<String>, 
	#[yaserde(rename = "AttributedHasActiveDeals", prefix = "nsi1", default)]
	pub attributed_has_active_deals: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "AttributedIsBusinessContact", prefix = "nsi1", default)]
	pub attributed_is_business_contact: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "SourceMailboxGuids", prefix = "nsi1", default)]
	pub source_mailbox_guids: Option<ArrayOfStringAttributedValuesType>, 
	#[yaserde(rename = "LastContactedDate", prefix = "nsi1", default)]
	pub last_contacted_date: Option<String>, 
	#[yaserde(rename = "ExtendedProperties", prefix = "nsi1", default)]
	pub extended_properties: Option<ArrayOfExtendedPropertyAttributedValueType>, 
	#[yaserde(rename = "ExternalDirectoryObjectId", prefix = "nsi1", default)]
	pub external_directory_object_id: Option<String>, 
	#[yaserde(rename = "MapiEntryId", prefix = "nsi1", default)]
	pub mapi_entry_id: Option<String>, 
	#[yaserde(rename = "MapiEmailAddress", prefix = "nsi1", default)]
	pub mapi_email_address: Option<String>, 
	#[yaserde(rename = "MapiAddressType", prefix = "nsi1", default)]
	pub mapi_address_type: Option<String>, 
	#[yaserde(rename = "MapiSearchKey", prefix = "nsi1", default)]
	pub mapi_search_key: Option<String>, 
	#[yaserde(rename = "MapiTransmittableDisplayName", prefix = "nsi1", default)]
	pub mapi_transmittable_display_name: Option<String>, 
	#[yaserde(rename = "MapiSendRichInfo", prefix = "nsi1", default)]
	pub mapi_send_rich_info: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfPeopleType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfPeopleType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MailboxLocatorType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MailboxLocatorType {
	#[yaserde(rename = "ExternalDirectoryObjectId", prefix = "nsi1", default)]
	pub external_directory_object_id: Option<String>, 
	#[yaserde(rename = "LegacyDn", prefix = "nsi1", default)]
	pub legacy_dn: Option<String>, 
	#[yaserde(rename = "EmailAddress", prefix = "nsi1", default)]
	pub email_address: Option<String>, 
	#[yaserde(rename = "DisplayName", prefix = "nsi1", default)]
	pub display_name: Option<String>, 
	#[yaserde(rename = "OrganizationId", prefix = "nsi1", default)]
	pub organization_id: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GroupLocatorType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct GroupLocatorType {
	#[yaserde(flatten, default)]
	pub mailbox_locator_type: MailboxLocatorType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "AccessType", prefix = "nsi1", default)]
	pub access_type: Option<i32>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UserLocatorType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct UserLocatorType {
	#[yaserde(flatten, default)]
	pub mailbox_locator_type: MailboxLocatorType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GroupMailboxConfigurationActionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct GroupMailboxConfigurationActionType {
	#[yaserde(flatten, default)]
	pub body: Box<GroupMailboxConfigurationActionType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "InstantSearchItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct InstantSearchItemType {
	#[yaserde(flatten, default)]
	pub body: Box<InstantSearchItemType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "InstantSearchResultType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct InstantSearchResultType {
	#[yaserde(flatten, default)]
	pub body: Box<InstantSearchResultType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfItemsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfItemsType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfCalendarItemsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfCalendarItemsType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "InstantSearchPayloadType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct InstantSearchPayloadType {
	#[yaserde(rename = "SearchSessionId", prefix = "nsi1", default)]
	pub search_session_id: String, 
	#[yaserde(rename = "SearchRequestId", prefix = "nsi1", default)]
	pub search_request_id: i64, 
	#[yaserde(rename = "ResultType", prefix = "nsi1", default)]
	pub result_type: InstantSearchResultType, 
	#[yaserde(rename = "Items", prefix = "nsi1", default)]
	pub items: Option<ArrayOfItemsType>, 
	#[yaserde(rename = "Conversations", prefix = "nsi1", default)]
	pub conversations: Option<ArrayOfConversationsType>, 
	#[yaserde(rename = "CalendarItems", prefix = "nsi1", default)]
	pub calendar_items: Option<ArrayOfCalendarItemsType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "QueryOptionsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct QueryOptionsType {
	#[yaserde(flatten, default)]
	pub body: Box<QueryOptionsType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "OneDriveViewType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct OneDriveViewType {
	#[yaserde(flatten, default)]
	pub body: Box<OneDriveViewType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DelveViewType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DelveViewType {
	#[yaserde(flatten, default)]
	pub body: Box<DelveViewType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GroupMemberIdentifierType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct GroupMemberIdentifierType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ModernGroupTypeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ModernGroupTypeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfStringsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfStringsType {
	#[yaserde(rename = "String", prefix = "nsi1", default)]
	pub string: Vec<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfRealItemsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfRealItemsType {
	#[yaserde(rename = "Item", prefix = "nsi1", default)]
	pub item: ItemType, 
	#[yaserde(rename = "Message", prefix = "nsi1", default)]
	pub message: MessageType, 
	#[yaserde(rename = "SharingMessage", prefix = "nsi1", default)]
	pub sharing_message: SharingMessageType, 
	#[yaserde(rename = "CalendarItem", prefix = "nsi1", default)]
	pub calendar_item: CalendarItemType, 
	#[yaserde(rename = "Contact", prefix = "nsi1", default)]
	pub contact: ContactItemType, 
	#[yaserde(rename = "DistributionList", prefix = "nsi1", default)]
	pub distribution_list: DistributionListType, 
	#[yaserde(rename = "MeetingMessage", prefix = "nsi1", default)]
	pub meeting_message: MeetingMessageType, 
	#[yaserde(rename = "MeetingRequest", prefix = "nsi1", default)]
	pub meeting_request: MeetingRequestMessageType, 
	#[yaserde(rename = "MeetingResponse", prefix = "nsi1", default)]
	pub meeting_response: MeetingResponseMessageType, 
	#[yaserde(rename = "MeetingCancellation", prefix = "nsi1", default)]
	pub meeting_cancellation: MeetingCancellationMessageType, 
	#[yaserde(rename = "Task", prefix = "nsi1", default)]
	pub task: TaskType, 
	#[yaserde(rename = "PostItem", prefix = "nsi1", default)]
	pub post_item: PostItemType, 
	#[yaserde(rename = "RoleMember", prefix = "nsi1", default)]
	pub role_member: RoleMemberItemType, 
	#[yaserde(rename = "Network", prefix = "nsi1", default)]
	pub network: NetworkItemType, 
	#[yaserde(rename = "Person", prefix = "nsi1", default)]
	pub person: AbchPersonItemType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfAllItemsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfAllItemsType {
	#[yaserde(rename = "Item", prefix = "nsi1", default)]
	pub item: ItemType, 
	#[yaserde(rename = "Message", prefix = "nsi1", default)]
	pub message: MessageType, 
	#[yaserde(rename = "SharingMessage", prefix = "nsi1", default)]
	pub sharing_message: SharingMessageType, 
	#[yaserde(rename = "CalendarItem", prefix = "nsi1", default)]
	pub calendar_item: Box<CalendarItemType>, 
	#[yaserde(rename = "Contact", prefix = "nsi1", default)]
	pub contact: ContactItemType, 
	#[yaserde(rename = "DistributionList", prefix = "nsi1", default)]
	pub distribution_list: DistributionListType, 
	#[yaserde(rename = "MeetingMessage", prefix = "nsi1", default)]
	pub meeting_message: MeetingMessageType, 
	#[yaserde(rename = "MeetingRequest", prefix = "nsi1", default)]
	pub meeting_request: Box<MeetingRequestMessageType>, 
	#[yaserde(rename = "MeetingResponse", prefix = "nsi1", default)]
	pub meeting_response: MeetingResponseMessageType, 
	#[yaserde(rename = "MeetingCancellation", prefix = "nsi1", default)]
	pub meeting_cancellation: MeetingCancellationMessageType, 
	#[yaserde(rename = "Task", prefix = "nsi1", default)]
	pub task: TaskType, 
	#[yaserde(rename = "PostItem", prefix = "nsi1", default)]
	pub post_item: PostItemType, 
	#[yaserde(rename = "ReplyToItem", prefix = "nsi1", default)]
	pub reply_to_item: ReplyToItemType, 
	#[yaserde(rename = "ForwardItem", prefix = "nsi1", default)]
	pub forward_item: ForwardItemType, 
	#[yaserde(rename = "ReplyAllToItem", prefix = "nsi1", default)]
	pub reply_all_to_item: ReplyAllToItemType, 
	#[yaserde(rename = "AcceptItem", prefix = "nsi1", default)]
	pub accept_item: AcceptItemType, 
	#[yaserde(rename = "TentativelyAcceptItem", prefix = "nsi1", default)]
	pub tentatively_accept_item: TentativelyAcceptItemType, 
	#[yaserde(rename = "DeclineItem", prefix = "nsi1", default)]
	pub decline_item: DeclineItemType, 
	#[yaserde(rename = "CancelCalendarItem", prefix = "nsi1", default)]
	pub cancel_calendar_item: CancelCalendarItemType, 
	#[yaserde(rename = "RemoveItem", prefix = "nsi1", default)]
	pub remove_item: RemoveItemType, 
	#[yaserde(rename = "SuppressReadReceipt", prefix = "nsi1", default)]
	pub suppress_read_receipt: SuppressReadReceiptType, 
	#[yaserde(rename = "PostReplyItem", prefix = "nsi1", default)]
	pub post_reply_item: PostReplyItemType, 
	#[yaserde(rename = "AcceptSharingInvitation", prefix = "nsi1", default)]
	pub accept_sharing_invitation: AcceptSharingInvitationType, 
	#[yaserde(rename = "RoleMember", prefix = "nsi1", default)]
	pub role_member: RoleMemberItemType, 
	#[yaserde(rename = "Network", prefix = "nsi1", default)]
	pub network: NetworkItemType, 
	#[yaserde(rename = "Person", prefix = "nsi1", default)]
	pub person: AbchPersonItemType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MeetingRegistrationResponseObjectType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MeetingRegistrationResponseObjectType {
	#[yaserde(flatten, default)]
	pub well_known_response_object_type: WellKnownResponseObjectType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ProposedStart", prefix = "nsi1", default)]
	pub proposed_start: Option<String>, 
	#[yaserde(rename = "ProposedEnd", prefix = "nsi1", default)]
	pub proposed_end: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AcceptItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AcceptItemType {
	#[yaserde(flatten, default)]
	pub meeting_registration_response_object_type: MeetingRegistrationResponseObjectType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "TentativelyAcceptItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct TentativelyAcceptItemType {
	#[yaserde(flatten, default)]
	pub meeting_registration_response_object_type: MeetingRegistrationResponseObjectType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeclineItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DeclineItemType {
	#[yaserde(flatten, default)]
	pub meeting_registration_response_object_type: MeetingRegistrationResponseObjectType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ProposeNewTimeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ProposeNewTimeType {
	#[yaserde(flatten, default)]
	pub response_object_type: ResponseObjectType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RemoveItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RemoveItemType {
	#[yaserde(flatten, default)]
	pub response_object_type: ResponseObjectType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AddItemToMyCalendarType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AddItemToMyCalendarType {
	#[yaserde(flatten, default)]
	pub response_object_type: ResponseObjectType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PostReplyItemBaseType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PostReplyItemBaseType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PostReplyItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PostReplyItemType {
	#[yaserde(flatten, default)]
	pub post_reply_item_base_type: PostReplyItemBaseType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "NewBodyContent", prefix = "nsi1", default)]
	pub new_body_content: Option<BodyType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MimeContentType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MimeContentType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MimeContentUTF8Type",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MimeContentUTF8Type {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MessageDispositionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MessageDispositionType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CalendarItemCreateOrDeleteOperationType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct CalendarItemCreateOrDeleteOperationType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CalendarItemUpdateOperationType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct CalendarItemUpdateOperationType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AffectedTaskOccurrencesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AffectedTaskOccurrencesType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MessageType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MessageType {
	#[yaserde(flatten, default)]
	pub item_type: ItemType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Sender", prefix = "nsi1", default)]
	pub sender: Option<SingleRecipientType>, 
	#[yaserde(rename = "ToRecipients", prefix = "nsi1", default)]
	pub to_recipients: Option<ArrayOfRecipientsType>, 
	#[yaserde(rename = "CcRecipients", prefix = "nsi1", default)]
	pub cc_recipients: Option<ArrayOfRecipientsType>, 
	#[yaserde(rename = "BccRecipients", prefix = "nsi1", default)]
	pub bcc_recipients: Option<ArrayOfRecipientsType>, 
	#[yaserde(rename = "IsReadReceiptRequested", prefix = "nsi1", default)]
	pub is_read_receipt_requested: Option<bool>, 
	#[yaserde(rename = "IsDeliveryReceiptRequested", prefix = "nsi1", default)]
	pub is_delivery_receipt_requested: Option<bool>, 
	#[yaserde(rename = "ConversationIndex", prefix = "nsi1", default)]
	pub conversation_index: Option<String>, 
	#[yaserde(rename = "ConversationTopic", prefix = "nsi1", default)]
	pub conversation_topic: Option<String>, 
	#[yaserde(rename = "From", prefix = "nsi1", default)]
	pub from: Option<SingleRecipientType>, 
	#[yaserde(rename = "InternetMessageId", prefix = "nsi1", default)]
	pub internet_message_id: Option<String>, 
	#[yaserde(rename = "IsRead", prefix = "nsi1", default)]
	pub is_read: Option<bool>, 
	#[yaserde(rename = "IsResponseRequested", prefix = "nsi1", default)]
	pub is_response_requested: Option<bool>, 
	#[yaserde(rename = "References", prefix = "nsi1", default)]
	pub references: Option<String>, 
	#[yaserde(rename = "ReplyTo", prefix = "nsi1", default)]
	pub reply_to: Option<ArrayOfRecipientsType>, 
	#[yaserde(rename = "ReceivedBy", prefix = "nsi1", default)]
	pub received_by: Option<SingleRecipientType>, 
	#[yaserde(rename = "ReceivedRepresenting", prefix = "nsi1", default)]
	pub received_representing: Option<SingleRecipientType>, 
	#[yaserde(rename = "ApprovalRequestData", prefix = "nsi1", default)]
	pub approval_request_data: Option<ApprovalRequestDataType>, 
	#[yaserde(rename = "VotingInformation", prefix = "nsi1", default)]
	pub voting_information: Option<VotingInformationType>, 
	#[yaserde(rename = "ReminderMessageData", prefix = "nsi1", default)]
	pub reminder_message_data: Option<ReminderMessageDataType>, 
	#[yaserde(rename = "MessageSafety", prefix = "nsi1", default)]
	pub message_safety: Option<MessageSafetyType>, 
	#[yaserde(rename = "SenderSMTPAddress", prefix = "nsi1", default)]
	pub sender_smtp_address: Option<SmtpAddressType>, 
	#[yaserde(rename = "MailboxGuids", prefix = "nsi1", default)]
	pub mailbox_guids: Option<MailboxGuids>, 
	#[yaserde(rename = "PublishedCalendarItemIcs", prefix = "nsi1", default)]
	pub published_calendar_item_ics: Option<String>, 
	#[yaserde(rename = "PublishedCalendarItemName", prefix = "nsi1", default)]
	pub published_calendar_item_name: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "TaskStatusType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct TaskStatusType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "TaskDelegateStateType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct TaskDelegateStateType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "TaskType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct TaskType {
	#[yaserde(flatten, default)]
	pub item_type: ItemType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ActualWork", prefix = "nsi1", default)]
	pub actual_work: Option<i32>, 
	#[yaserde(rename = "AssignedTime", prefix = "nsi1", default)]
	pub assigned_time: Option<String>, 
	#[yaserde(rename = "BillingInformation", prefix = "nsi1", default)]
	pub billing_information: Option<String>, 
	#[yaserde(rename = "ChangeCount", prefix = "nsi1", default)]
	pub change_count: Option<i32>, 
	#[yaserde(rename = "Companies", prefix = "nsi1", default)]
	pub companies: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "CompleteDate", prefix = "nsi1", default)]
	pub complete_date: Option<String>, 
	#[yaserde(rename = "Contacts", prefix = "nsi1", default)]
	pub contacts: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "DelegationState", prefix = "nsi1", default)]
	pub delegation_state: Option<TaskDelegateStateType>, 
	#[yaserde(rename = "Delegator", prefix = "nsi1", default)]
	pub delegator: Option<String>, 
	#[yaserde(rename = "DueDate", prefix = "nsi1", default)]
	pub due_date: Option<String>, 
	#[yaserde(rename = "IsAssignmentEditable", prefix = "nsi1", default)]
	pub is_assignment_editable: Option<i32>, 
	#[yaserde(rename = "IsComplete", prefix = "nsi1", default)]
	pub is_complete: Option<bool>, 
	#[yaserde(rename = "IsRecurring", prefix = "nsi1", default)]
	pub is_recurring: Option<bool>, 
	#[yaserde(rename = "IsTeamTask", prefix = "nsi1", default)]
	pub is_team_task: Option<bool>, 
	#[yaserde(rename = "Mileage", prefix = "nsi1", default)]
	pub mileage: Option<String>, 
	#[yaserde(rename = "Owner", prefix = "nsi1", default)]
	pub owner: Option<String>, 
	#[yaserde(rename = "PercentComplete", prefix = "nsi1", default)]
	pub percent_complete: Option<f64>, 
	#[yaserde(rename = "Recurrence", prefix = "nsi1", default)]
	pub recurrence: Option<TaskRecurrenceType>, 
	#[yaserde(rename = "StartDate", prefix = "nsi1", default)]
	pub start_date: Option<String>, 
	#[yaserde(rename = "Status", prefix = "nsi1", default)]
	pub status: Option<TaskStatusType>, 
	#[yaserde(rename = "StatusDescription", prefix = "nsi1", default)]
	pub status_description: Option<String>, 
	#[yaserde(rename = "TotalWork", prefix = "nsi1", default)]
	pub total_work: Option<i32>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PostItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PostItemType {
	#[yaserde(flatten, default)]
	pub item_type: ItemType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ConversationIndex", prefix = "nsi1", default)]
	pub conversation_index: Option<String>, 
	#[yaserde(rename = "ConversationTopic", prefix = "nsi1", default)]
	pub conversation_topic: Option<String>, 
	#[yaserde(rename = "From", prefix = "nsi1", default)]
	pub from: Option<SingleRecipientType>, 
	#[yaserde(rename = "InternetMessageId", prefix = "nsi1", default)]
	pub internet_message_id: Option<String>, 
	#[yaserde(rename = "IsRead", prefix = "nsi1", default)]
	pub is_read: Option<bool>, 
	#[yaserde(rename = "PostedTime", prefix = "nsi1", default)]
	pub posted_time: Option<String>, 
	#[yaserde(rename = "References", prefix = "nsi1", default)]
	pub references: Option<String>, 
	#[yaserde(rename = "Sender", prefix = "nsi1", default)]
	pub sender: Option<SingleRecipientType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SharingMessageType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SharingMessageType {
	#[yaserde(flatten, default)]
	pub message_type: MessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SharingMessageAction", prefix = "nsi1", default)]
	pub sharing_message_action: Option<SharingMessageActionType>, 
	#[yaserde(rename = "SharingMessageActions", prefix = "nsi1", default)]
	pub sharing_message_actions: Option<ArrayOfSharingMessageActionType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfSharingMessageActionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfSharingMessageActionType {
	#[yaserde(rename = "SharingMessageAction", prefix = "nsi1", default)]
	pub sharing_message_action: Option<SharingMessageActionType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SharingMessageActionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SharingMessageActionType {
	#[yaserde(rename = "Importance", prefix = "nsi1", default)]
	pub importance: Option<SharingActionImportance>, 
	#[yaserde(rename = "ActionType", prefix = "nsi1", default)]
	pub action_type: Option<SharingActionType>, 
	#[yaserde(rename = "Action", prefix = "nsi1", default)]
	pub action: Option<SharingAction>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SharingActionImportance",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SharingActionImportance {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SharingAction",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SharingAction {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SharingActionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SharingActionType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BasePagingType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct BasePagingType {
#[yaserde(rename="MaxEntriesReturned", attribute)]
pub max_entries_returned: Option<i32>,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "IndexBasePointType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct IndexBasePointType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "IndexedPageViewType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct IndexedPageViewType {
	#[yaserde(flatten, default)]
	pub base_paging_type: BasePagingType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FractionalPageViewType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FractionalPageViewType {
	#[yaserde(flatten, default)]
	pub base_paging_type: BasePagingType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SeekToConditionPageViewType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SeekToConditionPageViewType {
	#[yaserde(flatten, default)]
	pub base_paging_type: BasePagingType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Condition", prefix = "nsi1", default)]
	pub condition: RestrictionType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CalendarViewType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct CalendarViewType {
	#[yaserde(flatten, default)]
	pub base_paging_type: BasePagingType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ContactsViewType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ContactsViewType {
	#[yaserde(flatten, default)]
	pub base_paging_type: BasePagingType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ResolveNamesSearchScopeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ResolveNamesSearchScopeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ResolutionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ResolutionType {
	#[yaserde(rename = "Mailbox", prefix = "nsi1", default)]
	pub mailbox: EmailAddressType, 
	#[yaserde(rename = "Contact", prefix = "nsi1", default)]
	pub contact: Option<ContactItemType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfResolutionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfResolutionType {
	#[yaserde(rename = "Resolution", prefix = "nsi1", default)]
	pub resolution: Vec<ResolutionType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfDLExpansionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfDLExpansionType {
	#[yaserde(rename = "Mailbox", prefix = "nsi1", default)]
	pub mailbox: Vec<EmailAddressType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfTimeZoneIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfTimeZoneIdType {
	#[yaserde(rename = "Id", prefix = "nsi1", default)]
	pub id: Vec<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfTimeZoneDefinitionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfTimeZoneDefinitionType {
	#[yaserde(rename = "TimeZoneDefinition", prefix = "nsi1", default)]
	pub time_zone_definition: TimeZoneDefinitionType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MeetingRequestTypeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MeetingRequestTypeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ReminderMinutesBeforeStartType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ReminderMinutesBeforeStartType {
	#[yaserde(flatten, default)]
	pub body: Box<ReminderMinutesBeforeStartType>
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AvailabilityStatusType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AvailabilityStatusType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "LegacyFreeBusyType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct LegacyFreeBusyType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CalendarItemTypeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct CalendarItemTypeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ResponseTypeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ResponseTypeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "OnlineMeetingSettingsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct OnlineMeetingSettingsType {
	#[yaserde(rename = "LobbyBypass", prefix = "nsi1", default)]
	pub lobby_bypass: LobbyBypassType, 
	#[yaserde(rename = "AccessLevel", prefix = "nsi1", default)]
	pub access_level: OnlineMeetingAccessLevelType, 
	#[yaserde(rename = "Presenters", prefix = "nsi1", default)]
	pub presenters: PresentersType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "LobbyBypassType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct LobbyBypassType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "OnlineMeetingAccessLevelType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct OnlineMeetingAccessLevelType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PresentersType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PresentersType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AttendeeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AttendeeType {
	#[yaserde(rename = "Mailbox", prefix = "nsi1", default)]
	pub mailbox: EmailAddressType, 
	#[yaserde(rename = "ResponseType", prefix = "nsi1", default)]
	pub response_type: Option<ResponseTypeType>, 
	#[yaserde(rename = "LastResponseTime", prefix = "nsi1", default)]
	pub last_response_time: Option<String>, 
	#[yaserde(rename = "ProposedStart", prefix = "nsi1", default)]
	pub proposed_start: Option<String>, 
	#[yaserde(rename = "ProposedEnd", prefix = "nsi1", default)]
	pub proposed_end: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfAttendeesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfAttendeesType {
	#[yaserde(rename = "Attendee", prefix = "nsi1", default)]
	pub attendee: Vec<AttendeeType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "OccurrenceItemIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct OccurrenceItemIdType {
	#[yaserde(flatten, default)]
	pub base_item_id_type: BaseItemIdType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RecurringMasterItemIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RecurringMasterItemIdType {
	#[yaserde(flatten, default)]
	pub base_item_id_type: BaseItemIdType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RecurringMasterItemIdRangesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RecurringMasterItemIdRangesType {
	#[yaserde(flatten, default)]
	pub item_id_type: ItemIdType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Ranges", prefix = "nsi1", default)]
	pub ranges: Option<ArrayOfOccurrenceRangesType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfOccurrenceRangesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfOccurrenceRangesType {
	#[yaserde(rename = "Range", prefix = "nsi1", default)]
	pub range: Vec<OccurrencesRangeType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "OccurrencesRangeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct OccurrencesRangeType {
#[yaserde(rename="Start", attribute)]
pub start: Option<String>,
#[yaserde(rename="End", attribute)]
pub end: Option<String>,
#[yaserde(rename="Count", attribute)]
pub count: Option<i32>,
#[yaserde(rename="CompareOriginalStartTime", attribute)]
pub compare_original_start_time: Option<bool>,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DayOfWeekType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DayOfWeekType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DaysOfWeekType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DaysOfWeekType {
	#[yaserde(flatten, default)]
	pub body: Box<DaysOfWeekType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DayOfWeekIndexType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DayOfWeekIndexType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MonthNamesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MonthNamesType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RecurrencePatternBaseType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RecurrencePatternBaseType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "IntervalRecurrencePatternBaseType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct IntervalRecurrencePatternBaseType {
	#[yaserde(flatten, default)]
	pub recurrence_pattern_base_type: RecurrencePatternBaseType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Interval", prefix = "nsi1", default)]
	pub interval: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RegeneratingPatternBaseType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RegeneratingPatternBaseType {
	#[yaserde(flatten, default)]
	pub interval_recurrence_pattern_base_type: IntervalRecurrencePatternBaseType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DailyRegeneratingPatternType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DailyRegeneratingPatternType {
	#[yaserde(flatten, default)]
	pub regenerating_pattern_base_type: RegeneratingPatternBaseType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "WeeklyRegeneratingPatternType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct WeeklyRegeneratingPatternType {
	#[yaserde(flatten, default)]
	pub regenerating_pattern_base_type: RegeneratingPatternBaseType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MonthlyRegeneratingPatternType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MonthlyRegeneratingPatternType {
	#[yaserde(flatten, default)]
	pub regenerating_pattern_base_type: RegeneratingPatternBaseType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "YearlyRegeneratingPatternType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct YearlyRegeneratingPatternType {
	#[yaserde(flatten, default)]
	pub regenerating_pattern_base_type: RegeneratingPatternBaseType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RelativeYearlyRecurrencePatternType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RelativeYearlyRecurrencePatternType {
	#[yaserde(flatten, default)]
	pub recurrence_pattern_base_type: RecurrencePatternBaseType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "DaysOfWeek", prefix = "nsi1", default)]
	pub days_of_week: DayOfWeekType, 
	#[yaserde(rename = "DayOfWeekIndex", prefix = "nsi1", default)]
	pub day_of_week_index: DayOfWeekIndexType, 
	#[yaserde(rename = "Month", prefix = "nsi1", default)]
	pub month: MonthNamesType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AbsoluteYearlyRecurrencePatternType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AbsoluteYearlyRecurrencePatternType {
	#[yaserde(flatten, default)]
	pub recurrence_pattern_base_type: RecurrencePatternBaseType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "DayOfMonth", prefix = "nsi1", default)]
	pub day_of_month: i32, 
	#[yaserde(rename = "Month", prefix = "nsi1", default)]
	pub month: MonthNamesType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RelativeMonthlyRecurrencePatternType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RelativeMonthlyRecurrencePatternType {
	#[yaserde(flatten, default)]
	pub interval_recurrence_pattern_base_type: IntervalRecurrencePatternBaseType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "DaysOfWeek", prefix = "nsi1", default)]
	pub days_of_week: DayOfWeekType, 
	#[yaserde(rename = "DayOfWeekIndex", prefix = "nsi1", default)]
	pub day_of_week_index: DayOfWeekIndexType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AbsoluteMonthlyRecurrencePatternType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AbsoluteMonthlyRecurrencePatternType {
	#[yaserde(flatten, default)]
	pub interval_recurrence_pattern_base_type: IntervalRecurrencePatternBaseType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "DayOfMonth", prefix = "nsi1", default)]
	pub day_of_month: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "WeeklyRecurrencePatternType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct WeeklyRecurrencePatternType {
	#[yaserde(flatten, default)]
	pub interval_recurrence_pattern_base_type: IntervalRecurrencePatternBaseType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "DaysOfWeek", prefix = "nsi1", default)]
	pub days_of_week: DaysOfWeekType, 
	#[yaserde(rename = "FirstDayOfWeek", prefix = "nsi1", default)]
	pub first_day_of_week: Option<DayOfWeekType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DailyRecurrencePatternType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DailyRecurrencePatternType {
	#[yaserde(flatten, default)]
	pub interval_recurrence_pattern_base_type: IntervalRecurrencePatternBaseType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "LocationSourceType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct LocationSourceType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EnhancedLocationType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct EnhancedLocationType {
	#[yaserde(rename = "DisplayName", prefix = "nsi1", default)]
	pub display_name: String, 
	#[yaserde(rename = "Annotation", prefix = "nsi1", default)]
	pub annotation: Option<String>, 
	#[yaserde(rename = "PostalAddress", prefix = "nsi1", default)]
	pub postal_address: Option<PersonaPostalAddressType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "TimeChangeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct TimeChangeType {
#[yaserde(rename="TimeZoneName", attribute)]
pub time_zone_name: Option<String>,
	#[yaserde(rename = "Offset", prefix = "nsi1", default)]
	pub offset: Duration, 
	#[yaserde(rename = "Time", prefix = "nsi1", default)]
	pub time: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "TimeZoneType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct TimeZoneType {
#[yaserde(rename="TimeZoneName", attribute)]
pub time_zone_name: Option<String>,
	#[yaserde(rename = "BaseOffset", prefix = "nsi1", default)]
	pub base_offset: Duration, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "TimeZoneContextType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct TimeZoneContextType {
	#[yaserde(rename = "TimeZoneDefinition", prefix = "nsi1", default)]
	pub time_zone_definition: TimeZoneDefinitionType, 
}
pub type TimeZoneContext = TimeZoneContextType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "TransitionTargetKindType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct TransitionTargetKindType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "TransitionTargetType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct TransitionTargetType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "TransitionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct TransitionType {
	#[yaserde(rename = "To", prefix = "nsi1", default)]
	pub to: TransitionTargetType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AbsoluteDateTransitionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AbsoluteDateTransitionType {
	#[yaserde(flatten, default)]
	pub transition_type: TransitionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "DateTime", prefix = "nsi1", default)]
	pub date_time: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RecurringTimeTransitionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RecurringTimeTransitionType {
	#[yaserde(flatten, default)]
	pub transition_type: TransitionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "TimeOffset", prefix = "nsi1", default)]
	pub time_offset: Duration, 
	#[yaserde(rename = "Month", prefix = "nsi1", default)]
	pub month: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RecurringDateTransitionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RecurringDateTransitionType {
	#[yaserde(flatten, default)]
	pub recurring_time_transition_type: RecurringTimeTransitionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Day", prefix = "nsi1", default)]
	pub day: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RecurringDayTransitionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RecurringDayTransitionType {
	#[yaserde(flatten, default)]
	pub recurring_time_transition_type: RecurringTimeTransitionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "DayOfWeek", prefix = "nsi1", default)]
	pub day_of_week: DayOfWeekType, 
	#[yaserde(rename = "Occurrence", prefix = "nsi1", default)]
	pub occurrence: i32, 
}
pub type Transition = TransitionType;

pub type AbsoluteDateTransition = AbsoluteDateTransitionType;

pub type RecurringDayTransition = RecurringDayTransitionType;

pub type RecurringDateTransition = RecurringDateTransitionType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "TimeZoneDefinitionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct TimeZoneDefinitionType {
#[yaserde(rename="Id", attribute)]
pub id: String,
#[yaserde(rename="Name", attribute)]
pub name: String,
	#[yaserde(rename = "Periods", prefix = "nsi1", default)]
	pub periods: NonEmptyArrayOfPeriodsType, 
	#[yaserde(rename = "TransitionsGroups", prefix = "nsi1", default)]
	pub transitions_groups: Option<ArrayOfTransitionsGroupsType>, 
	#[yaserde(rename = "Transitions", prefix = "nsi1", default)]
	pub transitions: Option<ArrayOfTransitionsType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfPeriodsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfPeriodsType {
	#[yaserde(rename = "Period", prefix = "nsi1", default)]
	pub period: Vec<PeriodType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PeriodType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PeriodType {
#[yaserde(rename="Bias", attribute)]
pub bias: Duration,
#[yaserde(rename="Name", attribute)]
pub name: String,
#[yaserde(rename="Id", attribute)]
pub id: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfTransitionsGroupsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfTransitionsGroupsType {
	#[yaserde(rename = "TransitionsGroup", prefix = "nsi1", default)]
	pub transitions_group: Vec<ArrayOfTransitionsType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfTransitionsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfTransitionsType {
#[yaserde(rename="Id", attribute)]
pub id: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RecurrenceRangeBaseType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RecurrenceRangeBaseType {
	#[yaserde(rename = "StartDate", prefix = "nsi1", default)]
	pub start_date: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NoEndRecurrenceRangeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NoEndRecurrenceRangeType {
	#[yaserde(flatten, default)]
	pub recurrence_range_base_type: RecurrenceRangeBaseType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EndDateRecurrenceRangeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct EndDateRecurrenceRangeType {
	#[yaserde(flatten, default)]
	pub recurrence_range_base_type: RecurrenceRangeBaseType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "EndDate", prefix = "nsi1", default)]
	pub end_date: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NumberedRecurrenceRangeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NumberedRecurrenceRangeType {
	#[yaserde(flatten, default)]
	pub recurrence_range_base_type: RecurrenceRangeBaseType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "NumberOfOccurrences", prefix = "nsi1", default)]
	pub number_of_occurrences: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RecurrenceType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RecurrenceType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "TaskRecurrenceType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct TaskRecurrenceType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "OccurrenceInfoType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct OccurrenceInfoType {
	#[yaserde(rename = "ItemId", prefix = "nsi1", default)]
	pub item_id: ItemIdType, 
	#[yaserde(rename = "Start", prefix = "nsi1", default)]
	pub start: String, 
	#[yaserde(rename = "End", prefix = "nsi1", default)]
	pub end: String, 
	#[yaserde(rename = "OriginalStart", prefix = "nsi1", default)]
	pub original_start: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfOccurrenceInfoType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfOccurrenceInfoType {
	#[yaserde(rename = "Occurrence", prefix = "nsi1", default)]
	pub occurrence: Vec<OccurrenceInfoType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeletedOccurrenceInfoType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DeletedOccurrenceInfoType {
	#[yaserde(rename = "Start", prefix = "nsi1", default)]
	pub start: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfDeletedOccurrencesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfDeletedOccurrencesType {
	#[yaserde(rename = "DeletedOccurrence", prefix = "nsi1", default)]
	pub deleted_occurrence: Vec<DeletedOccurrenceInfoType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CalendarItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct CalendarItemType {
	#[yaserde(flatten, default)]
	pub item_type: ItemType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "UID", prefix = "nsi1", default)]
	pub uid: Option<String>, 
	#[yaserde(rename = "RecurrenceId", prefix = "nsi1", default)]
	pub recurrence_id: Option<String>, 
	#[yaserde(rename = "DateTimeStamp", prefix = "nsi1", default)]
	pub date_time_stamp: Option<String>, 
	#[yaserde(rename = "Start", prefix = "nsi1", default)]
	pub start: Option<String>, 
	#[yaserde(rename = "End", prefix = "nsi1", default)]
	pub end: Option<String>, 
	#[yaserde(rename = "OriginalStart", prefix = "nsi1", default)]
	pub original_start: Option<String>, 
	#[yaserde(rename = "IsAllDayEvent", prefix = "nsi1", default)]
	pub is_all_day_event: Option<bool>, 
	#[yaserde(rename = "LegacyFreeBusyStatus", prefix = "nsi1", default)]
	pub legacy_free_busy_status: Option<LegacyFreeBusyType>, 
	#[yaserde(rename = "Location", prefix = "nsi1", default)]
	pub location: Option<String>, 
	#[yaserde(rename = "When", prefix = "nsi1", default)]
	pub when: Option<String>, 
	#[yaserde(rename = "IsMeeting", prefix = "nsi1", default)]
	pub is_meeting: Option<bool>, 
	#[yaserde(rename = "IsCancelled", prefix = "nsi1", default)]
	pub is_cancelled: Option<bool>, 
	#[yaserde(rename = "IsRecurring", prefix = "nsi1", default)]
	pub is_recurring: Option<bool>, 
	#[yaserde(rename = "MeetingRequestWasSent", prefix = "nsi1", default)]
	pub meeting_request_was_sent: Option<bool>, 
	#[yaserde(rename = "IsResponseRequested", prefix = "nsi1", default)]
	pub is_response_requested: Option<bool>, 
	#[yaserde(rename = "CalendarItemType", prefix = "nsi1", default)]
	pub calendar_item_type: Option<CalendarItemTypeType>, 
	#[yaserde(rename = "MyResponseType", prefix = "nsi1", default)]
	pub my_response_type: Option<ResponseTypeType>, 
	#[yaserde(rename = "Organizer", prefix = "nsi1", default)]
	pub organizer: Option<SingleRecipientType>, 
	#[yaserde(rename = "RequiredAttendees", prefix = "nsi1", default)]
	pub required_attendees: Option<NonEmptyArrayOfAttendeesType>, 
	#[yaserde(rename = "OptionalAttendees", prefix = "nsi1", default)]
	pub optional_attendees: Option<NonEmptyArrayOfAttendeesType>, 
	#[yaserde(rename = "Resources", prefix = "nsi1", default)]
	pub resources: Option<NonEmptyArrayOfAttendeesType>, 
	#[yaserde(rename = "InboxReminders", prefix = "nsi1", default)]
	pub inbox_reminders: Option<ArrayOfInboxReminderType>, 
	#[yaserde(rename = "ConflictingMeetingCount", prefix = "nsi1", default)]
	pub conflicting_meeting_count: Option<i32>, 
	#[yaserde(rename = "AdjacentMeetingCount", prefix = "nsi1", default)]
	pub adjacent_meeting_count: Option<i32>, 
	#[yaserde(rename = "ConflictingMeetings", prefix = "nsi1", default)]
	pub conflicting_meetings: Option<NonEmptyArrayOfAllItemsType>, 
	#[yaserde(rename = "AdjacentMeetings", prefix = "nsi1", default)]
	pub adjacent_meetings: Option<NonEmptyArrayOfAllItemsType>, 
	#[yaserde(rename = "Duration", prefix = "nsi1", default)]
	pub duration: Option<String>, 
	#[yaserde(rename = "TimeZone", prefix = "nsi1", default)]
	pub time_zone: Option<String>, 
	#[yaserde(rename = "AppointmentReplyTime", prefix = "nsi1", default)]
	pub appointment_reply_time: Option<String>, 
	#[yaserde(rename = "AppointmentSequenceNumber", prefix = "nsi1", default)]
	pub appointment_sequence_number: Option<i32>, 
	#[yaserde(rename = "AppointmentState", prefix = "nsi1", default)]
	pub appointment_state: Option<i32>, 
	#[yaserde(rename = "Recurrence", prefix = "nsi1", default)]
	pub recurrence: Option<RecurrenceType>, 
	#[yaserde(rename = "FirstOccurrence", prefix = "nsi1", default)]
	pub first_occurrence: Option<OccurrenceInfoType>, 
	#[yaserde(rename = "LastOccurrence", prefix = "nsi1", default)]
	pub last_occurrence: Option<OccurrenceInfoType>, 
	#[yaserde(rename = "ModifiedOccurrences", prefix = "nsi1", default)]
	pub modified_occurrences: Option<NonEmptyArrayOfOccurrenceInfoType>, 
	#[yaserde(rename = "DeletedOccurrences", prefix = "nsi1", default)]
	pub deleted_occurrences: Option<NonEmptyArrayOfDeletedOccurrencesType>, 
	#[yaserde(rename = "MeetingTimeZone", prefix = "nsi1", default)]
	pub meeting_time_zone: Option<TimeZoneType>, 
	#[yaserde(rename = "StartTimeZone", prefix = "nsi1", default)]
	pub start_time_zone: Option<TimeZoneDefinitionType>, 
	#[yaserde(rename = "EndTimeZone", prefix = "nsi1", default)]
	pub end_time_zone: Option<TimeZoneDefinitionType>, 
	#[yaserde(rename = "ConferenceType", prefix = "nsi1", default)]
	pub conference_type: Option<i32>, 
	#[yaserde(rename = "AllowNewTimeProposal", prefix = "nsi1", default)]
	pub allow_new_time_proposal: Option<bool>, 
	#[yaserde(rename = "IsOnlineMeeting", prefix = "nsi1", default)]
	pub is_online_meeting: Option<bool>, 
	#[yaserde(rename = "MeetingWorkspaceUrl", prefix = "nsi1", default)]
	pub meeting_workspace_url: Option<String>, 
	#[yaserde(rename = "NetShowUrl", prefix = "nsi1", default)]
	pub net_show_url: Option<String>, 
	#[yaserde(rename = "EnhancedLocation", prefix = "nsi1", default)]
	pub enhanced_location: Option<EnhancedLocationType>, 
	#[yaserde(rename = "StartWallClock", prefix = "nsi1", default)]
	pub start_wall_clock: Option<String>, 
	#[yaserde(rename = "EndWallClock", prefix = "nsi1", default)]
	pub end_wall_clock: Option<String>, 
	#[yaserde(rename = "StartTimeZoneId", prefix = "nsi1", default)]
	pub start_time_zone_id: Option<String>, 
	#[yaserde(rename = "EndTimeZoneId", prefix = "nsi1", default)]
	pub end_time_zone_id: Option<String>, 
	#[yaserde(rename = "IntendedFreeBusyStatus", prefix = "nsi1", default)]
	pub intended_free_busy_status: Option<LegacyFreeBusyType>, 
	#[yaserde(rename = "JoinOnlineMeetingUrl", prefix = "nsi1", default)]
	pub join_online_meeting_url: Option<String>, 
	#[yaserde(rename = "OnlineMeetingSettings", prefix = "nsi1", default)]
	pub online_meeting_settings: Option<OnlineMeetingSettingsType>, 
	#[yaserde(rename = "IsOrganizer", prefix = "nsi1", default)]
	pub is_organizer: Option<bool>, 
	#[yaserde(rename = "CalendarActivityData", prefix = "nsi1", default)]
	pub calendar_activity_data: Option<CalendarActivityDataType>, 
	#[yaserde(rename = "DoNotForwardMeeting", prefix = "nsi1", default)]
	pub do_not_forward_meeting: Option<bool>, 
	#[yaserde(rename = "RequestedAttendanceMode", prefix = "nsi1", default)]
	pub requested_attendance_mode: Option<RequestedAttendanceModeType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MeetingMessageType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MeetingMessageType {
	#[yaserde(flatten, default)]
	pub message_type: MessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "AssociatedCalendarItemId", prefix = "nsi1", default)]
	pub associated_calendar_item_id: Option<ItemIdType>, 
	#[yaserde(rename = "IsDelegated", prefix = "nsi1", default)]
	pub is_delegated: Option<bool>, 
	#[yaserde(rename = "IsOutOfDate", prefix = "nsi1", default)]
	pub is_out_of_date: Option<bool>, 
	#[yaserde(rename = "HasBeenProcessed", prefix = "nsi1", default)]
	pub has_been_processed: Option<bool>, 
	#[yaserde(rename = "ResponseType", prefix = "nsi1", default)]
	pub response_type: Option<ResponseTypeType>, 
	#[yaserde(rename = "UID", prefix = "nsi1", default)]
	pub uid: Option<String>, 
	#[yaserde(rename = "RecurrenceId", prefix = "nsi1", default)]
	pub recurrence_id: Option<String>, 
	#[yaserde(rename = "DateTimeStamp", prefix = "nsi1", default)]
	pub date_time_stamp: Option<String>, 
	#[yaserde(rename = "IsOrganizer", prefix = "nsi1", default)]
	pub is_organizer: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ChangeHighlightsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ChangeHighlightsType {
	#[yaserde(rename = "HasLocationChanged", prefix = "nsi1", default)]
	pub has_location_changed: Option<bool>, 
	#[yaserde(rename = "Location", prefix = "nsi1", default)]
	pub location: Option<String>, 
	#[yaserde(rename = "HasStartTimeChanged", prefix = "nsi1", default)]
	pub has_start_time_changed: Option<bool>, 
	#[yaserde(rename = "Start", prefix = "nsi1", default)]
	pub start: Option<String>, 
	#[yaserde(rename = "HasEndTimeChanged", prefix = "nsi1", default)]
	pub has_end_time_changed: Option<bool>, 
	#[yaserde(rename = "End", prefix = "nsi1", default)]
	pub end: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MeetingRequestMessageType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MeetingRequestMessageType {
	#[yaserde(flatten, default)]
	pub meeting_message_type: MeetingMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MeetingRequestType", prefix = "nsi1", default)]
	pub meeting_request_type: Option<MeetingRequestTypeType>, 
	#[yaserde(rename = "IntendedFreeBusyStatus", prefix = "nsi1", default)]
	pub intended_free_busy_status: Option<LegacyFreeBusyType>, 
	#[yaserde(rename = "Start", prefix = "nsi1", default)]
	pub start: Option<String>, 
	#[yaserde(rename = "End", prefix = "nsi1", default)]
	pub end: Option<String>, 
	#[yaserde(rename = "OriginalStart", prefix = "nsi1", default)]
	pub original_start: Option<String>, 
	#[yaserde(rename = "IsAllDayEvent", prefix = "nsi1", default)]
	pub is_all_day_event: Option<bool>, 
	#[yaserde(rename = "LegacyFreeBusyStatus", prefix = "nsi1", default)]
	pub legacy_free_busy_status: Option<LegacyFreeBusyType>, 
	#[yaserde(rename = "Location", prefix = "nsi1", default)]
	pub location: Option<String>, 
	#[yaserde(rename = "When", prefix = "nsi1", default)]
	pub when: Option<String>, 
	#[yaserde(rename = "IsMeeting", prefix = "nsi1", default)]
	pub is_meeting: Option<bool>, 
	#[yaserde(rename = "IsCancelled", prefix = "nsi1", default)]
	pub is_cancelled: Option<bool>, 
	#[yaserde(rename = "IsRecurring", prefix = "nsi1", default)]
	pub is_recurring: Option<bool>, 
	#[yaserde(rename = "MeetingRequestWasSent", prefix = "nsi1", default)]
	pub meeting_request_was_sent: Option<bool>, 
	#[yaserde(rename = "CalendarItemType", prefix = "nsi1", default)]
	pub calendar_item_type: Option<CalendarItemTypeType>, 
	#[yaserde(rename = "MyResponseType", prefix = "nsi1", default)]
	pub my_response_type: Option<ResponseTypeType>, 
	#[yaserde(rename = "Organizer", prefix = "nsi1", default)]
	pub organizer: Option<SingleRecipientType>, 
	#[yaserde(rename = "RequiredAttendees", prefix = "nsi1", default)]
	pub required_attendees: Option<NonEmptyArrayOfAttendeesType>, 
	#[yaserde(rename = "OptionalAttendees", prefix = "nsi1", default)]
	pub optional_attendees: Option<NonEmptyArrayOfAttendeesType>, 
	#[yaserde(rename = "Resources", prefix = "nsi1", default)]
	pub resources: Option<NonEmptyArrayOfAttendeesType>, 
	#[yaserde(rename = "ConflictingMeetingCount", prefix = "nsi1", default)]
	pub conflicting_meeting_count: Option<i32>, 
	#[yaserde(rename = "AdjacentMeetingCount", prefix = "nsi1", default)]
	pub adjacent_meeting_count: Option<i32>, 
	#[yaserde(rename = "ConflictingMeetings", prefix = "nsi1", default)]
	pub conflicting_meetings: Option<NonEmptyArrayOfAllItemsType>, 
	#[yaserde(rename = "AdjacentMeetings", prefix = "nsi1", default)]
	pub adjacent_meetings: Option<NonEmptyArrayOfAllItemsType>, 
	#[yaserde(rename = "Duration", prefix = "nsi1", default)]
	pub duration: Option<String>, 
	#[yaserde(rename = "TimeZone", prefix = "nsi1", default)]
	pub time_zone: Option<String>, 
	#[yaserde(rename = "AppointmentReplyTime", prefix = "nsi1", default)]
	pub appointment_reply_time: Option<String>, 
	#[yaserde(rename = "AppointmentSequenceNumber", prefix = "nsi1", default)]
	pub appointment_sequence_number: Option<i32>, 
	#[yaserde(rename = "AppointmentState", prefix = "nsi1", default)]
	pub appointment_state: Option<i32>, 
	#[yaserde(rename = "Recurrence", prefix = "nsi1", default)]
	pub recurrence: Option<RecurrenceType>, 
	#[yaserde(rename = "FirstOccurrence", prefix = "nsi1", default)]
	pub first_occurrence: Option<OccurrenceInfoType>, 
	#[yaserde(rename = "LastOccurrence", prefix = "nsi1", default)]
	pub last_occurrence: Option<OccurrenceInfoType>, 
	#[yaserde(rename = "ModifiedOccurrences", prefix = "nsi1", default)]
	pub modified_occurrences: Option<NonEmptyArrayOfOccurrenceInfoType>, 
	#[yaserde(rename = "DeletedOccurrences", prefix = "nsi1", default)]
	pub deleted_occurrences: Option<NonEmptyArrayOfDeletedOccurrencesType>, 
	#[yaserde(rename = "MeetingTimeZone", prefix = "nsi1", default)]
	pub meeting_time_zone: Option<TimeZoneType>, 
	#[yaserde(rename = "StartTimeZone", prefix = "nsi1", default)]
	pub start_time_zone: Option<TimeZoneDefinitionType>, 
	#[yaserde(rename = "EndTimeZone", prefix = "nsi1", default)]
	pub end_time_zone: Option<TimeZoneDefinitionType>, 
	#[yaserde(rename = "ConferenceType", prefix = "nsi1", default)]
	pub conference_type: Option<i32>, 
	#[yaserde(rename = "AllowNewTimeProposal", prefix = "nsi1", default)]
	pub allow_new_time_proposal: Option<bool>, 
	#[yaserde(rename = "IsOnlineMeeting", prefix = "nsi1", default)]
	pub is_online_meeting: Option<bool>, 
	#[yaserde(rename = "MeetingWorkspaceUrl", prefix = "nsi1", default)]
	pub meeting_workspace_url: Option<String>, 
	#[yaserde(rename = "NetShowUrl", prefix = "nsi1", default)]
	pub net_show_url: Option<String>, 
	#[yaserde(rename = "EnhancedLocation", prefix = "nsi1", default)]
	pub enhanced_location: Option<EnhancedLocationType>, 
	#[yaserde(rename = "ChangeHighlights", prefix = "nsi1", default)]
	pub change_highlights: Option<ChangeHighlightsType>, 
	#[yaserde(rename = "StartWallClock", prefix = "nsi1", default)]
	pub start_wall_clock: Option<String>, 
	#[yaserde(rename = "EndWallClock", prefix = "nsi1", default)]
	pub end_wall_clock: Option<String>, 
	#[yaserde(rename = "StartTimeZoneId", prefix = "nsi1", default)]
	pub start_time_zone_id: Option<String>, 
	#[yaserde(rename = "EndTimeZoneId", prefix = "nsi1", default)]
	pub end_time_zone_id: Option<String>, 
	#[yaserde(rename = "DoNotForwardMeeting", prefix = "nsi1", default)]
	pub do_not_forward_meeting: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MeetingResponseMessageType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MeetingResponseMessageType {
	#[yaserde(flatten, default)]
	pub meeting_message_type: MeetingMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Start", prefix = "nsi1", default)]
	pub start: Option<String>, 
	#[yaserde(rename = "End", prefix = "nsi1", default)]
	pub end: Option<String>, 
	#[yaserde(rename = "Location", prefix = "nsi1", default)]
	pub location: Option<String>, 
	#[yaserde(rename = "Recurrence", prefix = "nsi1", default)]
	pub recurrence: Option<RecurrenceType>, 
	#[yaserde(rename = "CalendarItemType", prefix = "nsi1", default)]
	pub calendar_item_type: Option<String>, 
	#[yaserde(rename = "ProposedStart", prefix = "nsi1", default)]
	pub proposed_start: Option<String>, 
	#[yaserde(rename = "ProposedEnd", prefix = "nsi1", default)]
	pub proposed_end: Option<String>, 
	#[yaserde(rename = "EnhancedLocation", prefix = "nsi1", default)]
	pub enhanced_location: Option<EnhancedLocationType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MeetingCancellationMessageType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MeetingCancellationMessageType {
	#[yaserde(flatten, default)]
	pub meeting_message_type: MeetingMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Start", prefix = "nsi1", default)]
	pub start: Option<String>, 
	#[yaserde(rename = "End", prefix = "nsi1", default)]
	pub end: Option<String>, 
	#[yaserde(rename = "Location", prefix = "nsi1", default)]
	pub location: Option<String>, 
	#[yaserde(rename = "Recurrence", prefix = "nsi1", default)]
	pub recurrence: Option<RecurrenceType>, 
	#[yaserde(rename = "CalendarItemType", prefix = "nsi1", default)]
	pub calendar_item_type: Option<String>, 
	#[yaserde(rename = "EnhancedLocation", prefix = "nsi1", default)]
	pub enhanced_location: Option<EnhancedLocationType>, 
	#[yaserde(rename = "DoNotForwardMeeting", prefix = "nsi1", default)]
	pub do_not_forward_meeting: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RoleMemberTypeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RoleMemberTypeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ImAddressKeyType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ImAddressKeyType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EmailAddressKeyType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct EmailAddressKeyType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AbchEmailAddressTypeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AbchEmailAddressTypeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ContactUrlKeyType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ContactUrlKeyType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PhoneNumberKeyType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PhoneNumberKeyType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PhysicalAddressIndexType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PhysicalAddressIndexType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PhysicalAddressKeyType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PhysicalAddressKeyType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FileAsMappingType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FileAsMappingType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ContactSourceType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ContactSourceType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CompleteNameType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct CompleteNameType {
	#[yaserde(rename = "Title", prefix = "nsi1", default)]
	pub title: Option<String>, 
	#[yaserde(rename = "FirstName", prefix = "nsi1", default)]
	pub first_name: Option<String>, 
	#[yaserde(rename = "MiddleName", prefix = "nsi1", default)]
	pub middle_name: Option<String>, 
	#[yaserde(rename = "LastName", prefix = "nsi1", default)]
	pub last_name: Option<String>, 
	#[yaserde(rename = "Suffix", prefix = "nsi1", default)]
	pub suffix: Option<String>, 
	#[yaserde(rename = "Initials", prefix = "nsi1", default)]
	pub initials: Option<String>, 
	#[yaserde(rename = "FullName", prefix = "nsi1", default)]
	pub full_name: Option<String>, 
	#[yaserde(rename = "Nickname", prefix = "nsi1", default)]
	pub nickname: Option<String>, 
	#[yaserde(rename = "YomiFirstName", prefix = "nsi1", default)]
	pub yomi_first_name: Option<String>, 
	#[yaserde(rename = "YomiLastName", prefix = "nsi1", default)]
	pub yomi_last_name: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ImAddressDictionaryEntryType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ImAddressDictionaryEntryType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ContactUrlDictionaryEntryType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ContactUrlDictionaryEntryType {
	#[yaserde(rename = "Type", prefix = "nsi1", default)]
	pub rs_type: ContactUrlKeyType, 
	#[yaserde(rename = "Name", prefix = "nsi1", default)]
	pub name: Option<String>, 
	#[yaserde(rename = "Address", prefix = "nsi1", default)]
	pub address: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AbchEmailAddressDictionaryEntryType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AbchEmailAddressDictionaryEntryType {
	#[yaserde(rename = "Type", prefix = "nsi1", default)]
	pub rs_type: AbchEmailAddressTypeType, 
	#[yaserde(rename = "Address", prefix = "nsi1", default)]
	pub address: String, 
	#[yaserde(rename = "IsMessengerEnabled", prefix = "nsi1", default)]
	pub is_messenger_enabled: Option<bool>, 
	#[yaserde(rename = "Capabilities", prefix = "nsi1", default)]
	pub capabilities: Option<i64>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EmailAddressDictionaryEntryType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct EmailAddressDictionaryEntryType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PhoneNumberDictionaryEntryType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PhoneNumberDictionaryEntryType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PhysicalAddressDictionaryEntryType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PhysicalAddressDictionaryEntryType {
#[yaserde(rename="Key", attribute)]
pub key: PhysicalAddressKeyType,
	#[yaserde(rename = "Street", prefix = "nsi1", default)]
	pub street: Option<String>, 
	#[yaserde(rename = "City", prefix = "nsi1", default)]
	pub city: Option<String>, 
	#[yaserde(rename = "State", prefix = "nsi1", default)]
	pub state: Option<String>, 
	#[yaserde(rename = "CountryOrRegion", prefix = "nsi1", default)]
	pub country_or_region: Option<String>, 
	#[yaserde(rename = "PostalCode", prefix = "nsi1", default)]
	pub postal_code: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ContactUrlDictionaryType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ContactUrlDictionaryType {
	#[yaserde(rename = "Url", prefix = "nsi1", default)]
	pub url: Vec<ContactUrlDictionaryEntryType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AbchEmailAddressDictionaryType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AbchEmailAddressDictionaryType {
	#[yaserde(rename = "Email", prefix = "nsi1", default)]
	pub email: Vec<AbchEmailAddressDictionaryEntryType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ImAddressDictionaryType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ImAddressDictionaryType {
	#[yaserde(rename = "Entry", prefix = "nsi1", default)]
	pub entry: Vec<ImAddressDictionaryEntryType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EmailAddressDictionaryType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct EmailAddressDictionaryType {
	#[yaserde(rename = "Entry", prefix = "nsi1", default)]
	pub entry: Vec<EmailAddressDictionaryEntryType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PhoneNumberDictionaryType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PhoneNumberDictionaryType {
	#[yaserde(rename = "Entry", prefix = "nsi1", default)]
	pub entry: Vec<PhoneNumberDictionaryEntryType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PhysicalAddressDictionaryType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PhysicalAddressDictionaryType {
	#[yaserde(rename = "Entry", prefix = "nsi1", default)]
	pub entry: Vec<PhysicalAddressDictionaryEntryType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MemberStatusType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MemberStatusType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MembersListType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MembersListType {
	#[yaserde(rename = "Member", prefix = "nsi1", default)]
	pub member: Vec<MemberType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MemberType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MemberType {
#[yaserde(rename="Key", attribute)]
pub key: Option<String>,
	#[yaserde(rename = "Mailbox", prefix = "nsi1", default)]
	pub mailbox: Option<EmailAddressType>, 
	#[yaserde(rename = "Status", prefix = "nsi1", default)]
	pub status: Option<MemberStatusType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RoleMemberItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RoleMemberItemType {
	#[yaserde(flatten, default)]
	pub item_type: ItemType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "DisplayName", prefix = "nsi1", default)]
	pub display_name: Option<String>, 
	#[yaserde(rename = "Type", prefix = "nsi1", default)]
	pub rs_type: Option<RoleMemberTypeType>, 
	#[yaserde(rename = "MemberId", prefix = "nsi1", default)]
	pub member_id: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NetworkItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NetworkItemType {
	#[yaserde(flatten, default)]
	pub item_type: ItemType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "DomainId", prefix = "nsi1", default)]
	pub domain_id: Option<i32>, 
	#[yaserde(rename = "DomainTag", prefix = "nsi1", default)]
	pub domain_tag: Option<String>, 
	#[yaserde(rename = "UserTileUrl", prefix = "nsi1", default)]
	pub user_tile_url: Option<String>, 
	#[yaserde(rename = "ProfileUrl", prefix = "nsi1", default)]
	pub profile_url: Option<String>, 
	#[yaserde(rename = "Settings", prefix = "nsi1", default)]
	pub settings: Option<i32>, 
	#[yaserde(rename = "IsDefault", prefix = "nsi1", default)]
	pub is_default: Option<bool>, 
	#[yaserde(rename = "AutoLinkError", prefix = "nsi1", default)]
	pub auto_link_error: Option<String>, 
	#[yaserde(rename = "AutoLinkSuccess", prefix = "nsi1", default)]
	pub auto_link_success: Option<String>, 
	#[yaserde(rename = "UserEmail", prefix = "nsi1", default)]
	pub user_email: Option<String>, 
	#[yaserde(rename = "ClientPublishSecret", prefix = "nsi1", default)]
	pub client_publish_secret: Option<String>, 
	#[yaserde(rename = "ClientToken", prefix = "nsi1", default)]
	pub client_token: Option<String>, 
	#[yaserde(rename = "ClientToken2", prefix = "nsi1", default)]
	pub client_token_2: Option<String>, 
	#[yaserde(rename = "ContactSyncError", prefix = "nsi1", default)]
	pub contact_sync_error: Option<String>, 
	#[yaserde(rename = "ContactSyncSuccess", prefix = "nsi1", default)]
	pub contact_sync_success: Option<String>, 
	#[yaserde(rename = "ErrorOffers", prefix = "nsi1", default)]
	pub error_offers: Option<i32>, 
	#[yaserde(rename = "FirstAuthErrorDates", prefix = "nsi1", default)]
	pub first_auth_error_dates: Option<String>, 
	#[yaserde(rename = "LastVersionSaved", prefix = "nsi1", default)]
	pub last_version_saved: Option<i32>, 
	#[yaserde(rename = "LastWelcomeContact", prefix = "nsi1", default)]
	pub last_welcome_contact: Option<String>, 
	#[yaserde(rename = "Offers", prefix = "nsi1", default)]
	pub offers: Option<i32>, 
	#[yaserde(rename = "PsaLastChanged", prefix = "nsi1", default)]
	pub psa_last_changed: Option<String>, 
	#[yaserde(rename = "RefreshToken2", prefix = "nsi1", default)]
	pub refresh_token_2: Option<String>, 
	#[yaserde(rename = "RefreshTokenExpiry2", prefix = "nsi1", default)]
	pub refresh_token_expiry_2: Option<String>, 
	#[yaserde(rename = "SessionHandle", prefix = "nsi1", default)]
	pub session_handle: Option<String>, 
	#[yaserde(rename = "RejectedOffers", prefix = "nsi1", default)]
	pub rejected_offers: Option<i32>, 
	#[yaserde(rename = "SyncEnabled", prefix = "nsi1", default)]
	pub sync_enabled: Option<bool>, 
	#[yaserde(rename = "TokenRefreshLastAttempted", prefix = "nsi1", default)]
	pub token_refresh_last_attempted: Option<String>, 
	#[yaserde(rename = "TokenRefreshLastCompleted", prefix = "nsi1", default)]
	pub token_refresh_last_completed: Option<String>, 
	#[yaserde(rename = "PsaState", prefix = "nsi1", default)]
	pub psa_state: Option<String>, 
	#[yaserde(rename = "SourceEntryID", prefix = "nsi1", default)]
	pub source_entry_id: Option<String>, 
	#[yaserde(rename = "AccountName", prefix = "nsi1", default)]
	pub account_name: Option<String>, 
	#[yaserde(rename = "LastSync", prefix = "nsi1", default)]
	pub last_sync: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AbchPersonItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AbchPersonItemType {
	#[yaserde(flatten, default)]
	pub item_type: ItemType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "AntiLinkInfo", prefix = "nsi1", default)]
	pub anti_link_info: Option<String>, 
	#[yaserde(rename = "PersonId", prefix = "nsi1", default)]
	pub person_id: Option<GuidType>, 
	#[yaserde(rename = "ContactHandles", prefix = "nsi1", default)]
	pub contact_handles: Option<ArrayOfAbchPersonContactHandlesType>, 
	#[yaserde(rename = "ContactCategories", prefix = "nsi1", default)]
	pub contact_categories: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "RelevanceOrder1", prefix = "nsi1", default)]
	pub relevance_order_1: Option<String>, 
	#[yaserde(rename = "RelevanceOrder2", prefix = "nsi1", default)]
	pub relevance_order_2: Option<String>, 
	#[yaserde(rename = "TrustLevel", prefix = "nsi1", default)]
	pub trust_level: Option<i32>, 
	#[yaserde(rename = "FavoriteOrder", prefix = "nsi1", default)]
	pub favorite_order: Option<i32>, 
	#[yaserde(rename = "ExchangePersonIdGuid", prefix = "nsi1", default)]
	pub exchange_person_id_guid: Option<GuidType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfAbchPersonContactHandlesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfAbchPersonContactHandlesType {
	#[yaserde(rename = "ContactHandle", prefix = "nsi1", default)]
	pub contact_handle: Vec<AbchPersonContactHandle>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AbchPersonContactHandle",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AbchPersonContactHandle {
	#[yaserde(rename = "SourceId", prefix = "nsi1", default)]
	pub source_id: String, 
	#[yaserde(rename = "ObjectId", prefix = "nsi1", default)]
	pub object_id: String, 
	#[yaserde(rename = "AccountName", prefix = "nsi1", default)]
	pub account_name: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfGuidType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfGuidType {
	#[yaserde(rename = "Guid", prefix = "nsi1", default)]
	pub guid: Vec<GuidType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ContactItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ContactItemType {
	#[yaserde(flatten, default)]
	pub item_type: ItemType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "FileAs", prefix = "nsi1", default)]
	pub file_as: Option<String>, 
	#[yaserde(rename = "FileAsMapping", prefix = "nsi1", default)]
	pub file_as_mapping: Option<FileAsMappingType>, 
	#[yaserde(rename = "DisplayName", prefix = "nsi1", default)]
	pub display_name: Option<String>, 
	#[yaserde(rename = "GivenName", prefix = "nsi1", default)]
	pub given_name: Option<String>, 
	#[yaserde(rename = "Initials", prefix = "nsi1", default)]
	pub initials: Option<String>, 
	#[yaserde(rename = "MiddleName", prefix = "nsi1", default)]
	pub middle_name: Option<String>, 
	#[yaserde(rename = "Nickname", prefix = "nsi1", default)]
	pub nickname: Option<String>, 
	#[yaserde(rename = "CompleteName", prefix = "nsi1", default)]
	pub complete_name: Option<CompleteNameType>, 
	#[yaserde(rename = "CompanyName", prefix = "nsi1", default)]
	pub company_name: Option<String>, 
	#[yaserde(rename = "EmailAddresses", prefix = "nsi1", default)]
	pub email_addresses: Option<EmailAddressDictionaryType>, 
	#[yaserde(rename = "AbchEmailAddresses", prefix = "nsi1", default)]
	pub abch_email_addresses: Option<AbchEmailAddressDictionaryType>, 
	#[yaserde(rename = "PhysicalAddresses", prefix = "nsi1", default)]
	pub physical_addresses: Option<PhysicalAddressDictionaryType>, 
	#[yaserde(rename = "PhoneNumbers", prefix = "nsi1", default)]
	pub phone_numbers: Option<PhoneNumberDictionaryType>, 
	#[yaserde(rename = "AssistantName", prefix = "nsi1", default)]
	pub assistant_name: Option<String>, 
	#[yaserde(rename = "Birthday", prefix = "nsi1", default)]
	pub birthday: Option<String>, 
	#[yaserde(rename = "BusinessHomePage", prefix = "nsi1", default)]
	pub business_home_page: Option<String>, 
	#[yaserde(rename = "Children", prefix = "nsi1", default)]
	pub children: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "Companies", prefix = "nsi1", default)]
	pub companies: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "ContactSource", prefix = "nsi1", default)]
	pub contact_source: Option<ContactSourceType>, 
	#[yaserde(rename = "Department", prefix = "nsi1", default)]
	pub department: Option<String>, 
	#[yaserde(rename = "Generation", prefix = "nsi1", default)]
	pub generation: Option<String>, 
	#[yaserde(rename = "ImAddresses", prefix = "nsi1", default)]
	pub im_addresses: Option<ImAddressDictionaryType>, 
	#[yaserde(rename = "JobTitle", prefix = "nsi1", default)]
	pub job_title: Option<String>, 
	#[yaserde(rename = "Manager", prefix = "nsi1", default)]
	pub manager: Option<String>, 
	#[yaserde(rename = "Mileage", prefix = "nsi1", default)]
	pub mileage: Option<String>, 
	#[yaserde(rename = "OfficeLocation", prefix = "nsi1", default)]
	pub office_location: Option<String>, 
	#[yaserde(rename = "PostalAddressIndex", prefix = "nsi1", default)]
	pub postal_address_index: Option<PhysicalAddressIndexType>, 
	#[yaserde(rename = "Profession", prefix = "nsi1", default)]
	pub profession: Option<String>, 
	#[yaserde(rename = "SpouseName", prefix = "nsi1", default)]
	pub spouse_name: Option<String>, 
	#[yaserde(rename = "Surname", prefix = "nsi1", default)]
	pub surname: Option<String>, 
	#[yaserde(rename = "WeddingAnniversary", prefix = "nsi1", default)]
	pub wedding_anniversary: Option<String>, 
	#[yaserde(rename = "HasPicture", prefix = "nsi1", default)]
	pub has_picture: Option<bool>, 
	#[yaserde(rename = "PhoneticFullName", prefix = "nsi1", default)]
	pub phonetic_full_name: Option<String>, 
	#[yaserde(rename = "PhoneticFirstName", prefix = "nsi1", default)]
	pub phonetic_first_name: Option<String>, 
	#[yaserde(rename = "PhoneticLastName", prefix = "nsi1", default)]
	pub phonetic_last_name: Option<String>, 
	#[yaserde(rename = "Alias", prefix = "nsi1", default)]
	pub alias: Option<String>, 
	#[yaserde(rename = "Notes", prefix = "nsi1", default)]
	pub notes: Option<String>, 
	#[yaserde(rename = "Photo", prefix = "nsi1", default)]
	pub photo: Option<String>, 
	#[yaserde(rename = "UserSMIMECertificate", prefix = "nsi1", default)]
	pub user_smime_certificate: Option<ArrayOfBinaryType>, 
	#[yaserde(rename = "MSExchangeCertificate", prefix = "nsi1", default)]
	pub ms_exchange_certificate: Option<ArrayOfBinaryType>, 
	#[yaserde(rename = "DirectoryId", prefix = "nsi1", default)]
	pub directory_id: Option<String>, 
	#[yaserde(rename = "ManagerMailbox", prefix = "nsi1", default)]
	pub manager_mailbox: Option<SingleRecipientType>, 
	#[yaserde(rename = "DirectReports", prefix = "nsi1", default)]
	pub direct_reports: Option<ArrayOfRecipientsType>, 
	#[yaserde(rename = "AccountName", prefix = "nsi1", default)]
	pub account_name: Option<String>, 
	#[yaserde(rename = "IsAutoUpdateDisabled", prefix = "nsi1", default)]
	pub is_auto_update_disabled: Option<bool>, 
	#[yaserde(rename = "IsMessengerEnabled", prefix = "nsi1", default)]
	pub is_messenger_enabled: Option<bool>, 
	#[yaserde(rename = "Comment", prefix = "nsi1", default)]
	pub comment: Option<String>, 
	#[yaserde(rename = "ContactShortId", prefix = "nsi1", default)]
	pub contact_short_id: Option<i32>, 
	#[yaserde(rename = "ContactType", prefix = "nsi1", default)]
	pub contact_type: Option<String>, 
	#[yaserde(rename = "Gender", prefix = "nsi1", default)]
	pub gender: Option<String>, 
	#[yaserde(rename = "IsHidden", prefix = "nsi1", default)]
	pub is_hidden: Option<bool>, 
	#[yaserde(rename = "ObjectId", prefix = "nsi1", default)]
	pub object_id: Option<String>, 
	#[yaserde(rename = "PassportId", prefix = "nsi1", default)]
	pub passport_id: Option<i64>, 
	#[yaserde(rename = "IsPrivate", prefix = "nsi1", default)]
	pub is_private: Option<bool>, 
	#[yaserde(rename = "SourceId", prefix = "nsi1", default)]
	pub source_id: Option<String>, 
	#[yaserde(rename = "TrustLevel", prefix = "nsi1", default)]
	pub trust_level: Option<i32>, 
	#[yaserde(rename = "CreatedBy", prefix = "nsi1", default)]
	pub created_by: Option<String>, 
	#[yaserde(rename = "Urls", prefix = "nsi1", default)]
	pub urls: Option<ContactUrlDictionaryType>, 
	#[yaserde(rename = "Cid", prefix = "nsi1", default)]
	pub cid: Option<i64>, 
	#[yaserde(rename = "SkypeAuthCertificate", prefix = "nsi1", default)]
	pub skype_auth_certificate: Option<String>, 
	#[yaserde(rename = "SkypeContext", prefix = "nsi1", default)]
	pub skype_context: Option<String>, 
	#[yaserde(rename = "SkypeId", prefix = "nsi1", default)]
	pub skype_id: Option<String>, 
	#[yaserde(rename = "SkypeRelationship", prefix = "nsi1", default)]
	pub skype_relationship: Option<String>, 
	#[yaserde(rename = "YomiNickname", prefix = "nsi1", default)]
	pub yomi_nickname: Option<String>, 
	#[yaserde(rename = "XboxLiveTag", prefix = "nsi1", default)]
	pub xbox_live_tag: Option<String>, 
	#[yaserde(rename = "InviteFree", prefix = "nsi1", default)]
	pub invite_free: Option<bool>, 
	#[yaserde(rename = "HidePresenceAndProfile", prefix = "nsi1", default)]
	pub hide_presence_and_profile: Option<bool>, 
	#[yaserde(rename = "IsPendingOutbound", prefix = "nsi1", default)]
	pub is_pending_outbound: Option<bool>, 
	#[yaserde(rename = "SupportGroupFeeds", prefix = "nsi1", default)]
	pub support_group_feeds: Option<bool>, 
	#[yaserde(rename = "UserTileHash", prefix = "nsi1", default)]
	pub user_tile_hash: Option<String>, 
	#[yaserde(rename = "UnifiedInbox", prefix = "nsi1", default)]
	pub unified_inbox: Option<bool>, 
	#[yaserde(rename = "Mris", prefix = "nsi1", default)]
	pub mris: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "Wlid", prefix = "nsi1", default)]
	pub wlid: Option<String>, 
	#[yaserde(rename = "AbchContactId", prefix = "nsi1", default)]
	pub abch_contact_id: Option<GuidType>, 
	#[yaserde(rename = "NotInBirthdayCalendar", prefix = "nsi1", default)]
	pub not_in_birthday_calendar: Option<bool>, 
	#[yaserde(rename = "ShellContactType", prefix = "nsi1", default)]
	pub shell_contact_type: Option<String>, 
	#[yaserde(rename = "ImMri", prefix = "nsi1", default)]
	pub im_mri: Option<String>, 
	#[yaserde(rename = "PresenceTrustLevel", prefix = "nsi1", default)]
	pub presence_trust_level: Option<i32>, 
	#[yaserde(rename = "OtherMri", prefix = "nsi1", default)]
	pub other_mri: Option<String>, 
	#[yaserde(rename = "ProfileLastChanged", prefix = "nsi1", default)]
	pub profile_last_changed: Option<String>, 
	#[yaserde(rename = "MobileIMEnabled", prefix = "nsi1", default)]
	pub mobile_im_enabled: Option<bool>, 
	#[yaserde(rename = "PartnerNetworkProfilePhotoUrl", prefix = "nsi1", default)]
	pub partner_network_profile_photo_url: Option<String>, 
	#[yaserde(rename = "PartnerNetworkThumbnailPhotoUrl", prefix = "nsi1", default)]
	pub partner_network_thumbnail_photo_url: Option<String>, 
	#[yaserde(rename = "PersonId", prefix = "nsi1", default)]
	pub person_id: Option<String>, 
	#[yaserde(rename = "ConversationGuid", prefix = "nsi1", default)]
	pub conversation_guid: Option<GuidType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfBinaryType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfBinaryType {
	#[yaserde(rename = "Base64Binary", prefix = "nsi1", default)]
	pub base_64_binary: Vec<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DistributionListType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DistributionListType {
	#[yaserde(flatten, default)]
	pub item_type: ItemType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "DisplayName", prefix = "nsi1", default)]
	pub display_name: Option<String>, 
	#[yaserde(rename = "FileAs", prefix = "nsi1", default)]
	pub file_as: Option<String>, 
	#[yaserde(rename = "ContactSource", prefix = "nsi1", default)]
	pub contact_source: Option<ContactSourceType>, 
	#[yaserde(rename = "Members", prefix = "nsi1", default)]
	pub members: Option<MembersListType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchParametersType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SearchParametersType {
#[yaserde(rename="Traversal", attribute)]
pub traversal: Option<SearchFolderTraversalType>,
	#[yaserde(rename = "Restriction", prefix = "nsi1", default)]
	pub restriction: RestrictionType, 
	#[yaserde(rename = "BaseFolderIds", prefix = "nsi1", default)]
	pub base_folder_ids: NonEmptyArrayOfBaseFolderIdsType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ConstantValueType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ConstantValueType {
#[yaserde(rename="Value", attribute)]
pub value: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchExpressionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SearchExpressionType {
}
pub type SearchExpression = SearchExpressionType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AggregateType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AggregateType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AggregateOnType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AggregateOnType {
#[yaserde(rename="Aggregate", attribute)]
pub aggregate: AggregateType,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BaseGroupByType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct BaseGroupByType {
#[yaserde(rename="Order", attribute)]
pub order: SortDirectionType,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GroupByType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct GroupByType {
	#[yaserde(flatten, default)]
	pub base_group_by_type: BaseGroupByType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "FieldURI", prefix = "nsi1", default)]
	pub field_uri: PathToUnindexedFieldType, 
	#[yaserde(rename = "IndexedFieldURI", prefix = "nsi1", default)]
	pub indexed_field_uri: PathToIndexedFieldType, 
	#[yaserde(rename = "ExtendedFieldURI", prefix = "nsi1", default)]
	pub extended_field_uri: PathToExtendedFieldType, 
	#[yaserde(rename = "AggregateOn", prefix = "nsi1", default)]
	pub aggregate_on: AggregateOnType, 
	#[yaserde(rename = "UseCollapsibleGroups", prefix = "nsi1", default)]
	pub use_collapsible_groups: Option<bool>, 
	#[yaserde(rename = "ItemsPerGroup", prefix = "nsi1", default)]
	pub items_per_group: Option<i32>, 
	#[yaserde(rename = "MaxItemsPerGroup", prefix = "nsi1", default)]
	pub max_items_per_group: Option<i32>, 
	#[yaserde(rename = "GroupsToExpand", prefix = "nsi1", default)]
	pub groups_to_expand: Option<ArrayOfGroupIdType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "StandardGroupByType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct StandardGroupByType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DistinguishedGroupByType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DistinguishedGroupByType {
	#[yaserde(flatten, default)]
	pub base_group_by_type: BaseGroupByType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "StandardGroupBy", prefix = "nsi1", default)]
	pub standard_group_by: StandardGroupByType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfGroupIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfGroupIdType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GroupedItemsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct GroupedItemsType {
	#[yaserde(rename = "GroupIndex", prefix = "nsi1", default)]
	pub group_index: String, 
	#[yaserde(rename = "Items", prefix = "nsi1", default)]
	pub items: ArrayOfRealItemsType, 
	#[yaserde(rename = "GroupSummary", prefix = "nsi1", default)]
	pub group_summary: Option<GroupSummaryType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfGroupedItemsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfGroupedItemsType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GroupSummaryType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct GroupSummaryType {
	#[yaserde(rename = "GroupCount", prefix = "nsi1", default)]
	pub group_count: i32, 
	#[yaserde(rename = "UnreadCount", prefix = "nsi1", default)]
	pub unread_count: i32, 
	#[yaserde(rename = "InstanceKey", prefix = "nsi1", default)]
	pub instance_key: String, 
	#[yaserde(rename = "GroupByValue", prefix = "nsi1", default)]
	pub group_by_value: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ExistsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ExistsType {
	#[yaserde(flatten, default)]
	pub search_expression_type: SearchExpressionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type Exists = ExistsType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FieldURIOrConstantType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FieldURIOrConstantType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "TwoOperandExpressionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct TwoOperandExpressionType {
	#[yaserde(flatten, default)]
	pub search_expression_type: SearchExpressionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "FieldURIOrConstant", prefix = "nsi1", default)]
	pub field_uri_or_constant: FieldURIOrConstantType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ExcludesAttributeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ExcludesAttributeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ExcludesValueType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ExcludesValueType {
#[yaserde(rename="Value", attribute)]
pub value: ExcludesAttributeType,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ExcludesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ExcludesType {
	#[yaserde(flatten, default)]
	pub search_expression_type: SearchExpressionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Bitmask", prefix = "nsi1", default)]
	pub bitmask: ExcludesValueType, 
}
pub type Excludes = ExcludesType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "IsEqualToType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct IsEqualToType {
	#[yaserde(flatten, default)]
	pub two_operand_expression_type: TwoOperandExpressionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type IsEqualTo = IsEqualToType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "IsNotEqualToType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct IsNotEqualToType {
	#[yaserde(flatten, default)]
	pub two_operand_expression_type: TwoOperandExpressionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type IsNotEqualTo = IsNotEqualToType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "IsGreaterThanType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct IsGreaterThanType {
	#[yaserde(flatten, default)]
	pub two_operand_expression_type: TwoOperandExpressionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type IsGreaterThan = IsGreaterThanType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "IsGreaterThanOrEqualToType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct IsGreaterThanOrEqualToType {
	#[yaserde(flatten, default)]
	pub two_operand_expression_type: TwoOperandExpressionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type IsGreaterThanOrEqualTo = IsGreaterThanOrEqualToType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "IsLessThanType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct IsLessThanType {
	#[yaserde(flatten, default)]
	pub two_operand_expression_type: TwoOperandExpressionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type IsLessThan = IsLessThanType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "IsLessThanOrEqualToType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct IsLessThanOrEqualToType {
	#[yaserde(flatten, default)]
	pub two_operand_expression_type: TwoOperandExpressionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type IsLessThanOrEqualTo = IsLessThanOrEqualToType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ContainmentModeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ContainmentModeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ContainmentComparisonType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ContainmentComparisonType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ContainsExpressionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ContainsExpressionType {
	#[yaserde(flatten, default)]
	pub search_expression_type: SearchExpressionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Constant", prefix = "nsi1", default)]
	pub constant: ConstantValueType, 
}
pub type Contains = ContainsExpressionType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NotType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NotType {
	#[yaserde(flatten, default)]
	pub search_expression_type: SearchExpressionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type Not = NotType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MultipleOperandBooleanExpressionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MultipleOperandBooleanExpressionType {
	#[yaserde(flatten, default)]
	pub search_expression_type: SearchExpressionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AndType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AndType {
	#[yaserde(flatten, default)]
	pub multiple_operand_boolean_expression_type: MultipleOperandBooleanExpressionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type And = AndType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "OrType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct OrType {
	#[yaserde(flatten, default)]
	pub multiple_operand_boolean_expression_type: MultipleOperandBooleanExpressionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type Or = OrType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NearType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NearType {
	#[yaserde(flatten, default)]
	pub multiple_operand_boolean_expression_type: MultipleOperandBooleanExpressionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Distance", prefix = "nsi1", default)]
	pub distance: u32, 
	#[yaserde(rename = "Ordered", prefix = "nsi1", default)]
	pub ordered: bool, 
}
pub type Near = NearType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RestrictionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RestrictionType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SortDirectionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SortDirectionType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FieldOrderType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FieldOrderType {
#[yaserde(rename="Order", attribute)]
pub order: SortDirectionType,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfFieldOrdersType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfFieldOrdersType {
	#[yaserde(rename = "FieldOrder", prefix = "nsi1", default)]
	pub field_order: Vec<FieldOrderType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfFolderNamesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfFolderNamesType {
	#[yaserde(rename = "FolderName", prefix = "nsi1", default)]
	pub folder_name: Vec<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "WatermarkType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct WatermarkType {
	#[yaserde(flatten, default)]
	pub body: NonEmptyStringType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SubscriptionIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SubscriptionIdType {
	#[yaserde(flatten, default)]
	pub body: NonEmptyStringType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BaseNotificationEventType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct BaseNotificationEventType {
	#[yaserde(rename = "Watermark", prefix = "nsi1", default)]
	pub watermark: Option<WatermarkType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BaseObjectChangedEventType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct BaseObjectChangedEventType {
	#[yaserde(flatten, default)]
	pub base_notification_event_type: BaseNotificationEventType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "TimeStamp", prefix = "nsi1", default)]
	pub time_stamp: String, 
	#[yaserde(rename = "FolderId", prefix = "nsi1", default)]
	pub folder_id: FolderIdType, 
	#[yaserde(rename = "ItemId", prefix = "nsi1", default)]
	pub item_id: ItemIdType, 
	#[yaserde(rename = "ParentFolderId", prefix = "nsi1", default)]
	pub parent_folder_id: FolderIdType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ModifiedEventType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ModifiedEventType {
	#[yaserde(flatten, default)]
	pub base_object_changed_event_type: BaseObjectChangedEventType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "UnreadCount", prefix = "nsi1", default)]
	pub unread_count: Option<i32>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MovedCopiedEventType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MovedCopiedEventType {
	#[yaserde(flatten, default)]
	pub base_object_changed_event_type: BaseObjectChangedEventType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "OldFolderId", prefix = "nsi1", default)]
	pub old_folder_id: FolderIdType, 
	#[yaserde(rename = "OldItemId", prefix = "nsi1", default)]
	pub old_item_id: ItemIdType, 
	#[yaserde(rename = "OldParentFolderId", prefix = "nsi1", default)]
	pub old_parent_folder_id: FolderIdType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NotificationType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NotificationType {
	#[yaserde(rename = "SubscriptionId", prefix = "nsi1", default)]
	pub subscription_id: SubscriptionIdType, 
	#[yaserde(rename = "PreviousWatermark", prefix = "nsi1", default)]
	pub previous_watermark: Option<WatermarkType>, 
	#[yaserde(rename = "MoreEvents", prefix = "nsi1", default)]
	pub more_events: Option<bool>, 
	#[yaserde(rename = "CopiedEvent", prefix = "nsi1", default)]
	pub copied_event: MovedCopiedEventType, 
	#[yaserde(rename = "CreatedEvent", prefix = "nsi1", default)]
	pub created_event: BaseObjectChangedEventType, 
	#[yaserde(rename = "DeletedEvent", prefix = "nsi1", default)]
	pub deleted_event: BaseObjectChangedEventType, 
	#[yaserde(rename = "ModifiedEvent", prefix = "nsi1", default)]
	pub modified_event: ModifiedEventType, 
	#[yaserde(rename = "MovedEvent", prefix = "nsi1", default)]
	pub moved_event: MovedCopiedEventType, 
	#[yaserde(rename = "NewMailEvent", prefix = "nsi1", default)]
	pub new_mail_event: BaseObjectChangedEventType, 
	#[yaserde(rename = "StatusEvent", prefix = "nsi1", default)]
	pub status_event: BaseNotificationEventType, 
	#[yaserde(rename = "FreeBusyChangedEvent", prefix = "nsi1", default)]
	pub free_busy_changed_event: BaseObjectChangedEventType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NotificationEventTypeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NotificationEventTypeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfNotificationEventTypesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfNotificationEventTypesType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SubscriptionTimeoutType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SubscriptionTimeoutType {
	#[yaserde(default)]
	pub body: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SubscriptionStatusFrequencyType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SubscriptionStatusFrequencyType {
	#[yaserde(default)]
	pub body: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BaseSubscriptionRequestType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct BaseSubscriptionRequestType {
#[yaserde(rename="SubscribeToAllFolders", attribute)]
pub subscribe_to_all_folders: Option<bool>,
	#[yaserde(rename = "FolderIds", prefix = "nsi1", default)]
	pub folder_ids: Option<NonEmptyArrayOfBaseFolderIdsType>, 
	#[yaserde(rename = "EventTypes", prefix = "nsi1", default)]
	pub event_types: NonEmptyArrayOfNotificationEventTypesType, 
	#[yaserde(rename = "Watermark", prefix = "nsi1", default)]
	pub watermark: Option<WatermarkType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PushSubscriptionRequestType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PushSubscriptionRequestType {
	#[yaserde(flatten, default)]
	pub base_subscription_request_type: BaseSubscriptionRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "StatusFrequency", prefix = "nsi1", default)]
	pub status_frequency: SubscriptionStatusFrequencyType, 
	#[yaserde(rename = "URL", prefix = "nsi1", default)]
	pub url: String, 
	#[yaserde(rename = "CallerData", prefix = "nsi1", default)]
	pub caller_data: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PullSubscriptionRequestType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PullSubscriptionRequestType {
	#[yaserde(flatten, default)]
	pub base_subscription_request_type: BaseSubscriptionRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Timeout", prefix = "nsi1", default)]
	pub timeout: SubscriptionTimeoutType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "StreamingSubscriptionRequestType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct StreamingSubscriptionRequestType {
#[yaserde(rename="SubscribeToAllFolders", attribute)]
pub subscribe_to_all_folders: Option<bool>,
	#[yaserde(rename = "FolderIds", prefix = "nsi1", default)]
	pub folder_ids: Option<NonEmptyArrayOfBaseFolderIdsType>, 
	#[yaserde(rename = "EventTypes", prefix = "nsi1", default)]
	pub event_types: NonEmptyArrayOfNotificationEventTypesType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SubscriptionStatusType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SubscriptionStatusType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfSubscriptionIdsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfSubscriptionIdsType {
	#[yaserde(rename = "SubscriptionId", prefix = "nsi1", default)]
	pub subscription_id: Vec<SubscriptionIdType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfNotificationsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfNotificationsType {
	#[yaserde(rename = "Notification", prefix = "nsi1", default)]
	pub notification: Vec<NotificationType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "StreamingSubscriptionConnectionTimeoutType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct StreamingSubscriptionConnectionTimeoutType {
	#[yaserde(default)]
	pub body: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ConnectionStatusType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ConnectionStatusType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UnifiedGroupAccessType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct UnifiedGroupAccessType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SyncFolderItemsDeleteType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SyncFolderItemsDeleteType {
	#[yaserde(rename = "ItemId", prefix = "nsi1", default)]
	pub item_id: ItemIdType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SyncFolderItemsReadFlagType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SyncFolderItemsReadFlagType {
	#[yaserde(rename = "ItemId", prefix = "nsi1", default)]
	pub item_id: ItemIdType, 
	#[yaserde(rename = "IsRead", prefix = "nsi1", default)]
	pub is_read: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SyncFolderItemsChangesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SyncFolderItemsChangesType {
	#[yaserde(rename = "Create", prefix = "nsi1", default)]
	pub create: SyncFolderItemsCreateOrUpdateType, 
	#[yaserde(rename = "Update", prefix = "nsi1", default)]
	pub update: SyncFolderItemsCreateOrUpdateType, 
	#[yaserde(rename = "Delete", prefix = "nsi1", default)]
	pub delete: SyncFolderItemsDeleteType, 
	#[yaserde(rename = "ReadFlagChange", prefix = "nsi1", default)]
	pub read_flag_change: SyncFolderItemsReadFlagType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SyncFolderHierarchyCreateOrUpdateType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SyncFolderHierarchyCreateOrUpdateType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SyncFolderHierarchyDeleteType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SyncFolderHierarchyDeleteType {
	#[yaserde(rename = "FolderId", prefix = "nsi1", default)]
	pub folder_id: FolderIdType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SyncFolderHierarchyChangesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SyncFolderHierarchyChangesType {
	#[yaserde(rename = "Create", prefix = "nsi1", default)]
	pub create: SyncFolderHierarchyCreateOrUpdateType, 
	#[yaserde(rename = "Update", prefix = "nsi1", default)]
	pub update: SyncFolderHierarchyCreateOrUpdateType, 
	#[yaserde(rename = "Delete", prefix = "nsi1", default)]
	pub delete: SyncFolderHierarchyDeleteType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MaxSyncChangesReturnedType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MaxSyncChangesReturnedType {
	#[yaserde(default)]
	pub body: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SyncFolderItemsScopeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SyncFolderItemsScopeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AvailabilityProxyRequestType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AvailabilityProxyRequestType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RequestTypeHeader",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RequestTypeHeader {
	#[yaserde(rename = "RequestType", prefix = "nsi1", default)]
	pub request_type: AvailabilityProxyRequestType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MeetingAttendeeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MeetingAttendeeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CalendarEventDetails",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct CalendarEventDetails {
	#[yaserde(rename = "ID", prefix = "nsi1", default)]
	pub id: Option<String>, 
	#[yaserde(rename = "Subject", prefix = "nsi1", default)]
	pub subject: Option<String>, 
	#[yaserde(rename = "Location", prefix = "nsi1", default)]
	pub location: Option<String>, 
	#[yaserde(rename = "IsMeeting", prefix = "nsi1", default)]
	pub is_meeting: bool, 
	#[yaserde(rename = "IsRecurring", prefix = "nsi1", default)]
	pub is_recurring: bool, 
	#[yaserde(rename = "IsException", prefix = "nsi1", default)]
	pub is_exception: bool, 
	#[yaserde(rename = "IsReminderSet", prefix = "nsi1", default)]
	pub is_reminder_set: bool, 
	#[yaserde(rename = "IsPrivate", prefix = "nsi1", default)]
	pub is_private: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CalendarEvent",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct CalendarEvent {
	#[yaserde(rename = "StartTime", prefix = "nsi1", default)]
	pub start_time: String, 
	#[yaserde(rename = "EndTime", prefix = "nsi1", default)]
	pub end_time: String, 
	#[yaserde(rename = "BusyType", prefix = "nsi1", default)]
	pub busy_type: LegacyFreeBusyType, 
	#[yaserde(rename = "CalendarEventDetails", prefix = "nsi1", default)]
	pub calendar_event_details: Option<CalendarEventDetails>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfCalendarEvent",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfCalendarEvent {
	#[yaserde(rename = "CalendarEvent", prefix = "nsi1", default)]
	pub calendar_event: Vec<CalendarEvent>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Duration",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct Duration {
	#[yaserde(rename = "StartTime", prefix = "nsi1", default)]
	pub start_time: String, 
	#[yaserde(rename = "EndTime", prefix = "nsi1", default)]
	pub end_time: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EmailAddress",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct EmailAddress {
	#[yaserde(rename = "Name", prefix = "nsi1", default)]
	pub name: Option<String>, 
	#[yaserde(rename = "Address", prefix = "nsi1", default)]
	pub address: String, 
	#[yaserde(rename = "RoutingType", prefix = "nsi1", default)]
	pub routing_type: Option<String>, 
}
pub type Mailbox = EmailAddress;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FreeBusyViewType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FreeBusyViewType {
	#[yaserde(flatten, default)]
	pub body: Box<FreeBusyViewType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FreeBusyViewOptionsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FreeBusyViewOptionsType {
	#[yaserde(rename = "TimeWindow", prefix = "nsi1", default)]
	pub time_window: Duration, 
	#[yaserde(rename = "MergedFreeBusyIntervalInMinutes", prefix = "nsi1", default)]
	pub merged_free_busy_interval_in_minutes: Option<i32>, 
	#[yaserde(rename = "RequestedView", prefix = "nsi1", default)]
	pub requested_view: Option<FreeBusyViewType>, 
}
pub type FreeBusyViewOptions = FreeBusyViewOptionsType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "WorkingPeriod",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct WorkingPeriod {
	#[yaserde(rename = "DayOfWeek", prefix = "nsi1", default)]
	pub day_of_week: DaysOfWeekType, 
	#[yaserde(rename = "StartTimeInMinutes", prefix = "nsi1", default)]
	pub start_time_in_minutes: i32, 
	#[yaserde(rename = "EndTimeInMinutes", prefix = "nsi1", default)]
	pub end_time_in_minutes: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfWorkingPeriod",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfWorkingPeriod {
	#[yaserde(rename = "WorkingPeriod", prefix = "nsi1", default)]
	pub working_period: Vec<WorkingPeriod>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SerializableTimeZoneTime",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SerializableTimeZoneTime {
	#[yaserde(rename = "Bias", prefix = "nsi1", default)]
	pub bias: i32, 
	#[yaserde(rename = "Time", prefix = "nsi1", default)]
	pub time: String, 
	#[yaserde(rename = "DayOrder", prefix = "nsi1", default)]
	pub day_order: i16, 
	#[yaserde(rename = "Month", prefix = "nsi1", default)]
	pub month: i16, 
	#[yaserde(rename = "DayOfWeek", prefix = "nsi1", default)]
	pub day_of_week: DayOfWeekType, 
	#[yaserde(rename = "Year", prefix = "nsi1", default)]
	pub year: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SerializableTimeZone",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SerializableTimeZone {
	#[yaserde(rename = "Bias", prefix = "nsi1", default)]
	pub bias: i32, 
	#[yaserde(rename = "StandardTime", prefix = "nsi1", default)]
	pub standard_time: SerializableTimeZoneTime, 
	#[yaserde(rename = "DaylightTime", prefix = "nsi1", default)]
	pub daylight_time: SerializableTimeZoneTime, 
}
pub type TimeZone = SerializableTimeZone;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "WorkingHours",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct WorkingHours {
	#[yaserde(rename = "TimeZone", prefix = "nsi1", default)]
	pub time_zone: SerializableTimeZone, 
	#[yaserde(rename = "WorkingPeriodArray", prefix = "nsi1", default)]
	pub working_period_array: ArrayOfWorkingPeriod, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FreeBusyView",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FreeBusyView {
	#[yaserde(rename = "FreeBusyViewType", prefix = "nsi1", default)]
	pub free_busy_view_type: FreeBusyViewType, 
	#[yaserde(rename = "MergedFreeBusy", prefix = "nsi1", default)]
	pub merged_free_busy: Option<String>, 
	#[yaserde(rename = "CalendarEventArray", prefix = "nsi1", default)]
	pub calendar_event_array: Option<ArrayOfCalendarEvent>, 
	#[yaserde(rename = "WorkingHours", prefix = "nsi1", default)]
	pub working_hours: Option<WorkingHours>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MailboxData",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MailboxData {
	#[yaserde(rename = "Email", prefix = "nsi1", default)]
	pub email: EmailAddress, 
	#[yaserde(rename = "AttendeeType", prefix = "nsi1", default)]
	pub attendee_type: MeetingAttendeeType, 
	#[yaserde(rename = "ExcludeConflicts", prefix = "nsi1", default)]
	pub exclude_conflicts: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfMailboxData",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfMailboxData {
	#[yaserde(rename = "MailboxData", prefix = "nsi1", default)]
	pub mailbox_data: Vec<MailboxData>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SuggestionQuality",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SuggestionQuality {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SuggestionsViewOptionsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SuggestionsViewOptionsType {
	#[yaserde(rename = "GoodThreshold", prefix = "nsi1", default)]
	pub good_threshold: Option<i32>, 
	#[yaserde(rename = "MaximumResultsByDay", prefix = "nsi1", default)]
	pub maximum_results_by_day: Option<i32>, 
	#[yaserde(rename = "MaximumNonWorkHourResultsByDay", prefix = "nsi1", default)]
	pub maximum_non_work_hour_results_by_day: Option<i32>, 
	#[yaserde(rename = "MeetingDurationInMinutes", prefix = "nsi1", default)]
	pub meeting_duration_in_minutes: Option<i32>, 
	#[yaserde(rename = "MinimumSuggestionQuality", prefix = "nsi1", default)]
	pub minimum_suggestion_quality: Option<SuggestionQuality>, 
	#[yaserde(rename = "DetailedSuggestionsWindow", prefix = "nsi1", default)]
	pub detailed_suggestions_window: Duration, 
	#[yaserde(rename = "CurrentMeetingTime", prefix = "nsi1", default)]
	pub current_meeting_time: Option<String>, 
	#[yaserde(rename = "GlobalObjectId", prefix = "nsi1", default)]
	pub global_object_id: Option<String>, 
}
pub type SuggestionsViewOptions = SuggestionsViewOptionsType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfAttendeeConflictData",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfAttendeeConflictData {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AttendeeConflictData",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AttendeeConflictData {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UnknownAttendeeConflictData",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct UnknownAttendeeConflictData {
	#[yaserde(flatten, default)]
	pub attendee_conflict_data: AttendeeConflictData, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "TooBigGroupAttendeeConflictData",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct TooBigGroupAttendeeConflictData {
	#[yaserde(flatten, default)]
	pub attendee_conflict_data: AttendeeConflictData, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "IndividualAttendeeConflictData",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct IndividualAttendeeConflictData {
	#[yaserde(flatten, default)]
	pub attendee_conflict_data: AttendeeConflictData, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "BusyType", prefix = "nsi1", default)]
	pub busy_type: LegacyFreeBusyType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GroupAttendeeConflictData",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct GroupAttendeeConflictData {
	#[yaserde(flatten, default)]
	pub attendee_conflict_data: AttendeeConflictData, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "NumberOfMembers", prefix = "nsi1", default)]
	pub number_of_members: i32, 
	#[yaserde(rename = "NumberOfMembersAvailable", prefix = "nsi1", default)]
	pub number_of_members_available: i32, 
	#[yaserde(rename = "NumberOfMembersWithConflict", prefix = "nsi1", default)]
	pub number_of_members_with_conflict: i32, 
	#[yaserde(rename = "NumberOfMembersWithNoData", prefix = "nsi1", default)]
	pub number_of_members_with_no_data: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Suggestion",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct Suggestion {
	#[yaserde(rename = "MeetingTime", prefix = "nsi1", default)]
	pub meeting_time: String, 
	#[yaserde(rename = "IsWorkTime", prefix = "nsi1", default)]
	pub is_work_time: bool, 
	#[yaserde(rename = "SuggestionQuality", prefix = "nsi1", default)]
	pub suggestion_quality: SuggestionQuality, 
	#[yaserde(rename = "AttendeeConflictDataArray", prefix = "nsi1", default)]
	pub attendee_conflict_data_array: Option<ArrayOfAttendeeConflictData>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfSuggestion",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfSuggestion {
	#[yaserde(rename = "Suggestion", prefix = "nsi1", default)]
	pub suggestion: Vec<Suggestion>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SuggestionDayResult",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SuggestionDayResult {
	#[yaserde(rename = "Date", prefix = "nsi1", default)]
	pub date: String, 
	#[yaserde(rename = "DayQuality", prefix = "nsi1", default)]
	pub day_quality: SuggestionQuality, 
	#[yaserde(rename = "SuggestionArray", prefix = "nsi1", default)]
	pub suggestion_array: Option<ArrayOfSuggestion>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfSuggestionDayResult",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfSuggestionDayResult {
	#[yaserde(rename = "SuggestionDayResult", prefix = "nsi1", default)]
	pub suggestion_day_result: Vec<SuggestionDayResult>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "OofState",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct OofState {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ExternalAudience",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ExternalAudience {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ReplyBody",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ReplyBody {
	#[yaserde(rename = "Message", prefix = "nsi1", default)]
	pub message: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfEventIDType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfEventIDType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UserOofSettings",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct UserOofSettings {
	#[yaserde(rename = "OofState", prefix = "nsi1", default)]
	pub oof_state: OofState, 
	#[yaserde(rename = "ExternalAudience", prefix = "nsi1", default)]
	pub external_audience: ExternalAudience, 
	#[yaserde(rename = "Duration", prefix = "nsi1", default)]
	pub duration: Option<Duration>, 
	#[yaserde(rename = "InternalReply", prefix = "nsi1", default)]
	pub internal_reply: Option<ReplyBody>, 
	#[yaserde(rename = "ExternalReply", prefix = "nsi1", default)]
	pub external_reply: Option<ReplyBody>, 
	#[yaserde(rename = "DeclineMeetingReply", prefix = "nsi1", default)]
	pub decline_meeting_reply: Option<ReplyBody>, 
	#[yaserde(rename = "DeclineEventsForScheduledOOF", prefix = "nsi1", default)]
	pub decline_events_for_scheduled_oof: Option<bool>, 
	#[yaserde(rename = "DeclineAllEventsForScheduledOOF", prefix = "nsi1", default)]
	pub decline_all_events_for_scheduled_oof: Option<bool>, 
	#[yaserde(rename = "CreateOOFEvent", prefix = "nsi1", default)]
	pub create_oof_event: Option<bool>, 
	#[yaserde(rename = "OOFEventSubject", prefix = "nsi1", default)]
	pub oof_event_subject: Option<String>, 
	#[yaserde(rename = "AutoDeclineFutureRequestsWhenOOF", prefix = "nsi1", default)]
	pub auto_decline_future_requests_when_oof: Option<bool>, 
	#[yaserde(rename = "OOFEventID", prefix = "nsi1", default)]
	pub oof_event_id: Option<String>, 
	#[yaserde(rename = "EventsToDeleteIDs", prefix = "nsi1", default)]
	pub events_to_delete_i_ds: Option<ArrayOfEventIDType>, 
}
pub type OofSettings = UserOofSettings;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Value",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct Value {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ApprovalRequestDataType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ApprovalRequestDataType {
	#[yaserde(rename = "IsUndecidedApprovalRequest", prefix = "nsi1", default)]
	pub is_undecided_approval_request: Option<bool>, 
	#[yaserde(rename = "ApprovalDecision", prefix = "nsi1", default)]
	pub approval_decision: Option<i32>, 
	#[yaserde(rename = "ApprovalDecisionMaker", prefix = "nsi1", default)]
	pub approval_decision_maker: Option<String>, 
	#[yaserde(rename = "ApprovalDecisionTime", prefix = "nsi1", default)]
	pub approval_decision_time: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ReminderMessageDataType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ReminderMessageDataType {
	#[yaserde(rename = "ReminderText", prefix = "nsi1", default)]
	pub reminder_text: Option<String>, 
	#[yaserde(rename = "Location", prefix = "nsi1", default)]
	pub location: Option<String>, 
	#[yaserde(rename = "StartTime", prefix = "nsi1", default)]
	pub start_time: Option<String>, 
	#[yaserde(rename = "EndTime", prefix = "nsi1", default)]
	pub end_time: Option<String>, 
	#[yaserde(rename = "AssociatedCalendarItemId", prefix = "nsi1", default)]
	pub associated_calendar_item_id: Option<ItemIdType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MessageSafetyType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MessageSafetyType {
	#[yaserde(rename = "MessageSafetyLevel", prefix = "nsi1", default)]
	pub message_safety_level: Option<i32>, 
	#[yaserde(rename = "MessageSafetyReason", prefix = "nsi1", default)]
	pub message_safety_reason: Option<i32>, 
	#[yaserde(rename = "Info", prefix = "nsi1", default)]
	pub info: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SendPromptType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SendPromptType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "VotingOptionDataType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct VotingOptionDataType {
	#[yaserde(rename = "DisplayName", prefix = "nsi1", default)]
	pub display_name: Option<String>, 
	#[yaserde(rename = "SendPrompt", prefix = "nsi1", default)]
	pub send_prompt: Option<SendPromptType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfVotingOptionDataType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfVotingOptionDataType {
	#[yaserde(rename = "VotingOptionData", prefix = "nsi1", default)]
	pub voting_option_data: Vec<VotingOptionDataType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "VotingInformationType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct VotingInformationType {
	#[yaserde(rename = "UserOptions", prefix = "nsi1", default)]
	pub user_options: Option<ArrayOfVotingOptionDataType>, 
	#[yaserde(rename = "VotingResponse", prefix = "nsi1", default)]
	pub voting_response: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "IdFormatType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct IdFormatType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AlternateIdBaseType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AlternateIdBaseType {
#[yaserde(rename="Format", attribute)]
pub format: IdFormatType,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AlternateIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AlternateIdType {
	#[yaserde(flatten, default)]
	pub alternate_id_base_type: AlternateIdBaseType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AlternatePublicFolderIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AlternatePublicFolderIdType {
	#[yaserde(flatten, default)]
	pub alternate_id_base_type: AlternateIdBaseType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AlternatePublicFolderItemIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AlternatePublicFolderItemIdType {
	#[yaserde(flatten, default)]
	pub alternate_public_folder_id_type: AlternatePublicFolderIdType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfAlternateIdsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfAlternateIdsType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UserIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct UserIdType {
	#[yaserde(rename = "SID", prefix = "nsi1", default)]
	pub sid: Option<String>, 
	#[yaserde(rename = "PrimarySmtpAddress", prefix = "nsi1", default)]
	pub primary_smtp_address: Option<String>, 
	#[yaserde(rename = "DisplayName", prefix = "nsi1", default)]
	pub display_name: Option<String>, 
	#[yaserde(rename = "DistinguishedUser", prefix = "nsi1", default)]
	pub distinguished_user: Option<DistinguishedUserType>, 
	#[yaserde(rename = "ExternalUserIdentity", prefix = "nsi1", default)]
	pub external_user_identity: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DistinguishedUserType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DistinguishedUserType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfPermissionsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfPermissionsType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfCalendarPermissionsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfCalendarPermissionsType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfUnknownEntriesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfUnknownEntriesType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PermissionReadAccessType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PermissionReadAccessType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CalendarPermissionReadAccessType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct CalendarPermissionReadAccessType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BasePermissionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct BasePermissionType {
	#[yaserde(rename = "UserId", prefix = "nsi1", default)]
	pub user_id: UserIdType, 
	#[yaserde(rename = "CanCreateItems", prefix = "nsi1", default)]
	pub can_create_items: Option<bool>, 
	#[yaserde(rename = "CanCreateSubFolders", prefix = "nsi1", default)]
	pub can_create_sub_folders: Option<bool>, 
	#[yaserde(rename = "IsFolderOwner", prefix = "nsi1", default)]
	pub is_folder_owner: Option<bool>, 
	#[yaserde(rename = "IsFolderVisible", prefix = "nsi1", default)]
	pub is_folder_visible: Option<bool>, 
	#[yaserde(rename = "IsFolderContact", prefix = "nsi1", default)]
	pub is_folder_contact: Option<bool>, 
	#[yaserde(rename = "EditItems", prefix = "nsi1", default)]
	pub edit_items: Option<PermissionActionType>, 
	#[yaserde(rename = "DeleteItems", prefix = "nsi1", default)]
	pub delete_items: Option<PermissionActionType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PermissionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PermissionType {
	#[yaserde(flatten, default)]
	pub base_permission_type: BasePermissionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ReadItems", prefix = "nsi1", default)]
	pub read_items: Option<PermissionReadAccessType>, 
	#[yaserde(rename = "PermissionLevel", prefix = "nsi1", default)]
	pub permission_level: PermissionLevelType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CalendarPermissionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct CalendarPermissionType {
	#[yaserde(flatten, default)]
	pub base_permission_type: BasePermissionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ReadItems", prefix = "nsi1", default)]
	pub read_items: Option<CalendarPermissionReadAccessType>, 
	#[yaserde(rename = "CalendarPermissionLevel", prefix = "nsi1", default)]
	pub calendar_permission_level: CalendarPermissionLevelType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PermissionActionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PermissionActionType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PermissionLevelType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PermissionLevelType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CalendarPermissionLevelType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct CalendarPermissionLevelType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PermissionSetType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PermissionSetType {
	#[yaserde(rename = "Permissions", prefix = "nsi1", default)]
	pub permissions: ArrayOfPermissionsType, 
	#[yaserde(rename = "UnknownEntries", prefix = "nsi1", default)]
	pub unknown_entries: Option<ArrayOfUnknownEntriesType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CalendarPermissionSetType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct CalendarPermissionSetType {
	#[yaserde(rename = "CalendarPermissions", prefix = "nsi1", default)]
	pub calendar_permissions: ArrayOfCalendarPermissionsType, 
	#[yaserde(rename = "UnknownEntries", prefix = "nsi1", default)]
	pub unknown_entries: Option<ArrayOfUnknownEntriesType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EffectiveRightsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct EffectiveRightsType {
	#[yaserde(rename = "CreateAssociated", prefix = "nsi1", default)]
	pub create_associated: bool, 
	#[yaserde(rename = "CreateContents", prefix = "nsi1", default)]
	pub create_contents: bool, 
	#[yaserde(rename = "CreateHierarchy", prefix = "nsi1", default)]
	pub create_hierarchy: bool, 
	#[yaserde(rename = "Delete", prefix = "nsi1", default)]
	pub delete: bool, 
	#[yaserde(rename = "Modify", prefix = "nsi1", default)]
	pub modify: bool, 
	#[yaserde(rename = "Read", prefix = "nsi1", default)]
	pub read: bool, 
	#[yaserde(rename = "ViewPrivateItems", prefix = "nsi1", default)]
	pub view_private_items: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfDelegateUserType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfDelegateUserType {
	#[yaserde(rename = "DelegateUser", prefix = "nsi1", default)]
	pub delegate_user: Vec<DelegateUserType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfUserIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfUserIdType {
	#[yaserde(rename = "UserId", prefix = "nsi1", default)]
	pub user_id: Vec<UserIdType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeliverMeetingRequestsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DeliverMeetingRequestsType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DelegateUserType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DelegateUserType {
	#[yaserde(rename = "UserId", prefix = "nsi1", default)]
	pub user_id: UserIdType, 
	#[yaserde(rename = "DelegatePermissions", prefix = "nsi1", default)]
	pub delegate_permissions: Option<DelegatePermissionsType>, 
	#[yaserde(rename = "ReceiveCopiesOfMeetingMessages", prefix = "nsi1", default)]
	pub receive_copies_of_meeting_messages: Option<bool>, 
	#[yaserde(rename = "ViewPrivateItems", prefix = "nsi1", default)]
	pub view_private_items: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DelegatePermissionsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DelegatePermissionsType {
	#[yaserde(rename = "CalendarFolderPermissionLevel", prefix = "nsi1", default)]
	pub calendar_folder_permission_level: Option<DelegateFolderPermissionLevelType>, 
	#[yaserde(rename = "TasksFolderPermissionLevel", prefix = "nsi1", default)]
	pub tasks_folder_permission_level: Option<DelegateFolderPermissionLevelType>, 
	#[yaserde(rename = "InboxFolderPermissionLevel", prefix = "nsi1", default)]
	pub inbox_folder_permission_level: Option<DelegateFolderPermissionLevelType>, 
	#[yaserde(rename = "ContactsFolderPermissionLevel", prefix = "nsi1", default)]
	pub contacts_folder_permission_level: Option<DelegateFolderPermissionLevelType>, 
	#[yaserde(rename = "NotesFolderPermissionLevel", prefix = "nsi1", default)]
	pub notes_folder_permission_level: Option<DelegateFolderPermissionLevelType>, 
	#[yaserde(rename = "JournalFolderPermissionLevel", prefix = "nsi1", default)]
	pub journal_folder_permission_level: Option<DelegateFolderPermissionLevelType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DelegateFolderPermissionLevelType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DelegateFolderPermissionLevelType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ConflictResultsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ConflictResultsType {
	#[yaserde(rename = "Count", prefix = "nsi1", default)]
	pub count: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MailTipTypes",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MailTipTypes {
	#[yaserde(flatten, default)]
	pub body: Box<MailTipTypes>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "OutOfOfficeMailTip",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct OutOfOfficeMailTip {
	#[yaserde(rename = "ReplyBody", prefix = "nsi1", default)]
	pub reply_body: ReplyBody, 
	#[yaserde(rename = "Duration", prefix = "nsi1", default)]
	pub duration: Option<Duration>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SmtpDomainList",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SmtpDomainList {
	#[yaserde(rename = "Domain", prefix = "nsi1", default)]
	pub domain: Vec<SmtpDomain>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SmtpDomain",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SmtpDomain {
#[yaserde(rename="Name", attribute)]
pub name: String,
#[yaserde(rename="IncludeSubdomains", attribute)]
pub include_subdomains: Option<bool>,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MailTips",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MailTips {
	#[yaserde(rename = "RecipientAddress", prefix = "nsi1", default)]
	pub recipient_address: EmailAddressType, 
	#[yaserde(rename = "PendingMailTips", prefix = "nsi1", default)]
	pub pending_mail_tips: MailTipTypes, 
	#[yaserde(rename = "OutOfOffice", prefix = "nsi1", default)]
	pub out_of_office: Option<OutOfOfficeMailTip>, 
	#[yaserde(rename = "MailboxFull", prefix = "nsi1", default)]
	pub mailbox_full: Option<bool>, 
	#[yaserde(rename = "CustomMailTip", prefix = "nsi1", default)]
	pub custom_mail_tip: Option<String>, 
	#[yaserde(rename = "TotalMemberCount", prefix = "nsi1", default)]
	pub total_member_count: Option<i32>, 
	#[yaserde(rename = "ExternalMemberCount", prefix = "nsi1", default)]
	pub external_member_count: Option<i32>, 
	#[yaserde(rename = "MaxMessageSize", prefix = "nsi1", default)]
	pub max_message_size: Option<i32>, 
	#[yaserde(rename = "DeliveryRestricted", prefix = "nsi1", default)]
	pub delivery_restricted: Option<bool>, 
	#[yaserde(rename = "IsModerated", prefix = "nsi1", default)]
	pub is_moderated: Option<bool>, 
	#[yaserde(rename = "InvalidRecipient", prefix = "nsi1", default)]
	pub invalid_recipient: Option<bool>, 
	#[yaserde(rename = "Scope", prefix = "nsi1", default)]
	pub scope: Option<i32>, 
	#[yaserde(rename = "RecipientSuggestions", prefix = "nsi1", default)]
	pub recipient_suggestions: Option<ArrayOfRecipientSuggestionsType>, 
	#[yaserde(rename = "PreferAccessibleContent", prefix = "nsi1", default)]
	pub prefer_accessible_content: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfRecipientSuggestionsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfRecipientSuggestionsType {
	#[yaserde(rename = "RecipientSuggestion", prefix = "nsi1", default)]
	pub recipient_suggestion: Vec<RecipientSuggestionType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RecipientSuggestionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RecipientSuggestionType {
	#[yaserde(rename = "DisplayName", prefix = "nsi1", default)]
	pub display_name: String, 
	#[yaserde(rename = "EmailAddress", prefix = "nsi1", default)]
	pub email_address: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ServiceConfigurationType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ServiceConfigurationType {
	#[yaserde(flatten, default)]
	pub body: Box<ServiceConfigurationType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ServiceConfiguration",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ServiceConfiguration {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MailTipsServiceConfiguration",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MailTipsServiceConfiguration {
	#[yaserde(flatten, default)]
	pub service_configuration: ServiceConfiguration, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MailTipsEnabled", prefix = "nsi1", default)]
	pub mail_tips_enabled: bool, 
	#[yaserde(rename = "MaxRecipientsPerGetMailTipsRequest", prefix = "nsi1", default)]
	pub max_recipients_per_get_mail_tips_request: i32, 
	#[yaserde(rename = "MaxMessageSize", prefix = "nsi1", default)]
	pub max_message_size: i32, 
	#[yaserde(rename = "LargeAudienceThreshold", prefix = "nsi1", default)]
	pub large_audience_threshold: i32, 
	#[yaserde(rename = "ShowExternalRecipientCount", prefix = "nsi1", default)]
	pub show_external_recipient_count: bool, 
	#[yaserde(rename = "InternalDomains", prefix = "nsi1", default)]
	pub internal_domains: SmtpDomainList, 
	#[yaserde(rename = "PolicyTipsEnabled", prefix = "nsi1", default)]
	pub policy_tips_enabled: bool, 
	#[yaserde(rename = "LargeAudienceCap", prefix = "nsi1", default)]
	pub large_audience_cap: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UnifiedMessageServiceConfiguration",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct UnifiedMessageServiceConfiguration {
	#[yaserde(flatten, default)]
	pub service_configuration: ServiceConfiguration, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "UmEnabled", prefix = "nsi1", default)]
	pub um_enabled: bool, 
	#[yaserde(rename = "PlayOnPhoneDialString", prefix = "nsi1", default)]
	pub play_on_phone_dial_string: String, 
	#[yaserde(rename = "PlayOnPhoneEnabled", prefix = "nsi1", default)]
	pub play_on_phone_enabled: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SharePointURLsServiceConfiguration",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SharePointURLsServiceConfiguration {
	#[yaserde(flatten, default)]
	pub service_configuration: ServiceConfiguration, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "InternalSPMySiteHostURL", prefix = "nsi1", default)]
	pub internal_sp_my_site_host_url: String, 
	#[yaserde(rename = "ExternalSPMySiteHostURL", prefix = "nsi1", default)]
	pub external_sp_my_site_host_url: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "OfficeIntegrationConfiguration",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct OfficeIntegrationConfiguration {
	#[yaserde(flatten, default)]
	pub service_configuration: ServiceConfiguration, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "OfficeSidebarURL", prefix = "nsi1", default)]
	pub office_sidebar_url: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ConfigurationRequestDetailsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ConfigurationRequestDetailsType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfSmtpAddressType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfSmtpAddressType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfEncryptedSharedFolderDataType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfEncryptedSharedFolderDataType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EncryptedSharedFolderDataType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct EncryptedSharedFolderDataType {
	#[yaserde(rename = "Token", prefix = "nsi1", default)]
	pub token: EncryptedDataContainerType, 
	#[yaserde(rename = "Data", prefix = "nsi1", default)]
	pub data: EncryptedDataContainerType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EncryptedDataContainerType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct EncryptedDataContainerType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfInvalidRecipientsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfInvalidRecipientsType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "InvalidRecipientType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct InvalidRecipientType {
	#[yaserde(rename = "SmtpAddress", prefix = "nsi1", default)]
	pub smtp_address: NonEmptyStringType, 
	#[yaserde(rename = "ResponseCode", prefix = "nsi1", default)]
	pub response_code: InvalidRecipientResponseCodeType, 
	#[yaserde(rename = "MessageText", prefix = "nsi1", default)]
	pub message_text: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "InvalidRecipientResponseCodeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct InvalidRecipientResponseCodeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AcceptSharingInvitationType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AcceptSharingInvitationType {
	#[yaserde(flatten, default)]
	pub reference_item_response_type: ReferenceItemResponseType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SharingDataType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SharingDataType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PhoneCallIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PhoneCallIdType {
#[yaserde(rename="Id", attribute)]
pub id: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PhoneCallInformationType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PhoneCallInformationType {
	#[yaserde(rename = "PhoneCallState", prefix = "nsi1", default)]
	pub phone_call_state: PhoneCallStateType, 
	#[yaserde(rename = "ConnectionFailureCause", prefix = "nsi1", default)]
	pub connection_failure_cause: ConnectionFailureCauseType, 
	#[yaserde(rename = "SIPResponseText", prefix = "nsi1", default)]
	pub sip_response_text: Option<String>, 
	#[yaserde(rename = "SIPResponseCode", prefix = "nsi1", default)]
	pub sip_response_code: Option<i32>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PhoneCallStateType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PhoneCallStateType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ConnectionFailureCauseType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ConnectionFailureCauseType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UserConfigurationNameType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct UserConfigurationNameType {
	#[yaserde(flatten, default)]
	pub target_folder_id_type: TargetFolderIdType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UserConfigurationDictionaryObjectTypesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct UserConfigurationDictionaryObjectTypesType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UserConfigurationDictionaryObjectType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct UserConfigurationDictionaryObjectType {
	#[yaserde(rename = "Type", prefix = "nsi1", default)]
	pub rs_type: UserConfigurationDictionaryObjectTypesType, 
	#[yaserde(rename = "Value", prefix = "nsi1", default)]
	pub value: Vec<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UserConfigurationDictionaryEntryType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct UserConfigurationDictionaryEntryType {
	#[yaserde(rename = "DictionaryKey", prefix = "nsi1", default)]
	pub dictionary_key: UserConfigurationDictionaryObjectType, 
	#[yaserde(rename = "DictionaryValue", prefix = "nsi1", default)]
	pub dictionary_value: Option<UserConfigurationDictionaryObjectType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UserConfigurationDictionaryType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct UserConfigurationDictionaryType {
	#[yaserde(rename = "DictionaryEntry", prefix = "nsi1", default)]
	pub dictionary_entry: Vec<UserConfigurationDictionaryEntryType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UserConfigurationType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct UserConfigurationType {
	#[yaserde(rename = "UserConfigurationName", prefix = "nsi1", default)]
	pub user_configuration_name: UserConfigurationNameType, 
	#[yaserde(rename = "ItemId", prefix = "nsi1", default)]
	pub item_id: Option<ItemIdType>, 
	#[yaserde(rename = "Dictionary", prefix = "nsi1", default)]
	pub dictionary: Option<UserConfigurationDictionaryType>, 
	#[yaserde(rename = "XmlData", prefix = "nsi1", default)]
	pub xml_data: Option<String>, 
	#[yaserde(rename = "BinaryData", prefix = "nsi1", default)]
	pub binary_data: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UserConfigurationPropertyType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct UserConfigurationPropertyType {
	#[yaserde(flatten, default)]
	pub body: Box<UserConfigurationPropertyType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ProtectionRulesServiceConfiguration",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ProtectionRulesServiceConfiguration {
	#[yaserde(flatten, default)]
	pub service_configuration: ServiceConfiguration, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Rules", prefix = "nsi1", default)]
	pub rules: ArrayOfProtectionRulesType, 
	#[yaserde(rename = "InternalDomains", prefix = "nsi1", default)]
	pub internal_domains: SmtpDomainList, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfProtectionRulesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfProtectionRulesType {
	#[yaserde(rename = "Rule", prefix = "nsi1", default)]
	pub rule: Vec<ProtectionRuleType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ProtectionRuleType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ProtectionRuleType {
#[yaserde(rename="UserOverridable", attribute)]
pub user_overridable: bool,
	#[yaserde(rename = "Condition", prefix = "nsi1", default)]
	pub condition: ProtectionRuleConditionType, 
	#[yaserde(rename = "Action", prefix = "nsi1", default)]
	pub action: ProtectionRuleActionType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ProtectionRuleConditionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ProtectionRuleConditionType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ProtectionRuleAndType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ProtectionRuleAndType {
	#[yaserde(rename = "AllInternal", prefix = "nsi1", default)]
	pub all_internal: ProtectionRuleAllInternalType, 
	#[yaserde(rename = "And", prefix = "nsi1", default)]
	pub and: Box<ProtectionRuleAndType>, 
	#[yaserde(rename = "RecipientIs", prefix = "nsi1", default)]
	pub recipient_is: ProtectionRuleRecipientIsType, 
	#[yaserde(rename = "SenderDepartments", prefix = "nsi1", default)]
	pub sender_departments: ProtectionRuleSenderDepartmentsType, 
	#[yaserde(rename = "True", prefix = "nsi1", default)]
	pub r#true: ProtectionRuleTrueType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ProtectionRuleRecipientIsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ProtectionRuleRecipientIsType {
	#[yaserde(rename = "Value", prefix = "nsi1", default)]
	pub value: Vec<ProtectionRuleValueType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ProtectionRuleSenderDepartmentsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ProtectionRuleSenderDepartmentsType {
	#[yaserde(rename = "Value", prefix = "nsi1", default)]
	pub value: Vec<ProtectionRuleValueType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ProtectionRuleTrueType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ProtectionRuleTrueType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ProtectionRuleAllInternalType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ProtectionRuleAllInternalType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ProtectionRuleValueType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ProtectionRuleValueType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ProtectionRuleActionKindType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ProtectionRuleActionKindType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ProtectionRuleActionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ProtectionRuleActionType {
#[yaserde(rename="Name", attribute)]
pub name: ProtectionRuleActionKindType,
	#[yaserde(rename = "Argument", prefix = "nsi1", default)]
	pub argument: Vec<ProtectionRuleArgumentType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ProtectionRuleArgumentType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ProtectionRuleArgumentType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PolicyNudgeRulesServiceConfiguration",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PolicyNudgeRulesServiceConfiguration {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfFolderIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfFolderIdType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "TeamMailboxLifecycleStateType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct TeamMailboxLifecycleStateType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ReportMessageActionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ReportMessageActionType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ReportMessagePlatformType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ReportMessagePlatformType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindMessageTrackingSearchResultType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FindMessageTrackingSearchResultType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfFindMessageTrackingSearchResultType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfFindMessageTrackingSearchResultType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MessageTrackingReportTemplateType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MessageTrackingReportTemplateType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RecipientTrackingEventType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RecipientTrackingEventType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MessageTrackingReportType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MessageTrackingReportType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "TrackingPropertyType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct TrackingPropertyType {
	#[yaserde(rename = "Name", prefix = "nsi1", default)]
	pub name: String, 
	#[yaserde(rename = "Value", prefix = "nsi1", default)]
	pub value: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfTrackingPropertiesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfTrackingPropertiesType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfArraysOfTrackingPropertiesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfArraysOfTrackingPropertiesType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfRecipientTrackingEventType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfRecipientTrackingEventType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RulePredicateSizeRangeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RulePredicateSizeRangeType {
	#[yaserde(rename = "MinimumSize", prefix = "nsi1", default)]
	pub minimum_size: Option<i32>, 
	#[yaserde(rename = "MaximumSize", prefix = "nsi1", default)]
	pub maximum_size: Option<i32>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RulePredicateDateRangeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RulePredicateDateRangeType {
	#[yaserde(rename = "StartDateTime", prefix = "nsi1", default)]
	pub start_date_time: Option<String>, 
	#[yaserde(rename = "EndDateTime", prefix = "nsi1", default)]
	pub end_date_time: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FlaggedForActionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FlaggedForActionType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RulePredicatesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RulePredicatesType {
	#[yaserde(rename = "Categories", prefix = "nsi1", default)]
	pub categories: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "ContainsBodyStrings", prefix = "nsi1", default)]
	pub contains_body_strings: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "ContainsHeaderStrings", prefix = "nsi1", default)]
	pub contains_header_strings: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "ContainsRecipientStrings", prefix = "nsi1", default)]
	pub contains_recipient_strings: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "ContainsSenderStrings", prefix = "nsi1", default)]
	pub contains_sender_strings: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "ContainsSubjectOrBodyStrings", prefix = "nsi1", default)]
	pub contains_subject_or_body_strings: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "ContainsSubjectStrings", prefix = "nsi1", default)]
	pub contains_subject_strings: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "FlaggedForAction", prefix = "nsi1", default)]
	pub flagged_for_action: Option<FlaggedForActionType>, 
	#[yaserde(rename = "FromAddresses", prefix = "nsi1", default)]
	pub from_addresses: Option<ArrayOfEmailAddressesType>, 
	#[yaserde(rename = "FromConnectedAccounts", prefix = "nsi1", default)]
	pub from_connected_accounts: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "HasAttachments", prefix = "nsi1", default)]
	pub has_attachments: Option<bool>, 
	#[yaserde(rename = "Importance", prefix = "nsi1", default)]
	pub importance: Option<ImportanceChoicesType>, 
	#[yaserde(rename = "IsApprovalRequest", prefix = "nsi1", default)]
	pub is_approval_request: Option<bool>, 
	#[yaserde(rename = "IsAutomaticForward", prefix = "nsi1", default)]
	pub is_automatic_forward: Option<bool>, 
	#[yaserde(rename = "IsAutomaticReply", prefix = "nsi1", default)]
	pub is_automatic_reply: Option<bool>, 
	#[yaserde(rename = "IsEncrypted", prefix = "nsi1", default)]
	pub is_encrypted: Option<bool>, 
	#[yaserde(rename = "IsMeetingRequest", prefix = "nsi1", default)]
	pub is_meeting_request: Option<bool>, 
	#[yaserde(rename = "IsMeetingResponse", prefix = "nsi1", default)]
	pub is_meeting_response: Option<bool>, 
	#[yaserde(rename = "IsNDR", prefix = "nsi1", default)]
	pub is_ndr: Option<bool>, 
	#[yaserde(rename = "IsPermissionControlled", prefix = "nsi1", default)]
	pub is_permission_controlled: Option<bool>, 
	#[yaserde(rename = "IsReadReceipt", prefix = "nsi1", default)]
	pub is_read_receipt: Option<bool>, 
	#[yaserde(rename = "IsSigned", prefix = "nsi1", default)]
	pub is_signed: Option<bool>, 
	#[yaserde(rename = "IsVoicemail", prefix = "nsi1", default)]
	pub is_voicemail: Option<bool>, 
	#[yaserde(rename = "ItemClasses", prefix = "nsi1", default)]
	pub item_classes: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "MessageClassifications", prefix = "nsi1", default)]
	pub message_classifications: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "NotSentToMe", prefix = "nsi1", default)]
	pub not_sent_to_me: Option<bool>, 
	#[yaserde(rename = "SentCcMe", prefix = "nsi1", default)]
	pub sent_cc_me: Option<bool>, 
	#[yaserde(rename = "SentOnlyToMe", prefix = "nsi1", default)]
	pub sent_only_to_me: Option<bool>, 
	#[yaserde(rename = "SentToAddresses", prefix = "nsi1", default)]
	pub sent_to_addresses: Option<ArrayOfEmailAddressesType>, 
	#[yaserde(rename = "SentToMe", prefix = "nsi1", default)]
	pub sent_to_me: Option<bool>, 
	#[yaserde(rename = "SentToOrCcMe", prefix = "nsi1", default)]
	pub sent_to_or_cc_me: Option<bool>, 
	#[yaserde(rename = "Sensitivity", prefix = "nsi1", default)]
	pub sensitivity: Option<SensitivityChoicesType>, 
	#[yaserde(rename = "WithinDateRange", prefix = "nsi1", default)]
	pub within_date_range: Option<RulePredicateDateRangeType>, 
	#[yaserde(rename = "WithinSizeRange", prefix = "nsi1", default)]
	pub within_size_range: Option<RulePredicateSizeRangeType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RuleActionsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RuleActionsType {
	#[yaserde(rename = "AssignCategories", prefix = "nsi1", default)]
	pub assign_categories: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "CopyToFolder", prefix = "nsi1", default)]
	pub copy_to_folder: Option<TargetFolderIdType>, 
	#[yaserde(rename = "Delete", prefix = "nsi1", default)]
	pub delete: Option<bool>, 
	#[yaserde(rename = "ForwardAsAttachmentToRecipients", prefix = "nsi1", default)]
	pub forward_as_attachment_to_recipients: Option<ArrayOfEmailAddressesType>, 
	#[yaserde(rename = "ForwardToRecipients", prefix = "nsi1", default)]
	pub forward_to_recipients: Option<ArrayOfEmailAddressesType>, 
	#[yaserde(rename = "MarkImportance", prefix = "nsi1", default)]
	pub mark_importance: Option<ImportanceChoicesType>, 
	#[yaserde(rename = "MarkAsRead", prefix = "nsi1", default)]
	pub mark_as_read: Option<bool>, 
	#[yaserde(rename = "MoveToFolder", prefix = "nsi1", default)]
	pub move_to_folder: Option<TargetFolderIdType>, 
	#[yaserde(rename = "PermanentDelete", prefix = "nsi1", default)]
	pub permanent_delete: Option<bool>, 
	#[yaserde(rename = "RedirectToRecipients", prefix = "nsi1", default)]
	pub redirect_to_recipients: Option<ArrayOfEmailAddressesType>, 
	#[yaserde(rename = "SendSMSAlertToRecipients", prefix = "nsi1", default)]
	pub send_sms_alert_to_recipients: Option<ArrayOfEmailAddressesType>, 
	#[yaserde(rename = "ServerReplyWithMessage", prefix = "nsi1", default)]
	pub server_reply_with_message: Option<ItemIdType>, 
	#[yaserde(rename = "StopProcessingRules", prefix = "nsi1", default)]
	pub stop_processing_rules: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RuleType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RuleType {
	#[yaserde(rename = "RuleId", prefix = "nsi1", default)]
	pub rule_id: Option<String>, 
	#[yaserde(rename = "DisplayName", prefix = "nsi1", default)]
	pub display_name: String, 
	#[yaserde(rename = "Priority", prefix = "nsi1", default)]
	pub priority: i32, 
	#[yaserde(rename = "IsEnabled", prefix = "nsi1", default)]
	pub is_enabled: bool, 
	#[yaserde(rename = "IsNotSupported", prefix = "nsi1", default)]
	pub is_not_supported: Option<bool>, 
	#[yaserde(rename = "IsInError", prefix = "nsi1", default)]
	pub is_in_error: Option<bool>, 
	#[yaserde(rename = "Conditions", prefix = "nsi1", default)]
	pub conditions: Option<RulePredicatesType>, 
	#[yaserde(rename = "Exceptions", prefix = "nsi1", default)]
	pub exceptions: Option<RulePredicatesType>, 
	#[yaserde(rename = "Actions", prefix = "nsi1", default)]
	pub actions: Option<RuleActionsType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfRulesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfRulesType {
	#[yaserde(rename = "Rule", prefix = "nsi1", default)]
	pub rule: Vec<RuleType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RuleFieldURIType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RuleFieldURIType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RuleValidationErrorCodeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RuleValidationErrorCodeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RuleValidationErrorType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RuleValidationErrorType {
	#[yaserde(rename = "FieldURI", prefix = "nsi1", default)]
	pub field_uri: RuleFieldURIType, 
	#[yaserde(rename = "ErrorCode", prefix = "nsi1", default)]
	pub error_code: RuleValidationErrorCodeType, 
	#[yaserde(rename = "ErrorMessage", prefix = "nsi1", default)]
	pub error_message: String, 
	#[yaserde(rename = "FieldValue", prefix = "nsi1", default)]
	pub field_value: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfRuleValidationErrorsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfRuleValidationErrorsType {
	#[yaserde(rename = "Error", prefix = "nsi1", default)]
	pub error: Vec<RuleValidationErrorType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RuleOperationType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RuleOperationType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfRuleOperationsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfRuleOperationsType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CreateRuleOperationType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct CreateRuleOperationType {
	#[yaserde(flatten, default)]
	pub rule_operation_type: RuleOperationType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Rule", prefix = "nsi1", default)]
	pub rule: RuleType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SetRuleOperationType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SetRuleOperationType {
	#[yaserde(flatten, default)]
	pub rule_operation_type: RuleOperationType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Rule", prefix = "nsi1", default)]
	pub rule: RuleType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeleteRuleOperationType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DeleteRuleOperationType {
	#[yaserde(flatten, default)]
	pub rule_operation_type: RuleOperationType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "RuleId", prefix = "nsi1", default)]
	pub rule_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RuleOperationErrorType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RuleOperationErrorType {
	#[yaserde(rename = "OperationIndex", prefix = "nsi1", default)]
	pub operation_index: i32, 
	#[yaserde(rename = "ValidationErrors", prefix = "nsi1", default)]
	pub validation_errors: ArrayOfRuleValidationErrorsType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfRuleOperationErrorsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfRuleOperationErrorsType {
	#[yaserde(rename = "RuleOperationError", prefix = "nsi1", default)]
	pub rule_operation_error: Vec<RuleOperationErrorType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchItemKindType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SearchItemKindType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfSearchItemKindsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfSearchItemKindsType {
	#[yaserde(rename = "SearchItemKind", prefix = "nsi1", default)]
	pub search_item_kind: Vec<SearchItemKindType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UserMailboxType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct UserMailboxType {
#[yaserde(rename="Id", attribute)]
pub id: String,
#[yaserde(rename="IsArchive", attribute)]
pub is_archive: bool,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfUserMailboxesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfUserMailboxesType {
	#[yaserde(rename = "UserMailbox", prefix = "nsi1", default)]
	pub user_mailbox: Vec<UserMailboxType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchableMailboxType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SearchableMailboxType {
	#[yaserde(rename = "Guid", prefix = "nsi1", default)]
	pub guid: GuidType, 
	#[yaserde(rename = "PrimarySmtpAddress", prefix = "nsi1", default)]
	pub primary_smtp_address: String, 
	#[yaserde(rename = "IsExternalMailbox", prefix = "nsi1", default)]
	pub is_external_mailbox: bool, 
	#[yaserde(rename = "ExternalEmailAddress", prefix = "nsi1", default)]
	pub external_email_address: String, 
	#[yaserde(rename = "DisplayName", prefix = "nsi1", default)]
	pub display_name: String, 
	#[yaserde(rename = "IsMembershipGroup", prefix = "nsi1", default)]
	pub is_membership_group: bool, 
	#[yaserde(rename = "ReferenceId", prefix = "nsi1", default)]
	pub reference_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfSearchableMailboxesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfSearchableMailboxesType {
	#[yaserde(rename = "SearchableMailbox", prefix = "nsi1", default)]
	pub searchable_mailbox: Vec<SearchableMailboxType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "KeywordStatisticsSearchResultType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct KeywordStatisticsSearchResultType {
	#[yaserde(rename = "Keyword", prefix = "nsi1", default)]
	pub keyword: String, 
	#[yaserde(rename = "ItemHits", prefix = "nsi1", default)]
	pub item_hits: i32, 
	#[yaserde(rename = "Size", prefix = "nsi1", default)]
	pub size: i64, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfKeywordStatisticsSearchResultsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfKeywordStatisticsSearchResultsType {
	#[yaserde(rename = "KeywordStat", prefix = "nsi1", default)]
	pub keyword_stat: Vec<KeywordStatisticsSearchResultType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MailboxStatisticsSearchResultType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MailboxStatisticsSearchResultType {
	#[yaserde(rename = "UserMailbox", prefix = "nsi1", default)]
	pub user_mailbox: UserMailboxType, 
	#[yaserde(rename = "KeywordStatisticsSearchResult", prefix = "nsi1", default)]
	pub keyword_statistics_search_result: Option<KeywordStatisticsSearchResultType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ExtendedAttributeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ExtendedAttributeType {
	#[yaserde(rename = "Name", prefix = "nsi1", default)]
	pub name: String, 
	#[yaserde(rename = "Value", prefix = "nsi1", default)]
	pub value: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfExtendedAttributesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfExtendedAttributesType {
	#[yaserde(rename = "ExtendedAttribute", prefix = "nsi1", default)]
	pub extended_attribute: Vec<ExtendedAttributeType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MailboxSearchLocationType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MailboxSearchLocationType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MailboxSearchScopeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MailboxSearchScopeType {
	#[yaserde(rename = "Mailbox", prefix = "nsi1", default)]
	pub mailbox: String, 
	#[yaserde(rename = "SearchScope", prefix = "nsi1", default)]
	pub search_scope: MailboxSearchLocationType, 
	#[yaserde(rename = "ExtendedAttributes", prefix = "nsi1", default)]
	pub extended_attributes: Option<ArrayOfExtendedAttributesType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfMailboxSearchScopesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfMailboxSearchScopesType {
	#[yaserde(rename = "MailboxSearchScope", prefix = "nsi1", default)]
	pub mailbox_search_scope: Vec<MailboxSearchScopeType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MailboxQueryType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MailboxQueryType {
	#[yaserde(rename = "Query", prefix = "nsi1", default)]
	pub query: String, 
	#[yaserde(rename = "MailboxSearchScopes", prefix = "nsi1", default)]
	pub mailbox_search_scopes: NonEmptyArrayOfMailboxSearchScopesType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PreviewItemMailboxType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PreviewItemMailboxType {
	#[yaserde(rename = "MailboxId", prefix = "nsi1", default)]
	pub mailbox_id: String, 
	#[yaserde(rename = "PrimarySmtpAddress", prefix = "nsi1", default)]
	pub primary_smtp_address: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfMailboxQueriesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfMailboxQueriesType {
	#[yaserde(rename = "MailboxQuery", prefix = "nsi1", default)]
	pub mailbox_query: Vec<MailboxQueryType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchResultType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SearchResultType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchPageDirectionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SearchPageDirectionType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PreviewItemBaseShapeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PreviewItemBaseShapeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfExtendedFieldURIsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfExtendedFieldURIsType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PreviewItemResponseShapeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PreviewItemResponseShapeType {
	#[yaserde(rename = "BaseShape", prefix = "nsi1", default)]
	pub base_shape: PreviewItemBaseShapeType, 
	#[yaserde(rename = "AdditionalProperties", prefix = "nsi1", default)]
	pub additional_properties: Option<NonEmptyArrayOfExtendedFieldURIsType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchPreviewItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SearchPreviewItemType {
	#[yaserde(rename = "Id", prefix = "nsi1", default)]
	pub id: ItemIdType, 
	#[yaserde(rename = "Mailbox", prefix = "nsi1", default)]
	pub mailbox: Option<PreviewItemMailboxType>, 
	#[yaserde(rename = "ParentId", prefix = "nsi1", default)]
	pub parent_id: Option<ItemIdType>, 
	#[yaserde(rename = "ItemClass", prefix = "nsi1", default)]
	pub item_class: Option<ItemClassType>, 
	#[yaserde(rename = "UniqueHash", prefix = "nsi1", default)]
	pub unique_hash: Option<String>, 
	#[yaserde(rename = "SortValue", prefix = "nsi1", default)]
	pub sort_value: Option<String>, 
	#[yaserde(rename = "OwaLink", prefix = "nsi1", default)]
	pub owa_link: Option<String>, 
	#[yaserde(rename = "Sender", prefix = "nsi1", default)]
	pub sender: Option<String>, 
	#[yaserde(rename = "ToRecipients", prefix = "nsi1", default)]
	pub to_recipients: Option<ArrayOfSmtpAddressType>, 
	#[yaserde(rename = "CcRecipients", prefix = "nsi1", default)]
	pub cc_recipients: Option<ArrayOfSmtpAddressType>, 
	#[yaserde(rename = "BccRecipients", prefix = "nsi1", default)]
	pub bcc_recipients: Option<ArrayOfSmtpAddressType>, 
	#[yaserde(rename = "CreatedTime", prefix = "nsi1", default)]
	pub created_time: Option<String>, 
	#[yaserde(rename = "ReceivedTime", prefix = "nsi1", default)]
	pub received_time: Option<String>, 
	#[yaserde(rename = "SentTime", prefix = "nsi1", default)]
	pub sent_time: Option<String>, 
	#[yaserde(rename = "Subject", prefix = "nsi1", default)]
	pub subject: Option<String>, 
	#[yaserde(rename = "Size", prefix = "nsi1", default)]
	pub size: Option<i64>, 
	#[yaserde(rename = "Preview", prefix = "nsi1", default)]
	pub preview: Option<String>, 
	#[yaserde(rename = "Importance", prefix = "nsi1", default)]
	pub importance: Option<ImportanceChoicesType>, 
	#[yaserde(rename = "Read", prefix = "nsi1", default)]
	pub read: Option<bool>, 
	#[yaserde(rename = "HasAttachment", prefix = "nsi1", default)]
	pub has_attachment: Option<bool>, 
	#[yaserde(rename = "ExtendedProperties", prefix = "nsi1", default)]
	pub extended_properties: Option<NonEmptyArrayOfExtendedPropertyType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfSearchPreviewItemsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfSearchPreviewItemsType {
	#[yaserde(rename = "SearchPreviewItem", prefix = "nsi1", default)]
	pub search_preview_item: Vec<SearchPreviewItemType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FailedSearchMailboxType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FailedSearchMailboxType {
	#[yaserde(rename = "Mailbox", prefix = "nsi1", default)]
	pub mailbox: String, 
	#[yaserde(rename = "ErrorCode", prefix = "nsi1", default)]
	pub error_code: i32, 
	#[yaserde(rename = "ErrorMessage", prefix = "nsi1", default)]
	pub error_message: String, 
	#[yaserde(rename = "IsArchive", prefix = "nsi1", default)]
	pub is_archive: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfFailedSearchMailboxesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfFailedSearchMailboxesType {
	#[yaserde(rename = "FailedMailbox", prefix = "nsi1", default)]
	pub failed_mailbox: Vec<FailedSearchMailboxType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchMailboxesResultType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SearchMailboxesResultType {
	#[yaserde(rename = "SearchQueries", prefix = "nsi1", default)]
	pub search_queries: NonEmptyArrayOfMailboxQueriesType, 
	#[yaserde(rename = "ResultType", prefix = "nsi1", default)]
	pub result_type: SearchResultType, 
	#[yaserde(rename = "ItemCount", prefix = "nsi1", default)]
	pub item_count: i64, 
	#[yaserde(rename = "Size", prefix = "nsi1", default)]
	pub size: i64, 
	#[yaserde(rename = "PageItemCount", prefix = "nsi1", default)]
	pub page_item_count: i32, 
	#[yaserde(rename = "PageItemSize", prefix = "nsi1", default)]
	pub page_item_size: i64, 
	#[yaserde(rename = "KeywordStats", prefix = "nsi1", default)]
	pub keyword_stats: Option<ArrayOfKeywordStatisticsSearchResultsType>, 
	#[yaserde(rename = "Items", prefix = "nsi1", default)]
	pub items: Option<ArrayOfSearchPreviewItemsType>, 
	#[yaserde(rename = "FailedMailboxes", prefix = "nsi1", default)]
	pub failed_mailboxes: Option<ArrayOfFailedSearchMailboxesType>, 
	#[yaserde(rename = "Refiners", prefix = "nsi1", default)]
	pub refiners: Option<ArrayOfSearchRefinerItemsType>, 
	#[yaserde(rename = "MailboxStats", prefix = "nsi1", default)]
	pub mailbox_stats: Option<ArrayOfMailboxStatisticsItemsType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchRefinerItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SearchRefinerItemType {
	#[yaserde(rename = "Name", prefix = "nsi1", default)]
	pub name: String, 
	#[yaserde(rename = "Value", prefix = "nsi1", default)]
	pub value: String, 
	#[yaserde(rename = "Count", prefix = "nsi1", default)]
	pub count: i64, 
	#[yaserde(rename = "Token", prefix = "nsi1", default)]
	pub token: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfSearchRefinerItemsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfSearchRefinerItemsType {
	#[yaserde(rename = "Refiner", prefix = "nsi1", default)]
	pub refiner: Vec<SearchRefinerItemType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "OneDriveItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct OneDriveItemType {
	#[yaserde(flatten, default)]
	pub item_type: ItemType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ResourceId", prefix = "nsi1", default)]
	pub resource_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FileItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FileItemType {
	#[yaserde(flatten, default)]
	pub item_type: ItemType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "FileName", prefix = "nsi1", default)]
	pub file_name: Option<String>, 
	#[yaserde(rename = "FileExtension", prefix = "nsi1", default)]
	pub file_extension: Option<String>, 
	#[yaserde(rename = "FileSize", prefix = "nsi1", default)]
	pub file_size: Option<i64>, 
	#[yaserde(rename = "FileCreatedTime", prefix = "nsi1", default)]
	pub file_created_time: Option<String>, 
	#[yaserde(rename = "FileModifiedTime", prefix = "nsi1", default)]
	pub file_modified_time: Option<String>, 
	#[yaserde(rename = "StorageProviderContext", prefix = "nsi1", default)]
	pub storage_provider_context: Option<String>, 
	#[yaserde(rename = "FileID", prefix = "nsi1", default)]
	pub file_id: Option<String>, 
	#[yaserde(rename = "ItemReferenceId", prefix = "nsi1", default)]
	pub item_reference_id: Option<String>, 
	#[yaserde(rename = "ReferenceId", prefix = "nsi1", default)]
	pub reference_id: Option<String>, 
	#[yaserde(rename = "Sender", prefix = "nsi1", default)]
	pub sender: Option<String>, 
	#[yaserde(rename = "ItemReceivedTime", prefix = "nsi1", default)]
	pub item_received_time: Option<String>, 
	#[yaserde(rename = "ItemPath", prefix = "nsi1", default)]
	pub item_path: Option<String>, 
	#[yaserde(rename = "ItemSentTime", prefix = "nsi1", default)]
	pub item_sent_time: Option<String>, 
	#[yaserde(rename = "FileContexts", prefix = "nsi1", default)]
	pub file_contexts: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "VisualizationContainerUrl", prefix = "nsi1", default)]
	pub visualization_container_url: Option<String>, 
	#[yaserde(rename = "VisualizationContainerTitle", prefix = "nsi1", default)]
	pub visualization_container_title: Option<String>, 
	#[yaserde(rename = "VisualizationAccessUrl", prefix = "nsi1", default)]
	pub visualization_access_url: Option<String>, 
	#[yaserde(rename = "ReferenceAttachmentProviderEndpoint", prefix = "nsi1", default)]
	pub reference_attachment_provider_endpoint: Option<String>, 
	#[yaserde(rename = "ReferenceAttachmentProviderType", prefix = "nsi1", default)]
	pub reference_attachment_provider_type: Option<String>, 
	#[yaserde(rename = "ItemConversationId", prefix = "nsi1", default)]
	pub item_conversation_id: Option<String>, 
	#[yaserde(rename = "SharepointItemListId", prefix = "nsi1", default)]
	pub sharepoint_item_list_id: Option<GuidType>, 
	#[yaserde(rename = "SharepointItemListItemId", prefix = "nsi1", default)]
	pub sharepoint_item_list_item_id: Option<String>, 
	#[yaserde(rename = "SharepointItemSiteId", prefix = "nsi1", default)]
	pub sharepoint_item_site_id: Option<GuidType>, 
	#[yaserde(rename = "SharepointItemSitePath", prefix = "nsi1", default)]
	pub sharepoint_item_site_path: Option<String>, 
	#[yaserde(rename = "SharepointItemWebId", prefix = "nsi1", default)]
	pub sharepoint_item_web_id: Option<GuidType>, 
	#[yaserde(rename = "AttachmentId", prefix = "nsi1", default)]
	pub attachment_id: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DocumentFileItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DocumentFileItemType {
	#[yaserde(flatten, default)]
	pub file_item_type: FileItemType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Author", prefix = "nsi1", default)]
	pub author: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "Title", prefix = "nsi1", default)]
	pub title: Option<String>, 
	#[yaserde(rename = "LastModifiedBy", prefix = "nsi1", default)]
	pub last_modified_by: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DelveItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DelveItemType {
	#[yaserde(flatten, default)]
	pub item_type: ItemType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "GraphNodeLogicalId", prefix = "nsi1", default)]
	pub graph_node_logical_id: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MailboxStatisticsItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MailboxStatisticsItemType {
	#[yaserde(rename = "MailboxId", prefix = "nsi1", default)]
	pub mailbox_id: String, 
	#[yaserde(rename = "DisplayName", prefix = "nsi1", default)]
	pub display_name: String, 
	#[yaserde(rename = "ItemCount", prefix = "nsi1", default)]
	pub item_count: i64, 
	#[yaserde(rename = "Size", prefix = "nsi1", default)]
	pub size: i64, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfMailboxStatisticsItemsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfMailboxStatisticsItemsType {
	#[yaserde(rename = "MailboxStat", prefix = "nsi1", default)]
	pub mailbox_stat: Vec<MailboxStatisticsItemType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "HoldActionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct HoldActionType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "HoldStatusType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct HoldStatusType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MailboxHoldStatusType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MailboxHoldStatusType {
	#[yaserde(rename = "Mailbox", prefix = "nsi1", default)]
	pub mailbox: String, 
	#[yaserde(rename = "Status", prefix = "nsi1", default)]
	pub status: HoldStatusType, 
	#[yaserde(rename = "AdditionalInfo", prefix = "nsi1", default)]
	pub additional_info: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfMailboxHoldStatusType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfMailboxHoldStatusType {
	#[yaserde(rename = "MailboxHoldStatus", prefix = "nsi1", default)]
	pub mailbox_hold_status: Vec<MailboxHoldStatusType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MailboxHoldResultType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MailboxHoldResultType {
	#[yaserde(rename = "HoldId", prefix = "nsi1", default)]
	pub hold_id: String, 
	#[yaserde(rename = "Query", prefix = "nsi1", default)]
	pub query: Option<String>, 
	#[yaserde(rename = "MailboxHoldStatuses", prefix = "nsi1", default)]
	pub mailbox_hold_statuses: ArrayOfMailboxHoldStatusType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfLegacyDNsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfLegacyDNsType {
	#[yaserde(rename = "LegacyDN", prefix = "nsi1", default)]
	pub legacy_dn: Vec<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonIndexableItemStatisticType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonIndexableItemStatisticType {
	#[yaserde(rename = "Mailbox", prefix = "nsi1", default)]
	pub mailbox: String, 
	#[yaserde(rename = "ItemCount", prefix = "nsi1", default)]
	pub item_count: i64, 
	#[yaserde(rename = "ErrorMessage", prefix = "nsi1", default)]
	pub error_message: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfNonIndexableItemStatisticsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfNonIndexableItemStatisticsType {
	#[yaserde(rename = "NonIndexableItemStatistic", prefix = "nsi1", default)]
	pub non_indexable_item_statistic: Vec<NonIndexableItemStatisticType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ItemIndexErrorType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ItemIndexErrorType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonIndexableItemDetailType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonIndexableItemDetailType {
	#[yaserde(rename = "ItemId", prefix = "nsi1", default)]
	pub item_id: ItemIdType, 
	#[yaserde(rename = "ErrorCode", prefix = "nsi1", default)]
	pub error_code: ItemIndexErrorType, 
	#[yaserde(rename = "ErrorDescription", prefix = "nsi1", default)]
	pub error_description: String, 
	#[yaserde(rename = "IsPartiallyIndexed", prefix = "nsi1", default)]
	pub is_partially_indexed: bool, 
	#[yaserde(rename = "IsPermanentFailure", prefix = "nsi1", default)]
	pub is_permanent_failure: bool, 
	#[yaserde(rename = "SortValue", prefix = "nsi1", default)]
	pub sort_value: String, 
	#[yaserde(rename = "AttemptCount", prefix = "nsi1", default)]
	pub attempt_count: i32, 
	#[yaserde(rename = "LastAttemptTime", prefix = "nsi1", default)]
	pub last_attempt_time: Option<String>, 
	#[yaserde(rename = "AdditionalInfo", prefix = "nsi1", default)]
	pub additional_info: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfNonIndexableItemDetailsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfNonIndexableItemDetailsType {
	#[yaserde(rename = "NonIndexableItemDetail", prefix = "nsi1", default)]
	pub non_indexable_item_detail: Vec<NonIndexableItemDetailType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonIndexableItemDetailResultType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonIndexableItemDetailResultType {
	#[yaserde(rename = "Items", prefix = "nsi1", default)]
	pub items: Option<ArrayOfNonIndexableItemDetailsType>, 
	#[yaserde(rename = "FailedMailboxes", prefix = "nsi1", default)]
	pub failed_mailboxes: Option<ArrayOfFailedSearchMailboxesType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DiscoverySearchConfigurationType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DiscoverySearchConfigurationType {
	#[yaserde(rename = "SearchId", prefix = "nsi1", default)]
	pub search_id: String, 
	#[yaserde(rename = "SearchQuery", prefix = "nsi1", default)]
	pub search_query: String, 
	#[yaserde(rename = "SearchableMailboxes", prefix = "nsi1", default)]
	pub searchable_mailboxes: Option<ArrayOfSearchableMailboxesType>, 
	#[yaserde(rename = "InPlaceHoldIdentity", prefix = "nsi1", default)]
	pub in_place_hold_identity: Option<String>, 
	#[yaserde(rename = "ManagedByOrganization", prefix = "nsi1", default)]
	pub managed_by_organization: Option<String>, 
	#[yaserde(rename = "Language", prefix = "nsi1", default)]
	pub language: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfDiscoverySearchConfigurationType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfDiscoverySearchConfigurationType {
	#[yaserde(rename = "DiscoverySearchConfiguration", prefix = "nsi1", default)]
	pub discovery_search_configuration: Vec<DiscoverySearchConfigurationType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RetentionTagType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RetentionTagType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UserPhotoSizeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct UserPhotoSizeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UserPhotoTypeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct UserPhotoTypeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ElcFolderType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ElcFolderType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RetentionActionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RetentionActionType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RetentionPolicyTagType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RetentionPolicyTagType {
	#[yaserde(rename = "DisplayName", prefix = "nsi1", default)]
	pub display_name: String, 
	#[yaserde(rename = "RetentionId", prefix = "nsi1", default)]
	pub retention_id: GuidType, 
	#[yaserde(rename = "RetentionPeriod", prefix = "nsi1", default)]
	pub retention_period: i32, 
	#[yaserde(rename = "Type", prefix = "nsi1", default)]
	pub rs_type: ElcFolderType, 
	#[yaserde(rename = "RetentionAction", prefix = "nsi1", default)]
	pub retention_action: RetentionActionType, 
	#[yaserde(rename = "Description", prefix = "nsi1", default)]
	pub description: Option<String>, 
	#[yaserde(rename = "IsVisible", prefix = "nsi1", default)]
	pub is_visible: bool, 
	#[yaserde(rename = "OptedInto", prefix = "nsi1", default)]
	pub opted_into: bool, 
	#[yaserde(rename = "IsArchive", prefix = "nsi1", default)]
	pub is_archive: bool, 
	#[yaserde(rename = "ParentLabelIdentity", prefix = "nsi1", default)]
	pub parent_label_identity: Option<String>, 
	#[yaserde(rename = "Priority", prefix = "nsi1", default)]
	pub priority: Option<i32>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfRetentionPolicyTagsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfRetentionPolicyTagsType {
	#[yaserde(rename = "RetentionPolicyTag", prefix = "nsi1", default)]
	pub retention_policy_tag: Vec<RetentionPolicyTagType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RetentionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RetentionType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ClientExtensionProvidedToType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ClientExtensionProvidedToType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ClientExtensionTypeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ClientExtensionTypeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ClientExtensionScopeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ClientExtensionScopeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ClientExtensionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ClientExtensionType {
#[yaserde(rename="IsAvailable", attribute)]
pub is_available: Option<bool>,
#[yaserde(rename="IsMandatory", attribute)]
pub is_mandatory: Option<bool>,
#[yaserde(rename="IsEnabledByDefault", attribute)]
pub is_enabled_by_default: Option<bool>,
#[yaserde(rename="ProvidedTo", attribute)]
pub provided_to: Option<ClientExtensionProvidedToType>,
#[yaserde(rename="Type", attribute)]
pub r#type: Option<ClientExtensionTypeType>,
#[yaserde(rename="Scope", attribute)]
pub scope: Option<ClientExtensionScopeType>,
#[yaserde(rename="MarketplaceAssetId", attribute)]
pub marketplace_asset_id: Option<String>,
#[yaserde(rename="MarketplaceContentMarket", attribute)]
pub marketplace_content_market: Option<String>,
#[yaserde(rename="AppStatus", attribute)]
pub app_status: Option<String>,
#[yaserde(rename="Etoken", attribute)]
pub etoken: Option<String>,
#[yaserde(rename="InstalledDateTime", attribute)]
pub installed_date_time: Option<String>,
#[yaserde(rename="Version", attribute)]
pub version: Option<String>,
#[yaserde(rename="ExtensionId", attribute)]
pub extension_id: Option<String>,
	#[yaserde(rename = "SpecificUsers", prefix = "nsi1", default)]
	pub specific_users: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "Manifest", prefix = "nsi1", default)]
	pub manifest: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ConsentStateType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ConsentStateType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ImGroupType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ImGroupType {
	#[yaserde(rename = "DisplayName", prefix = "nsi1", default)]
	pub display_name: NonEmptyStringType, 
	#[yaserde(rename = "GroupType", prefix = "nsi1", default)]
	pub group_type: NonEmptyStringType, 
	#[yaserde(rename = "ExchangeStoreId", prefix = "nsi1", default)]
	pub exchange_store_id: Option<ItemIdType>, 
	#[yaserde(rename = "MemberCorrelationKey", prefix = "nsi1", default)]
	pub member_correlation_key: Option<NonEmptyArrayOfItemIdsType>, 
	#[yaserde(rename = "ExtendedProperties", prefix = "nsi1", default)]
	pub extended_properties: Option<NonEmptyArrayOfExtendedPropertyType>, 
	#[yaserde(rename = "SmtpAddress", prefix = "nsi1", default)]
	pub smtp_address: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfImGroupType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfImGroupType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ImItemListType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ImItemListType {
	#[yaserde(rename = "Groups", prefix = "nsi1", default)]
	pub groups: Option<ArrayOfImGroupType>, 
	#[yaserde(rename = "Personas", prefix = "nsi1", default)]
	pub personas: Option<ArrayOfPeopleType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfExtendedFieldURIs",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfExtendedFieldURIs {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DisableReasonType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DisableReasonType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyStateDefinitionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyStateDefinitionType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BaseCalendarItemStateDefinitionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct BaseCalendarItemStateDefinitionType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeletedOccurrenceStateDefinitionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DeletedOccurrenceStateDefinitionType {
	#[yaserde(flatten, default)]
	pub base_calendar_item_state_definition_type: BaseCalendarItemStateDefinitionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "OccurrenceDate", prefix = "nsi1", default)]
	pub occurrence_date: String, 
	#[yaserde(rename = "IsOccurrencePresent", prefix = "nsi1", default)]
	pub is_occurrence_present: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeleteFromFolderStateDefinitionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DeleteFromFolderStateDefinitionType {
	#[yaserde(flatten, default)]
	pub base_calendar_item_state_definition_type: BaseCalendarItemStateDefinitionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "LocationBasedStateDefinitionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct LocationBasedStateDefinitionType {
	#[yaserde(flatten, default)]
	pub base_calendar_item_state_definition_type: BaseCalendarItemStateDefinitionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "OrganizerLocation", prefix = "nsi1", default)]
	pub organizer_location: String, 
	#[yaserde(rename = "AttendeeLocation", prefix = "nsi1", default)]
	pub attendee_location: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "IconIndexType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct IconIndexType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MeetingSpaceTypeEnum",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MeetingSpaceTypeEnum {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MeetingSpaceType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MeetingSpaceType {
	#[yaserde(rename = "Id", prefix = "nsi1", default)]
	pub id: Option<String>, 
	#[yaserde(rename = "ChangeKey", prefix = "nsi1", default)]
	pub change_key: Option<String>, 
	#[yaserde(rename = "Type", prefix = "nsi1", default)]
	pub rs_type: Option<MeetingSpaceTypeEnum>, 
	#[yaserde(rename = "Version", prefix = "nsi1", default)]
	pub version: Option<String>, 
	#[yaserde(rename = "JoinUrl", prefix = "nsi1", default)]
	pub join_url: Option<String>, 
	#[yaserde(rename = "DateTimeCreated", prefix = "nsi1", default)]
	pub date_time_created: Option<String>, 
	#[yaserde(rename = "DateTimeModified", prefix = "nsi1", default)]
	pub date_time_modified: Option<String>, 
	#[yaserde(rename = "ExpiryTime", prefix = "nsi1", default)]
	pub expiry_time: Option<String>, 
	#[yaserde(rename = "Meadata", prefix = "nsi1", default)]
	pub meadata: Option<String>, 
	#[yaserde(rename = "Tag", prefix = "nsi1", default)]
	pub tag: Option<String>, 
}
pub type MeetingSpace = MeetingSpaceType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ParticipantActivityRole",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ParticipantActivityRole {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ParticipantActivityMediaType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ParticipantActivityMediaType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ParticipantActivity",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ParticipantActivity {
	#[yaserde(rename = "Id", prefix = "nsi1", default)]
	pub id: GuidType, 
	#[yaserde(rename = "CreatedBy", prefix = "nsi1", default)]
	pub created_by: String, 
	#[yaserde(rename = "StartTime", prefix = "nsi1", default)]
	pub start_time: String, 
	#[yaserde(rename = "EndTime", prefix = "nsi1", default)]
	pub end_time: String, 
	#[yaserde(rename = "ClientVersion", prefix = "nsi1", default)]
	pub client_version: String, 
	#[yaserde(rename = "Role", prefix = "nsi1", default)]
	pub role: ParticipantActivityRole, 
	#[yaserde(rename = "MediaType", prefix = "nsi1", default)]
	pub media_type: ParticipantActivityMediaType, 
	#[yaserde(rename = "MediaDetails", prefix = "nsi1", default)]
	pub media_details: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfParticipantActivities",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfParticipantActivities {
	#[yaserde(rename = "ParticipantActivity", prefix = "nsi1", default)]
	pub participant_activity: Vec<ParticipantActivity>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ContentActivity",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ContentActivity {
	#[yaserde(rename = "Id", prefix = "nsi1", default)]
	pub id: GuidType, 
	#[yaserde(rename = "SharedBy", prefix = "nsi1", default)]
	pub shared_by: String, 
	#[yaserde(rename = "ContentLocation", prefix = "nsi1", default)]
	pub content_location: String, 
	#[yaserde(rename = "StartTime", prefix = "nsi1", default)]
	pub start_time: String, 
	#[yaserde(rename = "EndTime", prefix = "nsi1", default)]
	pub end_time: String, 
	#[yaserde(rename = "Type", prefix = "nsi1", default)]
	pub rs_type: ContentActivityType, 
	#[yaserde(rename = "MediaType", prefix = "nsi1", default)]
	pub media_type: ContentActivityMediaType, 
	#[yaserde(rename = "Acl", prefix = "nsi1", default)]
	pub acl: ContentActivityAcl, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ContentActivityType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ContentActivityType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ContentActivityMediaType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ContentActivityMediaType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ContentActivityAcl",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ContentActivityAcl {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfContentActivities",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfContentActivities {
	#[yaserde(rename = "ContentActivity", prefix = "nsi1", default)]
	pub content_activity: Vec<ContentActivity>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MeetingInstanceType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MeetingInstanceType {
	#[yaserde(rename = "Id", prefix = "nsi1", default)]
	pub id: Option<String>, 
	#[yaserde(rename = "ChangeKey", prefix = "nsi1", default)]
	pub change_key: Option<String>, 
	#[yaserde(rename = "Version", prefix = "nsi1", default)]
	pub version: Option<String>, 
	#[yaserde(rename = "JoinUrl", prefix = "nsi1", default)]
	pub join_url: Option<String>, 
	#[yaserde(rename = "DateTimeCreated", prefix = "nsi1", default)]
	pub date_time_created: Option<String>, 
	#[yaserde(rename = "DateTimeModified", prefix = "nsi1", default)]
	pub date_time_modified: Option<String>, 
	#[yaserde(rename = "Meadata", prefix = "nsi1", default)]
	pub meadata: Option<String>, 
	#[yaserde(rename = "Tag", prefix = "nsi1", default)]
	pub tag: Option<String>, 
	#[yaserde(rename = "ParentGoid", prefix = "nsi1", default)]
	pub parent_goid: Option<String>, 
	#[yaserde(rename = "ParticipantActivities", prefix = "nsi1", default)]
	pub participant_activities: Option<NonEmptyArrayOfParticipantActivities>, 
	#[yaserde(rename = "ContentActivities", prefix = "nsi1", default)]
	pub content_activities: Option<NonEmptyArrayOfContentActivities>, 
}
pub type MeetingInstance = MeetingInstanceType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "WarmupOptionsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct WarmupOptionsType {
	#[yaserde(flatten, default)]
	pub body: Box<WarmupOptionsType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchApplicationIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SearchApplicationIdType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ItemTypesFilterType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ItemTypesFilterType {
	#[yaserde(flatten, default)]
	pub body: Box<ItemTypesFilterType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SuggestionKindType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SuggestionKindType {
	#[yaserde(flatten, default)]
	pub body: Box<SuggestionKindType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RefinerTypeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RefinerTypeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchResultsPropertySetNameType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SearchResultsPropertySetNameType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchScopeGroupsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SearchScopeGroupsType {
	#[yaserde(flatten, default)]
	pub body: Box<SearchScopeGroupsType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchScopeArchivesType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SearchScopeArchivesType {
	#[yaserde(flatten, default)]
	pub body: Box<SearchScopeArchivesType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ExecuteSearchSortOrderType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ExecuteSearchSortOrderType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MatchOptionsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MatchOptionsType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchFolderScopeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SearchFolderScopeType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PrimaryMailboxSearchScopeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PrimaryMailboxSearchScopeType {
	#[yaserde(rename = "FolderScope", prefix = "nsi1", default)]
	pub folder_scope: Option<SearchFolderScopeType>, 
	#[yaserde(rename = "IsDeepTraversal", prefix = "nsi1", default)]
	pub is_deep_traversal: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "LargeArchiveSearchScopeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct LargeArchiveSearchScopeType {
	#[yaserde(rename = "ArchiveTypes", prefix = "nsi1", default)]
	pub archive_types: SearchScopeArchivesType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GroupSearchScopeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct GroupSearchScopeType {
	#[yaserde(rename = "GroupTypes", prefix = "nsi1", default)]
	pub group_types: SearchScopeGroupsType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SingleLargeArchiveSearchScopeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SingleLargeArchiveSearchScopeType {
	#[yaserde(rename = "MailboxGuid", prefix = "nsi1", default)]
	pub mailbox_guid: Option<GuidType>, 
	#[yaserde(rename = "FolderScope", prefix = "nsi1", default)]
	pub folder_scope: SearchFolderScopeType, 
	#[yaserde(rename = "IsDeepTraversal", prefix = "nsi1", default)]
	pub is_deep_traversal: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "OneDriveSearchScopeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct OneDriveSearchScopeType {
	#[yaserde(rename = "OneDriveView", prefix = "nsi1", default)]
	pub one_drive_view: OneDriveViewType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DelveSearchScopeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DelveSearchScopeType {
	#[yaserde(rename = "DelveView", prefix = "nsi1", default)]
	pub delve_view: DelveViewType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CustomSearchScopeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct CustomSearchScopeType {
	#[yaserde(rename = "MailboxGuid", prefix = "nsi1", default)]
	pub mailbox_guid: GuidType, 
	#[yaserde(rename = "FolderScope", prefix = "nsi1", default)]
	pub folder_scope: SearchFolderScopeType, 
	#[yaserde(rename = "IsDeepTraversal", prefix = "nsi1", default)]
	pub is_deep_traversal: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfSearchScopeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfSearchScopeType {
	#[yaserde(rename = "PrimaryMailboxSearchScope", prefix = "nsi1", default)]
	pub primary_mailbox_search_scope: Option<PrimaryMailboxSearchScopeType>, 
	#[yaserde(rename = "LargeArchiveSearchScope", prefix = "nsi1", default)]
	pub large_archive_search_scope: Option<LargeArchiveSearchScopeType>, 
	#[yaserde(rename = "GroupSearchScope", prefix = "nsi1", default)]
	pub group_search_scope: Option<GroupSearchScopeType>, 
	#[yaserde(rename = "CustomSearchScope", prefix = "nsi1", default)]
	pub custom_search_scope: Vec<CustomSearchScopeType>, 
	#[yaserde(rename = "OneDriveSearchScope", prefix = "nsi1", default)]
	pub one_drive_search_scope: Option<OneDriveSearchScopeType>, 
	#[yaserde(rename = "SingleLargeArchiveSearchScope", prefix = "nsi1", default)]
	pub single_large_archive_search_scope: Option<SingleLargeArchiveSearchScopeType>, 
	#[yaserde(rename = "DelveSearchScope", prefix = "nsi1", default)]
	pub delve_search_scope: Option<DelveSearchScopeType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DynamicRefinerQueryType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DynamicRefinerQueryType {
	#[yaserde(rename = "RefinerQuery", prefix = "nsi1", default)]
	pub refiner_query: String, 
	#[yaserde(rename = "TDRefinerId", prefix = "nsi1", default)]
	pub td_refiner_id: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ExtendedKeywordDefinitionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ExtendedKeywordDefinitionType {
	#[yaserde(rename = "Keyword", prefix = "nsi1", default)]
	pub keyword: String, 
	#[yaserde(rename = "Properties", prefix = "nsi1", default)]
	pub properties: NonEmptyArrayOfExtendedFieldURIsType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SuggestionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SuggestionType {
	#[yaserde(rename = "SuggestedQuery", prefix = "nsi1", default)]
	pub suggested_query: String, 
	#[yaserde(rename = "DisplayText", prefix = "nsi1", default)]
	pub display_text: String, 
	#[yaserde(rename = "SuggestionType", prefix = "nsi1", default)]
	pub suggestion_type: SuggestionKindType, 
	#[yaserde(rename = "Trigger", prefix = "nsi1", default)]
	pub trigger: Option<String>, 
	#[yaserde(rename = "TDSuggestionId", prefix = "nsi1", default)]
	pub td_suggestion_id: i32, 
	#[yaserde(rename = "IsDeletable", prefix = "nsi1", default)]
	pub is_deletable: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PeopleSuggestionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PeopleSuggestionType {
	#[yaserde(flatten, default)]
	pub suggestion_type: SuggestionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "PrimarySmtpAddress", prefix = "nsi1", default)]
	pub primary_smtp_address: String, 
	#[yaserde(rename = "PersonType", prefix = "nsi1", default)]
	pub person_type: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PeopleSuggestionPersonType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PeopleSuggestionPersonType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FileSuggestionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FileSuggestionType {
	#[yaserde(flatten, default)]
	pub suggestion_type: SuggestionType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "FileName", prefix = "nsi1", default)]
	pub file_name: String, 
	#[yaserde(rename = "FileExtension", prefix = "nsi1", default)]
	pub file_extension: String, 
	#[yaserde(rename = "FileReferenceId", prefix = "nsi1", default)]
	pub file_reference_id: Option<String>, 
	#[yaserde(rename = "FileTitle", prefix = "nsi1", default)]
	pub file_title: Option<String>, 
	#[yaserde(rename = "ContainerTitle", prefix = "nsi1", default)]
	pub container_title: Option<String>, 
	#[yaserde(rename = "ContainerUrl", prefix = "nsi1", default)]
	pub container_url: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Suggestions",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct Suggestions {
	#[yaserde(rename = "Suggestion", prefix = "nsi1", default)]
	pub suggestion: Vec<SuggestionType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchSuggestionsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SearchSuggestionsType {
	#[yaserde(rename = "TDSuggestionsBatchId", prefix = "nsi1", default)]
	pub td_suggestions_batch_id: i64, 
	#[yaserde(rename = "TDSuggestionsInstanceId", prefix = "nsi1", default)]
	pub td_suggestions_instance_id: GuidType, 
	#[yaserde(rename = "Suggestions", prefix = "nsi1", default)]
	pub suggestions: Option<Suggestions>, 
	#[yaserde(rename = "DiagnosticsData", prefix = "nsi1", default)]
	pub diagnostics_data: Option<SearchDiagnosticsType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeleteSearchSuggestionResponseType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct DeleteSearchSuggestionResponseType {
	#[yaserde(rename = "Success", prefix = "nsi1", default)]
	pub success: bool, 
	#[yaserde(rename = "StatusMessage", prefix = "nsi1", default)]
	pub status_message: Option<String>, 
	#[yaserde(rename = "DiagnosticsData", prefix = "nsi1", default)]
	pub diagnostics_data: Option<SearchDiagnosticsType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AdditionalEntries",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AdditionalEntries {
	#[yaserde(rename = "Entry", prefix = "nsi1", default)]
	pub entry: Vec<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchDiagnosticsStepType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SearchDiagnosticsStepType {
	#[yaserde(rename = "StartTime", prefix = "nsi1", default)]
	pub start_time: Option<String>, 
	#[yaserde(rename = "StepTime", prefix = "nsi1", default)]
	pub step_time: Option<i64>, 
	#[yaserde(rename = "StepType", prefix = "nsi1", default)]
	pub step_type: Option<String>, 
	#[yaserde(rename = "AdditionalEntries", prefix = "nsi1", default)]
	pub additional_entries: Option<AdditionalEntries>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchRefinerType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SearchRefinerType {
	#[yaserde(rename = "RefinerType", prefix = "nsi1", default)]
	pub refiner_type: RefinerTypeType, 
	#[yaserde(rename = "Refiner", prefix = "nsi1", default)]
	pub refiner: DynamicRefinerQueryType, 
	#[yaserde(rename = "ResultCount", prefix = "nsi1", default)]
	pub result_count: Option<i64>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FolderRefinerType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FolderRefinerType {
	#[yaserde(flatten, default)]
	pub search_refiner_type: SearchRefinerType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "FolderId", prefix = "nsi1", default)]
	pub folder_id: Option<FolderIdType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PeopleRefinerType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct PeopleRefinerType {
	#[yaserde(flatten, default)]
	pub search_refiner_type: SearchRefinerType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "DisplayName", prefix = "nsi1", default)]
	pub display_name: Option<String>, 
	#[yaserde(rename = "SmtpAddress", prefix = "nsi1", default)]
	pub smtp_address: Option<SmtpAddressType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MailboxRefinerType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MailboxRefinerType {
	#[yaserde(flatten, default)]
	pub search_refiner_type: SearchRefinerType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MailboxDisplayName", prefix = "nsi1", default)]
	pub mailbox_display_name: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AttachmentRefinerType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AttachmentRefinerType {
	#[yaserde(flatten, default)]
	pub search_refiner_type: SearchRefinerType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "HasAttachment", prefix = "nsi1", default)]
	pub has_attachment: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "HashtagRefinerType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct HashtagRefinerType {
	#[yaserde(flatten, default)]
	pub search_refiner_type: SearchRefinerType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "DisplayName", prefix = "nsi1", default)]
	pub display_name: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchDiagnosticsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SearchDiagnosticsType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MailboxInformationType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MailboxInformationType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ExecuteSearchQueryIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ExecuteSearchQueryIdType {
#[yaserde(rename="Id", attribute)]
pub id: GuidType,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Items",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct Items {
	#[yaserde(rename = "Item", prefix = "nsi1", default)]
	pub item: Vec<ItemType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Conversations",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct Conversations {
	#[yaserde(rename = "Conversation", prefix = "nsi1", default)]
	pub conversation: Vec<ConversationType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "People",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct People {
	#[yaserde(rename = "Persona", prefix = "nsi1", default)]
	pub persona: Vec<PersonaType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchRefiners",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SearchRefiners {
	#[yaserde(rename = "SearchRefiner", prefix = "nsi1", default)]
	pub search_refiner: Vec<SearchRefinerType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MailboxesInformation",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MailboxesInformation {
	#[yaserde(rename = "MailboxInformation", prefix = "nsi1", default)]
	pub mailbox_information: Vec<MailboxInformationType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchResultsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SearchResultsType {
	#[yaserde(rename = "Items", prefix = "nsi1", default)]
	pub items: Option<Items>, 
	#[yaserde(rename = "Conversations", prefix = "nsi1", default)]
	pub conversations: Option<Conversations>, 
	#[yaserde(rename = "People", prefix = "nsi1", default)]
	pub people: Option<People>, 
	#[yaserde(rename = "MoreResultsAvailable", prefix = "nsi1", default)]
	pub more_results_available: bool, 
	#[yaserde(rename = "RefinerTelemetryBatchId", prefix = "nsi1", default)]
	pub refiner_telemetry_batch_id: i32, 
	#[yaserde(rename = "SearchRefiners", prefix = "nsi1", default)]
	pub search_refiners: Option<SearchRefiners>, 
	#[yaserde(rename = "DiagnosticsData", prefix = "nsi1", default)]
	pub diagnostics_data: Option<SearchDiagnosticsType>, 
	#[yaserde(rename = "SearchResultsCount", prefix = "nsi1", default)]
	pub search_results_count: Option<i32>, 
	#[yaserde(rename = "TotalResultsCount", prefix = "nsi1", default)]
	pub total_results_count: Option<i32>, 
	#[yaserde(rename = "SearchTerms", prefix = "nsi1", default)]
	pub search_terms: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "QueryId", prefix = "nsi1", default)]
	pub query_id: Option<ExecuteSearchQueryIdType>, 
	#[yaserde(rename = "MailboxesInformation", prefix = "nsi1", default)]
	pub mailboxes_information: Option<MailboxesInformation>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchResultItemIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SearchResultItemIdType {
	#[yaserde(flatten, default)]
	pub item_id_type: ItemIdType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "EntryId", prefix = "nsi1", default)]
	pub entry_id: Option<String>, 
	#[yaserde(rename = "OutlookItemId", prefix = "nsi1", default)]
	pub outlook_item_id: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchResultConversationIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SearchResultConversationIdType {
	#[yaserde(flatten, default)]
	pub item_id_type: ItemIdType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "OutlookConversationId", prefix = "nsi1", default)]
	pub outlook_conversation_id: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ClientIdType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ClientIdType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EntityFeedbackEntityAddSourceType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct EntityFeedbackEntityAddSourceType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfWorkHours",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfWorkHours {
	#[yaserde(rename = "WorkHours", prefix = "nsi1", default)]
	pub work_hours: Vec<WorkHoursType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "WorkHoursType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct WorkHoursType {
	#[yaserde(rename = "WorkDay", prefix = "nsi1", default)]
	pub work_day: Option<SystemDayOfWeek>, 
	#[yaserde(rename = "TimeSlots", prefix = "nsi1", default)]
	pub time_slots: Option<ArrayOfWorkTimeSlot>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SystemDayOfWeek",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct SystemDayOfWeek {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfWorkTimeSlot",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfWorkTimeSlot {
	#[yaserde(rename = "TimeSlot", prefix = "nsi1", default)]
	pub time_slot: Vec<WorkTimeSlot>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "WorkTimeSlot",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct WorkTimeSlot {
	#[yaserde(rename = "StartTimeInMinutes", prefix = "nsi1", default)]
	pub start_time_in_minutes: i32, 
	#[yaserde(rename = "EndTimeInMinutes", prefix = "nsi1", default)]
	pub end_time_in_minutes: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AnalyzedQuery",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AnalyzedQuery {
	#[yaserde(rename = "QueryLanguage", prefix = "nsi1", default)]
	pub query_language: String, 
	#[yaserde(rename = "SearchRestrictions", prefix = "nsi1", default)]
	pub search_restrictions: RestrictionType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ContextPropertyType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ContextPropertyType {
	#[yaserde(rename = "Key", prefix = "nsi1", default)]
	pub key: String, 
	#[yaserde(rename = "Value", prefix = "nsi1", default)]
	pub value: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfContextProperty",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfContextProperty {
	#[yaserde(rename = "ContextProperty", prefix = "nsi1", default)]
	pub context_property: Vec<ContextPropertyType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfEntityFeedbackEntry",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfEntityFeedbackEntry {
	#[yaserde(rename = "EntityFeedbackEntry", prefix = "nsi1", default)]
	pub entity_feedback_entry: Vec<EntityFeedbackEntryType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EntityFeedbackEntryType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct EntityFeedbackEntryType {
	#[yaserde(rename = "ClientEventTimeUtc", prefix = "nsi1", default)]
	pub client_event_time_utc: String, 
	#[yaserde(rename = "ClientEventTimeLocal", prefix = "nsi1", default)]
	pub client_event_time_local: String, 
	#[yaserde(rename = "ClientSessionId", prefix = "nsi1", default)]
	pub client_session_id: GuidType, 
	#[yaserde(rename = "ClientVersion", prefix = "nsi1", default)]
	pub client_version: String, 
	#[yaserde(rename = "ClientId", prefix = "nsi1", default)]
	pub client_id: ClientIdType, 
	#[yaserde(rename = "EntrySequenceNumber", prefix = "nsi1", default)]
	pub entry_sequence_number: i32, 
	#[yaserde(rename = "TransactionId", prefix = "nsi1", default)]
	pub transaction_id: Option<String>, 
	#[yaserde(rename = "EventType", prefix = "nsi1", default)]
	pub event_type: String, 
	#[yaserde(rename = "TargetEntityList", prefix = "nsi1", default)]
	pub target_entity_list: Option<String>, 
	#[yaserde(rename = "EntityAddSource", prefix = "nsi1", default)]
	pub entity_add_source: Option<EntityFeedbackEntityAddSourceType>, 
	#[yaserde(rename = "JsonPropertyBag", prefix = "nsi1", default)]
	pub json_property_bag: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfInboxReminderType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfInboxReminderType {
	#[yaserde(rename = "InboxReminder", prefix = "nsi1", default)]
	pub inbox_reminder: Vec<InboxReminderType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "InboxReminderType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct InboxReminderType {
	#[yaserde(rename = "Id", prefix = "nsi1", default)]
	pub id: Option<GuidType>, 
	#[yaserde(rename = "ReminderOffset", prefix = "nsi1", default)]
	pub reminder_offset: Option<i32>, 
	#[yaserde(rename = "Message", prefix = "nsi1", default)]
	pub message: Option<String>, 
	#[yaserde(rename = "IsOrganizerReminder", prefix = "nsi1", default)]
	pub is_organizer_reminder: Option<bool>, 
	#[yaserde(rename = "OccurrenceChange", prefix = "nsi1", default)]
	pub occurrence_change: Option<EmailReminderChangeType>, 
	#[yaserde(rename = "SendOption", prefix = "nsi1", default)]
	pub send_option: Option<EmailReminderSendOption>, 
	#[yaserde(rename = "Subject", prefix = "nsi1", default)]
	pub subject: Option<String>, 
	#[yaserde(rename = "RichTextMessage", prefix = "nsi1", default)]
	pub rich_text_message: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EmailReminderChangeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct EmailReminderChangeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EmailReminderSendOption",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct EmailReminderSendOption {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "InsightStatusType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct InsightStatusType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "InsightContextItem",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct InsightContextItem {
	#[yaserde(rename = "Name", prefix = "nsi1", default)]
	pub name: Option<String>, 
	#[yaserde(rename = "Val", prefix = "nsi1", default)]
	pub val: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfInsightContextItem",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfInsightContextItem {
	#[yaserde(rename = "Context", prefix = "nsi1", default)]
	pub context: Vec<InsightContextItem>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "InsightItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct InsightItemType {
	#[yaserde(rename = "ItemId", prefix = "nsi1", default)]
	pub item_id: ItemIdType, 
	#[yaserde(rename = "InsightId", prefix = "nsi1", default)]
	pub insight_id: GuidType, 
	#[yaserde(rename = "Type", prefix = "nsi1", default)]
	pub rs_type: Option<String>, 
	#[yaserde(rename = "StartTimeUtc", prefix = "nsi1", default)]
	pub start_time_utc: Option<String>, 
	#[yaserde(rename = "EndTimeUtc", prefix = "nsi1", default)]
	pub end_time_utc: Option<String>, 
	#[yaserde(rename = "Status", prefix = "nsi1", default)]
	pub status: Option<InsightStatusType>, 
	#[yaserde(rename = "Version", prefix = "nsi1", default)]
	pub version: Option<String>, 
	#[yaserde(rename = "Context", prefix = "nsi1", default)]
	pub context: Option<ArrayOfInsightContextItem>, 
	#[yaserde(rename = "Text", prefix = "nsi1", default)]
	pub text: Option<String>, 
	#[yaserde(rename = "ApplicationsIds", prefix = "nsi1", default)]
	pub applications_ids: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "SuggestedActions", prefix = "nsi1", default)]
	pub suggested_actions: Option<String>, 
	#[yaserde(rename = "AppContexts", prefix = "nsi1", default)]
	pub app_contexts: Option<ArrayOfStringsType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfInsightItemType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfInsightItemType {
	#[yaserde(rename = "Insight", prefix = "nsi1", default)]
	pub insight: Vec<InsightItemType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ResponseMessageInfoType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ResponseMessageInfoType {
#[yaserde(rename="ResponseClass", attribute)]
pub response_class: ResponseClassType,
	#[yaserde(rename = "MessageText", prefix = "nsi1", default)]
	pub message_text: Option<String>, 
	#[yaserde(rename = "ResponseCode", prefix = "nsi1", default)]
	pub response_code: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BaseAvailabilityCalendarViewType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct BaseAvailabilityCalendarViewType {
	#[yaserde(rename = "FreeBusyViewType", prefix = "nsi1", default)]
	pub free_busy_view_type: FreeBusyViewType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfPeopleQuerySource",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfPeopleQuerySource {
	#[yaserde(rename = "Source", prefix = "nsi1", default)]
	pub source: Vec<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CalendarActivityDataType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct CalendarActivityDataType {
	#[yaserde(rename = "ActivityAction", prefix = "nsi1", default)]
	pub activity_action: String, 
	#[yaserde(rename = "ClientId", prefix = "nsi1", default)]
	pub client_id: String, 
	#[yaserde(rename = "CasRequestId", prefix = "nsi1", default)]
	pub cas_request_id: GuidType, 
	#[yaserde(rename = "IndexSelected", prefix = "nsi1", default)]
	pub index_selected: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MentionActionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MentionActionType {
	#[yaserde(rename = "Id", prefix = "nsi1", default)]
	pub id: String, 
	#[yaserde(rename = "CreatedBy", prefix = "nsi1", default)]
	pub created_by: EmailAddressExtendedType, 
	#[yaserde(rename = "CreatedDateTime", prefix = "nsi1", default)]
	pub created_date_time: Option<String>, 
	#[yaserde(rename = "ServerCreatedDateTime", prefix = "nsi1", default)]
	pub server_created_date_time: Option<String>, 
	#[yaserde(rename = "DeepLink", prefix = "nsi1", default)]
	pub deep_link: Option<String>, 
	#[yaserde(rename = "Application", prefix = "nsi1", default)]
	pub application: Option<String>, 
	#[yaserde(rename = "Mentioned", prefix = "nsi1", default)]
	pub mentioned: EmailAddressExtendedType, 
	#[yaserde(rename = "MentionText", prefix = "nsi1", default)]
	pub mention_text: Option<String>, 
	#[yaserde(rename = "ClientReference", prefix = "nsi1", default)]
	pub client_reference: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfMentionActionsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfMentionActionsType {
	#[yaserde(rename = "MentionAction", prefix = "nsi1", default)]
	pub mention_action: Vec<MentionActionType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AppliedHashtagType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AppliedHashtagType {
	#[yaserde(rename = "Id", prefix = "nsi1", default)]
	pub id: String, 
	#[yaserde(rename = "CreatedBy", prefix = "nsi1", default)]
	pub created_by: EmailAddressExtendedType, 
	#[yaserde(rename = "CreatedDateTime", prefix = "nsi1", default)]
	pub created_date_time: Option<String>, 
	#[yaserde(rename = "ServerCreatedDateTime", prefix = "nsi1", default)]
	pub server_created_date_time: Option<String>, 
	#[yaserde(rename = "DeepLink", prefix = "nsi1", default)]
	pub deep_link: Option<String>, 
	#[yaserde(rename = "Application", prefix = "nsi1", default)]
	pub application: Option<String>, 
	#[yaserde(rename = "Tag", prefix = "nsi1", default)]
	pub tag: String, 
	#[yaserde(rename = "IsAutoTagged", prefix = "nsi1", default)]
	pub is_auto_tagged: bool, 
	#[yaserde(rename = "IsInlined", prefix = "nsi1", default)]
	pub is_inlined: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AppliedHashtagsPreviewType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AppliedHashtagsPreviewType {
	#[yaserde(rename = "Hashtags", prefix = "nsi1", default)]
	pub hashtags: ArrayOfStringsType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfAppliedHashtagType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfAppliedHashtagType {
	#[yaserde(rename = "AppliedHashtag", prefix = "nsi1", default)]
	pub applied_hashtag: Vec<AppliedHashtagType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "LikeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct LikeType {
	#[yaserde(rename = "Id", prefix = "nsi1", default)]
	pub id: String, 
	#[yaserde(rename = "CreatedBy", prefix = "nsi1", default)]
	pub created_by: EmailAddressExtendedType, 
	#[yaserde(rename = "CreatedDateTime", prefix = "nsi1", default)]
	pub created_date_time: Option<String>, 
	#[yaserde(rename = "ServerCreatedDateTime", prefix = "nsi1", default)]
	pub server_created_date_time: Option<String>, 
	#[yaserde(rename = "DeepLink", prefix = "nsi1", default)]
	pub deep_link: Option<String>, 
	#[yaserde(rename = "Application", prefix = "nsi1", default)]
	pub application: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "LikesPreviewType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct LikesPreviewType {
	#[yaserde(rename = "LikeCount", prefix = "nsi1", default)]
	pub like_count: i32, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MentionsPreviewType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MentionsPreviewType {
	#[yaserde(rename = "IsMentioned", prefix = "nsi1", default)]
	pub is_mentioned: bool, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "NonEmptyArrayOfLikeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct NonEmptyArrayOfLikeType {
	#[yaserde(rename = "Like", prefix = "nsi1", default)]
	pub like: Vec<LikeType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AttendeeAvailability",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AttendeeAvailability {
	#[yaserde(rename = "EmailAddress", prefix = "nsi1", default)]
	pub email_address: String, 
	#[yaserde(rename = "Availability", prefix = "nsi1", default)]
	pub availability: AvailabilityStatusType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfMeetingLocation",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfMeetingLocation {
	#[yaserde(rename = "MeetingLocation", prefix = "nsi1", default)]
	pub meeting_location: Vec<MeetingLocation>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfAttendeeAvailability",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfAttendeeAvailability {
	#[yaserde(rename = "AttendeeAvailability", prefix = "nsi1", default)]
	pub attendee_availability: Vec<AttendeeAvailability>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MeetingTimeCandidate",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MeetingTimeCandidate {
	#[yaserde(rename = "MeetingTimeslot", prefix = "nsi1", default)]
	pub meeting_timeslot: TimeSlot, 
	#[yaserde(rename = "Confidence", prefix = "nsi1", default)]
	pub confidence: f64, 
	#[yaserde(rename = "Score", prefix = "nsi1", default)]
	pub score: i32, 
	#[yaserde(rename = "OrganizerAvailability", prefix = "nsi1", default)]
	pub organizer_availability: AvailabilityStatusType, 
	#[yaserde(rename = "AttendeeAvailabilities", prefix = "nsi1", default)]
	pub attendee_availabilities: ArrayOfAttendeeAvailability, 
	#[yaserde(rename = "Locations", prefix = "nsi1", default)]
	pub locations: ArrayOfMeetingLocation, 
	#[yaserde(rename = "SuggestionHint", prefix = "nsi1", default)]
	pub suggestion_hint: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfMeetingTimeCandidate",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfMeetingTimeCandidate {
	#[yaserde(rename = "MeetingTimeCandidate", prefix = "nsi1", default)]
	pub meeting_time_candidate: Vec<MeetingTimeCandidate>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MeetingTimeCandidatesConstraintItem",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct MeetingTimeCandidatesConstraintItem {
	#[yaserde(rename = "Email", prefix = "nsi1", default)]
	pub email: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AttendeeConstraintItem",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AttendeeConstraintItem {
	#[yaserde(flatten, default)]
	pub meeting_time_candidates_constraint_item: MeetingTimeCandidatesConstraintItem, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "IsRequired", prefix = "nsi1", default)]
	pub is_required: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "LocationConstraintItem",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct LocationConstraintItem {
	#[yaserde(flatten, default)]
	pub meeting_time_candidates_constraint_item: MeetingTimeCandidatesConstraintItem, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Name", prefix = "nsi1", default)]
	pub name: Option<String>, 
	#[yaserde(rename = "ResolveAvailability", prefix = "nsi1", default)]
	pub resolve_availability: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfLocationConstraintItems",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfLocationConstraintItems {
	#[yaserde(rename = "LocationItem", prefix = "nsi1", default)]
	pub location_item: Vec<LocationConstraintItem>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfAttendeeConstraintItems",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfAttendeeConstraintItems {
	#[yaserde(rename = "AttendeeItem", prefix = "nsi1", default)]
	pub attendee_item: Vec<AttendeeConstraintItem>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfTimeSlot",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfTimeSlot {
	#[yaserde(rename = "TimeSlot", prefix = "nsi1", default)]
	pub time_slot: Vec<TimeSlot>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EmptySuggestionReason",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct EmptySuggestionReason {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindMeetingTimesAttendeeConstraints",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FindMeetingTimesAttendeeConstraints {
	#[yaserde(rename = "AttendeeEntries", prefix = "nsi1", default)]
	pub attendee_entries: Option<ArrayOfAttendeeConstraintItems>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindMeetingTimesLocationConstraints",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FindMeetingTimesLocationConstraints {
	#[yaserde(rename = "LocationEntries", prefix = "nsi1", default)]
	pub location_entries: Option<ArrayOfLocationConstraintItems>, 
	#[yaserde(rename = "IsRequired", prefix = "nsi1", default)]
	pub is_required: Option<bool>, 
	#[yaserde(rename = "SuggestLocation", prefix = "nsi1", default)]
	pub suggest_location: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindMeetingTimesSearchConstraints",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FindMeetingTimesSearchConstraints {
	#[yaserde(rename = "SearchWindows", prefix = "nsi1", default)]
	pub search_windows: Option<ArrayOfTimeSlot>, 
	#[yaserde(rename = "MeetingDurationInMinutes", prefix = "nsi1", default)]
	pub meeting_duration_in_minutes: Option<i32>, 
	#[yaserde(rename = "ActivityDomain", prefix = "nsi1", default)]
	pub activity_domain: Option<ActivityDomainType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindMeetingTimesConstraints",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct FindMeetingTimesConstraints {
	#[yaserde(rename = "MaxCandidates", prefix = "nsi1", default)]
	pub max_candidates: Option<i32>, 
	#[yaserde(rename = "IsOrganizerOptional", prefix = "nsi1", default)]
	pub is_organizer_optional: Option<bool>, 
	#[yaserde(rename = "ReturnSuggestionHints", prefix = "nsi1", default)]
	pub return_suggestion_hints: Option<bool>, 
	#[yaserde(rename = "AppName", prefix = "nsi1", default)]
	pub app_name: Option<String>, 
	#[yaserde(rename = "AppScenario", prefix = "nsi1", default)]
	pub app_scenario: Option<String>, 
	#[yaserde(rename = "MinimumAttendeePercentage", prefix = "nsi1", default)]
	pub minimum_attendee_percentage: Option<f64>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AddInStateType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AddInStateType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RequestedAttendanceModeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct RequestedAttendanceModeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AADOfficeExtensionStatusType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AadofficeExtensionStatusType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "OfficeClientType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct OfficeClientType {
#[yaserde(rename="Code", attribute)]
pub code: OfficeClientCodeType,
#[yaserde(rename="Version", attribute)]
pub version: VersionType,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "OfficeClientCodeType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct OfficeClientCodeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "VersionType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct VersionType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfAppsType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct ArrayOfAppsType {
	#[yaserde(rename = "App", prefix = "nsi1", default)]
	pub app: Vec<AppType>, 
	#[yaserde(rename = "Metadata", prefix = "nsi1", default)]
	pub metadata: Option<Metadata>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AppType",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AppType {
	#[yaserde(rename = "Metadata", prefix = "nsi1", default)]
	pub metadata: Option<AppMetadata>, 
	#[yaserde(rename = "Manifest", prefix = "nsi1", default)]
	pub manifest: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "Metadata",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct Metadata {
	#[yaserde(rename = "CustomApps", prefix = "nsi1", default)]
	pub custom_apps: String, 
	#[yaserde(rename = "GenericInfo", prefix = "nsi1", default)]
	pub generic_info: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AppMetadata",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct AppMetadata {
	#[yaserde(rename = "EndNodeUrl", prefix = "nsi1", default)]
	pub end_node_url: Option<String>, 
	#[yaserde(rename = "AppStatus", prefix = "nsi1", default)]
	pub app_status: Option<String>, 
	#[yaserde(rename = "ActionUrl", prefix = "nsi1", default)]
	pub action_url: Option<String>, 
	#[yaserde(rename = "ProductId", prefix = "nsi1", default)]
	pub product_id: Option<String>, 
	#[yaserde(rename = "EnabledStatus", prefix = "nsi1", default)]
	pub enabled_status: Option<bool>, 
	#[yaserde(rename = "ConsentState", prefix = "nsi1", default)]
	pub consent_state: Option<String>, 
	#[yaserde(rename = "ExtensionType", prefix = "nsi1", default)]
	pub extension_type: Option<String>, 
	#[yaserde(rename = "MarketplaceAssetId", prefix = "nsi1", default)]
	pub marketplace_asset_id: Option<String>, 
	#[yaserde(rename = "LicenseStatus", prefix = "nsi1", default)]
	pub license_status: Option<String>, 
	#[yaserde(rename = "TrialExpirationDate", prefix = "nsi1", default)]
	pub trial_expiration_date: Option<String>, 
	#[yaserde(rename = "InstalledBy", prefix = "nsi1", default)]
	pub installed_by: Option<String>, 
	#[yaserde(rename = "IsMandatory", prefix = "nsi1", default)]
	pub is_mandatory: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "OMEMessageRevocationStatus",
	namespace = "nsi1: http://schemas.microsoft.com/exchange/services/2006/types",
	prefix = "nsi1",
)]
pub struct OmemessageRevocationStatus {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ResponseCodeType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ResponseCodeType {
	#[yaserde(text, default)]
	pub body: String, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MessageXml",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct MessageXml {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ResponseMessageType {
#[yaserde(rename="ResponseClass", attribute)]
pub response_class: ResponseClassType,
	#[yaserde(rename = "MessageText", prefix = "tns", default)]
	pub message_text: Option<String>, 
	#[yaserde(rename = "ResponseCode", prefix = "tns", default)]
	pub response_code: Option<ResponseCodeType>, 
	#[yaserde(rename = "DescriptiveLinkKey", prefix = "tns", default)]
	pub descriptive_link_key: Option<i32>, 
	#[yaserde(rename = "MessageXml", prefix = "tns", default)]
	pub message_xml: Option<MessageXml>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfResponseMessagesType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ArrayOfResponseMessagesType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BaseResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct BaseResponseMessageType {
	#[yaserde(rename = "ResponseMessages", prefix = "tns", default)]
	pub response_messages: ArrayOfResponseMessagesType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BaseRequestType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct BaseRequestType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetFolderType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetFolderType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "FolderShape", prefix = "tns", default)]
	pub folder_shape: FolderResponseShapeType, 
	#[yaserde(rename = "FolderIds", prefix = "tns", default)]
	pub folder_ids: NonEmptyArrayOfBaseFolderIdsType, 
}
pub type GetFolder = GetFolderType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UploadItemsType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UploadItemsType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Items", prefix = "tns", default)]
	pub items: NonEmptyArrayOfUploadItemsType, 
}
pub type UploadItems = UploadItemsType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UploadItemsResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UploadItemsResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ItemId", prefix = "tns", default)]
	pub item_id: Option<ItemIdType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UploadItemsResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UploadItemsResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type UploadItemsResponse = UploadItemsResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ExportItemsType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ExportItemsType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ItemIds", prefix = "tns", default)]
	pub item_ids: NonEmptyArrayOfItemIdsType, 
}
pub type ExportItems = ExportItemsType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ExportItemsResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ExportItemsResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ItemId", prefix = "tns", default)]
	pub item_id: Option<ItemIdType>, 
	#[yaserde(rename = "Data", prefix = "tns", default)]
	pub data: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ExportItemsResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ExportItemsResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type ExportItemsResponse = ExportItemsResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CreateFolderType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct CreateFolderType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ParentFolderId", prefix = "tns", default)]
	pub parent_folder_id: TargetFolderIdType, 
	#[yaserde(rename = "Folders", prefix = "tns", default)]
	pub folders: NonEmptyArrayOfFoldersType, 
}
pub type CreateFolder = CreateFolderType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindFolderType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct FindFolderType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "FolderShape", prefix = "tns", default)]
	pub folder_shape: FolderResponseShapeType, 
	#[yaserde(rename = "IndexedPageFolderView", prefix = "tns", default)]
	pub indexed_page_folder_view: IndexedPageViewType, 
	#[yaserde(rename = "FractionalPageFolderView", prefix = "tns", default)]
	pub fractional_page_folder_view: FractionalPageViewType, 
	#[yaserde(rename = "Restriction", prefix = "tns", default)]
	pub restriction: Option<RestrictionType>, 
	#[yaserde(rename = "ParentFolderIds", prefix = "tns", default)]
	pub parent_folder_ids: NonEmptyArrayOfBaseFolderIdsType, 
}
pub type FindFolder = FindFolderType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FolderInfoResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct FolderInfoResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Folders", prefix = "tns", default)]
	pub folders: Option<ArrayOfFoldersType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindFolderResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct FindFolderResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "RootFolder", prefix = "tns", default)]
	pub root_folder: Option<FindFolderParentType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindFolderResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct FindFolderResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type FindFolderResponse = FindFolderResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeleteFolderType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct DeleteFolderType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "FolderIds", prefix = "tns", default)]
	pub folder_ids: NonEmptyArrayOfBaseFolderIdsType, 
}
pub type DeleteFolder = DeleteFolderType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeleteFolderResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct DeleteFolderResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type DeleteFolderResponse = DeleteFolderResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EmptyFolderType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct EmptyFolderType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "FolderIds", prefix = "tns", default)]
	pub folder_ids: NonEmptyArrayOfBaseFolderIdsType, 
}
pub type EmptyFolder = EmptyFolderType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EmptyFolderResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct EmptyFolderResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type EmptyFolderResponse = EmptyFolderResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BaseMoveCopyFolderType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct BaseMoveCopyFolderType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ToFolderId", prefix = "tns", default)]
	pub to_folder_id: TargetFolderIdType, 
	#[yaserde(rename = "FolderIds", prefix = "tns", default)]
	pub folder_ids: NonEmptyArrayOfBaseFolderIdsType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MoveFolderType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct MoveFolderType {
	#[yaserde(flatten, default)]
	pub base_move_copy_folder_type: BaseMoveCopyFolderType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CopyFolderType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct CopyFolderType {
	#[yaserde(flatten, default)]
	pub base_move_copy_folder_type: BaseMoveCopyFolderType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type MoveFolder = MoveFolderType;

pub type CopyFolder = CopyFolderType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UpdateFolderType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UpdateFolderType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "FolderChanges", prefix = "tns", default)]
	pub folder_changes: NonEmptyArrayOfFolderChangesType, 
}
pub type UpdateFolder = UpdateFolderType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CreateFolderResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct CreateFolderResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type CreateFolderResponse = CreateFolderResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetFolderResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetFolderResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type GetFolderResponse = GetFolderResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UpdateFolderResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UpdateFolderResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type UpdateFolderResponse = UpdateFolderResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MoveFolderResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct MoveFolderResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CopyFolderResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct CopyFolderResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type MoveFolderResponse = MoveFolderResponseType;

pub type CopyFolderResponse = CopyFolderResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CreateFolderPathType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct CreateFolderPathType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ParentFolderId", prefix = "tns", default)]
	pub parent_folder_id: TargetFolderIdType, 
	#[yaserde(rename = "RelativeFolderPath", prefix = "tns", default)]
	pub relative_folder_path: NonEmptyArrayOfFoldersType, 
}
pub type CreateFolderPath = CreateFolderPathType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CreateFolderPathResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct CreateFolderPathResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type CreateFolderPathResponse = CreateFolderPathResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetItemType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetItemType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ItemShape", prefix = "tns", default)]
	pub item_shape: ItemResponseShapeType, 
	#[yaserde(rename = "ItemIds", prefix = "tns", default)]
	pub item_ids: NonEmptyArrayOfBaseItemIdsType, 
}
pub type GetItem = GetItemType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CreateItemType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct CreateItemType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SavedItemFolderId", prefix = "tns", default)]
	pub saved_item_folder_id: Option<TargetFolderIdType>, 
	#[yaserde(rename = "Items", prefix = "tns", default)]
	pub items: NonEmptyArrayOfAllItemsType, 
}
pub type CreateItem = CreateItemType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UpdateItemType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UpdateItemType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SavedItemFolderId", prefix = "tns", default)]
	pub saved_item_folder_id: Option<TargetFolderIdType>, 
	#[yaserde(rename = "ItemChanges", prefix = "tns", default)]
	pub item_changes: NonEmptyArrayOfItemChangesType, 
}
pub type UpdateItem = UpdateItemType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ItemInfoResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ItemInfoResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Items", prefix = "tns", default)]
	pub items: ArrayOfRealItemsType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UpdateItemResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UpdateItemResponseMessageType {
	#[yaserde(flatten, default)]
	pub item_info_response_message_type: ItemInfoResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ConflictResults", prefix = "tns", default)]
	pub conflict_results: Option<ConflictResultsType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UpdateItemInRecoverableItemsType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UpdateItemInRecoverableItemsType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ItemId", prefix = "tns", default)]
	pub item_id: ItemIdType, 
	#[yaserde(rename = "Updates", prefix = "tns", default)]
	pub updates: Option<NonEmptyArrayOfItemChangeDescriptionsType>, 
	#[yaserde(rename = "Attachments", prefix = "tns", default)]
	pub attachments: Option<NonEmptyArrayOfAttachmentsType>, 
	#[yaserde(rename = "MakeItemImmutable", prefix = "tns", default)]
	pub make_item_immutable: Option<bool>, 
}
pub type UpdateItemInRecoverableItems = UpdateItemInRecoverableItemsType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UpdateItemInRecoverableItemsResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UpdateItemInRecoverableItemsResponseMessageType {
	#[yaserde(flatten, default)]
	pub item_info_response_message_type: ItemInfoResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Attachments", prefix = "tns", default)]
	pub attachments: Option<ArrayOfAttachmentsType>, 
	#[yaserde(rename = "ConflictResults", prefix = "tns", default)]
	pub conflict_results: Option<ConflictResultsType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeleteItemType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct DeleteItemType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ItemIds", prefix = "tns", default)]
	pub item_ids: NonEmptyArrayOfBaseItemIdsType, 
}
pub type DeleteItem = DeleteItemType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AttachmentInfoResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct AttachmentInfoResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Attachments", prefix = "tns", default)]
	pub attachments: ArrayOfAttachmentsType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeleteAttachmentResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct DeleteAttachmentResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "RootItemId", prefix = "tns", default)]
	pub root_item_id: Option<RootItemIdType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BaseMoveCopyItemType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct BaseMoveCopyItemType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ToFolderId", prefix = "tns", default)]
	pub to_folder_id: TargetFolderIdType, 
	#[yaserde(rename = "ItemIds", prefix = "tns", default)]
	pub item_ids: NonEmptyArrayOfBaseItemIdsType, 
	#[yaserde(rename = "ReturnNewItemIds", prefix = "tns", default)]
	pub return_new_item_ids: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MoveItemType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct MoveItemType {
	#[yaserde(flatten, default)]
	pub base_move_copy_item_type: BaseMoveCopyItemType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CopyItemType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct CopyItemType {
	#[yaserde(flatten, default)]
	pub base_move_copy_item_type: BaseMoveCopyItemType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type MoveItem = MoveItemType;

pub type CopyItem = CopyItemType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArchiveItemType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ArchiveItemType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ArchiveSourceFolderId", prefix = "tns", default)]
	pub archive_source_folder_id: TargetFolderIdType, 
	#[yaserde(rename = "ItemIds", prefix = "tns", default)]
	pub item_ids: NonEmptyArrayOfBaseItemIdsType, 
}
pub type ArchiveItem = ArchiveItemType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SendItemType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SendItemType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ItemIds", prefix = "tns", default)]
	pub item_ids: NonEmptyArrayOfBaseItemIdsType, 
	#[yaserde(rename = "SavedItemFolderId", prefix = "tns", default)]
	pub saved_item_folder_id: Option<TargetFolderIdType>, 
}
pub type SendItem = SendItemType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SendItemResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SendItemResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type SendItemResponse = SendItemResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "QueryStringType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct QueryStringType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindItemType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct FindItemType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ItemShape", prefix = "tns", default)]
	pub item_shape: ItemResponseShapeType, 
	#[yaserde(rename = "IndexedPageItemView", prefix = "tns", default)]
	pub indexed_page_item_view: IndexedPageViewType, 
	#[yaserde(rename = "FractionalPageItemView", prefix = "tns", default)]
	pub fractional_page_item_view: FractionalPageViewType, 
	#[yaserde(rename = "SeekToConditionPageItemView", prefix = "tns", default)]
	pub seek_to_condition_page_item_view: SeekToConditionPageViewType, 
	#[yaserde(rename = "CalendarView", prefix = "tns", default)]
	pub calendar_view: CalendarViewType, 
	#[yaserde(rename = "ContactsView", prefix = "tns", default)]
	pub contacts_view: ContactsViewType, 
	#[yaserde(rename = "GroupBy", prefix = "tns", default)]
	pub group_by: GroupByType, 
	#[yaserde(rename = "DistinguishedGroupBy", prefix = "tns", default)]
	pub distinguished_group_by: DistinguishedGroupByType, 
	#[yaserde(rename = "Restriction", prefix = "tns", default)]
	pub restriction: Option<RestrictionType>, 
	#[yaserde(rename = "SortOrder", prefix = "tns", default)]
	pub sort_order: Option<NonEmptyArrayOfFieldOrdersType>, 
	#[yaserde(rename = "ParentFolderIds", prefix = "tns", default)]
	pub parent_folder_ids: NonEmptyArrayOfBaseFolderIdsType, 
	#[yaserde(rename = "QueryString", prefix = "tns", default)]
	pub query_string: Option<QueryStringType>, 
}
pub type FindItem = FindItemType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindConversationType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct FindConversationType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "IndexedPageItemView", prefix = "tns", default)]
	pub indexed_page_item_view: IndexedPageViewType, 
	#[yaserde(rename = "SeekToConditionPageItemView", prefix = "tns", default)]
	pub seek_to_condition_page_item_view: SeekToConditionPageViewType, 
	#[yaserde(rename = "SortOrder", prefix = "tns", default)]
	pub sort_order: Option<NonEmptyArrayOfFieldOrdersType>, 
	#[yaserde(rename = "ParentFolderId", prefix = "tns", default)]
	pub parent_folder_id: TargetFolderIdType, 
	#[yaserde(rename = "MailboxScope", prefix = "tns", default)]
	pub mailbox_scope: Option<MailboxSearchLocationType>, 
	#[yaserde(rename = "QueryString", prefix = "tns", default)]
	pub query_string: Option<QueryStringType>, 
	#[yaserde(rename = "ConversationShape", prefix = "tns", default)]
	pub conversation_shape: Option<ConversationResponseShapeType>, 
}
pub type FindConversation = FindConversationType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindConversationResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct FindConversationResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Conversations", prefix = "tns", default)]
	pub conversations: Option<ArrayOfConversationsType>, 
	#[yaserde(rename = "HighlightTerms", prefix = "tns", default)]
	pub highlight_terms: Option<ArrayOfHighlightTermsType>, 
	#[yaserde(rename = "TotalConversationsInView", prefix = "tns", default)]
	pub total_conversations_in_view: Option<i32>, 
	#[yaserde(rename = "IndexedOffset", prefix = "tns", default)]
	pub indexed_offset: Option<i32>, 
}
pub type FindConversationResponse = FindConversationResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PerformInstantSearchRequest",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct PerformInstantSearchRequest {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SearchSessionId", prefix = "tns", default)]
	pub search_session_id: String, 
	#[yaserde(rename = "ItemType", prefix = "tns", default)]
	pub item_type: InstantSearchItemType, 
	#[yaserde(rename = "QueryOptions", prefix = "tns", default)]
	pub query_options: QueryOptionsType, 
	#[yaserde(rename = "SearchRequestId", prefix = "tns", default)]
	pub search_request_id: i64, 
	#[yaserde(rename = "KqlQuery", prefix = "tns", default)]
	pub kql_query: String, 
	#[yaserde(rename = "FolderScope", prefix = "tns", default)]
	pub folder_scope: ArrayOfFolderIdType, 
	#[yaserde(rename = "DistinguishedFolderScope", prefix = "tns", default)]
	pub distinguished_folder_scope: Option<ArrayOfDistinguishedFolderIdType>, 
	#[yaserde(rename = "IsDeepTraversal", prefix = "tns", default)]
	pub is_deep_traversal: Option<bool>, 
	#[yaserde(rename = "WaitOnSearchResults", prefix = "tns", default)]
	pub wait_on_search_results: Option<bool>, 
}
pub type PerformInstantSearch = PerformInstantSearchRequest;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PerformInstantSearchResponse",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct PerformInstantSearchResponse {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Payload", prefix = "tns", default)]
	pub payload: Option<InstantSearchPayloadType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EndInstantSearchSessionRequest",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct EndInstantSearchSessionRequest {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SessionId", prefix = "tns", default)]
	pub session_id: String, 
}
pub type EndInstantSearchSession = EndInstantSearchSessionRequest;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ApplyConversationActionType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ApplyConversationActionType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ConversationActions", prefix = "tns", default)]
	pub conversation_actions: NonEmptyArrayOfApplyConversationActionType, 
}
pub type ApplyConversationAction = ApplyConversationActionType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ApplyConversationActionResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ApplyConversationActionResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type ApplyConversationActionResponse = ApplyConversationActionResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ApplyConversationActionResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ApplyConversationActionResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindPeopleType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct FindPeopleType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "PersonaShape", prefix = "tns", default)]
	pub persona_shape: Option<PersonaResponseShapeType>, 
	#[yaserde(rename = "IndexedPageItemView", prefix = "tns", default)]
	pub indexed_page_item_view: IndexedPageViewType, 
	#[yaserde(rename = "Restriction", prefix = "tns", default)]
	pub restriction: Option<RestrictionType>, 
	#[yaserde(rename = "AggregationRestriction", prefix = "tns", default)]
	pub aggregation_restriction: Option<RestrictionType>, 
	#[yaserde(rename = "SortOrder", prefix = "tns", default)]
	pub sort_order: Option<NonEmptyArrayOfFieldOrdersType>, 
	#[yaserde(rename = "ParentFolderId", prefix = "tns", default)]
	pub parent_folder_id: Option<TargetFolderIdType>, 
	#[yaserde(rename = "QueryString", prefix = "tns", default)]
	pub query_string: Option<String>, 
	#[yaserde(rename = "SearchPeopleSuggestionIndex", prefix = "tns", default)]
	pub search_people_suggestion_index: Option<bool>, 
	#[yaserde(rename = "TopicQueryString", prefix = "tns", default)]
	pub topic_query_string: Option<String>, 
	#[yaserde(rename = "Context", prefix = "tns", default)]
	pub context: Option<ArrayOfContextProperty>, 
	#[yaserde(rename = "QuerySources", prefix = "tns", default)]
	pub query_sources: Option<ArrayOfPeopleQuerySource>, 
	#[yaserde(rename = "ReturnFlattenedResults", prefix = "tns", default)]
	pub return_flattened_results: Option<bool>, 
}
pub type FindPeople = FindPeopleType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindPeopleResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct FindPeopleResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "People", prefix = "tns", default)]
	pub people: Option<ArrayOfPeopleType>, 
	#[yaserde(rename = "TotalNumberOfPeopleInView", prefix = "tns", default)]
	pub total_number_of_people_in_view: Option<i32>, 
	#[yaserde(rename = "FirstMatchingRowIndex", prefix = "tns", default)]
	pub first_matching_row_index: Option<i32>, 
	#[yaserde(rename = "FirstLoadedRowIndex", prefix = "tns", default)]
	pub first_loaded_row_index: Option<i32>, 
	#[yaserde(rename = "TransactionId", prefix = "tns", default)]
	pub transaction_id: Option<GuidType>, 
}
pub type FindPeopleResponse = FindPeopleResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindTagsType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct FindTagsType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "IndexedPageItemView", prefix = "tns", default)]
	pub indexed_page_item_view: IndexedPageViewType, 
	#[yaserde(rename = "SortOrder", prefix = "tns", default)]
	pub sort_order: Option<NonEmptyArrayOfFieldOrdersType>, 
	#[yaserde(rename = "QueryString", prefix = "tns", default)]
	pub query_string: Option<String>, 
	#[yaserde(rename = "Context", prefix = "tns", default)]
	pub context: Option<ArrayOfContextProperty>, 
}
pub type FindTags = FindTagsType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindTagsResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct FindTagsResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Tags", prefix = "tns", default)]
	pub tags: Option<ArrayOfStringsType>, 
}
pub type FindTagsResponse = FindTagsResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AddTagType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct AddTagType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Tag", prefix = "tns", default)]
	pub tag: String, 
	#[yaserde(rename = "AppName", prefix = "tns", default)]
	pub app_name: String, 
}
pub type AddTag = AddTagType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AddTagResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct AddTagResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "WasSuccessful", prefix = "tns", default)]
	pub was_successful: Option<bool>, 
}
pub type AddTagResponse = AddTagResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "HideTagType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct HideTagType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Tag", prefix = "tns", default)]
	pub tag: String, 
}
pub type HideTag = HideTagType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "HideTagResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct HideTagResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "WasSuccessful", prefix = "tns", default)]
	pub was_successful: Option<bool>, 
}
pub type HideTagResponse = HideTagResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetPersonaType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetPersonaType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "PersonaId", prefix = "tns", default)]
	pub persona_id: Option<ItemIdType>, 
	#[yaserde(rename = "EmailAddress", prefix = "tns", default)]
	pub email_address: Option<EmailAddressType>, 
	#[yaserde(rename = "ParentFolderId", prefix = "tns", default)]
	pub parent_folder_id: Option<TargetFolderIdType>, 
	#[yaserde(rename = "ItemLinkId", prefix = "tns", default)]
	pub item_link_id: Option<String>, 
	#[yaserde(rename = "AdditionalProperties", prefix = "tns", default)]
	pub additional_properties: Option<NonEmptyArrayOfPathsToElementType>, 
}
pub type GetPersona = GetPersonaType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetPersonaResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetPersonaResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Persona", prefix = "tns", default)]
	pub persona: PersonaType, 
}
pub type GetPersonaResponseMessage = GetPersonaResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CreateAttachmentType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct CreateAttachmentType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ParentItemId", prefix = "tns", default)]
	pub parent_item_id: ItemIdType, 
	#[yaserde(rename = "Attachments", prefix = "tns", default)]
	pub attachments: NonEmptyArrayOfAttachmentsType, 
}
pub type CreateAttachment = CreateAttachmentType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CreateAttachmentResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct CreateAttachmentResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type CreateAttachmentResponse = CreateAttachmentResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeleteAttachmentType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct DeleteAttachmentType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "AttachmentIds", prefix = "tns", default)]
	pub attachment_ids: NonEmptyArrayOfRequestAttachmentIdsType, 
}
pub type DeleteAttachment = DeleteAttachmentType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeleteAttachmentResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct DeleteAttachmentResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type DeleteAttachmentResponse = DeleteAttachmentResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetAttachmentType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetAttachmentType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "AttachmentShape", prefix = "tns", default)]
	pub attachment_shape: Option<AttachmentResponseShapeType>, 
	#[yaserde(rename = "AttachmentIds", prefix = "tns", default)]
	pub attachment_ids: NonEmptyArrayOfRequestAttachmentIdsType, 
}
pub type GetAttachment = GetAttachmentType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetAttachmentResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetAttachmentResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type GetAttachmentResponse = GetAttachmentResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CreateItemResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct CreateItemResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type CreateItemResponse = CreateItemResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UpdateItemResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UpdateItemResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type UpdateItemResponse = UpdateItemResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UpdateItemInRecoverableItemsResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UpdateItemInRecoverableItemsResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type UpdateItemInRecoverableItemsResponse = UpdateItemInRecoverableItemsResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetItemResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetItemResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type GetItemResponse = GetItemResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MoveItemResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct MoveItemResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CopyItemResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct CopyItemResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type MoveItemResponse = MoveItemResponseType;

pub type CopyItemResponse = CopyItemResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeleteItemResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct DeleteItemResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type DeleteItemResponse = DeleteItemResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeleteItemResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct DeleteItemResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindItemResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct FindItemResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "RootFolder", prefix = "tns", default)]
	pub root_folder: Option<FindItemParentType>, 
	#[yaserde(rename = "HighlightTerms", prefix = "tns", default)]
	pub highlight_terms: Option<ArrayOfHighlightTermsType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindItemResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct FindItemResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type FindItemResponse = FindItemResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArchiveItemResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ArchiveItemResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type ArchiveItemResponse = ArchiveItemResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetClientAccessTokenType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetClientAccessTokenType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "TokenRequests", prefix = "tns", default)]
	pub token_requests: NonEmptyArrayOfClientAccessTokenRequestsType, 
}
pub type GetClientAccessToken = GetClientAccessTokenType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetClientAccessTokenResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetClientAccessTokenResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Token", prefix = "tns", default)]
	pub token: Option<ClientAccessTokenType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetClientAccessTokenResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetClientAccessTokenResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type GetClientAccessTokenResponse = GetClientAccessTokenResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetFocusedOtherOverridesRequestType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetFocusedOtherOverridesRequestType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type GetFocusedOtherOverrides = GetFocusedOtherOverridesRequestType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetFocusedOtherOverridesResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetFocusedOtherOverridesResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Overrides", prefix = "tns", default)]
	pub overrides: ArrayOfInferenceClassificationOverridesType, 
}
pub type GetFocusedOtherOverridesResponse = GetFocusedOtherOverridesResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CreateOrUpdateFocusedOtherOverrideRequestType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct CreateOrUpdateFocusedOtherOverrideRequestType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SenderSmtpAddress", prefix = "tns", default)]
	pub sender_smtp_address: String, 
	#[yaserde(rename = "SenderDisplayName", prefix = "tns", default)]
	pub sender_display_name: Option<String>, 
	#[yaserde(rename = "AlwaysClassifyAs", prefix = "tns", default)]
	pub always_classify_as: InferenceClassificationType, 
}
pub type CreateOrUpdateFocusedOtherOverride = CreateOrUpdateFocusedOtherOverrideRequestType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CreateOrUpdateFocusedOtherOverrideResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct CreateOrUpdateFocusedOtherOverrideResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Override", prefix = "tns", default)]
	pub r#override: InferenceClassificationOverrideType, 
}
pub type CreateOrUpdateFocusedOtherOverrideResponse = CreateOrUpdateFocusedOtherOverrideResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeleteFocusedOtherOverrideRequestType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct DeleteFocusedOtherOverrideRequestType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Id", prefix = "tns", default)]
	pub id: GuidType, 
}
pub type DeleteFocusedOtherOverride = DeleteFocusedOtherOverrideRequestType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeleteFocusedOtherOverrideResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct DeleteFocusedOtherOverrideResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Override", prefix = "tns", default)]
	pub r#override: InferenceClassificationOverrideType, 
}
pub type DeleteFocusedOtherOverrideResponse = DeleteFocusedOtherOverrideResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ResolveNamesType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ResolveNamesType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ParentFolderIds", prefix = "tns", default)]
	pub parent_folder_ids: Option<NonEmptyArrayOfBaseFolderIdsType>, 
	#[yaserde(rename = "UnresolvedEntry", prefix = "tns", default)]
	pub unresolved_entry: NonEmptyStringType, 
}
pub type ResolveNames = ResolveNamesType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ResolveNamesResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ResolveNamesResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ResolutionSet", prefix = "tns", default)]
	pub resolution_set: Option<ArrayOfResolutionType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ResolveNamesResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ResolveNamesResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type ResolveNamesResponse = ResolveNamesResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetPasswordExpirationDateType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetPasswordExpirationDateType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MailboxSmtpAddress", prefix = "tns", default)]
	pub mailbox_smtp_address: Option<String>, 
}
pub type GetPasswordExpirationDate = GetPasswordExpirationDateType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetPasswordExpirationDateResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetPasswordExpirationDateResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "PasswordExpirationDate", prefix = "tns", default)]
	pub password_expiration_date: String, 
}
pub type GetPasswordExpirationDateResponse = GetPasswordExpirationDateResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetMailTipsType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetMailTipsType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SendingAs", prefix = "tns", default)]
	pub sending_as: EmailAddressType, 
	#[yaserde(rename = "Recipients", prefix = "tns", default)]
	pub recipients: ArrayOfRecipientsType, 
	#[yaserde(rename = "MailTipsRequested", prefix = "tns", default)]
	pub mail_tips_requested: MailTipTypes, 
}
pub type GetMailTips = GetMailTipsType;

pub type GetMailTipsResponse = GetMailTipsResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetMailTipsResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetMailTipsResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ResponseMessages", prefix = "tns", default)]
	pub response_messages: Option<ArrayOfMailTipsResponseMessageType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfMailTipsResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ArrayOfMailTipsResponseMessageType {
	#[yaserde(rename = "MailTipsResponseMessageType", prefix = "tns", default)]
	pub mail_tips_response_message_type: Vec<MailTipsResponseMessageType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MailTipsResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct MailTipsResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MailTips", prefix = "tns", default)]
	pub mail_tips: Option<MailTips>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PlayOnPhoneType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct PlayOnPhoneType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ItemId", prefix = "tns", default)]
	pub item_id: ItemIdType, 
	#[yaserde(rename = "DialString", prefix = "tns", default)]
	pub dial_string: String, 
}
pub type PlayOnPhone = PlayOnPhoneType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PlayOnPhoneResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct PlayOnPhoneResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "PhoneCallId", prefix = "tns", default)]
	pub phone_call_id: Option<PhoneCallIdType>, 
}
pub type PlayOnPhoneResponse = PlayOnPhoneResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetPhoneCallInformationType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetPhoneCallInformationType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "PhoneCallId", prefix = "tns", default)]
	pub phone_call_id: PhoneCallIdType, 
}
pub type GetPhoneCallInformation = GetPhoneCallInformationType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetPhoneCallInformationResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetPhoneCallInformationResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "PhoneCallInformation", prefix = "tns", default)]
	pub phone_call_information: Option<PhoneCallInformationType>, 
}
pub type GetPhoneCallInformationResponse = GetPhoneCallInformationResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DisconnectPhoneCallType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct DisconnectPhoneCallType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "PhoneCallId", prefix = "tns", default)]
	pub phone_call_id: PhoneCallIdType, 
}
pub type DisconnectPhoneCall = DisconnectPhoneCallType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DisconnectPhoneCallResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct DisconnectPhoneCallResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type DisconnectPhoneCallResponse = DisconnectPhoneCallResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ExpandDLType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ExpandDLType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Mailbox", prefix = "tns", default)]
	pub mailbox: EmailAddressType, 
}
pub type ExpandDL = ExpandDLType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ExpandDLResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ExpandDLResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "DLExpansion", prefix = "tns", default)]
	pub dl_expansion: Option<ArrayOfDLExpansionType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ExpandDLResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ExpandDLResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type ExpandDLResponse = ExpandDLResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetServerTimeZonesType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetServerTimeZonesType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Ids", prefix = "tns", default)]
	pub ids: Option<NonEmptyArrayOfTimeZoneIdType>, 
}
pub type GetServerTimeZones = GetServerTimeZonesType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetServerTimeZonesResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetServerTimeZonesResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "TimeZoneDefinitions", prefix = "tns", default)]
	pub time_zone_definitions: ArrayOfTimeZoneDefinitionType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetServerTimeZonesResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetServerTimeZonesResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type GetServerTimeZonesResponse = GetServerTimeZonesResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CreateManagedFolderRequestType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct CreateManagedFolderRequestType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "FolderNames", prefix = "tns", default)]
	pub folder_names: NonEmptyArrayOfFolderNamesType, 
	#[yaserde(rename = "Mailbox", prefix = "tns", default)]
	pub mailbox: Option<EmailAddressType>, 
}
pub type CreateManagedFolder = CreateManagedFolderRequestType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CreateManagedFolderResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct CreateManagedFolderResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type CreateManagedFolderResponse = CreateManagedFolderResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SubscribeType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SubscribeType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type Subscribe = SubscribeType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SubscribeResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SubscribeResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SubscriptionId", prefix = "tns", default)]
	pub subscription_id: Option<SubscriptionIdType>, 
	#[yaserde(rename = "Watermark", prefix = "tns", default)]
	pub watermark: Option<WatermarkType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SubscribeResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SubscribeResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type SubscribeResponse = SubscribeResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UnsubscribeType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UnsubscribeType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SubscriptionId", prefix = "tns", default)]
	pub subscription_id: SubscriptionIdType, 
}
pub type Unsubscribe = UnsubscribeType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UnsubscribeResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UnsubscribeResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type UnsubscribeResponse = UnsubscribeResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetEventsType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetEventsType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SubscriptionId", prefix = "tns", default)]
	pub subscription_id: SubscriptionIdType, 
	#[yaserde(rename = "Watermark", prefix = "tns", default)]
	pub watermark: WatermarkType, 
}
pub type GetEvents = GetEventsType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetEventsResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetEventsResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Notification", prefix = "tns", default)]
	pub notification: Option<NotificationType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetEventsResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetEventsResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type GetEventsResponse = GetEventsResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetStreamingEventsType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetStreamingEventsType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SubscriptionIds", prefix = "tns", default)]
	pub subscription_ids: NonEmptyArrayOfSubscriptionIdsType, 
	#[yaserde(rename = "ConnectionTimeout", prefix = "tns", default)]
	pub connection_timeout: StreamingSubscriptionConnectionTimeoutType, 
}
pub type GetStreamingEvents = GetStreamingEventsType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetStreamingEventsResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetStreamingEventsResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Notifications", prefix = "tns", default)]
	pub notifications: Option<NonEmptyArrayOfNotificationsType>, 
	#[yaserde(rename = "ErrorSubscriptionIds", prefix = "tns", default)]
	pub error_subscription_ids: Option<NonEmptyArrayOfSubscriptionIdsType>, 
	#[yaserde(rename = "ConnectionStatus", prefix = "tns", default)]
	pub connection_status: Option<ConnectionStatusType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetStreamingEventsResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetStreamingEventsResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type GetStreamingEventsResponse = GetStreamingEventsResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SendNotificationResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SendNotificationResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Notification", prefix = "tns", default)]
	pub notification: NotificationType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SendNotificationResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SendNotificationResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type SendNotification = SendNotificationResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SendNotificationResultType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SendNotificationResultType {
	#[yaserde(rename = "SubscriptionStatus", prefix = "tns", default)]
	pub subscription_status: SubscriptionStatusType, 
}
pub type SendNotificationResult = SendNotificationResultType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SyncFolderHierarchyType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SyncFolderHierarchyType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "FolderShape", prefix = "tns", default)]
	pub folder_shape: FolderResponseShapeType, 
	#[yaserde(rename = "SyncFolderId", prefix = "tns", default)]
	pub sync_folder_id: Option<TargetFolderIdType>, 
	#[yaserde(rename = "SyncState", prefix = "tns", default)]
	pub sync_state: Option<String>, 
}
pub type SyncFolderHierarchy = SyncFolderHierarchyType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SyncFolderHierarchyResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SyncFolderHierarchyResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SyncState", prefix = "tns", default)]
	pub sync_state: Option<String>, 
	#[yaserde(rename = "IncludesLastFolderInRange", prefix = "tns", default)]
	pub includes_last_folder_in_range: Option<bool>, 
	#[yaserde(rename = "Changes", prefix = "tns", default)]
	pub changes: Option<SyncFolderHierarchyChangesType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SyncFolderHierarchyResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SyncFolderHierarchyResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type SyncFolderHierarchyResponse = SyncFolderHierarchyResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SyncFolderItemsType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SyncFolderItemsType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ItemShape", prefix = "tns", default)]
	pub item_shape: ItemResponseShapeType, 
	#[yaserde(rename = "SyncFolderId", prefix = "tns", default)]
	pub sync_folder_id: TargetFolderIdType, 
	#[yaserde(rename = "SyncState", prefix = "tns", default)]
	pub sync_state: Option<String>, 
	#[yaserde(rename = "Ignore", prefix = "tns", default)]
	pub ignore: Option<ArrayOfBaseItemIdsType>, 
	#[yaserde(rename = "MaxChangesReturned", prefix = "tns", default)]
	pub max_changes_returned: MaxSyncChangesReturnedType, 
	#[yaserde(rename = "SyncScope", prefix = "tns", default)]
	pub sync_scope: Option<SyncFolderItemsScopeType>, 
}
pub type SyncFolderItems = SyncFolderItemsType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SyncFolderItemsResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SyncFolderItemsResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SyncState", prefix = "tns", default)]
	pub sync_state: Option<String>, 
	#[yaserde(rename = "IncludesLastItemInRange", prefix = "tns", default)]
	pub includes_last_item_in_range: Option<bool>, 
	#[yaserde(rename = "Changes", prefix = "tns", default)]
	pub changes: Option<SyncFolderItemsChangesType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SyncFolderItemsResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SyncFolderItemsResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type SyncFolderItemsResponse = SyncFolderItemsResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetUserAvailabilityRequestType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetUserAvailabilityRequestType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MailboxDataArray", prefix = "tns", default)]
	pub mailbox_data_array: ArrayOfMailboxData, 
}
pub type GetUserAvailabilityRequest = GetUserAvailabilityRequestType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FreeBusyResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct FreeBusyResponseType {
	#[yaserde(rename = "ResponseMessage", prefix = "tns", default)]
	pub response_message: Option<ResponseMessageType>, 
	#[yaserde(rename = "FreeBusyView", prefix = "tns", default)]
	pub free_busy_view: Option<FreeBusyView>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfFreeBusyResponse",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ArrayOfFreeBusyResponse {
	#[yaserde(rename = "FreeBusyResponse", prefix = "tns", default)]
	pub free_busy_response: Vec<FreeBusyResponseType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SuggestionsResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SuggestionsResponseType {
	#[yaserde(rename = "ResponseMessage", prefix = "tns", default)]
	pub response_message: Option<ResponseMessageType>, 
	#[yaserde(rename = "SuggestionDayResultArray", prefix = "tns", default)]
	pub suggestion_day_result_array: Option<ArrayOfSuggestionDayResult>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetUserAvailabilityResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetUserAvailabilityResponseType {
	#[yaserde(rename = "FreeBusyResponseArray", prefix = "tns", default)]
	pub free_busy_response_array: Option<ArrayOfFreeBusyResponse>, 
	#[yaserde(rename = "SuggestionsResponse", prefix = "tns", default)]
	pub suggestions_response: Option<SuggestionsResponseType>, 
}
pub type GetUserAvailabilityResponse = GetUserAvailabilityResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetUserOofSettingsRequest",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetUserOofSettingsRequest {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetUserOofSettingsResponse",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetUserOofSettingsResponse {
	#[yaserde(rename = "ResponseMessage", prefix = "tns", default)]
	pub response_message: ResponseMessageType, 
	#[yaserde(rename = "AllowExternalOof", prefix = "tns", default)]
	pub allow_external_oof: Option<ExternalAudience>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SetUserOofSettingsRequest",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SetUserOofSettingsRequest {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SetUserOofSettingsResponse",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SetUserOofSettingsResponse {
	#[yaserde(rename = "ResponseMessage", prefix = "tns", default)]
	pub response_message: Option<ResponseMessageType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ConvertIdType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ConvertIdType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SourceIds", prefix = "tns", default)]
	pub source_ids: NonEmptyArrayOfAlternateIdsType, 
}
pub type ConvertId = ConvertIdType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ConvertIdResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ConvertIdResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type ConvertIdResponse = ConvertIdResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ConvertIdResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ConvertIdResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "AlternateId", prefix = "tns", default)]
	pub alternate_id: Option<AlternateIdBaseType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetDelegateType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetDelegateType {
	#[yaserde(flatten, default)]
	pub base_delegate_type: BaseDelegateType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "UserIds", prefix = "tns", default)]
	pub user_ids: Option<ArrayOfUserIdType>, 
}
pub type GetDelegate = GetDelegateType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetDelegateResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetDelegateResponseMessageType {
	#[yaserde(flatten, default)]
	pub base_delegate_response_message_type: BaseDelegateResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "DeliverMeetingRequests", prefix = "tns", default)]
	pub deliver_meeting_requests: Option<DeliverMeetingRequestsType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfDelegateUserResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ArrayOfDelegateUserResponseMessageType {
	#[yaserde(rename = "DelegateUserResponseMessageType", prefix = "tns", default)]
	pub delegate_user_response_message_type: Vec<DelegateUserResponseMessageType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DelegateUserResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct DelegateUserResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "DelegateUser", prefix = "tns", default)]
	pub delegate_user: Option<DelegateUserType>, 
}
pub type GetDelegateResponse = GetDelegateResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AddDelegateType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct AddDelegateType {
	#[yaserde(flatten, default)]
	pub base_delegate_type: BaseDelegateType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "DelegateUsers", prefix = "tns", default)]
	pub delegate_users: ArrayOfDelegateUserType, 
	#[yaserde(rename = "DeliverMeetingRequests", prefix = "tns", default)]
	pub deliver_meeting_requests: Option<DeliverMeetingRequestsType>, 
}
pub type AddDelegate = AddDelegateType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BaseDelegateResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct BaseDelegateResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ResponseMessages", prefix = "tns", default)]
	pub response_messages: Option<ArrayOfDelegateUserResponseMessageType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "BaseDelegateType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct BaseDelegateType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Mailbox", prefix = "tns", default)]
	pub mailbox: EmailAddressType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AddDelegateResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct AddDelegateResponseMessageType {
	#[yaserde(flatten, default)]
	pub base_delegate_response_message_type: BaseDelegateResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type AddDelegateResponse = AddDelegateResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RemoveDelegateType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct RemoveDelegateType {
	#[yaserde(flatten, default)]
	pub base_delegate_type: BaseDelegateType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "UserIds", prefix = "tns", default)]
	pub user_ids: ArrayOfUserIdType, 
}
pub type RemoveDelegate = RemoveDelegateType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RemoveDelegateResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct RemoveDelegateResponseMessageType {
	#[yaserde(flatten, default)]
	pub base_delegate_response_message_type: BaseDelegateResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type RemoveDelegateResponse = RemoveDelegateResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UpdateDelegateType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UpdateDelegateType {
	#[yaserde(flatten, default)]
	pub base_delegate_type: BaseDelegateType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "DelegateUsers", prefix = "tns", default)]
	pub delegate_users: Option<ArrayOfDelegateUserType>, 
	#[yaserde(rename = "DeliverMeetingRequests", prefix = "tns", default)]
	pub deliver_meeting_requests: Option<DeliverMeetingRequestsType>, 
}
pub type UpdateDelegate = UpdateDelegateType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UpdateDelegateResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UpdateDelegateResponseMessageType {
	#[yaserde(flatten, default)]
	pub base_delegate_response_message_type: BaseDelegateResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type UpdateDelegateResponse = UpdateDelegateResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetSharingMetadataType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetSharingMetadataType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "IdOfFolderToShare", prefix = "tns", default)]
	pub id_of_folder_to_share: FolderIdType, 
	#[yaserde(rename = "SenderSmtpAddress", prefix = "tns", default)]
	pub sender_smtp_address: NonEmptyStringType, 
	#[yaserde(rename = "Recipients", prefix = "tns", default)]
	pub recipients: ArrayOfSmtpAddressType, 
}
pub type GetSharingMetadata = GetSharingMetadataType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetSharingMetadataResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetSharingMetadataResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "EncryptedSharedFolderDataCollection", prefix = "tns", default)]
	pub encrypted_shared_folder_data_collection: ArrayOfEncryptedSharedFolderDataType, 
	#[yaserde(rename = "InvalidRecipients", prefix = "tns", default)]
	pub invalid_recipients: ArrayOfInvalidRecipientsType, 
}
pub type GetSharingMetadataResponse = GetSharingMetadataResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RefreshSharingFolderType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct RefreshSharingFolderType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SharingFolderId", prefix = "tns", default)]
	pub sharing_folder_id: FolderIdType, 
}
pub type RefreshSharingFolder = RefreshSharingFolderType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RefreshSharingFolderResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct RefreshSharingFolderResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type RefreshSharingFolderResponse = RefreshSharingFolderResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetSharingFolderType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetSharingFolderType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SmtpAddress", prefix = "tns", default)]
	pub smtp_address: NonEmptyStringType, 
	#[yaserde(rename = "DataType", prefix = "tns", default)]
	pub data_type: Option<SharingDataType>, 
	#[yaserde(rename = "SharedFolderId", prefix = "tns", default)]
	pub shared_folder_id: Option<NonEmptyStringType>, 
}
pub type GetSharingFolder = GetSharingFolderType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetSharingFolderResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetSharingFolderResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SharingFolderId", prefix = "tns", default)]
	pub sharing_folder_id: FolderIdType, 
}
pub type GetSharingFolderResponse = GetSharingFolderResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CreateUserConfigurationType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct CreateUserConfigurationType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "UserConfiguration", prefix = "tns", default)]
	pub user_configuration: UserConfigurationType, 
}
pub type CreateUserConfiguration = CreateUserConfigurationType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CreateUserConfigurationResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct CreateUserConfigurationResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type CreateUserConfigurationResponse = CreateUserConfigurationResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeleteUserConfigurationType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct DeleteUserConfigurationType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "UserConfigurationName", prefix = "tns", default)]
	pub user_configuration_name: UserConfigurationNameType, 
}
pub type DeleteUserConfiguration = DeleteUserConfigurationType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeleteUserConfigurationResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct DeleteUserConfigurationResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type DeleteUserConfigurationResponse = DeleteUserConfigurationResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetUserConfigurationType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetUserConfigurationType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "UserConfigurationName", prefix = "tns", default)]
	pub user_configuration_name: UserConfigurationNameType, 
	#[yaserde(rename = "UserConfigurationProperties", prefix = "tns", default)]
	pub user_configuration_properties: UserConfigurationPropertyType, 
}
pub type GetUserConfiguration = GetUserConfigurationType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetUserConfigurationResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetUserConfigurationResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "UserConfiguration", prefix = "tns", default)]
	pub user_configuration: Option<UserConfigurationType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetUserConfigurationResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetUserConfigurationResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type GetUserConfigurationResponse = GetUserConfigurationResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetSpecificUserConfigurationType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetSpecificUserConfigurationType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "UserConfigurationName", prefix = "tns", default)]
	pub user_configuration_name: UserConfigurationNameType, 
	#[yaserde(rename = "UserConfigurationProperties", prefix = "tns", default)]
	pub user_configuration_properties: UserConfigurationPropertyType, 
}
pub type GetSpecificUserConfiguration = GetSpecificUserConfigurationType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetSpecificUserConfigurationResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetSpecificUserConfigurationResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "UserConfiguration", prefix = "tns", default)]
	pub user_configuration: Option<UserConfigurationType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetSpecificUserConfigurationResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetSpecificUserConfigurationResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type GetSpecificUserConfigurationResponse = GetSpecificUserConfigurationResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UpdateUserConfigurationType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UpdateUserConfigurationType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "UserConfiguration", prefix = "tns", default)]
	pub user_configuration: UserConfigurationType, 
}
pub type UpdateUserConfiguration = UpdateUserConfigurationType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UpdateUserConfigurationResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UpdateUserConfigurationResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type UpdateUserConfigurationResponse = UpdateUserConfigurationResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SetTeamMailboxRequestType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SetTeamMailboxRequestType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "EmailAddress", prefix = "tns", default)]
	pub email_address: EmailAddressType, 
	#[yaserde(rename = "SharePointSiteUrl", prefix = "tns", default)]
	pub share_point_site_url: String, 
	#[yaserde(rename = "State", prefix = "tns", default)]
	pub state: TeamMailboxLifecycleStateType, 
}
pub type SetTeamMailbox = SetTeamMailboxRequestType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UnpinTeamMailboxRequestType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UnpinTeamMailboxRequestType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "EmailAddress", prefix = "tns", default)]
	pub email_address: EmailAddressType, 
}
pub type UnpinTeamMailbox = UnpinTeamMailboxRequestType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SetTeamMailboxResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SetTeamMailboxResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type SetTeamMailboxResponse = SetTeamMailboxResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UnpinTeamMailboxResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UnpinTeamMailboxResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type UnpinTeamMailboxResponse = UnpinTeamMailboxResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetRoomListsType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetRoomListsType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type GetRoomLists = GetRoomListsType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetRoomListsResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetRoomListsResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "RoomLists", prefix = "tns", default)]
	pub room_lists: Option<ArrayOfEmailAddressesType>, 
}
pub type GetRoomListsResponse = GetRoomListsResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetRoomsType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetRoomsType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "RoomList", prefix = "tns", default)]
	pub room_list: EmailAddressType, 
}
pub type GetRooms = GetRoomsType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetRoomsResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetRoomsResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Rooms", prefix = "tns", default)]
	pub rooms: Option<ArrayOfRoomsType>, 
}
pub type GetRoomsResponse = GetRoomsResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetRemindersType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetRemindersType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "BeginTime", prefix = "tns", default)]
	pub begin_time: Option<String>, 
	#[yaserde(rename = "EndTime", prefix = "tns", default)]
	pub end_time: Option<String>, 
	#[yaserde(rename = "MaxItems", prefix = "tns", default)]
	pub max_items: Option<i32>, 
	#[yaserde(rename = "ReminderType", prefix = "tns", default)]
	pub reminder_type: Option<String>, 
}
pub type GetReminders = GetRemindersType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetRemindersResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetRemindersResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Reminders", prefix = "tns", default)]
	pub reminders: ArrayOfRemindersType, 
}
pub type GetRemindersResponse = GetRemindersResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PerformReminderActionType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct PerformReminderActionType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ReminderItemActions", prefix = "tns", default)]
	pub reminder_item_actions: NonEmptyArrayOfReminderItemActionType, 
}
pub type PerformReminderAction = PerformReminderActionType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PerformReminderActionResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct PerformReminderActionResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "UpdatedItemIds", prefix = "tns", default)]
	pub updated_item_ids: NonEmptyArrayOfItemIdsType, 
}
pub type PerformReminderActionResponse = PerformReminderActionResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfServiceConfigurationType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ArrayOfServiceConfigurationType {
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetServiceConfigurationType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetServiceConfigurationType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ActingAs", prefix = "tns", default)]
	pub acting_as: Option<EmailAddressType>, 
	#[yaserde(rename = "RequestedConfiguration", prefix = "tns", default)]
	pub requested_configuration: ArrayOfServiceConfigurationType, 
	#[yaserde(rename = "ConfigurationRequestDetails", prefix = "tns", default)]
	pub configuration_request_details: Option<ConfigurationRequestDetailsType>, 
}
pub type GetServiceConfiguration = GetServiceConfigurationType;

pub type GetServiceConfigurationResponse = GetServiceConfigurationResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetServiceConfigurationResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetServiceConfigurationResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ResponseMessages", prefix = "tns", default)]
	pub response_messages: Option<ArrayOfServiceConfigurationResponseMessageType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfServiceConfigurationResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ArrayOfServiceConfigurationResponseMessageType {
	#[yaserde(rename = "ServiceConfigurationResponseMessageType", prefix = "tns", default)]
	pub service_configuration_response_message_type: Vec<ServiceConfigurationResponseMessageType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ServiceConfigurationResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ServiceConfigurationResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MailTipsConfiguration", prefix = "tns", default)]
	pub mail_tips_configuration: Option<MailTipsServiceConfiguration>, 
	#[yaserde(rename = "UnifiedMessagingConfiguration", prefix = "tns", default)]
	pub unified_messaging_configuration: Option<UnifiedMessageServiceConfiguration>, 
	#[yaserde(rename = "ProtectionRulesConfiguration", prefix = "tns", default)]
	pub protection_rules_configuration: Option<ProtectionRulesServiceConfiguration>, 
	#[yaserde(rename = "PolicyNudgeRulesConfiguration", prefix = "tns", default)]
	pub policy_nudge_rules_configuration: Option<PolicyNudgeRulesServiceConfiguration>, 
	#[yaserde(rename = "SharePointURLsConfiguration", prefix = "tns", default)]
	pub share_point_ur_ls_configuration: Option<SharePointURLsServiceConfiguration>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindMessageTrackingReportRequestType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct FindMessageTrackingReportRequestType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type FindMessageTrackingReport = FindMessageTrackingReportRequestType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindMessageTrackingReportResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct FindMessageTrackingReportResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Diagnostics", prefix = "tns", default)]
	pub diagnostics: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "MessageTrackingSearchResults", prefix = "tns", default)]
	pub message_tracking_search_results: Option<ArrayOfFindMessageTrackingSearchResultType>, 
	#[yaserde(rename = "ExecutedSearchScope", prefix = "tns", default)]
	pub executed_search_scope: Option<String>, 
	#[yaserde(rename = "Errors", prefix = "tns", default)]
	pub errors: Option<ArrayOfArraysOfTrackingPropertiesType>, 
	#[yaserde(rename = "Properties", prefix = "tns", default)]
	pub properties: Option<ArrayOfTrackingPropertiesType>, 
}
pub type FindMessageTrackingReportResponse = FindMessageTrackingReportResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetMessageTrackingReportRequestType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetMessageTrackingReportRequestType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type GetMessageTrackingReport = GetMessageTrackingReportRequestType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetMessageTrackingReportResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetMessageTrackingReportResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MessageTrackingReport", prefix = "tns", default)]
	pub message_tracking_report: Option<MessageTrackingReportType>, 
	#[yaserde(rename = "Diagnostics", prefix = "tns", default)]
	pub diagnostics: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "Errors", prefix = "tns", default)]
	pub errors: Option<ArrayOfArraysOfTrackingPropertiesType>, 
	#[yaserde(rename = "Properties", prefix = "tns", default)]
	pub properties: Option<ArrayOfTrackingPropertiesType>, 
}
pub type GetMessageTrackingReportResponse = GetMessageTrackingReportResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetInboxRulesRequestType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetInboxRulesRequestType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MailboxSmtpAddress", prefix = "tns", default)]
	pub mailbox_smtp_address: Option<String>, 
}
pub type GetInboxRules = GetInboxRulesRequestType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetInboxRulesResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetInboxRulesResponseType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "OutlookRuleBlobExists", prefix = "tns", default)]
	pub outlook_rule_blob_exists: Option<bool>, 
	#[yaserde(rename = "InboxRules", prefix = "tns", default)]
	pub inbox_rules: Option<ArrayOfRulesType>, 
}
pub type GetInboxRulesResponse = GetInboxRulesResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UpdateInboxRulesRequestType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UpdateInboxRulesRequestType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MailboxSmtpAddress", prefix = "tns", default)]
	pub mailbox_smtp_address: Option<String>, 
	#[yaserde(rename = "RemoveOutlookRuleBlob", prefix = "tns", default)]
	pub remove_outlook_rule_blob: Option<bool>, 
	#[yaserde(rename = "Operations", prefix = "tns", default)]
	pub operations: ArrayOfRuleOperationsType, 
}
pub type UpdateInboxRules = UpdateInboxRulesRequestType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UpdateInboxRulesResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UpdateInboxRulesResponseType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "RuleOperationErrors", prefix = "tns", default)]
	pub rule_operation_errors: Option<ArrayOfRuleOperationErrorsType>, 
}
pub type UpdateInboxRulesResponse = UpdateInboxRulesResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindMailboxStatisticsByKeywordsType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct FindMailboxStatisticsByKeywordsType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Mailboxes", prefix = "tns", default)]
	pub mailboxes: ArrayOfUserMailboxesType, 
	#[yaserde(rename = "Keywords", prefix = "tns", default)]
	pub keywords: ArrayOfStringsType, 
	#[yaserde(rename = "Language", prefix = "tns", default)]
	pub language: Option<String>, 
	#[yaserde(rename = "Senders", prefix = "tns", default)]
	pub senders: Option<ArrayOfSmtpAddressType>, 
	#[yaserde(rename = "Recipients", prefix = "tns", default)]
	pub recipients: Option<ArrayOfSmtpAddressType>, 
	#[yaserde(rename = "FromDate", prefix = "tns", default)]
	pub from_date: Option<String>, 
	#[yaserde(rename = "ToDate", prefix = "tns", default)]
	pub to_date: Option<String>, 
	#[yaserde(rename = "MessageTypes", prefix = "tns", default)]
	pub message_types: Option<ArrayOfSearchItemKindsType>, 
	#[yaserde(rename = "SearchDumpster", prefix = "tns", default)]
	pub search_dumpster: Option<bool>, 
	#[yaserde(rename = "IncludePersonalArchive", prefix = "tns", default)]
	pub include_personal_archive: Option<bool>, 
	#[yaserde(rename = "IncludeUnsearchableItems", prefix = "tns", default)]
	pub include_unsearchable_items: Option<bool>, 
}
pub type FindMailboxStatisticsByKeywords = FindMailboxStatisticsByKeywordsType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindMailboxStatisticsByKeywordsResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct FindMailboxStatisticsByKeywordsResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type FindMailboxStatisticsByKeywordsResponse = FindMailboxStatisticsByKeywordsResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindMailboxStatisticsByKeywordsResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct FindMailboxStatisticsByKeywordsResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MailboxStatisticsSearchResult", prefix = "tns", default)]
	pub mailbox_statistics_search_result: MailboxStatisticsSearchResultType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetSearchableMailboxesType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetSearchableMailboxesType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SearchFilter", prefix = "tns", default)]
	pub search_filter: Option<String>, 
	#[yaserde(rename = "ExpandGroupMembership", prefix = "tns", default)]
	pub expand_group_membership: Option<bool>, 
}
pub type GetSearchableMailboxes = GetSearchableMailboxesType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetSearchableMailboxesResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetSearchableMailboxesResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SearchableMailboxes", prefix = "tns", default)]
	pub searchable_mailboxes: ArrayOfSearchableMailboxesType, 
	#[yaserde(rename = "FailedMailboxes", prefix = "tns", default)]
	pub failed_mailboxes: Option<ArrayOfFailedSearchMailboxesType>, 
}
pub type GetSearchableMailboxesResponse = GetSearchableMailboxesResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchMailboxesType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SearchMailboxesType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SearchQueries", prefix = "tns", default)]
	pub search_queries: NonEmptyArrayOfMailboxQueriesType, 
	#[yaserde(rename = "ResultType", prefix = "tns", default)]
	pub result_type: SearchResultType, 
	#[yaserde(rename = "PreviewItemResponseShape", prefix = "tns", default)]
	pub preview_item_response_shape: Option<PreviewItemResponseShapeType>, 
	#[yaserde(rename = "SortBy", prefix = "tns", default)]
	pub sort_by: Option<FieldOrderType>, 
	#[yaserde(rename = "Language", prefix = "tns", default)]
	pub language: Option<String>, 
	#[yaserde(rename = "Deduplication", prefix = "tns", default)]
	pub deduplication: Option<bool>, 
	#[yaserde(rename = "PageSize", prefix = "tns", default)]
	pub page_size: Option<i32>, 
	#[yaserde(rename = "PageItemReference", prefix = "tns", default)]
	pub page_item_reference: Option<String>, 
	#[yaserde(rename = "PageDirection", prefix = "tns", default)]
	pub page_direction: Option<SearchPageDirectionType>, 
}
pub type SearchMailboxes = SearchMailboxesType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchMailboxesResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SearchMailboxesResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type SearchMailboxesResponse = SearchMailboxesResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SearchMailboxesResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SearchMailboxesResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SearchMailboxesResult", prefix = "tns", default)]
	pub search_mailboxes_result: Option<SearchMailboxesResultType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetDiscoverySearchConfigurationType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetDiscoverySearchConfigurationType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SearchId", prefix = "tns", default)]
	pub search_id: Option<String>, 
	#[yaserde(rename = "ExpandGroupMembership", prefix = "tns", default)]
	pub expand_group_membership: Option<bool>, 
	#[yaserde(rename = "InPlaceHoldConfigurationOnly", prefix = "tns", default)]
	pub in_place_hold_configuration_only: Option<bool>, 
}
pub type GetDiscoverySearchConfiguration = GetDiscoverySearchConfigurationType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetDiscoverySearchConfigurationResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetDiscoverySearchConfigurationResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "DiscoverySearchConfigurations", prefix = "tns", default)]
	pub discovery_search_configurations: ArrayOfDiscoverySearchConfigurationType, 
}
pub type GetDiscoverySearchConfigurationResponse = GetDiscoverySearchConfigurationResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetHoldOnMailboxesType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetHoldOnMailboxesType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "HoldId", prefix = "tns", default)]
	pub hold_id: String, 
}
pub type GetHoldOnMailboxes = GetHoldOnMailboxesType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetHoldOnMailboxesResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetHoldOnMailboxesResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MailboxHoldResult", prefix = "tns", default)]
	pub mailbox_hold_result: Option<MailboxHoldResultType>, 
}
pub type GetHoldOnMailboxesResponse = GetHoldOnMailboxesResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SetHoldOnMailboxesType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SetHoldOnMailboxesType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ActionType", prefix = "tns", default)]
	pub action_type: HoldActionType, 
	#[yaserde(rename = "HoldId", prefix = "tns", default)]
	pub hold_id: String, 
	#[yaserde(rename = "Query", prefix = "tns", default)]
	pub query: String, 
	#[yaserde(rename = "Mailboxes", prefix = "tns", default)]
	pub mailboxes: Option<ArrayOfStringsType>, 
	#[yaserde(rename = "Language", prefix = "tns", default)]
	pub language: Option<String>, 
	#[yaserde(rename = "IncludeNonIndexableItems", prefix = "tns", default)]
	pub include_non_indexable_items: Option<bool>, 
	#[yaserde(rename = "Deduplication", prefix = "tns", default)]
	pub deduplication: Option<bool>, 
	#[yaserde(rename = "InPlaceHoldIdentity", prefix = "tns", default)]
	pub in_place_hold_identity: Option<String>, 
	#[yaserde(rename = "ItemHoldPeriod", prefix = "tns", default)]
	pub item_hold_period: Option<String>, 
}
pub type SetHoldOnMailboxes = SetHoldOnMailboxesType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SetHoldOnMailboxesResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SetHoldOnMailboxesResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MailboxHoldResult", prefix = "tns", default)]
	pub mailbox_hold_result: Option<MailboxHoldResultType>, 
}
pub type SetHoldOnMailboxesResponse = SetHoldOnMailboxesResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetNonIndexableItemStatisticsType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetNonIndexableItemStatisticsType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Mailboxes", prefix = "tns", default)]
	pub mailboxes: NonEmptyArrayOfLegacyDNsType, 
	#[yaserde(rename = "SearchArchiveOnly", prefix = "tns", default)]
	pub search_archive_only: Option<bool>, 
}
pub type GetNonIndexableItemStatistics = GetNonIndexableItemStatisticsType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetNonIndexableItemStatisticsResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetNonIndexableItemStatisticsResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "NonIndexableItemStatistics", prefix = "tns", default)]
	pub non_indexable_item_statistics: Option<ArrayOfNonIndexableItemStatisticsType>, 
}
pub type GetNonIndexableItemStatisticsResponse = GetNonIndexableItemStatisticsResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetNonIndexableItemDetailsType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetNonIndexableItemDetailsType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Mailboxes", prefix = "tns", default)]
	pub mailboxes: NonEmptyArrayOfLegacyDNsType, 
	#[yaserde(rename = "PageSize", prefix = "tns", default)]
	pub page_size: Option<i32>, 
	#[yaserde(rename = "PageItemReference", prefix = "tns", default)]
	pub page_item_reference: Option<String>, 
	#[yaserde(rename = "PageDirection", prefix = "tns", default)]
	pub page_direction: Option<SearchPageDirectionType>, 
	#[yaserde(rename = "SearchArchiveOnly", prefix = "tns", default)]
	pub search_archive_only: Option<bool>, 
}
pub type GetNonIndexableItemDetails = GetNonIndexableItemDetailsType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetNonIndexableItemDetailsResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetNonIndexableItemDetailsResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "NonIndexableItemDetailsResult", prefix = "tns", default)]
	pub non_indexable_item_details_result: Option<NonIndexableItemDetailResultType>, 
}
pub type GetNonIndexableItemDetailsResponse = GetNonIndexableItemDetailsResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MarkAllItemsAsReadType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct MarkAllItemsAsReadType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ReadFlag", prefix = "tns", default)]
	pub read_flag: bool, 
	#[yaserde(rename = "SuppressReadReceipts", prefix = "tns", default)]
	pub suppress_read_receipts: bool, 
	#[yaserde(rename = "FolderIds", prefix = "tns", default)]
	pub folder_ids: NonEmptyArrayOfBaseFolderIdsType, 
}
pub type MarkAllItemsAsRead = MarkAllItemsAsReadType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MarkAllItemsAsReadResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct MarkAllItemsAsReadResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type MarkAllItemsAsReadResponse = MarkAllItemsAsReadResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetConversationItemsType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetConversationItemsType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ItemShape", prefix = "tns", default)]
	pub item_shape: ItemResponseShapeType, 
	#[yaserde(rename = "FoldersToIgnore", prefix = "tns", default)]
	pub folders_to_ignore: Option<NonEmptyArrayOfBaseFolderIdsType>, 
	#[yaserde(rename = "MaxItemsToReturn", prefix = "tns", default)]
	pub max_items_to_return: Option<i32>, 
	#[yaserde(rename = "SortOrder", prefix = "tns", default)]
	pub sort_order: Option<ConversationNodeSortOrder>, 
	#[yaserde(rename = "MailboxScope", prefix = "tns", default)]
	pub mailbox_scope: Option<MailboxSearchLocationType>, 
	#[yaserde(rename = "Conversations", prefix = "tns", default)]
	pub conversations: ArrayOfConversationRequestsType, 
}
pub type GetConversationItems = GetConversationItemsType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetConversationItemsResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetConversationItemsResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Conversation", prefix = "tns", default)]
	pub conversation: Option<ConversationResponseType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetConversationItemsResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetConversationItemsResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type GetConversationItemsResponse = GetConversationItemsResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetOMEConfigurationType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetOMEConfigurationType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "TemplateName", prefix = "tns", default)]
	pub template_name: Option<String>, 
}
pub type GetOMEConfiguration = GetOMEConfigurationType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "OMEConfigurationResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct OmeconfigurationResponseType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Xml", prefix = "tns", default)]
	pub xml: Option<String>, 
	#[yaserde(rename = "ConfigList", prefix = "tns", default)]
	pub config_list: Option<ArrayOfStringsType>, 
}
pub type GetOMEConfigurationResponse = OmeconfigurationResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SetOMEConfigurationType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SetOMEConfigurationType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Xml", prefix = "tns", default)]
	pub xml: Option<String>, 
	#[yaserde(rename = "TemplateName", prefix = "tns", default)]
	pub template_name: Option<String>, 
}
pub type SetOMEConfiguration = SetOMEConfigurationType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SetOMEConfigurationResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SetOMEConfigurationResponseType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type SetOMEConfigurationResponse = SetOMEConfigurationResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetOMEMessageStatusType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetOMEMessageStatusType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MessageId", prefix = "tns", default)]
	pub message_id: Option<String>, 
}
pub type GetOMEMessageStatus = GetOMEMessageStatusType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetOMEMessageStatusResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetOMEMessageStatusResponseType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "IsRevoked", prefix = "tns", default)]
	pub is_revoked: Option<bool>, 
	#[yaserde(rename = "ReceivedTime", prefix = "tns", default)]
	pub received_time: Option<String>, 
	#[yaserde(rename = "Subject", prefix = "tns", default)]
	pub subject: Option<String>, 
	#[yaserde(rename = "Container", prefix = "tns", default)]
	pub container: Option<String>, 
	#[yaserde(rename = "IsRevocable", prefix = "tns", default)]
	pub is_revocable: Option<bool>, 
}
pub type GetOMEMessageStatusResponse = GetOMEMessageStatusResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SetOMEMessageStatusType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SetOMEMessageStatusType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MessageId", prefix = "tns", default)]
	pub message_id: Option<String>, 
	#[yaserde(rename = "IsRevoked", prefix = "tns", default)]
	pub is_revoked: Option<bool>, 
}
pub type SetOMEMessageStatus = SetOMEMessageStatusType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SetOMEMessageStatusResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SetOMEMessageStatusResponseType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Subject", prefix = "tns", default)]
	pub subject: Option<String>, 
	#[yaserde(rename = "MessageId", prefix = "tns", default)]
	pub message_id: Option<String>, 
	#[yaserde(rename = "CorrelationId", prefix = "tns", default)]
	pub correlation_id: Option<String>, 
	#[yaserde(rename = "RevokeStatus", prefix = "tns", default)]
	pub revoke_status: OmemessageRevocationStatus, 
	#[yaserde(rename = "IsRevoked", prefix = "tns", default)]
	pub is_revoked: Option<bool>, 
}
pub type SetOMEMessageStatusResponse = SetOMEMessageStatusResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetAppManifestsType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetAppManifestsType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ApiVersionSupported", prefix = "tns", default)]
	pub api_version_supported: Option<String>, 
	#[yaserde(rename = "SchemaVersionSupported", prefix = "tns", default)]
	pub schema_version_supported: Option<String>, 
	#[yaserde(rename = "IncludeAllInstalledAddIns", prefix = "tns", default)]
	pub include_all_installed_add_ins: Option<bool>, 
	#[yaserde(rename = "IncludeEntitlementData", prefix = "tns", default)]
	pub include_entitlement_data: Option<bool>, 
	#[yaserde(rename = "IncludeManifestData", prefix = "tns", default)]
	pub include_manifest_data: Option<bool>, 
	#[yaserde(rename = "IncludeCustomAppsData", prefix = "tns", default)]
	pub include_custom_apps_data: Option<bool>, 
	#[yaserde(rename = "ExtensionIds", prefix = "tns", default)]
	pub extension_ids: Option<ListOfExtensionIdsType>, 
	#[yaserde(rename = "AddIns", prefix = "tns", default)]
	pub add_ins: Option<ArrayOfPrivateCatalogAddInsType>, 
	#[yaserde(rename = "IncludeExtensionMetaData", prefix = "tns", default)]
	pub include_extension_meta_data: Option<bool>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ListOfExtensionIdsType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ListOfExtensionIdsType {
	#[yaserde(flatten, default)]
	pub body: Box<ListOfExtensionIdsType>, 
}
pub type GetAppManifests = GetAppManifestsType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetAppManifestsResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetAppManifestsResponseType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfAppManifestsType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ArrayOfAppManifestsType {
	#[yaserde(rename = "Manifest", prefix = "tns", default)]
	pub manifest: Vec<String>, 
}
pub type GetAppManifestsResponse = GetAppManifestsResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetAppMarketplaceUrlType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetAppMarketplaceUrlType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type GetAppMarketplaceUrl = GetAppMarketplaceUrlType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetAppMarketplaceUrlResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetAppMarketplaceUrlResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "AppMarketplaceUrl", prefix = "tns", default)]
	pub app_marketplace_url: Option<String>, 
	#[yaserde(rename = "ConnectorsManagementUrl", prefix = "tns", default)]
	pub connectors_management_url: Option<String>, 
}
pub type GetAppMarketplaceUrlResponse = GetAppMarketplaceUrlResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MarkAsJunkType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct MarkAsJunkType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ItemIds", prefix = "tns", default)]
	pub item_ids: NonEmptyArrayOfBaseItemIdsType, 
}
pub type MarkAsJunk = MarkAsJunkType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MarkAsJunkResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct MarkAsJunkResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type MarkAsJunkResponse = MarkAsJunkResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MarkAsJunkResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct MarkAsJunkResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MarkAsPhishingType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct MarkAsPhishingType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ItemIds", prefix = "tns", default)]
	pub item_ids: NonEmptyArrayOfBaseItemIdsType, 
}
pub type MarkAsPhishing = MarkAsPhishingType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MarkAsPhishingResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct MarkAsPhishingResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type MarkAsPhishingResponse = MarkAsPhishingResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "MarkAsPhishingResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct MarkAsPhishingResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ReportMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ReportMessageType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ItemIds", prefix = "tns", default)]
	pub item_ids: Option<ArrayOfBaseItemIdsType>, 
	#[yaserde(rename = "BlockReportingToMicrosoft", prefix = "tns", default)]
	pub block_reporting_to_microsoft: Option<bool>, 
	#[yaserde(rename = "Platform", prefix = "tns", default)]
	pub platform: Option<ReportMessagePlatformType>, 
}
pub type ReportMessage = ReportMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ReportMessageResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ReportMessageResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type ReportMessageResponse = ReportMessageResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ReportMessageResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ReportMessageResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "InstallAppType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct InstallAppType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Manifest", prefix = "tns", default)]
	pub manifest: String, 
	#[yaserde(rename = "MarketplaceAssetId", prefix = "tns", default)]
	pub marketplace_asset_id: Option<String>, 
	#[yaserde(rename = "MarketplaceContentMarket", prefix = "tns", default)]
	pub marketplace_content_market: Option<String>, 
	#[yaserde(rename = "SendWelcomeEmail", prefix = "tns", default)]
	pub send_welcome_email: Option<bool>, 
	#[yaserde(rename = "ManifestUrl", prefix = "tns", default)]
	pub manifest_url: Option<String>, 
	#[yaserde(rename = "MarketplaceCorrelationId", prefix = "tns", default)]
	pub marketplace_correlation_id: Option<String>, 
	#[yaserde(rename = "CampaignId", prefix = "tns", default)]
	pub campaign_id: Option<String>, 
	#[yaserde(rename = "Id", prefix = "tns", default)]
	pub id: Option<String>, 
	#[yaserde(rename = "IsMetaOSApp", prefix = "tns", default)]
	pub is_meta_os_app: Option<bool>, 
	#[yaserde(rename = "MetaOSSyncData", prefix = "tns", default)]
	pub meta_os_sync_data: Option<String>, 
}
pub type InstallApp = InstallAppType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "InstallAppResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct InstallAppResponseType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "WasFirstInstall", prefix = "tns", default)]
	pub was_first_install: Option<bool>, 
	#[yaserde(rename = "Extension", prefix = "tns", default)]
	pub extension: Option<InstalledAppType>, 
	#[yaserde(rename = "MissingTitles", prefix = "tns", default)]
	pub missing_titles: Option<String>, 
}
pub type InstallAppResponse = InstallAppResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UpdateExtensionUsageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UpdateExtensionUsageType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Client", prefix = "tns", default)]
	pub client: Option<String>, 
	#[yaserde(rename = "Extensions", prefix = "tns", default)]
	pub extensions: Option<ArrayOfUpdateExtensionUsageItemType>, 
}
pub type UpdateExtensionUsage = UpdateExtensionUsageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfUpdateExtensionUsageItemType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ArrayOfUpdateExtensionUsageItemType {
	#[yaserde(rename = "ExtensionId", prefix = "tns", default)]
	pub extension_id: Option<String>, 
	#[yaserde(rename = "Scenarios", prefix = "tns", default)]
	pub scenarios: Option<ArrayOfExtensionUsageScenarioCounterType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfExtensionUsageScenarioCounterType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ArrayOfExtensionUsageScenarioCounterType {
	#[yaserde(rename = "ScenarioName", prefix = "tns", default)]
	pub scenario_name: Option<String>, 
	#[yaserde(rename = "Count", prefix = "tns", default)]
	pub count: Option<i32>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UpdateExtensionUsageResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UpdateExtensionUsageResponseType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type UpdateExtensionUsageResponse = UpdateExtensionUsageResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UninstallAppType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UninstallAppType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ID", prefix = "tns", default)]
	pub id: String, 
	#[yaserde(rename = "IsMetaOSApp", prefix = "tns", default)]
	pub is_meta_os_app: Option<bool>, 
}
pub type UninstallApp = UninstallAppType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UninstallAppResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UninstallAppResponseType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type UninstallAppResponse = UninstallAppResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DisableAppType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct DisableAppType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ID", prefix = "tns", default)]
	pub id: String, 
	#[yaserde(rename = "DisableReason", prefix = "tns", default)]
	pub disable_reason: DisableReasonType, 
	#[yaserde(rename = "IsMetaOSApp", prefix = "tns", default)]
	pub is_meta_os_app: Option<bool>, 
}
pub type DisableApp = DisableAppType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DisableAppResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct DisableAppResponseType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type DisableAppResponse = DisableAppResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AddNewImContactToGroupType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct AddNewImContactToGroupType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ImAddress", prefix = "tns", default)]
	pub im_address: NonEmptyStringType, 
	#[yaserde(rename = "DisplayName", prefix = "tns", default)]
	pub display_name: Option<NonEmptyStringType>, 
	#[yaserde(rename = "GroupId", prefix = "tns", default)]
	pub group_id: Option<ItemIdType>, 
}
pub type AddNewImContactToGroup = AddNewImContactToGroupType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AddNewImContactToGroupResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct AddNewImContactToGroupResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Persona", prefix = "tns", default)]
	pub persona: Option<PersonaType>, 
}
pub type AddNewImContactToGroupResponse = AddNewImContactToGroupResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AddNewTelUriContactToGroupType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct AddNewTelUriContactToGroupType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "TelUriAddress", prefix = "tns", default)]
	pub tel_uri_address: NonEmptyStringType, 
	#[yaserde(rename = "ImContactSipUriAddress", prefix = "tns", default)]
	pub im_contact_sip_uri_address: NonEmptyStringType, 
	#[yaserde(rename = "ImTelephoneNumber", prefix = "tns", default)]
	pub im_telephone_number: NonEmptyStringType, 
	#[yaserde(rename = "GroupId", prefix = "tns", default)]
	pub group_id: Option<ItemIdType>, 
}
pub type AddNewTelUriContactToGroup = AddNewTelUriContactToGroupType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AddNewTelUriContactToGroupResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct AddNewTelUriContactToGroupResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Persona", prefix = "tns", default)]
	pub persona: Option<PersonaType>, 
}
pub type AddNewTelUriContactToGroupResponse = AddNewTelUriContactToGroupResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AddImContactToGroupType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct AddImContactToGroupType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ContactId", prefix = "tns", default)]
	pub contact_id: ItemIdType, 
	#[yaserde(rename = "GroupId", prefix = "tns", default)]
	pub group_id: Option<ItemIdType>, 
}
pub type AddImContactToGroup = AddImContactToGroupType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AddImContactToGroupResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct AddImContactToGroupResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type AddImContactToGroupResponse = AddImContactToGroupResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RemoveImContactFromGroupType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct RemoveImContactFromGroupType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ContactId", prefix = "tns", default)]
	pub contact_id: ItemIdType, 
	#[yaserde(rename = "GroupId", prefix = "tns", default)]
	pub group_id: ItemIdType, 
}
pub type RemoveImContactFromGroup = RemoveImContactFromGroupType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RemoveImContactFromGroupResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct RemoveImContactFromGroupResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type RemoveImContactFromGroupResponse = RemoveImContactFromGroupResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AddImGroupType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct AddImGroupType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "DisplayName", prefix = "tns", default)]
	pub display_name: NonEmptyStringType, 
}
pub type AddImGroup = AddImGroupType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AddImGroupResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct AddImGroupResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ImGroup", prefix = "tns", default)]
	pub im_group: Option<ImGroupType>, 
}
pub type AddImGroupResponse = AddImGroupResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AddDistributionGroupToImListType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct AddDistributionGroupToImListType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SmtpAddress", prefix = "tns", default)]
	pub smtp_address: NonEmptyStringType, 
	#[yaserde(rename = "DisplayName", prefix = "tns", default)]
	pub display_name: NonEmptyStringType, 
}
pub type AddDistributionGroupToImList = AddDistributionGroupToImListType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "AddDistributionGroupToImListResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct AddDistributionGroupToImListResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ImGroup", prefix = "tns", default)]
	pub im_group: Option<ImGroupType>, 
}
pub type AddDistributionGroupToImListResponse = AddDistributionGroupToImListResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetImItemListType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetImItemListType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ExtendedProperties", prefix = "tns", default)]
	pub extended_properties: Option<NonEmptyArrayOfExtendedFieldURIs>, 
}
pub type GetImItemList = GetImItemListType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetImItemListResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetImItemListResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ImItemList", prefix = "tns", default)]
	pub im_item_list: Option<ImItemListType>, 
}
pub type GetImItemListResponse = GetImItemListResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetImItemsType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetImItemsType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ContactIds", prefix = "tns", default)]
	pub contact_ids: Option<NonEmptyArrayOfBaseItemIdsType>, 
	#[yaserde(rename = "GroupIds", prefix = "tns", default)]
	pub group_ids: Option<NonEmptyArrayOfBaseItemIdsType>, 
	#[yaserde(rename = "ExtendedProperties", prefix = "tns", default)]
	pub extended_properties: Option<NonEmptyArrayOfExtendedFieldURIs>, 
}
pub type GetImItems = GetImItemsType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetImItemsResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetImItemsResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ImItemList", prefix = "tns", default)]
	pub im_item_list: Option<ImItemListType>, 
}
pub type GetImItemsResponse = GetImItemsResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RemoveContactFromImListType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct RemoveContactFromImListType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ContactId", prefix = "tns", default)]
	pub contact_id: ItemIdType, 
}
pub type RemoveContactFromImList = RemoveContactFromImListType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RemoveContactFromImListResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct RemoveContactFromImListResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type RemoveContactFromImListResponse = RemoveContactFromImListResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RemoveDistributionGroupFromImListType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct RemoveDistributionGroupFromImListType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "GroupId", prefix = "tns", default)]
	pub group_id: ItemIdType, 
}
pub type RemoveDistributionGroupFromImList = RemoveDistributionGroupFromImListType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RemoveDistributionGroupFromImListResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct RemoveDistributionGroupFromImListResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type RemoveDistributionGroupFromImListResponse = RemoveDistributionGroupFromImListResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RemoveImGroupType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct RemoveImGroupType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "GroupId", prefix = "tns", default)]
	pub group_id: ItemIdType, 
}
pub type RemoveImGroup = RemoveImGroupType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RemoveImGroupResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct RemoveImGroupResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type RemoveImGroupResponse = RemoveImGroupResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SetImGroupType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SetImGroupType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "GroupId", prefix = "tns", default)]
	pub group_id: ItemIdType, 
	#[yaserde(rename = "NewDisplayName", prefix = "tns", default)]
	pub new_display_name: NonEmptyStringType, 
}
pub type SetImGroup = SetImGroupType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SetImGroupResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SetImGroupResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type SetImGroupResponse = SetImGroupResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SetImListMigrationCompletedType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SetImListMigrationCompletedType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ImListMigrationCompleted", prefix = "tns", default)]
	pub im_list_migration_completed: bool, 
}
pub type SetImListMigrationCompleted = SetImListMigrationCompletedType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SetImListMigrationCompletedResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SetImListMigrationCompletedResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type SetImListMigrationCompletedResponse = SetImListMigrationCompletedResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetUserRetentionPolicyTagsType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetUserRetentionPolicyTagsType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type GetUserRetentionPolicyTags = GetUserRetentionPolicyTagsType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetUserRetentionPolicyTagsResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetUserRetentionPolicyTagsResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "RetentionPolicyTags", prefix = "tns", default)]
	pub retention_policy_tags: ArrayOfRetentionPolicyTagsType, 
}
pub type GetUserRetentionPolicyTagsResponse = GetUserRetentionPolicyTagsResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetUserPhotoType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetUserPhotoType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Email", prefix = "tns", default)]
	pub email: String, 
	#[yaserde(rename = "SizeRequested", prefix = "tns", default)]
	pub size_requested: UserPhotoSizeType, 
	#[yaserde(rename = "TypeRequested", prefix = "tns", default)]
	pub type_requested: Option<UserPhotoTypeType>, 
}
pub type GetUserPhoto = GetUserPhotoType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetUserPhotoResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetUserPhotoResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "HasChanged", prefix = "tns", default)]
	pub has_changed: bool, 
	#[yaserde(rename = "PictureData", prefix = "tns", default)]
	pub picture_data: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetUserPhotoResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetUserPhotoResponseType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type GetUserPhotoResponse = GetUserPhotoResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetMeetingSpaceType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetMeetingSpaceType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ItemId", prefix = "tns", default)]
	pub item_id: ItemIdType, 
}
pub type GetMeetingSpace = GetMeetingSpaceType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetMeetingSpaceResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetMeetingSpaceResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MeetingSpace", prefix = "tns", default)]
	pub meeting_space: MeetingSpaceType, 
}
pub type GetMeetingSpaceResponseMessage = GetMeetingSpaceResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CreateMeetingSpaceType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct CreateMeetingSpaceType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MeetingSpace", prefix = "tns", default)]
	pub meeting_space: MeetingSpaceType, 
}
pub type CreateMeetingSpace = CreateMeetingSpaceType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CreateMeetingSpaceResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct CreateMeetingSpaceResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MeetingSpace", prefix = "tns", default)]
	pub meeting_space: MeetingSpaceType, 
}
pub type CreateMeetingSpaceResponseMessage = CreateMeetingSpaceResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UpdateMeetingSpaceType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UpdateMeetingSpaceType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ItemId", prefix = "tns", default)]
	pub item_id: ItemIdType, 
	#[yaserde(rename = "MeetingSpace", prefix = "tns", default)]
	pub meeting_space: MeetingSpaceType, 
}
pub type UpdateMeetingSpace = UpdateMeetingSpaceType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UpdateMeetingSpaceResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UpdateMeetingSpaceResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MeetingSpace", prefix = "tns", default)]
	pub meeting_space: MeetingSpaceType, 
}
pub type UpdateMeetingSpaceResponseMessage = UpdateMeetingSpaceResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindMeetingSpaceByJoinUrlType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct FindMeetingSpaceByJoinUrlType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "JoinUrl", prefix = "tns", default)]
	pub join_url: String, 
}
pub type FindMeetingSpaceByJoinUrl = FindMeetingSpaceByJoinUrlType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindMeetingSpaceByJoinUrlResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct FindMeetingSpaceByJoinUrlResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MeetingSpace", prefix = "tns", default)]
	pub meeting_space: Option<MeetingSpaceType>, 
}
pub type FindMeetingSpaceByJoinUrlResponseMessage = FindMeetingSpaceByJoinUrlResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeleteMeetingSpaceType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct DeleteMeetingSpaceType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ItemId", prefix = "tns", default)]
	pub item_id: ItemIdType, 
}
pub type DeleteMeetingSpace = DeleteMeetingSpaceType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeleteMeetingSpaceResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct DeleteMeetingSpaceResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type DeleteMeetingSpaceResponseMessage = DeleteMeetingSpaceResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetMeetingInstanceRequestType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetMeetingInstanceRequestType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ItemId", prefix = "tns", default)]
	pub item_id: ItemIdType, 
}
pub type GetMeetingInstanceRequest = GetMeetingInstanceRequestType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetMeetingInstanceResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetMeetingInstanceResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MeetingInstance", prefix = "tns", default)]
	pub meeting_instance: MeetingInstanceType, 
}
pub type GetMeetingInstanceResponse = GetMeetingInstanceResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CreateMeetingInstanceRequestType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct CreateMeetingInstanceRequestType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MeetingInstance", prefix = "tns", default)]
	pub meeting_instance: MeetingInstanceType, 
}
pub type CreateMeetingInstanceRequest = CreateMeetingInstanceRequestType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "CreateMeetingInstanceResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct CreateMeetingInstanceResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MeetingInstance", prefix = "tns", default)]
	pub meeting_instance: MeetingInstanceType, 
}
pub type CreateMeetingInstanceResponse = CreateMeetingInstanceResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UpdateMeetingInstanceRequestType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UpdateMeetingInstanceRequestType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ItemId", prefix = "tns", default)]
	pub item_id: ItemIdType, 
	#[yaserde(rename = "MeetingInstance", prefix = "tns", default)]
	pub meeting_instance: Option<MeetingInstanceType>, 
	#[yaserde(rename = "ContentActivitiesToAdd", prefix = "tns", default)]
	pub content_activities_to_add: Option<NonEmptyArrayOfContentActivities>, 
	#[yaserde(rename = "ParticipantActivitiesToAdd", prefix = "tns", default)]
	pub participant_activities_to_add: Option<NonEmptyArrayOfParticipantActivities>, 
}
pub type UpdateMeetingInstanceRequest = UpdateMeetingInstanceRequestType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "UpdateMeetingInstanceResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct UpdateMeetingInstanceResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MeetingInstance", prefix = "tns", default)]
	pub meeting_instance: MeetingInstanceType, 
}
pub type UpdateMeetingInstanceResponse = UpdateMeetingInstanceResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeleteMeetingInstanceRequestType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct DeleteMeetingInstanceRequestType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ItemId", prefix = "tns", default)]
	pub item_id: ItemIdType, 
}
pub type DeleteMeetingInstanceRequest = DeleteMeetingInstanceRequestType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeleteMeetingInstanceResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct DeleteMeetingInstanceResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type DeleteMeetingInstanceResponse = DeleteMeetingInstanceResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SetUserPhotoType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SetUserPhotoType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Email", prefix = "tns", default)]
	pub email: NonEmptyStringType, 
	#[yaserde(rename = "Content", prefix = "tns", default)]
	pub content: String, 
	#[yaserde(rename = "TypeRequested", prefix = "tns", default)]
	pub type_requested: Option<UserPhotoTypeType>, 
}
pub type SetUserPhoto = SetUserPhotoType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "SetUserPhotoResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct SetUserPhotoResponseMessageType {
	#[yaserde(flatten, default)]
	pub base_response_message_type: BaseResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type SetUserPhotoResponse = SetUserPhotoResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RegisterConsentType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct RegisterConsentType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Id", prefix = "tns", default)]
	pub id: String, 
	#[yaserde(rename = "ConsentState", prefix = "tns", default)]
	pub consent_state: ConsentStateType, 
}
pub type RegisterConsent = RegisterConsentType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "RegisterConsentResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct RegisterConsentResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type RegisterConsentResponse = RegisterConsentResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindAvailableMeetingTimesType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct FindAvailableMeetingTimesType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Attendees", prefix = "tns", default)]
	pub attendees: Option<ArrayOfSmtpAddressType>, 
	#[yaserde(rename = "SearchWindowStart", prefix = "tns", default)]
	pub search_window_start: String, 
	#[yaserde(rename = "SearchWindowDuration", prefix = "tns", default)]
	pub search_window_duration: Duration, 
	#[yaserde(rename = "MeetingDurationInMinutes", prefix = "tns", default)]
	pub meeting_duration_in_minutes: Option<i32>, 
	#[yaserde(rename = "Location", prefix = "tns", default)]
	pub location: Option<String>, 
	#[yaserde(rename = "MaxCandidates", prefix = "tns", default)]
	pub max_candidates: Option<i32>, 
	#[yaserde(rename = "ActivityDomain", prefix = "tns", default)]
	pub activity_domain: Option<ActivityDomainType>, 
}
pub type FindAvailableMeetingTimes = FindAvailableMeetingTimesType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindAvailableMeetingTimesResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct FindAvailableMeetingTimesResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MeetingTimeCandidates", prefix = "tns", default)]
	pub meeting_time_candidates: ArrayOfMeetingTimeCandidate, 
	#[yaserde(rename = "EmptySuggestionsHint", prefix = "tns", default)]
	pub empty_suggestions_hint: Option<EmptySuggestionReason>, 
}
pub type FindAvailableMeetingTimesResponse = FindAvailableMeetingTimesResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindMeetingTimeCandidatesType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct FindMeetingTimeCandidatesType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "AttendeeConstraints", prefix = "tns", default)]
	pub attendee_constraints: Option<FindMeetingTimesAttendeeConstraints>, 
	#[yaserde(rename = "LocationConstraints", prefix = "tns", default)]
	pub location_constraints: Option<FindMeetingTimesLocationConstraints>, 
	#[yaserde(rename = "SearchConstraints", prefix = "tns", default)]
	pub search_constraints: Option<FindMeetingTimesSearchConstraints>, 
	#[yaserde(rename = "Constraints", prefix = "tns", default)]
	pub constraints: Option<FindMeetingTimesConstraints>, 
}
pub type FindMeetingTimeCandidates = FindMeetingTimeCandidatesType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "FindMeetingTimeCandidatesResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct FindMeetingTimeCandidatesResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "MeetingTimeCandidates", prefix = "tns", default)]
	pub meeting_time_candidates: ArrayOfMeetingTimeCandidate, 
}
pub type FindMeetingTimeCandidatesResponse = FindMeetingTimeCandidatesResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "StartSearchSession",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct StartSearchSession {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SearchSessionId", prefix = "tns", default)]
	pub search_session_id: GuidType, 
	#[yaserde(rename = "WarmupOptions", prefix = "tns", default)]
	pub warmup_options: WarmupOptionsType, 
	#[yaserde(rename = "SuggestionTypes", prefix = "tns", default)]
	pub suggestion_types: SuggestionKindType, 
	#[yaserde(rename = "SearchScope", prefix = "tns", default)]
	pub search_scope: ArrayOfSearchScopeType, 
	#[yaserde(rename = "IdFormat", prefix = "tns", default)]
	pub id_format: Option<IdFormatType>, 
	#[yaserde(rename = "ApplicationId", prefix = "tns", default)]
	pub application_id: Option<String>, 
	#[yaserde(rename = "Scenario", prefix = "tns", default)]
	pub scenario: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "StartSearchSessionResponseMessage",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct StartSearchSessionResponseMessage {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type StartSearchSessionResponse = StartSearchSessionResponseMessage;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetSearchSuggestions",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetSearchSuggestions {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SearchSessionId", prefix = "tns", default)]
	pub search_session_id: GuidType, 
	#[yaserde(rename = "Query", prefix = "tns", default)]
	pub query: Option<String>, 
	#[yaserde(rename = "SuggestionTypes", prefix = "tns", default)]
	pub suggestion_types: Option<SuggestionKindType>, 
	#[yaserde(rename = "SuggestionsPrimer", prefix = "tns", default)]
	pub suggestions_primer: Option<bool>, 
	#[yaserde(rename = "MaxSuggestionsCountPerSuggestionType", prefix = "tns", default)]
	pub max_suggestions_count_per_suggestion_type: Option<i64>, 
	#[yaserde(rename = "SearchScope", prefix = "tns", default)]
	pub search_scope: Option<ArrayOfSearchScopeType>, 
	#[yaserde(rename = "Scenario", prefix = "tns", default)]
	pub scenario: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetSearchSuggestionsResponseMessage",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetSearchSuggestionsResponseMessage {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SearchSuggestions", prefix = "tns", default)]
	pub search_suggestions: SearchSuggestionsType, 
}
pub type GetSearchSuggestionsResponse = GetSearchSuggestionsResponseMessage;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeleteSearchSuggestion",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct DeleteSearchSuggestion {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SearchSessionId", prefix = "tns", default)]
	pub search_session_id: GuidType, 
	#[yaserde(rename = "Query", prefix = "tns", default)]
	pub query: String, 
	#[yaserde(rename = "SuggestionTypes", prefix = "tns", default)]
	pub suggestion_types: SuggestionKindType, 
	#[yaserde(rename = "SearchScope", prefix = "tns", default)]
	pub search_scope: Option<ArrayOfSearchScopeType>, 
	#[yaserde(rename = "Scenario", prefix = "tns", default)]
	pub scenario: Option<String>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "DeleteSearchSuggestionResponseMessageType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct DeleteSearchSuggestionResponseMessageType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Response", prefix = "tns", default)]
	pub response: DeleteSearchSuggestionResponseType, 
}
pub type DeleteSearchSuggestionResponse = DeleteSearchSuggestionResponseMessageType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ExtendedKeywords",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ExtendedKeywords {
	#[yaserde(rename = "ExtendedKeywordDefinition", prefix = "tns", default)]
	pub extended_keyword_definition: Vec<ExtendedKeywordDefinitionType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ExecuteSearch",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ExecuteSearch {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "ApplicationId", prefix = "tns", default)]
	pub application_id: SearchApplicationIdType, 
	#[yaserde(rename = "Scenario", prefix = "tns", default)]
	pub scenario: String, 
	#[yaserde(rename = "SearchSessionId", prefix = "tns", default)]
	pub search_session_id: GuidType, 
	#[yaserde(rename = "SearchScope", prefix = "tns", default)]
	pub search_scope: ArrayOfSearchScopeType, 
	#[yaserde(rename = "Query", prefix = "tns", default)]
	pub query: String, 
	#[yaserde(rename = "AnalyzedQuery", prefix = "tns", default)]
	pub analyzed_query: Option<AnalyzedQuery>, 
	#[yaserde(rename = "ResultRowCount", prefix = "tns", default)]
	pub result_row_count: Option<i64>, 
	#[yaserde(rename = "ResultRowOffset", prefix = "tns", default)]
	pub result_row_offset: Option<i64>, 
	#[yaserde(rename = "MaxResultsCountHint", prefix = "tns", default)]
	pub max_results_count_hint: Option<i64>, 
	#[yaserde(rename = "MaxPreviewLength", prefix = "tns", default)]
	pub max_preview_length: Option<i64>, 
	#[yaserde(rename = "SearchRefiners", prefix = "tns", default)]
	pub search_refiners: Option<SearchRefiners>, 
	#[yaserde(rename = "ExtendedKeywords", prefix = "tns", default)]
	pub extended_keywords: Option<ExtendedKeywords>, 
	#[yaserde(rename = "RetrieveRefiners", prefix = "tns", default)]
	pub retrieve_refiners: Option<bool>, 
	#[yaserde(rename = "MaxRefinersCountPerRefinerType", prefix = "tns", default)]
	pub max_refiners_count_per_refiner_type: Option<i64>, 
	#[yaserde(rename = "IdFormat", prefix = "tns", default)]
	pub id_format: Option<IdFormatType>, 
	#[yaserde(rename = "ItemTypes", prefix = "tns", default)]
	pub item_types: ItemTypesFilterType, 
	#[yaserde(rename = "PropertySetName", prefix = "tns", default)]
	pub property_set_name: Option<SearchResultsPropertySetNameType>, 
	#[yaserde(rename = "SearchRestrictions", prefix = "tns", default)]
	pub search_restrictions: Option<RestrictionType>, 
	#[yaserde(rename = "IncludeDeleted", prefix = "tns", default)]
	pub include_deleted: Option<bool>, 
	#[yaserde(rename = "SortOrder", prefix = "tns", default)]
	pub sort_order: Option<ExecuteSearchSortOrderType>, 
	#[yaserde(rename = "KeywordMatchOption", prefix = "tns", default)]
	pub keyword_match_option: Option<MatchOptionsType>, 
	#[yaserde(rename = "ReturnAdditionalIds", prefix = "tns", default)]
	pub return_additional_ids: Option<bool>, 
	#[yaserde(rename = "RequestedProperties", prefix = "tns", default)]
	pub requested_properties: Option<ArrayOfStringsType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ExecuteSearchResponseMessage",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ExecuteSearchResponseMessage {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SearchResults", prefix = "tns", default)]
	pub search_results: SearchResultsType, 
}
pub type ExecuteSearchResponse = ExecuteSearchResponseMessage;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EndSearchSession",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct EndSearchSession {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "SearchSessionId", prefix = "tns", default)]
	pub search_session_id: GuidType, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "EndSearchSessionResponseMessage",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct EndSearchSessionResponseMessage {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type EndSearchSessionResponse = EndSearchSessionResponseMessage;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetLastPrivateCatalogUpdateType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetLastPrivateCatalogUpdateType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Client", prefix = "tns", default)]
	pub client: Option<OfficeClientType>, 
}
pub type GetLastPrivateCatalogUpdate = GetLastPrivateCatalogUpdateType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetLastPrivateCatalogUpdateResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetLastPrivateCatalogUpdateResponseType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
}
pub type GetLastPrivateCatalogUpdateResponse = GetLastPrivateCatalogUpdateResponseType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetPrivateCatalogAddInsType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetPrivateCatalogAddInsType {
	#[yaserde(flatten, default)]
	pub base_request_type: BaseRequestType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "Client", prefix = "tns", default)]
	pub client: Option<OfficeClientType>, 
}
pub type GetPrivateCatalogAddIns = GetPrivateCatalogAddInsType;

#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "GetPrivateCatalogAddInsResponseType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct GetPrivateCatalogAddInsResponseType {
	#[yaserde(flatten, default)]
	pub response_message_type: ResponseMessageType, 
#[yaserde(prefix = "xsi", rename="type", attribute)]
pub xsi_type: String,
	#[yaserde(rename = "AddIns", prefix = "tns", default)]
	pub add_ins: Option<ArrayOfPrivateCatalogAddInsType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "ArrayOfPrivateCatalogAddInsType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct ArrayOfPrivateCatalogAddInsType {
	#[yaserde(rename = "AddIn", prefix = "tns", default)]
	pub add_in: Vec<PrivateCatalogAddInsType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PrivateCatalogAddInsType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct PrivateCatalogAddInsType {
#[yaserde(rename="ProductId", attribute)]
pub product_id: String,
#[yaserde(rename="State", attribute)]
pub state: AddInStateType,
#[yaserde(rename="Version", attribute)]
pub version: VersionType,
#[yaserde(rename="DefaultEnabledStatus", attribute)]
pub default_enabled_status: Option<AadofficeExtensionStatusType>,
#[yaserde(rename="InstallTimeInTicks", attribute)]
pub install_time_in_ticks: Option<i64>,
	#[yaserde(rename = "StoreInfo", prefix = "tns", default)]
	pub store_info: Option<PrivateCatalogAddInStoreInfoType>, 
}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
	rename = "PrivateCatalogAddInStoreInfoType",
	namespace = "tns: http://schemas.microsoft.com/exchange/services/2006/messages",
	prefix = "tns",
)]
pub struct PrivateCatalogAddInStoreInfoType {
#[yaserde(rename="AssetId", attribute)]
pub asset_id: String,
#[yaserde(rename="ContentMarket", attribute)]
pub content_market: String,
}
pub type GetPrivateCatalogAddInsResponse = GetPrivateCatalogAddInsResponseType;

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

