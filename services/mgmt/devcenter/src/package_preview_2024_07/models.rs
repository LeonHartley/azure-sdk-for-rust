#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Represents an allowed environment type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AllowedEnvironmentType {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of an allowed environment type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AllowedEnvironmentTypeProperties>,
}
impl AllowedEnvironmentType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the allowed environment type list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AllowedEnvironmentTypeListResult {
    #[doc = "Current page of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AllowedEnvironmentType>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AllowedEnvironmentTypeListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AllowedEnvironmentTypeListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an allowed environment type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AllowedEnvironmentTypeProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The display name of the allowed environment type."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl AllowedEnvironmentTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an attached NetworkConnection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttachedNetworkConnection {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of an attached NetworkConnection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AttachedNetworkConnectionProperties>,
}
impl AttachedNetworkConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an attached NetworkConnection."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttachedNetworkConnectionProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The resource ID of the NetworkConnection you want to attach."]
    #[serde(rename = "networkConnectionId")]
    pub network_connection_id: String,
    #[doc = "The geo-location where the NetworkConnection resource specified in 'networkConnectionResourceId' property lives."]
    #[serde(rename = "networkConnectionLocation", default, skip_serializing_if = "Option::is_none")]
    pub network_connection_location: Option<String>,
    #[doc = "Health check status values"]
    #[serde(rename = "healthCheckStatus", default, skip_serializing_if = "Option::is_none")]
    pub health_check_status: Option<HealthCheckStatus>,
    #[doc = "Active Directory join type"]
    #[serde(rename = "domainJoinType", default, skip_serializing_if = "Option::is_none")]
    pub domain_join_type: Option<DomainJoinType>,
}
impl AttachedNetworkConnectionProperties {
    pub fn new(network_connection_id: String) -> Self {
        Self {
            provisioning_state: None,
            network_connection_id,
            network_connection_location: None,
            health_check_status: None,
            domain_join_type: None,
        }
    }
}
#[doc = "Results of the Attached Networks list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AttachedNetworkListResult {
    #[doc = "Current page of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<AttachedNetworkConnection>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AttachedNetworkListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl AttachedNetworkListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A name/value pair to describe a capability."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Capability {
    #[doc = "Name of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Value of the capability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl Capability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a catalog."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Catalog {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a catalog."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CatalogProperties>,
}
impl Catalog {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An individual conflict error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CatalogConflictError {
    #[doc = "The path of the file that has a conflicting name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Name of the conflicting catalog item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl CatalogConflictError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Catalog error details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CatalogErrorDetails {
    #[doc = "An identifier for the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CatalogErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Catalog item sync types enable or disable status. Indicates whether project catalogs are allowed to sync catalog items under projects associated to this dev center."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CatalogItemSyncEnableStatus")]
pub enum CatalogItemSyncEnableStatus {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CatalogItemSyncEnableStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CatalogItemSyncEnableStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CatalogItemSyncEnableStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("CatalogItemSyncEnableStatus", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("CatalogItemSyncEnableStatus", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Indicates catalog item types."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CatalogItemType")]
pub enum CatalogItemType {
    EnvironmentDefinition,
    ImageDefinition,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CatalogItemType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CatalogItemType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CatalogItemType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::EnvironmentDefinition => serializer.serialize_unit_variant("CatalogItemType", 0u32, "EnvironmentDefinition"),
            Self::ImageDefinition => serializer.serialize_unit_variant("CatalogItemType", 1u32, "ImageDefinition"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Results of the catalog list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CatalogListResult {
    #[doc = "Current page of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Catalog>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CatalogListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CatalogListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a catalog."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CatalogProperties {
    #[serde(flatten)]
    pub catalog_update_properties: CatalogUpdateProperties,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The synchronization state of the catalog."]
    #[serde(rename = "syncState", default, skip_serializing_if = "Option::is_none")]
    pub sync_state: Option<catalog_properties::SyncState>,
    #[doc = "Stats of the synchronization."]
    #[serde(rename = "lastSyncStats", default, skip_serializing_if = "Option::is_none")]
    pub last_sync_stats: Option<SyncStats>,
    #[doc = "The connection state of the catalog."]
    #[serde(rename = "connectionState", default, skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<catalog_properties::ConnectionState>,
    #[doc = "When the catalog was last connected."]
    #[serde(rename = "lastConnectionTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_connection_time: Option<::time::OffsetDateTime>,
    #[doc = "When the catalog was last synced."]
    #[serde(rename = "lastSyncTime", default, with = "azure_core::date::rfc3339::option")]
    pub last_sync_time: Option<::time::OffsetDateTime>,
}
impl CatalogProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod catalog_properties {
    use super::*;
    #[doc = "The synchronization state of the catalog."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SyncState")]
    pub enum SyncState {
        Succeeded,
        InProgress,
        Failed,
        Canceled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SyncState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SyncState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SyncState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Succeeded => serializer.serialize_unit_variant("SyncState", 0u32, "Succeeded"),
                Self::InProgress => serializer.serialize_unit_variant("SyncState", 1u32, "InProgress"),
                Self::Failed => serializer.serialize_unit_variant("SyncState", 2u32, "Failed"),
                Self::Canceled => serializer.serialize_unit_variant("SyncState", 3u32, "Canceled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The connection state of the catalog."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ConnectionState")]
    pub enum ConnectionState {
        Connected,
        Disconnected,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ConnectionState {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ConnectionState {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ConnectionState {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Connected => serializer.serialize_unit_variant("ConnectionState", 0u32, "Connected"),
                Self::Disconnected => serializer.serialize_unit_variant("ConnectionState", 1u32, "Disconnected"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "List of validator error details. Populated when changes are made to the resource or its dependent resources that impact the validity of the Catalog resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CatalogResourceValidationErrorDetails {
    #[doc = "Errors associated with resources synchronized from the catalog."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<CatalogErrorDetails>,
}
impl CatalogResourceValidationErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Catalog resource validation status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "CatalogResourceValidationStatus")]
pub enum CatalogResourceValidationStatus {
    Unknown,
    Pending,
    Succeeded,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for CatalogResourceValidationStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for CatalogResourceValidationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for CatalogResourceValidationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("CatalogResourceValidationStatus", 0u32, "Unknown"),
            Self::Pending => serializer.serialize_unit_variant("CatalogResourceValidationStatus", 1u32, "Pending"),
            Self::Succeeded => serializer.serialize_unit_variant("CatalogResourceValidationStatus", 2u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("CatalogResourceValidationStatus", 3u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "An individual synchronization error."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CatalogSyncError {
    #[doc = "The path of the file the error is associated with."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Errors associated with the file."]
    #[serde(
        rename = "errorDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub error_details: Vec<CatalogErrorDetails>,
}
impl CatalogSyncError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The catalog's properties for partial update. Properties not provided in the update request will not be changed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CatalogUpdate {
    #[doc = "Properties of a catalog. These properties can be updated after the resource has been created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CatalogUpdateProperties>,
}
impl CatalogUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a catalog. These properties can be updated after the resource has been created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CatalogUpdateProperties {
    #[doc = "Properties for a Git repository catalog."]
    #[serde(rename = "gitHub", default, skip_serializing_if = "Option::is_none")]
    pub git_hub: Option<GitCatalog>,
    #[doc = "Properties for a Git repository catalog."]
    #[serde(rename = "adoGit", default, skip_serializing_if = "Option::is_none")]
    pub ado_git: Option<GitCatalog>,
    #[doc = "Indicates the type of sync that is configured for the catalog."]
    #[serde(rename = "syncType", default, skip_serializing_if = "Option::is_none")]
    pub sync_type: Option<catalog_update_properties::SyncType>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl CatalogUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod catalog_update_properties {
    use super::*;
    #[doc = "Indicates the type of sync that is configured for the catalog."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "SyncType")]
    pub enum SyncType {
        Manual,
        Scheduled,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for SyncType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for SyncType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for SyncType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Manual => serializer.serialize_unit_variant("SyncType", 0u32, "Manual"),
                Self::Scheduled => serializer.serialize_unit_variant("SyncType", 1u32, "Scheduled"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The check availability request body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityRequest {
    #[doc = "The name of the resource for which availability needs to be checked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl CheckNameAvailabilityRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The check availability result."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResponse {
    #[doc = "Indicates if the resource name is available."]
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[doc = "The reason why the given name is not available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_response::Reason>,
    #[doc = "Detailed reason why the given name is available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_response {
    use super::*;
    #[doc = "The reason why the given name is not available."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Reason")]
    pub enum Reason {
        Invalid,
        AlreadyExists,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Reason {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Reason {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Reason {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Invalid => serializer.serialize_unit_variant("Reason", 0u32, "Invalid"),
                Self::AlreadyExists => serializer.serialize_unit_variant("Reason", 1u32, "AlreadyExists"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The scoped name check availability request body."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckScopedNameAvailabilityRequest {
    #[doc = "The name of the resource for which availability needs to be checked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The resource id to scope the name check."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
impl CheckScopedNameAvailabilityRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a Task to be used in customizing a Dev Box."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomizationTask {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of a Task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomizationTaskProperties>,
}
impl CustomizationTask {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Input for a Task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomizationTaskInput {
    #[doc = "Description of the input."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Type of the input."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<customization_task_input::Type>,
    #[doc = "Whether or not the input is required."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}
impl CustomizationTaskInput {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod customization_task_input {
    use super::*;
    #[doc = "Type of the input."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Type")]
    pub enum Type {
        #[serde(rename = "string")]
        String,
        #[serde(rename = "number")]
        Number,
        #[serde(rename = "boolean")]
        Boolean,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Type {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Type {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Type {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::String => serializer.serialize_unit_variant("Type", 0u32, "string"),
                Self::Number => serializer.serialize_unit_variant("Type", 1u32, "number"),
                Self::Boolean => serializer.serialize_unit_variant("Type", 2u32, "boolean"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "Results of the Task list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomizationTaskListResult {
    #[doc = "Current page of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<CustomizationTask>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for CustomizationTaskListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl CustomizationTaskListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomizationTaskProperties {
    #[doc = "Inputs to the task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<serde_json::Value>,
    #[doc = "The default timeout for the task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[doc = "Catalog resource validation status"]
    #[serde(rename = "validationStatus", default, skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<CatalogResourceValidationStatus>,
}
impl CustomizationTaskProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a definition for a Developer Machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevBoxDefinition {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of a Dev Box definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DevBoxDefinitionProperties>,
}
impl DevBoxDefinition {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Results of the Dev Box definition list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevBoxDefinitionListResult {
    #[doc = "Current page of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DevBoxDefinition>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DevBoxDefinitionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DevBoxDefinitionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Dev Box definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevBoxDefinitionProperties {
    #[serde(flatten)]
    pub dev_box_definition_update_properties: DevBoxDefinitionUpdateProperties,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Image validation status"]
    #[serde(rename = "imageValidationStatus", default, skip_serializing_if = "Option::is_none")]
    pub image_validation_status: Option<ImageValidationStatus>,
    #[doc = "Image validation error details"]
    #[serde(rename = "imageValidationErrorDetails", default, skip_serializing_if = "Option::is_none")]
    pub image_validation_error_details: Option<ImageValidationErrorDetails>,
    #[doc = "Catalog resource validation status"]
    #[serde(rename = "validationStatus", default, skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<CatalogResourceValidationStatus>,
    #[doc = "Image reference information"]
    #[serde(rename = "activeImageReference", default, skip_serializing_if = "Option::is_none")]
    pub active_image_reference: Option<ImageReference>,
}
impl DevBoxDefinitionProperties {
    pub fn new() -> Self {
        Self {
            dev_box_definition_update_properties: DevBoxDefinitionUpdateProperties::default(),
            provisioning_state: None,
            image_validation_status: None,
            image_validation_error_details: None,
            validation_status: None,
            active_image_reference: None,
        }
    }
}
#[doc = "Partial update of a Dev Box definition resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevBoxDefinitionUpdate {
    #[serde(flatten)]
    pub tracked_resource_update: TrackedResourceUpdate,
    #[doc = "Properties of a Dev Box definition. These properties can be updated after the resource has been created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DevBoxDefinitionUpdateProperties>,
}
impl DevBoxDefinitionUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Dev Box definition. These properties can be updated after the resource has been created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevBoxDefinitionUpdateProperties {
    #[doc = "Image reference information"]
    #[serde(rename = "imageReference", default, skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<ImageReference>,
    #[doc = "The resource model definition representing SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "The storage type used for the Operating System disk of Dev Boxes created using this definition."]
    #[serde(rename = "osStorageType", default, skip_serializing_if = "Option::is_none")]
    pub os_storage_type: Option<String>,
    #[doc = "Indicates whether hibernate is enabled/disabled."]
    #[serde(rename = "hibernateSupport", default, skip_serializing_if = "Option::is_none")]
    pub hibernate_support: Option<HibernateSupport>,
}
impl DevBoxDefinitionUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provisioning settings that apply to all Dev Boxes created in this dev center"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevBoxProvisioningSettings {
    #[doc = "Setting to be used when determining whether to install the Azure Monitor Agent service on Dev Boxes that belong to this dev center."]
    #[serde(rename = "installAzureMonitorAgentEnableStatus", default, skip_serializing_if = "Option::is_none")]
    pub install_azure_monitor_agent_enable_status: Option<InstallAzureMonitorAgentEnableStatus>,
}
impl DevBoxProvisioningSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a devcenter resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevCenter {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of the devcenter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DevCenterProperties>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl DevCenter {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            identity: None,
        }
    }
}
#[doc = "Represents a devcenter encryption set resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevCenterEncryptionSet {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of the devcenter encryption set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DevCenterEncryptionSetProperties>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl DevCenterEncryptionSet {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            identity: None,
        }
    }
}
#[doc = "Properties of the devcenter encryption set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevCenterEncryptionSetProperties {
    #[serde(flatten)]
    pub dev_center_encryption_set_update_properties: DevCenterEncryptionSetUpdateProperties,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl DevCenterEncryptionSetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the devcenter encryption set. These properties can be updated after the resource has been created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevCenterEncryptionSetUpdateProperties {
    #[doc = "Devbox disk encryption enable or disable status. Indicates if Devbox disks encryption is enabled or not."]
    #[serde(rename = "devboxDisksEncryptionEnableStatus", default, skip_serializing_if = "Option::is_none")]
    pub devbox_disks_encryption_enable_status: Option<DevboxDisksEncryptionEnableStatus>,
    #[doc = "Key encryption key Url, versioned or non-versioned. Ex: https://contosovault.vault.azure.net/keys/contosokek/562a4bb76b524a1493a6afe8e536ee78 or https://contosovault.vault.azure.net/keys/contosokek."]
    #[serde(rename = "keyEncryptionKeyUrl", default, skip_serializing_if = "Option::is_none")]
    pub key_encryption_key_url: Option<String>,
}
impl DevCenterEncryptionSetUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the list devcenters operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevCenterListResult {
    #[doc = "Current page of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DevCenter>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DevCenterListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl DevCenterListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network settings for the Dev Center."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevCenterNetworkSettings {
    #[doc = "Indicates whether pools in this Dev Center can use Microsoft Hosted Networks. Defaults to Enabled if not set."]
    #[serde(rename = "microsoftHostedNetworkEnableStatus", default, skip_serializing_if = "Option::is_none")]
    pub microsoft_hosted_network_enable_status: Option<MicrosoftHostedNetworkEnableStatus>,
}
impl DevCenterNetworkSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a devcenter plan resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevCenterPlan {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of the devcenter plan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PlanProperties>,
    #[doc = "The resource model definition representing SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl DevCenterPlan {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            sku: None,
        }
    }
}
#[doc = "Represents a devcenter plan member resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevCenterPlanMember {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of the devcenter plan member."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PlanMemberProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}
impl DevCenterPlanMember {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Project catalog settings for project catalogs under a project associated to this dev center."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevCenterProjectCatalogSettings {
    #[doc = "Catalog item sync types enable or disable status. Indicates whether project catalogs are allowed to sync catalog items under projects associated to this dev center."]
    #[serde(rename = "catalogItemSyncEnableStatus", default, skip_serializing_if = "Option::is_none")]
    pub catalog_item_sync_enable_status: Option<CatalogItemSyncEnableStatus>,
}
impl DevCenterProjectCatalogSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the devcenter."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevCenterProperties {
    #[serde(flatten)]
    pub dev_center_update_properties: DevCenterUpdateProperties,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The URI of the resource."]
    #[serde(rename = "devCenterUri", default, skip_serializing_if = "Option::is_none")]
    pub dev_center_uri: Option<DevCenterUri>,
}
impl DevCenterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition representing SKU for DevCenter resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevCenterSku {
    #[serde(flatten)]
    pub sku: Sku,
    #[doc = "The name of the resource type"]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "SKU supported locations."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub locations: Vec<String>,
    #[doc = "Collection of name/value pairs to describe the SKU capabilities."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub capabilities: Vec<Capability>,
}
impl DevCenterSku {
    pub fn new(sku: Sku) -> Self {
        Self {
            sku,
            resource_type: None,
            locations: Vec::new(),
            capabilities: Vec::new(),
        }
    }
}
#[doc = "The devcenter resource for partial updates. Properties not provided in the update request will not be changed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevCenterUpdate {
    #[serde(flatten)]
    pub tracked_resource_update: TrackedResourceUpdate,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "Properties of the devcenter. These properties can be updated after the resource has been created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DevCenterUpdateProperties>,
}
impl DevCenterUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the devcenter. These properties can be updated after the resource has been created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DevCenterUpdateProperties {
    #[doc = "Resource Id of an associated Plan"]
    #[serde(rename = "planId", default, skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[doc = "The display name of the devcenter."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Project catalog settings for project catalogs under a project associated to this dev center."]
    #[serde(rename = "projectCatalogSettings", default, skip_serializing_if = "Option::is_none")]
    pub project_catalog_settings: Option<DevCenterProjectCatalogSettings>,
    #[doc = "Network settings for the Dev Center."]
    #[serde(rename = "networkSettings", default, skip_serializing_if = "Option::is_none")]
    pub network_settings: Option<DevCenterNetworkSettings>,
    #[doc = "Provisioning settings that apply to all Dev Boxes created in this dev center"]
    #[serde(rename = "devBoxProvisioningSettings", default, skip_serializing_if = "Option::is_none")]
    pub dev_box_provisioning_settings: Option<DevBoxProvisioningSettings>,
}
impl DevCenterUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type DevCenterUri = String;
#[doc = "Devbox disk encryption enable or disable status. Indicates if Devbox disks encryption is enabled or not."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DevboxDisksEncryptionEnableStatus")]
pub enum DevboxDisksEncryptionEnableStatus {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DevboxDisksEncryptionEnableStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DevboxDisksEncryptionEnableStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DevboxDisksEncryptionEnableStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("DevboxDisksEncryptionEnableStatus", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("DevboxDisksEncryptionEnableStatus", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Active Directory join type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "DomainJoinType")]
pub enum DomainJoinType {
    #[serde(rename = "HybridAzureADJoin")]
    HybridAzureAdJoin,
    #[serde(rename = "AzureADJoin")]
    AzureAdJoin,
    None,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for DomainJoinType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for DomainJoinType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for DomainJoinType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::HybridAzureAdJoin => serializer.serialize_unit_variant("DomainJoinType", 0u32, "HybridAzureADJoin"),
            Self::AzureAdJoin => serializer.serialize_unit_variant("DomainJoinType", 1u32, "AzureADJoin"),
            Self::None => serializer.serialize_unit_variant("DomainJoinType", 2u32, "None"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Encryption {
    #[doc = "All Customer-managed key encryption properties for the resource."]
    #[serde(rename = "customerManagedKeyEncryption", default, skip_serializing_if = "Option::is_none")]
    pub customer_managed_key_encryption: Option<CustomerManagedKeyEncryption>,
}
impl Encryption {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the list devcenter encryption set operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionSetListResult {
    #[doc = "Current page of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DevCenterEncryptionSet>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EncryptionSetListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl EncryptionSetListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The devcenter encryption set resource for partial updates. Properties not provided in the update request will not be changed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionSetUpdate {
    #[serde(flatten)]
    pub tracked_resource_update: TrackedResourceUpdate,
    #[doc = "Properties of the devcenter encryption set. These properties can be updated after the resource has been created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DevCenterEncryptionSetUpdateProperties>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl EncryptionSetUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A domain name and connection details used to access a dependency."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointDependency {
    #[doc = "The domain name of the dependency. Domain names may be fully qualified or may contain a * wildcard."]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "Human-readable supplemental information about the dependency and when it is applicable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The list of connection details for this endpoint."]
    #[serde(
        rename = "endpointDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub endpoint_details: Vec<EndpointDetail>,
}
impl EndpointDependency {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details about the connection between the Batch service and the endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointDetail {
    #[doc = "The port an endpoint is connected to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}
impl EndpointDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an environment definition catalog item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentDefinition {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of an environment definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EnvironmentDefinitionProperties>,
}
impl EnvironmentDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Results of the environment definition list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentDefinitionListResult {
    #[doc = "Current page of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<EnvironmentDefinition>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EnvironmentDefinitionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl EnvironmentDefinitionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an Environment Definition parameter"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentDefinitionParameter {
    #[doc = "Unique ID of the parameter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Display name of the parameter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Description of the parameter"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The type of data a parameter accepts."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ParameterType>,
    #[doc = "Whether or not this parameter is read-only.  If true, default should have a value."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "Whether or not this parameter is required"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}
impl EnvironmentDefinitionParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an environment definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentDefinitionProperties {
    #[doc = "A short description of the environment definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Input parameters passed to an environment."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parameters: Vec<EnvironmentDefinitionParameter>,
    #[doc = "Path to the Environment Definition entrypoint file."]
    #[serde(rename = "templatePath", default, skip_serializing_if = "Option::is_none")]
    pub template_path: Option<String>,
    #[doc = "Catalog resource validation status"]
    #[serde(rename = "validationStatus", default, skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<CatalogResourceValidationStatus>,
}
impl EnvironmentDefinitionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A role that can be assigned to a user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentRole {
    #[doc = "The common name of the Role Assignment. This is a descriptive name such as 'AcrPush'."]
    #[serde(rename = "roleName", default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[doc = "This is a description of the Role Assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl EnvironmentRole {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an environment type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentType {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of an environment type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EnvironmentTypeProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}
impl EnvironmentType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Indicates whether the environment type is either enabled or disabled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "EnvironmentTypeEnableStatus")]
pub enum EnvironmentTypeEnableStatus {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for EnvironmentTypeEnableStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for EnvironmentTypeEnableStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for EnvironmentTypeEnableStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("EnvironmentTypeEnableStatus", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("EnvironmentTypeEnableStatus", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Result of the environment type list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentTypeListResult {
    #[doc = "Current page of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<EnvironmentType>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EnvironmentTypeListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl EnvironmentTypeListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an environment type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentTypeProperties {
    #[serde(flatten)]
    pub environment_type_update_properties: EnvironmentTypeUpdateProperties,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl EnvironmentTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The environment type for partial update. Properties not provided in the update request will not be changed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentTypeUpdate {
    #[doc = "Properties of an environment type. These properties can be updated after the resource has been created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EnvironmentTypeUpdateProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}
impl EnvironmentTypeUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an environment type. These properties can be updated after the resource has been created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentTypeUpdateProperties {
    #[doc = "The display name of the environment type."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl EnvironmentTypeUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource management error additional info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[doc = "The additional info type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The additional info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl ErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The error details."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(
        rename = "additionalInfo",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl azure_core::Continuable for ErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a gallery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Gallery {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a gallery."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GalleryProperties>,
}
impl Gallery {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Results of the gallery list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GalleryListResult {
    #[doc = "Current page of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Gallery>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for GalleryListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl GalleryListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a gallery."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GalleryProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The resource ID of the backing Azure Compute Gallery."]
    #[serde(rename = "galleryResourceId")]
    pub gallery_resource_id: String,
}
impl GalleryProperties {
    pub fn new(gallery_resource_id: String) -> Self {
        Self {
            provisioning_state: None,
            gallery_resource_id,
        }
    }
}
#[doc = "Properties for a Git repository catalog."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GitCatalog {
    #[doc = "Git URI."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[doc = "Git branch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[doc = "A reference to the Key Vault secret containing a security token to authenticate to a Git repository."]
    #[serde(rename = "secretIdentifier", default, skip_serializing_if = "Option::is_none")]
    pub secret_identifier: Option<String>,
    #[doc = "The folder where the catalog items can be found inside the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
impl GitCatalog {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An individual health check item"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthCheck {
    #[doc = "Health check status values"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<HealthCheckStatus>,
    #[doc = "The display name of this health check item."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Start time of health check item."]
    #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<::time::OffsetDateTime>,
    #[doc = "End time of the health check item."]
    #[serde(rename = "endDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_date_time: Option<::time::OffsetDateTime>,
    #[doc = "The type of error that occurred during this health check."]
    #[serde(rename = "errorType", default, skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
    #[doc = "The recommended action to fix the corresponding error."]
    #[serde(rename = "recommendedAction", default, skip_serializing_if = "Option::is_none")]
    pub recommended_action: Option<String>,
    #[doc = "Additional details about the health check or the recommended action."]
    #[serde(rename = "additionalDetails", default, skip_serializing_if = "Option::is_none")]
    pub additional_details: Option<String>,
}
impl HealthCheck {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Health check status values"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HealthCheckStatus")]
pub enum HealthCheckStatus {
    Unknown,
    Pending,
    Running,
    Passed,
    Warning,
    Failed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HealthCheckStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HealthCheckStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HealthCheckStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("HealthCheckStatus", 0u32, "Unknown"),
            Self::Pending => serializer.serialize_unit_variant("HealthCheckStatus", 1u32, "Pending"),
            Self::Running => serializer.serialize_unit_variant("HealthCheckStatus", 2u32, "Running"),
            Self::Passed => serializer.serialize_unit_variant("HealthCheckStatus", 3u32, "Passed"),
            Self::Warning => serializer.serialize_unit_variant("HealthCheckStatus", 4u32, "Warning"),
            Self::Failed => serializer.serialize_unit_variant("HealthCheckStatus", 5u32, "Failed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Health Check details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthCheckStatusDetails {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Health Check properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HealthCheckStatusDetailsProperties>,
}
impl HealthCheckStatusDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the network health check list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthCheckStatusDetailsListResult {
    #[doc = "Current page of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<HealthCheckStatusDetails>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for HealthCheckStatusDetailsListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl HealthCheckStatusDetailsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Health Check properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthCheckStatusDetailsProperties {
    #[doc = "Start time of last execution of the health checks."]
    #[serde(rename = "startDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_date_time: Option<::time::OffsetDateTime>,
    #[doc = "End time of last execution of the health checks."]
    #[serde(rename = "endDateTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_date_time: Option<::time::OffsetDateTime>,
    #[doc = "Details for each health check item."]
    #[serde(
        rename = "healthChecks",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub health_checks: Vec<HealthCheck>,
}
impl HealthCheckStatusDetailsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Health status indicating whether a pool is available to create Dev Boxes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HealthStatus")]
pub enum HealthStatus {
    Unknown,
    Pending,
    Healthy,
    Warning,
    Unhealthy,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HealthStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HealthStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HealthStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("HealthStatus", 0u32, "Unknown"),
            Self::Pending => serializer.serialize_unit_variant("HealthStatus", 1u32, "Pending"),
            Self::Healthy => serializer.serialize_unit_variant("HealthStatus", 2u32, "Healthy"),
            Self::Warning => serializer.serialize_unit_variant("HealthStatus", 3u32, "Warning"),
            Self::Unhealthy => serializer.serialize_unit_variant("HealthStatus", 4u32, "Unhealthy"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Pool health status detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HealthStatusDetail {
    #[doc = "An identifier for the issue."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the issue, intended to be suitable for display in a user interface"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl HealthStatusDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Indicates whether hibernate is enabled/disabled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "HibernateSupport")]
pub enum HibernateSupport {
    Disabled,
    Enabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for HibernateSupport {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for HibernateSupport {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for HibernateSupport {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("HibernateSupport", 0u32, "Disabled"),
            Self::Enabled => serializer.serialize_unit_variant("HibernateSupport", 1u32, "Enabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents an image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Image {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of an image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ImageProperties>,
}
impl Image {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Image creation error details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageCreationErrorDetails {
    #[doc = "An identifier for the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ImageCreationErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a definition for an Image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageDefinition {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of an Image Definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ImageDefinitionProperties>,
}
impl ImageDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a specific build of an Image Definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageDefinitionBuild {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of an Image Definition Build."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ImageDefinitionBuildProperties>,
}
impl ImageDefinitionBuild {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a specific build of an Image Definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageDefinitionBuildDetails {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Image reference information"]
    #[serde(rename = "imageReference", default, skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<ImageReference>,
    #[doc = "The state of an Image Definition Build."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ImageDefinitionBuildStatus>,
    #[doc = "Start time of the task group."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<::time::OffsetDateTime>,
    #[doc = "End time of the task group."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<::time::OffsetDateTime>,
    #[doc = "Image creation error details"]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ImageCreationErrorDetails>,
    #[doc = "The list of task groups executed during the image definition build."]
    #[serde(
        rename = "taskGroups",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub task_groups: Vec<ImageDefinitionBuildTaskGroup>,
}
impl ImageDefinitionBuildDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Results of the Image Definition Build list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageDefinitionBuildListResult {
    #[doc = "Current page of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ImageDefinitionBuild>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ImageDefinitionBuildListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ImageDefinitionBuildListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an Image Definition Build."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageDefinitionBuildProperties {
    #[doc = "Image reference information"]
    #[serde(rename = "imageReference", default, skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<ImageReference>,
    #[doc = "The state of an Image Definition Build."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ImageDefinitionBuildStatus>,
    #[doc = "Start time of the task group."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<::time::OffsetDateTime>,
    #[doc = "End time of the task group."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<::time::OffsetDateTime>,
    #[doc = "Image creation error details"]
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ImageCreationErrorDetails>,
}
impl ImageDefinitionBuildProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The state of an Image Definition Build."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ImageDefinitionBuildStatus")]
pub enum ImageDefinitionBuildStatus {
    Succeeded,
    Running,
    ValidationFailed,
    Failed,
    Cancelled,
    TimedOut,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ImageDefinitionBuildStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ImageDefinitionBuildStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ImageDefinitionBuildStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Succeeded => serializer.serialize_unit_variant("ImageDefinitionBuildStatus", 0u32, "Succeeded"),
            Self::Running => serializer.serialize_unit_variant("ImageDefinitionBuildStatus", 1u32, "Running"),
            Self::ValidationFailed => serializer.serialize_unit_variant("ImageDefinitionBuildStatus", 2u32, "ValidationFailed"),
            Self::Failed => serializer.serialize_unit_variant("ImageDefinitionBuildStatus", 3u32, "Failed"),
            Self::Cancelled => serializer.serialize_unit_variant("ImageDefinitionBuildStatus", 4u32, "Cancelled"),
            Self::TimedOut => serializer.serialize_unit_variant("ImageDefinitionBuildStatus", 5u32, "TimedOut"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "A task executed during the image definition build."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageDefinitionBuildTask {
    #[doc = "The name of the task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Parameters for the task."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parameters: Vec<serde_json::Value>,
    #[doc = "Display name to help differentiate multiple instances of the same task."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "ID of the task instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Start time of the task."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<::time::OffsetDateTime>,
    #[doc = "End time of the task."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<::time::OffsetDateTime>,
    #[doc = "The state of an Image Definition Build."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ImageDefinitionBuildStatus>,
    #[doc = "The URI for retrieving logs for the task execution."]
    #[serde(rename = "logUri", default, skip_serializing_if = "Option::is_none")]
    pub log_uri: Option<String>,
}
impl ImageDefinitionBuildTask {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A task group executed during the image definition build."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageDefinitionBuildTaskGroup {
    #[doc = "The name of the task group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The state of an Image Definition Build."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ImageDefinitionBuildStatus>,
    #[doc = "Start time of the task group."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<::time::OffsetDateTime>,
    #[doc = "End time of the task group."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<::time::OffsetDateTime>,
    #[doc = "The list of tasks executed during the task group."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub tasks: Vec<ImageDefinitionBuildTask>,
}
impl ImageDefinitionBuildTaskGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Results of the Image Definition list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageDefinitionListResult {
    #[doc = "Current page of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ImageDefinition>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ImageDefinitionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ImageDefinitionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an Image Definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageDefinitionProperties {
    #[doc = "Image reference information"]
    #[serde(rename = "imageReference", default, skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<ImageReference>,
    #[doc = "The URL to the repository file containing the image definition."]
    #[serde(rename = "fileUrl", default, skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
    #[doc = "Details about the latest build."]
    #[serde(rename = "latestBuild", default, skip_serializing_if = "Option::is_none")]
    pub latest_build: Option<LatestImageBuild>,
}
impl ImageDefinitionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Results of the image list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageListResult {
    #[doc = "Current page of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Image>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ImageListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ImageListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an image."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageProperties {
    #[doc = "The description of the image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The publisher of the image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "The name of the image offer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offer: Option<String>,
    #[doc = "The SKU name for the image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[doc = "Properties for a recommended machine configuration."]
    #[serde(rename = "recommendedMachineConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub recommended_machine_configuration: Option<RecommendedMachineConfiguration>,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Indicates whether hibernate is enabled/disabled."]
    #[serde(rename = "hibernateSupport", default, skip_serializing_if = "Option::is_none")]
    pub hibernate_support: Option<HibernateSupport>,
}
impl ImageProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Image reference information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageReference {
    #[doc = "Image ID, or Image version ID. When Image ID is provided, its latest version will be used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The actual version of the image after use. When id references a gallery image latest version, this will indicate the actual version in use."]
    #[serde(rename = "exactVersion", default, skip_serializing_if = "Option::is_none")]
    pub exact_version: Option<String>,
}
impl ImageReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Image validation error details"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageValidationErrorDetails {
    #[doc = "An identifier for the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "A message describing the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ImageValidationErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Image validation status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ImageValidationStatus")]
pub enum ImageValidationStatus {
    Unknown,
    Pending,
    Succeeded,
    Failed,
    TimedOut,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ImageValidationStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ImageValidationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ImageValidationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Unknown => serializer.serialize_unit_variant("ImageValidationStatus", 0u32, "Unknown"),
            Self::Pending => serializer.serialize_unit_variant("ImageValidationStatus", 1u32, "Pending"),
            Self::Succeeded => serializer.serialize_unit_variant("ImageValidationStatus", 2u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ImageValidationStatus", 3u32, "Failed"),
            Self::TimedOut => serializer.serialize_unit_variant("ImageValidationStatus", 4u32, "TimedOut"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents an image version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageVersion {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[doc = "Properties of an image version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ImageVersionProperties>,
}
impl ImageVersion {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Results of the image version list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageVersionListResult {
    #[doc = "Current page of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ImageVersion>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ImageVersionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ImageVersionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of an image version."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ImageVersionProperties {
    #[doc = "The semantic version string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The datetime that the backing image version was published."]
    #[serde(rename = "publishedDate", default, with = "azure_core::date::rfc3339::option")]
    pub published_date: Option<::time::OffsetDateTime>,
    #[doc = "If the version should be excluded from being treated as the latest version."]
    #[serde(rename = "excludeFromLatest", default, skip_serializing_if = "Option::is_none")]
    pub exclude_from_latest: Option<bool>,
    #[doc = "The size of the OS disk image, in GB."]
    #[serde(rename = "osDiskImageSizeInGb", default, skip_serializing_if = "Option::is_none")]
    pub os_disk_image_size_in_gb: Option<i32>,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ImageVersionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Applicable inherited settings for a project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InheritedSettingsForProject {
    #[doc = "Project catalog settings for project catalogs under a project associated to this dev center."]
    #[serde(rename = "projectCatalogSettings", default, skip_serializing_if = "Option::is_none")]
    pub project_catalog_settings: Option<DevCenterProjectCatalogSettings>,
    #[doc = "Network settings for the project."]
    #[serde(rename = "networkSettings", default, skip_serializing_if = "Option::is_none")]
    pub network_settings: Option<ProjectNetworkSettings>,
}
impl InheritedSettingsForProject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Setting to be used when determining whether to install the Azure Monitor Agent service on Dev Boxes that belong to this dev center."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "InstallAzureMonitorAgentEnableStatus")]
pub enum InstallAzureMonitorAgentEnableStatus {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for InstallAzureMonitorAgentEnableStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for InstallAzureMonitorAgentEnableStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for InstallAzureMonitorAgentEnableStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("InstallAzureMonitorAgentEnableStatus", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("InstallAzureMonitorAgentEnableStatus", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Details about the latest build."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LatestImageBuild {
    #[doc = "Identifier of a build."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Start time of the task group."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<::time::OffsetDateTime>,
    #[doc = "End time of the task group."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<::time::OffsetDateTime>,
    #[doc = "The state of an Image Definition Build."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ImageDefinitionBuildStatus>,
}
impl LatestImageBuild {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "License Types"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LicenseType")]
pub enum LicenseType {
    #[serde(rename = "Windows_Client")]
    WindowsClient,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LicenseType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LicenseType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LicenseType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::WindowsClient => serializer.serialize_unit_variant("LicenseType", 0u32, "Windows_Client"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "List of Core Usages."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListUsagesResult {
    #[doc = "The array page of Usages."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Usage>,
    #[doc = "The link to get the next page of Usage result."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ListUsagesResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ListUsagesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Local Administrator enable or disable status. Indicates whether owners of Dev Boxes are added as local administrators on the Dev Box."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LocalAdminStatus")]
pub enum LocalAdminStatus {
    Disabled,
    Enabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LocalAdminStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LocalAdminStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LocalAdminStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("LocalAdminStatus", 0u32, "Disabled"),
            Self::Enabled => serializer.serialize_unit_variant("LocalAdminStatus", 1u32, "Enabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Managed service identity (system assigned and/or user assigned identities)"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagedServiceIdentity {
    #[doc = "The service principal ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The tenant ID of the system assigned identity. This property will only be provided for a system assigned identity."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
    #[serde(rename = "type")]
    pub type_: ManagedServiceIdentityType,
    #[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<UserAssignedIdentities>,
}
impl ManagedServiceIdentity {
    pub fn new(type_: ManagedServiceIdentityType) -> Self {
        Self {
            principal_id: None,
            tenant_id: None,
            type_,
            user_assigned_identities: None,
        }
    }
}
#[doc = "Type of managed service identity (where both SystemAssigned and UserAssigned types are allowed)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ManagedServiceIdentityType")]
pub enum ManagedServiceIdentityType {
    None,
    SystemAssigned,
    UserAssigned,
    #[serde(rename = "SystemAssigned, UserAssigned")]
    SystemAssignedUserAssigned,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ManagedServiceIdentityType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ManagedServiceIdentityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ManagedServiceIdentityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("ManagedServiceIdentityType", 0u32, "None"),
            Self::SystemAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 1u32, "SystemAssigned"),
            Self::UserAssigned => serializer.serialize_unit_variant("ManagedServiceIdentityType", 2u32, "UserAssigned"),
            Self::SystemAssignedUserAssigned => {
                serializer.serialize_unit_variant("ManagedServiceIdentityType", 3u32, "SystemAssigned, UserAssigned")
            }
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Indicates whether pools in this Dev Center can use Microsoft Hosted Networks. Defaults to Enabled if not set."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MicrosoftHostedNetworkEnableStatus")]
pub enum MicrosoftHostedNetworkEnableStatus {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MicrosoftHostedNetworkEnableStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MicrosoftHostedNetworkEnableStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MicrosoftHostedNetworkEnableStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("MicrosoftHostedNetworkEnableStatus", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("MicrosoftHostedNetworkEnableStatus", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Network related settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkConnection {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Network properties"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkProperties>,
}
impl NetworkConnection {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Result of the network connection list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkConnectionListResult {
    #[doc = "Current page of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<NetworkConnection>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for NetworkConnectionListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl NetworkConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The network connection properties for partial update. Properties not provided in the update request will not be changed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkConnectionUpdate {
    #[serde(flatten)]
    pub tracked_resource_update: TrackedResourceUpdate,
    #[doc = "Properties of network connection. These properties can be updated after the resource has been created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NetworkConnectionUpdateProperties>,
}
impl NetworkConnectionUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of network connection. These properties can be updated after the resource has been created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkConnectionUpdateProperties {
    #[doc = "The subnet to attach Virtual Machines to"]
    #[serde(rename = "subnetId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[doc = "Active Directory domain name"]
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[doc = "Active Directory domain Organization Unit (OU)"]
    #[serde(rename = "organizationUnit", default, skip_serializing_if = "Option::is_none")]
    pub organization_unit: Option<String>,
    #[doc = "The username of an Active Directory account (user or service account) that has permissions to create computer objects in Active Directory. Required format: admin@contoso.com."]
    #[serde(rename = "domainUsername", default, skip_serializing_if = "Option::is_none")]
    pub domain_username: Option<String>,
    #[doc = "The password for the account used to join domain"]
    #[serde(rename = "domainPassword", default, skip_serializing_if = "Option::is_none")]
    pub domain_password: Option<String>,
}
impl NetworkConnectionUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkProperties {
    #[serde(flatten)]
    pub network_connection_update_properties: NetworkConnectionUpdateProperties,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Health check status values"]
    #[serde(rename = "healthCheckStatus", default, skip_serializing_if = "Option::is_none")]
    pub health_check_status: Option<HealthCheckStatus>,
    #[doc = "The name for resource group where NICs will be placed."]
    #[serde(rename = "networkingResourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub networking_resource_group_name: Option<String>,
    #[doc = "Active Directory join type"]
    #[serde(rename = "domainJoinType")]
    pub domain_join_type: DomainJoinType,
}
impl NetworkProperties {
    pub fn new(domain_join_type: DomainJoinType) -> Self {
        Self {
            network_connection_update_properties: NetworkConnectionUpdateProperties::default(),
            provisioning_state: None,
            health_check_status: None,
            networking_resource_group_name: None,
            domain_join_type,
        }
    }
}
#[doc = "Details of a REST API operation, returned from the Resource Provider Operations API"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "The name of the operation, as per Resource-Based Access Control (RBAC). Examples: \"Microsoft.Compute/virtualMachines/write\", \"Microsoft.Compute/virtualMachines/capture/action\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Whether the operation applies to data-plane. This is \"true\" for data-plane operations and \"false\" for ARM/control-plane operations."]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[doc = "Localized display information for this particular operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<operation::ActionType>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[doc = "Localized display information for this particular operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The localized friendly form of the resource provider name, e.g. \"Microsoft Monitoring Insights\" or \"Microsoft Compute\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The localized friendly name of the resource type related to this operation. E.g. \"Virtual Machines\" or \"Job Schedule Collections\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The concise, localized friendly name for the operation; suitable for dropdowns. E.g. \"Create or Update Virtual Machine\", \"Restart Virtual Machine\"."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The short, localized friendly description of the operation; suitable for tool tips and detailed views."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The intended executor of the operation; as in Resource Based Access Control (RBAC) and audit logs UX. Default value is \"user,system\""]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Origin")]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "user,system")]
        UserSystem,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Origin {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Origin {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Origin {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("Origin", 0u32, "user"),
                Self::System => serializer.serialize_unit_variant("Origin", 1u32, "system"),
                Self::UserSystem => serializer.serialize_unit_variant("Origin", 2u32, "user,system"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "Enum. Indicates the action type. \"Internal\" refers to actions that are for internal only APIs."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "ActionType")]
    pub enum ActionType {
        Internal,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for ActionType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for ActionType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for ActionType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Internal => serializer.serialize_unit_variant("ActionType", 0u32, "Internal"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "A list of REST API operations supported by an Azure Resource Provider. It contains an URL link to get the next set of results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of operations supported by the resource provider"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Operation>,
    #[doc = "URL to get the next set of operation list results (if there are any)."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The current status of an async operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStatus {
    #[serde(flatten)]
    pub operation_status_result: OperationStatusResult,
    #[doc = "Custom operation properties, populated only for a successful operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl OperationStatus {
    pub fn new(operation_status_result: OperationStatusResult) -> Self {
        Self {
            operation_status_result,
            properties: None,
        }
    }
}
#[doc = "The current status of an async operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationStatusResult {
    #[doc = "Fully qualified ID for the async operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Fully qualified ID of the resource against which the original async operation was started."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Name of the async operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Operation status."]
    pub status: String,
    #[doc = "Percent of the operation that is complete."]
    #[serde(rename = "percentComplete", default, skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<f64>,
    #[doc = "The start time of the operation."]
    #[serde(rename = "startTime", default, with = "azure_core::date::rfc3339::option")]
    pub start_time: Option<::time::OffsetDateTime>,
    #[doc = "The end time of the operation."]
    #[serde(rename = "endTime", default, with = "azure_core::date::rfc3339::option")]
    pub end_time: Option<::time::OffsetDateTime>,
    #[doc = "The operations list."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub operations: Vec<OperationStatusResult>,
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl OperationStatusResult {
    pub fn new(status: String) -> Self {
        Self {
            id: None,
            resource_id: None,
            name: None,
            status,
            percent_complete: None,
            start_time: None,
            end_time: None,
            operations: Vec::new(),
            error: None,
        }
    }
}
#[doc = "A collection of related endpoints from the same service for which the agent requires outbound access."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutboundEnvironmentEndpoint {
    #[doc = "The type of service that the agent connects to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "The endpoints for this service for which the agent requires outbound access."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub endpoints: Vec<EndpointDependency>,
}
impl OutboundEnvironmentEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Values returned by the List operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutboundEnvironmentEndpointCollection {
    #[doc = "The collection of outbound network dependency endpoints returned by the listing operation."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<OutboundEnvironmentEndpoint>,
    #[doc = "The continuation token."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OutboundEnvironmentEndpointCollection {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OutboundEnvironmentEndpointCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The type of data a parameter accepts."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ParameterType")]
pub enum ParameterType {
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "object")]
    Object,
    #[serde(rename = "string")]
    String,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ParameterType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ParameterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ParameterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Array => serializer.serialize_unit_variant("ParameterType", 0u32, "array"),
            Self::Boolean => serializer.serialize_unit_variant("ParameterType", 1u32, "boolean"),
            Self::Integer => serializer.serialize_unit_variant("ParameterType", 2u32, "integer"),
            Self::Number => serializer.serialize_unit_variant("ParameterType", 3u32, "number"),
            Self::Object => serializer.serialize_unit_variant("ParameterType", 4u32, "object"),
            Self::String => serializer.serialize_unit_variant("ParameterType", 5u32, "string"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Result of the list devcenter plans operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanListResult {
    #[doc = "Current page of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DevCenterPlan>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PlanListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PlanListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the devcenter plan member."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanMemberProperties {
    #[doc = "The unique id of the member."]
    #[serde(rename = "memberId", default, skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[doc = "The type of the member (user, group)"]
    #[serde(rename = "memberType", default, skip_serializing_if = "Option::is_none")]
    pub member_type: Option<plan_member_properties::MemberType>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl PlanMemberProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod plan_member_properties {
    use super::*;
    #[doc = "The type of the member (user, group)"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "MemberType")]
    pub enum MemberType {
        User,
        Group,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for MemberType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for MemberType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for MemberType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("MemberType", 0u32, "User"),
                Self::Group => serializer.serialize_unit_variant("MemberType", 1u32, "Group"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The devcenter plan member resource for partial updates. Properties not provided in the update request will not be changed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanMemberUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}
impl PlanMemberUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the list devcenter plan members operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanMembersListResult {
    #[doc = "Current page of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DevCenterPlanMember>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PlanMembersListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PlanMembersListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of the devcenter plan."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanProperties {
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl PlanProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The devcenter plan resource for partial updates. Properties not provided in the update request will not be changed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PlanUpdate {
    #[serde(flatten)]
    pub tracked_resource_update: TrackedResourceUpdate,
    #[doc = "The resource model definition representing SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl PlanUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A pool of Virtual Machines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pool {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of a Pool"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PoolProperties>,
}
impl Pool {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[doc = "Represents a definition for a Developer Machine."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PoolDevBoxDefinition {
    #[doc = "Image reference information"]
    #[serde(rename = "imageReference", default, skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<ImageReference>,
    #[doc = "The resource model definition representing SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[doc = "Image reference information"]
    #[serde(rename = "activeImageReference", default, skip_serializing_if = "Option::is_none")]
    pub active_image_reference: Option<ImageReference>,
}
impl PoolDevBoxDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Indicates if the pool is created from an existing Dev Box Definition or if one is provided directly."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "PoolDevBoxDefinitionType")]
pub enum PoolDevBoxDefinitionType {
    Reference,
    Value,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for PoolDevBoxDefinitionType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for PoolDevBoxDefinitionType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for PoolDevBoxDefinitionType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Reference => serializer.serialize_unit_variant("PoolDevBoxDefinitionType", 0u32, "Reference"),
            Self::Value => serializer.serialize_unit_variant("PoolDevBoxDefinitionType", 1u32, "Value"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Results of the machine pool list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PoolListResult {
    #[doc = "Current page of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Pool>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PoolListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PoolListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Pool"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolProperties {
    #[serde(flatten)]
    pub pool_update_properties: PoolUpdateProperties,
    #[doc = "Health status indicating whether a pool is available to create Dev Boxes."]
    #[serde(rename = "healthStatus", default, skip_serializing_if = "Option::is_none")]
    pub health_status: Option<HealthStatus>,
    #[doc = "Details on the Pool health status to help diagnose issues. This is only populated when the pool status indicates the pool is in a non-healthy state"]
    #[serde(
        rename = "healthStatusDetails",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub health_status_details: Vec<HealthStatusDetail>,
    #[doc = "Indicates the number of provisioned Dev Boxes in this pool."]
    #[serde(rename = "devBoxCount", default, skip_serializing_if = "Option::is_none")]
    pub dev_box_count: Option<i32>,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl PoolProperties {
    pub fn new() -> Self {
        Self {
            pool_update_properties: PoolUpdateProperties::default(),
            health_status: None,
            health_status_details: Vec::new(),
            dev_box_count: None,
            provisioning_state: None,
        }
    }
}
#[doc = "The pool properties for partial update. Properties not provided in the update request will not be changed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PoolUpdate {
    #[serde(flatten)]
    pub tracked_resource_update: TrackedResourceUpdate,
    #[doc = "Properties of a Pool. These properties can be updated after the resource has been created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PoolUpdateProperties>,
}
impl PoolUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a Pool. These properties can be updated after the resource has been created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PoolUpdateProperties {
    #[doc = "Indicates if the pool is created from an existing Dev Box Definition or if one is provided directly."]
    #[serde(rename = "devBoxDefinitionType", default, skip_serializing_if = "Option::is_none")]
    pub dev_box_definition_type: Option<PoolDevBoxDefinitionType>,
    #[doc = "Name of a Dev Box definition in parent Project of this Pool. Will be ignored if devBoxDefinitionType is Value."]
    #[serde(rename = "devBoxDefinitionName", default, skip_serializing_if = "Option::is_none")]
    pub dev_box_definition_name: Option<String>,
    #[doc = "Represents a definition for a Developer Machine."]
    #[serde(rename = "devBoxDefinition", default, skip_serializing_if = "Option::is_none")]
    pub dev_box_definition: Option<PoolDevBoxDefinition>,
    #[doc = "Name of a Network Connection in parent Project of this Pool"]
    #[serde(rename = "networkConnectionName", default, skip_serializing_if = "Option::is_none")]
    pub network_connection_name: Option<String>,
    #[doc = "License Types"]
    #[serde(rename = "licenseType", default, skip_serializing_if = "Option::is_none")]
    pub license_type: Option<LicenseType>,
    #[doc = "Local Administrator enable or disable status. Indicates whether owners of Dev Boxes are added as local administrators on the Dev Box."]
    #[serde(rename = "localAdministrator", default, skip_serializing_if = "Option::is_none")]
    pub local_administrator: Option<LocalAdminStatus>,
    #[doc = "Stop on disconnect configuration settings for Dev Boxes created in this pool."]
    #[serde(rename = "stopOnDisconnect", default, skip_serializing_if = "Option::is_none")]
    pub stop_on_disconnect: Option<StopOnDisconnectConfiguration>,
    #[doc = "SingleSignOn (SSO) enable or disable status. Indicates whether Dev Boxes in the Pool will have SSO enabled or disabled."]
    #[serde(rename = "singleSignOnStatus", default, skip_serializing_if = "Option::is_none")]
    pub single_sign_on_status: Option<SingleSignOnStatus>,
    #[doc = "The display name of the pool."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Indicates a pool uses a Virtual Network managed by Microsoft (Managed), or a customer provided Network (Unmanaged)."]
    #[serde(rename = "virtualNetworkType", default, skip_serializing_if = "Option::is_none")]
    pub virtual_network_type: Option<VirtualNetworkType>,
    #[doc = "The regions of the managed virtual network (required when managedNetworkType is Managed)."]
    #[serde(
        rename = "managedVirtualNetworkRegions",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub managed_virtual_network_regions: Vec<String>,
}
impl PoolUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a project resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[doc = "Properties of a project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProjectProperties>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl Project {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            identity: None,
        }
    }
}
#[doc = "Settings to be used when associating a project with a catalog."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectCatalogSettings {
    #[doc = "Indicates catalog item types that can be synced."]
    #[serde(
        rename = "catalogItemSyncTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub catalog_item_sync_types: Vec<CatalogItemType>,
}
impl ProjectCatalogSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an environment type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectEnvironmentType {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties of a project environment type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProjectEnvironmentTypeProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
    #[doc = "The geo-location for the environment type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl ProjectEnvironmentType {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the project environment type list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectEnvironmentTypeListResult {
    #[doc = "Current page of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ProjectEnvironmentType>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProjectEnvironmentTypeListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ProjectEnvironmentTypeListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a project environment type."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectEnvironmentTypeProperties {
    #[serde(flatten)]
    pub project_environment_type_update_properties: ProjectEnvironmentTypeUpdateProperties,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The number of environments of this type."]
    #[serde(rename = "environmentCount", default, skip_serializing_if = "Option::is_none")]
    pub environment_count: Option<i32>,
}
impl ProjectEnvironmentTypeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The project environment type for partial update. Properties not provided in the update request will not be changed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectEnvironmentTypeUpdate {
    #[doc = "Properties of a project environment type. These properties can be updated after the resource has been created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProjectEnvironmentTypeUpdateProperties>,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl ProjectEnvironmentTypeUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a project environment type. These properties can be updated after the resource has been created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectEnvironmentTypeUpdateProperties {
    #[doc = "Id of a subscription that the environment type will be mapped to. The environment's resources will be deployed into this subscription."]
    #[serde(rename = "deploymentTargetId", default, skip_serializing_if = "Option::is_none")]
    pub deployment_target_id: Option<String>,
    #[doc = "The display name of the project environment type."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Indicates whether the environment type is either enabled or disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EnvironmentTypeEnableStatus>,
    #[doc = "The role definition assigned to the environment creator on backing resources."]
    #[serde(rename = "creatorRoleAssignment", default, skip_serializing_if = "Option::is_none")]
    pub creator_role_assignment: Option<project_environment_type_update_properties::CreatorRoleAssignment>,
    #[doc = "Role Assignments created on environment backing resources. This is a mapping from a user object ID to an object of role definition IDs."]
    #[serde(rename = "userRoleAssignments", default, skip_serializing_if = "Option::is_none")]
    pub user_role_assignments: Option<serde_json::Value>,
}
impl ProjectEnvironmentTypeUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod project_environment_type_update_properties {
    use super::*;
    #[doc = "The role definition assigned to the environment creator on backing resources."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct CreatorRoleAssignment {
        #[doc = "A map of roles to assign to the environment creator."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub roles: Option<serde_json::Value>,
    }
    impl CreatorRoleAssignment {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "Results of the project list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectListResult {
    #[doc = "Current page of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Project>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ProjectListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ProjectListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Network settings for the project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectNetworkSettings {
    #[doc = "Indicates whether pools in this Dev Center can use Microsoft Hosted Networks. Defaults to Enabled if not set."]
    #[serde(rename = "microsoftHostedNetworkEnableStatus", default, skip_serializing_if = "Option::is_none")]
    pub microsoft_hosted_network_enable_status: Option<MicrosoftHostedNetworkEnableStatus>,
}
impl ProjectNetworkSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a project."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectProperties {
    #[serde(flatten)]
    pub project_update_properties: ProjectUpdateProperties,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "The URI of the resource."]
    #[serde(rename = "devCenterUri", default, skip_serializing_if = "Option::is_none")]
    pub dev_center_uri: Option<DevCenterUri>,
}
impl ProjectProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The project properties for partial update. Properties not provided in the update request will not be changed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectUpdate {
    #[serde(flatten)]
    pub tracked_resource_update: TrackedResourceUpdate,
    #[doc = "Properties of a project. These properties can be updated after the resource has been created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProjectUpdateProperties>,
    #[doc = "Managed service identity (system assigned and/or user assigned identities)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ManagedServiceIdentity>,
}
impl ProjectUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties of a project. These properties can be updated after the resource has been created."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectUpdateProperties {
    #[doc = "Resource Id of an associated DevCenter"]
    #[serde(rename = "devCenterId", default, skip_serializing_if = "Option::is_none")]
    pub dev_center_id: Option<String>,
    #[doc = "Description of the project."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "When specified, limits the maximum number of Dev Boxes a single user can create across all pools in the project. This will have no effect on existing Dev Boxes when reduced."]
    #[serde(rename = "maxDevBoxesPerUser", default, skip_serializing_if = "Option::is_none")]
    pub max_dev_boxes_per_user: Option<i32>,
    #[doc = "The display name of the project."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Settings to be used when associating a project with a catalog."]
    #[serde(rename = "catalogSettings", default, skip_serializing_if = "Option::is_none")]
    pub catalog_settings: Option<ProjectCatalogSettings>,
}
impl ProjectUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provisioning state of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    NotSpecified,
    Accepted,
    Running,
    Creating,
    Created,
    Updating,
    Updated,
    Deleting,
    Deleted,
    Succeeded,
    Failed,
    Canceled,
    MovingResources,
    TransientFailure,
    RolloutInProgress,
    StorageProvisioningFailed,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ProvisioningState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ProvisioningState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ProvisioningState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::NotSpecified => serializer.serialize_unit_variant("ProvisioningState", 0u32, "NotSpecified"),
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Accepted"),
            Self::Running => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Running"),
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Creating"),
            Self::Created => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Created"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Updating"),
            Self::Updated => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Updated"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Deleting"),
            Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 8u32, "Deleted"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 9u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 10u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 11u32, "Canceled"),
            Self::MovingResources => serializer.serialize_unit_variant("ProvisioningState", 12u32, "MovingResources"),
            Self::TransientFailure => serializer.serialize_unit_variant("ProvisioningState", 13u32, "TransientFailure"),
            Self::RolloutInProgress => serializer.serialize_unit_variant("ProvisioningState", 14u32, "RolloutInProgress"),
            Self::StorageProvisioningFailed => serializer.serialize_unit_variant("ProvisioningState", 15u32, "StorageProvisioningFailed"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The resource model definition for a Azure Resource Manager proxy resource. It will not have tags and a location"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for a recommended machine configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RecommendedMachineConfiguration {
    #[doc = "Properties for a range of values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<ResourceRange>,
    #[doc = "Properties for a range of values."]
    #[serde(rename = "vCPUs", default, skip_serializing_if = "Option::is_none")]
    pub v_cp_us: Option<ResourceRange>,
}
impl RecommendedMachineConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common fields that are returned in the response for all Azure Resource Manager resources"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified resource ID for the resource. E.g. \"/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource. E.g. \"Microsoft.Compute/virtualMachines\" or \"Microsoft.Storage/storageAccounts\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for a range of values."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceRange {
    #[doc = "Minimum value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
    #[doc = "Maximum value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
}
impl ResourceRange {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a Schedule to execute a task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Schedule {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "The Schedule properties defining when and what to execute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScheduleProperties>,
}
impl Schedule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Schedule enable or disable status. Indicates whether the schedule applied to is either enabled or disabled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScheduleEnableStatus")]
pub enum ScheduleEnableStatus {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScheduleEnableStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScheduleEnableStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScheduleEnableStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("ScheduleEnableStatus", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("ScheduleEnableStatus", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Result of the schedule list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleListResult {
    #[doc = "Current page of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<Schedule>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ScheduleListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ScheduleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Schedule properties defining when and what to execute."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleProperties {
    #[serde(flatten)]
    pub schedule_update_properties: ScheduleUpdateProperties,
    #[doc = "Provisioning state of the resource."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
}
impl ScheduleProperties {
    pub fn new() -> Self {
        Self {
            schedule_update_properties: ScheduleUpdateProperties::default(),
            provisioning_state: None,
        }
    }
}
#[doc = "The schedule properties for partial update. Properties not provided in the update request will not be changed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleUpdate {
    #[doc = "Updatable properties of a Schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScheduleUpdateProperties>,
}
impl ScheduleUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Updatable properties of a Schedule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleUpdateProperties {
    #[serde(flatten)]
    pub tracked_resource_update: TrackedResourceUpdate,
    #[doc = "The supported types for a scheduled task."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ScheduledType>,
    #[doc = "The frequency of task execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<ScheduledFrequency>,
    #[doc = "The target time to trigger the action. The format is HH:MM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[doc = "The IANA timezone id at which the schedule should execute."]
    #[serde(rename = "timeZone", default, skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[doc = "Schedule enable or disable status. Indicates whether the schedule applied to is either enabled or disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<ScheduleEnableStatus>,
}
impl ScheduleUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The frequency of task execution."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScheduledFrequency")]
pub enum ScheduledFrequency {
    Daily,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScheduledFrequency {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScheduledFrequency {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScheduledFrequency {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Daily => serializer.serialize_unit_variant("ScheduledFrequency", 0u32, "Daily"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The supported types for a scheduled task."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ScheduledType")]
pub enum ScheduledType {
    StopDevBox,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ScheduledType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ScheduledType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ScheduledType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::StopDevBox => serializer.serialize_unit_variant("ScheduledType", 0u32, "StopDevBox"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "SingleSignOn (SSO) enable or disable status. Indicates whether Dev Boxes in the Pool will have SSO enabled or disabled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SingleSignOnStatus")]
pub enum SingleSignOnStatus {
    Disabled,
    Enabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SingleSignOnStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SingleSignOnStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SingleSignOnStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Disabled => serializer.serialize_unit_variant("SingleSignOnStatus", 0u32, "Disabled"),
            Self::Enabled => serializer.serialize_unit_variant("SingleSignOnStatus", 1u32, "Enabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "The resource model definition representing SKU"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[doc = "The name of the SKU. E.g. P3. It is typically a letter+number code"]
    pub name: String,
    #[doc = "This field is required to be implemented by the Resource Provider if the service has more than one tier, but is not required on a PUT."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<SkuTier>,
    #[doc = "The SKU size. When the name field is the combination of tier and some other value, this would be the standalone code. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[doc = "If the service has different generations of hardware, for the same SKU, then that can be captured here."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[doc = "If the SKU supports scale out/in then the capacity integer should be included. If scale out/in is not possible for the resource this may be omitted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl Sku {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tier: None,
            size: None,
            family: None,
            capacity: None,
        }
    }
}
#[doc = "Results of the Microsoft.DevCenter SKU list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuListResult {
    #[doc = "Current page of results."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<DevCenterSku>,
    #[doc = "URL to get the next set of results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SkuListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl SkuListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "This field is required to be implemented by the Resource Provider if the service has more than one tier, but is not required on a PUT."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SkuTier {
    Free,
    Basic,
    Standard,
    Premium,
}
#[doc = "Stop on disconnect configuration settings for Dev Boxes created in this pool."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StopOnDisconnectConfiguration {
    #[doc = "Stop on disconnect enable or disable status. Indicates whether stop on disconnect to is either enabled or disabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StopOnDisconnectEnableStatus>,
    #[doc = "The specified time in minutes to wait before stopping a Dev Box once disconnect is detected."]
    #[serde(rename = "gracePeriodMinutes", default, skip_serializing_if = "Option::is_none")]
    pub grace_period_minutes: Option<i32>,
}
impl StopOnDisconnectConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Stop on disconnect enable or disable status. Indicates whether stop on disconnect to is either enabled or disabled."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "StopOnDisconnectEnableStatus")]
pub enum StopOnDisconnectEnableStatus {
    Enabled,
    Disabled,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for StopOnDisconnectEnableStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for StopOnDisconnectEnableStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for StopOnDisconnectEnableStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Enabled => serializer.serialize_unit_variant("StopOnDisconnectEnableStatus", 0u32, "Enabled"),
            Self::Disabled => serializer.serialize_unit_variant("StopOnDisconnectEnableStatus", 1u32, "Disabled"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Synchronization error details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncErrorDetails {
    #[doc = "Catalog error details"]
    #[serde(rename = "operationError", default, skip_serializing_if = "Option::is_none")]
    pub operation_error: Option<CatalogErrorDetails>,
    #[doc = "Catalog items that have conflicting names."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub conflicts: Vec<CatalogConflictError>,
    #[doc = "Errors that occured during synchronization."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub errors: Vec<CatalogSyncError>,
}
impl SyncErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Stats of the synchronization."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncStats {
    #[doc = "Count of catalog items added during synchronization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub added: Option<i32>,
    #[doc = "Count of catalog items updated during synchronization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<i32>,
    #[doc = "Count of catalog items that were unchanged during synchronization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unchanged: Option<i32>,
    #[doc = "Count of catalog items removed during synchronization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removed: Option<i32>,
    #[doc = "Count of catalog items that had validation errors during synchronization."]
    #[serde(rename = "validationErrors", default, skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<i32>,
    #[doc = "Count of synchronization errors that occured during synchronization."]
    #[serde(rename = "synchronizationErrors", default, skip_serializing_if = "Option::is_none")]
    pub synchronization_errors: Option<i32>,
    #[doc = "Indicates catalog item types that were synced."]
    #[serde(
        rename = "syncedCatalogItemTypes",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub synced_catalog_item_types: Vec<CatalogItemType>,
}
impl SyncStats {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource tags."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tags {}
impl Tags {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource model definition for an Azure Resource Manager tracked top level resource which has 'tags' and a 'location'"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives"]
    pub location: String,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            tags: None,
            location,
        }
    }
}
#[doc = "Base tracked resource type for PATCH updates"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackedResourceUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[doc = "The geo-location where the resource lives"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl TrackedResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The core usage details."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Usage {
    #[doc = "The current usage."]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[doc = "The limit integer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[doc = "The unit details."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<usage::Unit>,
    #[doc = "The Usage Names."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<UsageName>,
    #[doc = "The fully qualified arm resource id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl Usage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod usage {
    use super::*;
    #[doc = "The unit details."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "Unit")]
    pub enum Unit {
        Count,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for Unit {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for Unit {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for Unit {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::Count => serializer.serialize_unit_variant("Unit", 0u32, "Count"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
#[doc = "The Usage Names."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageName {
    #[doc = "The localized name of the resource."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl UsageName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The set of user assigned identities associated with the resource. The userAssignedIdentities dictionary keys will be ARM resource ids in the form: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.ManagedIdentity/userAssignedIdentities/{identityName}. The dictionary values can be empty objects ({}) in requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentities {}
impl UserAssignedIdentities {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "User assigned identity properties"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[doc = "The principal ID of the assigned identity."]
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[doc = "The client ID of the assigned identity."]
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Mapping of user object ID to role assignments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserRoleAssignment {
    #[doc = "A map of roles to assign to the parent user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<serde_json::Value>,
}
impl UserRoleAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Indicates a pool uses a Virtual Network managed by Microsoft (Managed), or a customer provided Network (Unmanaged)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "VirtualNetworkType")]
pub enum VirtualNetworkType {
    Managed,
    Unmanaged,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for VirtualNetworkType {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for VirtualNetworkType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for VirtualNetworkType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Managed => serializer.serialize_unit_variant("VirtualNetworkType", 0u32, "Managed"),
            Self::Unmanaged => serializer.serialize_unit_variant("VirtualNetworkType", 1u32, "Unmanaged"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "All Customer-managed key encryption properties for the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomerManagedKeyEncryption {
    #[doc = "All identity configuration for Customer-managed key settings defining which identity should be used to auth to Key Vault."]
    #[serde(rename = "keyEncryptionKeyIdentity", default, skip_serializing_if = "Option::is_none")]
    pub key_encryption_key_identity: Option<customer_managed_key_encryption::KeyEncryptionKeyIdentity>,
    #[doc = "key encryption key Url, versioned or non-versioned. Ex: https://contosovault.vault.azure.net/keys/contosokek/562a4bb76b524a1493a6afe8e536ee78 or https://contosovault.vault.azure.net/keys/contosokek."]
    #[serde(rename = "keyEncryptionKeyUrl", default, skip_serializing_if = "Option::is_none")]
    pub key_encryption_key_url: Option<String>,
}
impl CustomerManagedKeyEncryption {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod customer_managed_key_encryption {
    use super::*;
    #[doc = "All identity configuration for Customer-managed key settings defining which identity should be used to auth to Key Vault."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct KeyEncryptionKeyIdentity {
        #[doc = "Values can be systemAssignedIdentity or userAssignedIdentity"]
        #[serde(rename = "identityType", default, skip_serializing_if = "Option::is_none")]
        pub identity_type: Option<key_encryption_key_identity::IdentityType>,
        #[doc = "user assigned identity to use for accessing key encryption key Url. Ex: /subscriptions/fa5fc227-a624-475e-b696-cdd604c735bc/resourceGroups/<resource group>/providers/Microsoft.ManagedIdentity/userAssignedIdentities/myId. Mutually exclusive with identityType systemAssignedIdentity and delegatedResourceIdentity."]
        #[serde(rename = "userAssignedIdentityResourceId", default, skip_serializing_if = "Option::is_none")]
        pub user_assigned_identity_resource_id: Option<String>,
        #[doc = "delegated identity to use for accessing key encryption key Url. Ex: /subscriptions/fa5fc227-a624-475e-b696-cdd604c735bc/resourceGroups/<resource group>/providers/Microsoft.ManagedIdentity/userAssignedIdentities/myId. Mutually exclusive with identityType systemAssignedIdentity and userAssignedIdentity - internal use only."]
        #[serde(rename = "delegatedIdentityClientId", default, skip_serializing_if = "Option::is_none")]
        pub delegated_identity_client_id: Option<String>,
    }
    impl KeyEncryptionKeyIdentity {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod key_encryption_key_identity {
        use super::*;
        #[doc = "Values can be systemAssignedIdentity or userAssignedIdentity"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #[serde(remote = "IdentityType")]
        pub enum IdentityType {
            #[serde(rename = "systemAssignedIdentity")]
            SystemAssignedIdentity,
            #[serde(rename = "userAssignedIdentity")]
            UserAssignedIdentity,
            #[serde(rename = "delegatedResourceIdentity")]
            DelegatedResourceIdentity,
            #[serde(skip_deserializing)]
            UnknownValue(String),
        }
        impl FromStr for IdentityType {
            type Err = value::Error;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                Self::deserialize(s.into_deserializer())
            }
        }
        impl<'de> Deserialize<'de> for IdentityType {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
                Ok(deserialized)
            }
        }
        impl Serialize for IdentityType {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                match self {
                    Self::SystemAssignedIdentity => serializer.serialize_unit_variant("IdentityType", 0u32, "systemAssignedIdentity"),
                    Self::UserAssignedIdentity => serializer.serialize_unit_variant("IdentityType", 1u32, "userAssignedIdentity"),
                    Self::DelegatedResourceIdentity => serializer.serialize_unit_variant("IdentityType", 2u32, "delegatedResourceIdentity"),
                    Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                }
            }
        }
    }
}
#[doc = "Metadata pertaining to creation and last modification of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, with = "azure_core::date::rfc3339::option")]
    pub created_at: Option<::time::OffsetDateTime>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, with = "azure_core::date::rfc3339::option")]
    pub last_modified_at: Option<::time::OffsetDateTime>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_data {
    use super::*;
    #[doc = "The type of identity that created the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "CreatedByType")]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for CreatedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for CreatedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for CreatedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("CreatedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("CreatedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("CreatedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("CreatedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
    #[doc = "The type of identity that last modified the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(remote = "LastModifiedByType")]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
        #[serde(skip_deserializing)]
        UnknownValue(String),
    }
    impl FromStr for LastModifiedByType {
        type Err = value::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::deserialize(s.into_deserializer())
        }
    }
    impl<'de> Deserialize<'de> for LastModifiedByType {
        fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
            Ok(deserialized)
        }
    }
    impl Serialize for LastModifiedByType {
        fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                Self::User => serializer.serialize_unit_variant("LastModifiedByType", 0u32, "User"),
                Self::Application => serializer.serialize_unit_variant("LastModifiedByType", 1u32, "Application"),
                Self::ManagedIdentity => serializer.serialize_unit_variant("LastModifiedByType", 2u32, "ManagedIdentity"),
                Self::Key => serializer.serialize_unit_variant("LastModifiedByType", 3u32, "Key"),
                Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
            }
        }
    }
}
