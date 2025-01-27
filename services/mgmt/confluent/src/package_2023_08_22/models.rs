#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "Invite User Account model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessInviteUserAccountModel {
    #[doc = "Id of the organization"]
    #[serde(rename = "organizationId", default, skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    #[doc = "Email of the logged in user"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Upn of the logged in user"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upn: Option<String>,
    #[doc = "Details of the user being invited"]
    #[serde(rename = "invitedUserDetails", default, skip_serializing_if = "Option::is_none")]
    pub invited_user_details: Option<AccessInvitedUserDetails>,
}
impl AccessInviteUserAccountModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Details of the user being invited"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessInvitedUserDetails {
    #[doc = "UPN/Email of the user who is being invited"]
    #[serde(rename = "invitedEmail", default, skip_serializing_if = "Option::is_none")]
    pub invited_email: Option<String>,
    #[doc = "Auth type of the user"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
}
impl AccessInvitedUserDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List cluster success response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessListClusterSuccessResponse {
    #[doc = "Type of response"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Metadata of the list"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ConfluentListMetadata>,
    #[doc = "Data of the invitations list"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ClusterList>,
}
impl AccessListClusterSuccessResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List environments success response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessListEnvironmentsSuccessResponse {
    #[doc = "Type of response"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Metadata of the list"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ConfluentListMetadata>,
    #[doc = "Data of the invitations list"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<EnvironmentsList>,
}
impl AccessListEnvironmentsSuccessResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List invitations success response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessListInvitationsSuccessResponse {
    #[doc = "Type of response"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Metadata of the list"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ConfluentListMetadata>,
    #[doc = "Data of the invitations list"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<InvitationsList>,
}
impl AccessListInvitationsSuccessResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List cluster success response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessListRoleBindingsSuccessResponse {
    #[doc = "Type of response"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Metadata of the list"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ConfluentListMetadata>,
    #[doc = "Data of the invitations list"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<RoleBindingList>,
}
impl AccessListRoleBindingsSuccessResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List service accounts success response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessListServiceAccountsSuccessResponse {
    #[doc = "Type of response"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Metadata of the list"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ConfluentListMetadata>,
    #[doc = "Data of the service accounts list"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ServiceAccountsList>,
}
impl AccessListServiceAccountsSuccessResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List users success response"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessListUsersSuccessResponse {
    #[doc = "Type of response"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Metadata of the list"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ConfluentListMetadata>,
    #[doc = "Data of the users list"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<UsersList>,
}
impl AccessListUsersSuccessResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The network associated with this object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterByokEntity {
    #[doc = "ID of the referred resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "API URL for accessing or modifying the referred object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<String>,
    #[doc = "CRN reference to the referred resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}
impl ClusterByokEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The configuration of the Kafka cluster"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterConfigEntity {
    #[doc = "The lifecycle phase of the cluster"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}
impl ClusterConfigEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The environment to which cluster belongs"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterEnvironmentEntity {
    #[doc = "ID of the referred resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Environment of the referred resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[doc = "API URL for accessing or modifying the referred object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<String>,
    #[doc = "CRN reference to the referred resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}
impl ClusterEnvironmentEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ClusterList = Vec<ClusterRecord>;
#[doc = "The network associated with this object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterNetworkEntity {
    #[doc = "ID of the referred resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Environment of the referred resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[doc = "API URL for accessing or modifying the referred object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<String>,
    #[doc = "CRN reference to the referred resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}
impl ClusterNetworkEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Record of the environment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterRecord {
    #[doc = "Type of environment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Id of the environment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Metadata of the data record"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataEntity>,
    #[doc = "Display name of the user"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Spec of the cluster record"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ClusterSpecEntity>,
    #[doc = "Status of the cluster record"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ClusterStatusEntity>,
}
impl ClusterRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Spec of the cluster record"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterSpecEntity {
    #[doc = "The name of the cluster"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The availability zone configuration of the cluster"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability: Option<String>,
    #[doc = "The cloud service provider "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cloud: Option<String>,
    #[doc = "type of zone availability"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
    #[doc = "The cloud service provider region"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "The bootstrap endpoint used by Kafka clients to connect to the cluster"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kafka_bootstrap_endpoint: Option<String>,
    #[doc = "The cluster HTTP request URL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_endpoint: Option<String>,
    #[doc = "The Kafka API cluster endpoint"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_endpoint: Option<String>,
    #[doc = "The configuration of the Kafka cluster"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<ClusterConfigEntity>,
    #[doc = "The environment to which cluster belongs"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<ClusterEnvironmentEntity>,
    #[doc = "The network associated with this object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<ClusterNetworkEntity>,
    #[doc = "The network associated with this object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub byok: Option<ClusterByokEntity>,
}
impl ClusterSpecEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Status of the cluster record"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterStatusEntity {
    #[doc = "The lifecycle phase of the cluster"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    #[doc = "The number of Confluent Kafka Units "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cku: Option<i32>,
}
impl ClusterStatusEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Terms properties for Marketplace and Confluent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfluentAgreementProperties {
    #[doc = "Publisher identifier string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[doc = "Product identifier string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[doc = "Plan identifier string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,
    #[doc = "Link to HTML with Microsoft and Publisher terms."]
    #[serde(rename = "licenseTextLink", default, skip_serializing_if = "Option::is_none")]
    pub license_text_link: Option<String>,
    #[doc = "Link to the privacy policy of the publisher."]
    #[serde(rename = "privacyPolicyLink", default, skip_serializing_if = "Option::is_none")]
    pub privacy_policy_link: Option<String>,
    #[doc = "Date and time in UTC of when the terms were accepted. This is empty if Accepted is false."]
    #[serde(rename = "retrieveDatetime", default, with = "azure_core::date::rfc3339::option")]
    pub retrieve_datetime: Option<::time::OffsetDateTime>,
    #[doc = "Terms signature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[doc = "If any version of the terms have been accepted, otherwise false."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepted: Option<bool>,
}
impl ConfluentAgreementProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Agreement Terms definition"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfluentAgreementResource {
    #[doc = "The ARM id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the agreement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the agreement."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Terms properties for Marketplace and Confluent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConfluentAgreementProperties>,
}
impl ConfluentAgreementResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfluentAgreementResourceListResponse {
    #[doc = "Results of a list operation."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<ConfluentAgreementResource>,
    #[doc = "Link to the next set of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ConfluentAgreementResourceListResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl ConfluentAgreementResourceListResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata of the list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfluentListMetadata {
    #[doc = "First page of the list"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first: Option<String>,
    #[doc = "Last page of the list"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,
    #[doc = "Previous page of the list"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[doc = "Next page of the list"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[doc = "Total size of the list"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_size: Option<i32>,
}
impl ConfluentListMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Record of the environment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentRecord {
    #[doc = "Type of environment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Id of the environment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Metadata of the data record"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataEntity>,
    #[doc = "Display name of the user"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl EnvironmentRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type EnvironmentsList = Vec<EnvironmentRecord>;
#[doc = "Response body of Error"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseBody {
    #[doc = "Error code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Error target"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "Error detail"]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub details: Vec<ErrorResponseBody>,
}
impl ErrorResponseBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Record of the invitation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InvitationRecord {
    #[doc = "Type of account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Id of the invitation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Metadata of the data record"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataEntity>,
    #[doc = "Email of the user"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Auth type of the user"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[doc = "Status of the invitation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Accepted date time of the invitation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<String>,
    #[doc = "Expiration date time of the invitation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}
impl InvitationRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type InvitationsList = Vec<InvitationRecord>;
#[doc = "Link an existing Confluent organization"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkOrganization {
    #[doc = "User auth token"]
    pub token: String,
}
impl LinkOrganization {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}
#[doc = "List Access Request Model"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListAccessRequestModel {
    #[doc = "Search filters for the request"]
    #[serde(rename = "searchFilters", default, skip_serializing_if = "Option::is_none")]
    pub search_filters: Option<serde_json::Value>,
}
impl ListAccessRequestModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata of the data record"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataEntity {
    #[doc = "Self lookup url"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[doc = "Resource name of the record"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[doc = "Created Date Time"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[doc = "Updated Date time"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[doc = "Deleted Date time"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
}
impl MetadataEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Confluent Offer detail"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OfferDetail {
    #[doc = "Publisher Id"]
    #[serde(rename = "publisherId")]
    pub publisher_id: String,
    #[doc = "Offer Id"]
    pub id: String,
    #[doc = "Offer Plan Id"]
    #[serde(rename = "planId")]
    pub plan_id: String,
    #[doc = "Offer Plan Name"]
    #[serde(rename = "planName")]
    pub plan_name: String,
    #[doc = "Offer Plan Term unit"]
    #[serde(rename = "termUnit")]
    pub term_unit: String,
    #[doc = "Offer Plan Term Id"]
    #[serde(rename = "termId", default, skip_serializing_if = "Option::is_none")]
    pub term_id: Option<String>,
    #[doc = "Private Offer Id"]
    #[serde(rename = "privateOfferId", default, skip_serializing_if = "Option::is_none")]
    pub private_offer_id: Option<String>,
    #[doc = "Array of Private Offer Ids"]
    #[serde(
        rename = "privateOfferIds",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub private_offer_ids: Vec<String>,
    #[doc = "SaaS Offer Status for confluent RP"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SaaSOfferStatus>,
}
impl OfferDetail {
    pub fn new(publisher_id: String, id: String, plan_id: String, plan_name: String, term_unit: String) -> Self {
        Self {
            publisher_id,
            id,
            plan_id,
            plan_name,
            term_unit,
            term_id: None,
            private_offer_id: None,
            private_offer_ids: Vec::new(),
            status: None,
        }
    }
}
#[doc = "The object that represents the operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[doc = "Service provider: Microsoft.Confluent"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Type on which the operation is performed, e.g., 'clusters'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Operation type, e.g., read, write, delete, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the operation, e.g., 'Write confluent'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of GET request to list Confluent operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "List of Confluent operations supported by the Microsoft.Confluent provider."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<OperationResult>,
    #[doc = "URL to get the next set of operation list results if there are any."]
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
#[doc = "An Confluent REST API operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResult {
    #[doc = "Operation name: {provider}/{resource}/{operation}"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The object that represents the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[doc = "Indicates whether the operation is a data action"]
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
}
impl OperationResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Organization resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationResource {
    #[doc = "The ARM id of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[doc = "Organization resource property"]
    pub properties: OrganizationResourceProperties,
    #[doc = "Organization resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Location of Organization resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl OrganizationResource {
    pub fn new(properties: OrganizationResourceProperties) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            system_data: None,
            properties,
            tags: None,
            location: None,
        }
    }
}
#[doc = "The response of a list operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrganizationResourceListResult {
    #[doc = "Result of a list operation."]
    #[serde(
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub value: Vec<OrganizationResource>,
    #[doc = "Link to the next set of results, if any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OrganizationResourceListResult {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl OrganizationResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Organization resource property"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationResourceProperties {
    #[doc = "The creation time of the resource."]
    #[serde(rename = "createdTime", default, with = "azure_core::date::rfc3339::option")]
    pub created_time: Option<::time::OffsetDateTime>,
    #[doc = "Provision states for confluent RP"]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ProvisioningState>,
    #[doc = "Id of the Confluent organization."]
    #[serde(rename = "organizationId", default, skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    #[doc = "SSO url for the Confluent organization."]
    #[serde(rename = "ssoUrl", default, skip_serializing_if = "Option::is_none")]
    pub sso_url: Option<String>,
    #[doc = "Confluent Offer detail"]
    #[serde(rename = "offerDetail")]
    pub offer_detail: OfferDetail,
    #[doc = "Subscriber detail"]
    #[serde(rename = "userDetail")]
    pub user_detail: UserDetail,
    #[doc = "Link an existing Confluent organization"]
    #[serde(rename = "linkOrganization", default, skip_serializing_if = "Option::is_none")]
    pub link_organization: Option<LinkOrganization>,
}
impl OrganizationResourceProperties {
    pub fn new(offer_detail: OfferDetail, user_detail: UserDetail) -> Self {
        Self {
            created_time: None,
            provisioning_state: None,
            organization_id: None,
            sso_url: None,
            offer_detail,
            user_detail,
            link_organization: None,
        }
    }
}
#[doc = "Organization Resource update"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OrganizationResourceUpdate {
    #[doc = "ARM resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl OrganizationResourceUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Provision states for confluent RP"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ProvisioningState")]
pub enum ProvisioningState {
    Accepted,
    Creating,
    Updating,
    Deleting,
    Succeeded,
    Failed,
    Canceled,
    Deleted,
    NotSpecified,
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
            Self::Accepted => serializer.serialize_unit_variant("ProvisioningState", 0u32, "Accepted"),
            Self::Creating => serializer.serialize_unit_variant("ProvisioningState", 1u32, "Creating"),
            Self::Updating => serializer.serialize_unit_variant("ProvisioningState", 2u32, "Updating"),
            Self::Deleting => serializer.serialize_unit_variant("ProvisioningState", 3u32, "Deleting"),
            Self::Succeeded => serializer.serialize_unit_variant("ProvisioningState", 4u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("ProvisioningState", 5u32, "Failed"),
            Self::Canceled => serializer.serialize_unit_variant("ProvisioningState", 6u32, "Canceled"),
            Self::Deleted => serializer.serialize_unit_variant("ProvisioningState", 7u32, "Deleted"),
            Self::NotSpecified => serializer.serialize_unit_variant("ProvisioningState", 8u32, "NotSpecified"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Default error response for resource provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceProviderDefaultErrorResponse {
    #[doc = "Response body of Error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponseBody>,
}
impl azure_core::Continuable for ResourceProviderDefaultErrorResponse {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ResourceProviderDefaultErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type RoleBindingList = Vec<RoleBindingRecord>;
#[doc = "Record of the environment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RoleBindingRecord {
    #[doc = "The type of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Id of the role"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Metadata of the data record"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataEntity>,
    #[doc = "The principal User or Group to bind the role to"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
    #[doc = "The name of the role to bind to the principal"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[doc = "A CRN that specifies the scope and resource patterns necessary for the role to bind"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crn_pattern: Option<String>,
}
impl RoleBindingRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "SaaS Offer Status for confluent RP"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "SaaSOfferStatus")]
pub enum SaaSOfferStatus {
    Started,
    PendingFulfillmentStart,
    InProgress,
    Subscribed,
    Suspended,
    Reinstated,
    Succeeded,
    Failed,
    Unsubscribed,
    Updating,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for SaaSOfferStatus {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for SaaSOfferStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for SaaSOfferStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Started => serializer.serialize_unit_variant("SaaSOfferStatus", 0u32, "Started"),
            Self::PendingFulfillmentStart => serializer.serialize_unit_variant("SaaSOfferStatus", 1u32, "PendingFulfillmentStart"),
            Self::InProgress => serializer.serialize_unit_variant("SaaSOfferStatus", 2u32, "InProgress"),
            Self::Subscribed => serializer.serialize_unit_variant("SaaSOfferStatus", 3u32, "Subscribed"),
            Self::Suspended => serializer.serialize_unit_variant("SaaSOfferStatus", 4u32, "Suspended"),
            Self::Reinstated => serializer.serialize_unit_variant("SaaSOfferStatus", 5u32, "Reinstated"),
            Self::Succeeded => serializer.serialize_unit_variant("SaaSOfferStatus", 6u32, "Succeeded"),
            Self::Failed => serializer.serialize_unit_variant("SaaSOfferStatus", 7u32, "Failed"),
            Self::Unsubscribed => serializer.serialize_unit_variant("SaaSOfferStatus", 8u32, "Unsubscribed"),
            Self::Updating => serializer.serialize_unit_variant("SaaSOfferStatus", 9u32, "Updating"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Record of the service account"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServiceAccountRecord {
    #[doc = "Type of account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Id of the service account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Metadata of the data record"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataEntity>,
    #[doc = "Name of the service account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Description of the service account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ServiceAccountRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ServiceAccountsList = Vec<ServiceAccountRecord>;
#[doc = "Subscriber detail"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserDetail {
    #[doc = "First name"]
    #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "Last name"]
    #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "Email address"]
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    #[doc = "User principal name"]
    #[serde(rename = "userPrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    #[doc = "AAD email address"]
    #[serde(rename = "aadEmail", default, skip_serializing_if = "Option::is_none")]
    pub aad_email: Option<String>,
}
impl UserDetail {
    pub fn new(email_address: String) -> Self {
        Self {
            first_name: None,
            last_name: None,
            email_address,
            user_principal_name: None,
            aad_email: None,
        }
    }
}
#[doc = "Record of the user"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserRecord {
    #[doc = "Type of account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Id of the user"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Metadata of the data record"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MetadataEntity>,
    #[doc = "Email of the user"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Name of the user"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[doc = "Auth type of the user"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
}
impl UserRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type UsersList = Vec<UserRecord>;
#[doc = "Validation response from the provider"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidationResponse {
    #[doc = "Info from the response"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl ValidationResponse {
    pub fn new() -> Self {
        Self::default()
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
