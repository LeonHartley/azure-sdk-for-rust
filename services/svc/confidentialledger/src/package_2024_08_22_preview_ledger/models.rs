#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::de::{value, Deserializer, IntoDeserializer};
use serde::{Deserialize, Serialize, Serializer};
use std::str::FromStr;
#[doc = "A claim of a ledger application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationClaim {
    #[doc = "An application claim in digested form."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<ClaimDigest>,
    #[doc = "Represents the kind of an application claim."]
    pub kind: ApplicationClaimKind,
    #[doc = "An application claim derived from ledger entry data."]
    #[serde(rename = "ledgerEntry", default, skip_serializing_if = "Option::is_none")]
    pub ledger_entry: Option<LedgerEntryClaim>,
}
impl ApplicationClaim {
    pub fn new(kind: ApplicationClaimKind) -> Self {
        Self {
            digest: None,
            kind,
            ledger_entry: None,
        }
    }
}
#[doc = "Represents the kind of an application claim."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ApplicationClaimKind")]
pub enum ApplicationClaimKind {
    LedgerEntry,
    ClaimDigest,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ApplicationClaimKind {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ApplicationClaimKind {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ApplicationClaimKind {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::LedgerEntry => serializer.serialize_unit_variant("ApplicationClaimKind", 0u32, "LedgerEntry"),
            Self::ClaimDigest => serializer.serialize_unit_variant("ApplicationClaimKind", 1u32, "ClaimDigest"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Represents the protocol to be used to compute the digest of a claim from the given claim data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ApplicationClaimProtocol")]
pub enum ApplicationClaimProtocol {
    LedgerEntryV1,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ApplicationClaimProtocol {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ApplicationClaimProtocol {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ApplicationClaimProtocol {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::LedgerEntryV1 => serializer.serialize_unit_variant("ApplicationClaimProtocol", 0u32, "LedgerEntryV1"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
pub type ApplicationClaims = Vec<ApplicationClaim>;
#[doc = "bundle for the user defined endpoints"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bundle {
    pub metadata: Metadata,
    pub modules: ModuleDefArray,
}
impl Bundle {
    pub fn new(metadata: Metadata, modules: ModuleDefArray) -> Self {
        Self { metadata, modules }
    }
}
#[doc = "An application claim in digested form."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClaimDigest {
    #[doc = "The digest of the application claim, in hexadecimal form."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Represents the protocol to be used to compute the digest of a claim from the given claim data."]
    pub protocol: ApplicationClaimProtocol,
}
impl ClaimDigest {
    pub fn new(protocol: ApplicationClaimProtocol) -> Self {
        Self { value: None, protocol }
    }
}
#[doc = "Identifier for collections."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Collection {
    #[serde(rename = "collectionId")]
    pub collection_id: String,
}
impl Collection {
    pub fn new(collection_id: String) -> Self {
        Self { collection_id }
    }
}
#[doc = "Information about the enclaves running the Confidential Ledger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfidentialLedgerEnclaves {
    #[doc = "Identifier for an entity."]
    #[serde(rename = "currentNodeId")]
    pub current_node_id: EntityId,
    #[doc = "Dictionary of enclave quotes, indexed by node id."]
    #[serde(rename = "enclaveQuotes")]
    pub enclave_quotes: EnclaveQuotes,
}
impl ConfidentialLedgerEnclaves {
    pub fn new(current_node_id: EntityId, enclave_quotes: EnclaveQuotes) -> Self {
        Self {
            current_node_id,
            enclave_quotes,
        }
    }
}
#[doc = "An error response from Confidential Ledger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfidentialLedgerError {
    #[doc = "An error response from Confidential Ledger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ConfidentialLedgerErrorBody>,
}
impl azure_core::Continuable for ConfidentialLedgerError {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        None
    }
}
impl ConfidentialLedgerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response from Confidential Ledger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfidentialLedgerErrorBody {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ConfidentialLedgerErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of members in the consortium."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Consortium {
    pub members: Vec<ConsortiumMember>,
    #[doc = "Path from which to retrieve the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for Consortium {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl Consortium {
    pub fn new(members: Vec<ConsortiumMember>) -> Self {
        Self { members, next_link: None }
    }
}
#[doc = "Describes a member of the consortium."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsortiumMember {
    #[doc = "PEM-encoded certificate associated with the member."]
    pub certificate: String,
    #[doc = "Identifier for an entity."]
    pub id: EntityId,
}
impl ConsortiumMember {
    pub fn new(certificate: String, id: EntityId) -> Self {
        Self { certificate, id }
    }
}
#[doc = "The governance script for the application."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Constitution {
    #[doc = "SHA256 digest of the constitution script."]
    pub digest: String,
    #[doc = "Contents of the constitution."]
    pub script: String,
}
impl Constitution {
    pub fn new(digest: String, script: String) -> Self {
        Self { digest, script }
    }
}
#[doc = "Contains the enclave quote."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnclaveQuote {
    #[doc = "Identifier for an entity."]
    #[serde(rename = "nodeId")]
    pub node_id: EntityId,
    #[doc = "MRENCLAVE value of the code running in the enclave."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mrenclave: Option<String>,
    #[doc = "Version of the quote presented."]
    #[serde(rename = "quoteVersion")]
    pub quote_version: String,
    #[doc = "Raw SGX quote, parsable by tools like Open Enclave's oeverify."]
    pub raw: String,
}
impl EnclaveQuote {
    pub fn new(node_id: EntityId, quote_version: String, raw: String) -> Self {
        Self {
            node_id,
            mrenclave: None,
            quote_version,
            raw,
        }
    }
}
#[doc = "Dictionary of enclave quotes, indexed by node id."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnclaveQuotes {}
impl EnclaveQuotes {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointProperties {
    pub authn_policies: JsonArray,
    pub forwarding_required: ForwardingRequired,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpreter_reuse: Option<InterpreterReusePolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub js_function: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub js_module: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub openapi: Option<Json>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub openapi_hidden: Option<Boolean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirection_strategy: Option<RedirectionStrategy>,
}
impl EndpointProperties {
    pub fn new(authn_policies: JsonArray, forwarding_required: ForwardingRequired) -> Self {
        Self {
            authn_policies,
            forwarding_required,
            interpreter_reuse: None,
            js_function: None,
            js_module: None,
            mode: None,
            openapi: None,
            openapi_hidden: None,
            redirection_strategy: None,
        }
    }
}
pub type EntityId = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "ForwardingRequired")]
pub enum ForwardingRequired {
    #[serde(rename = "sometimes")]
    Sometimes,
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "never")]
    Never,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for ForwardingRequired {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for ForwardingRequired {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for ForwardingRequired {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Sometimes => serializer.serialize_unit_variant("ForwardingRequired", 0u32, "sometimes"),
            Self::Always => serializer.serialize_unit_variant("ForwardingRequired", 1u32, "always"),
            Self::Never => serializer.serialize_unit_variant("ForwardingRequired", 2u32, "never"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InterpreterReusePolicy {
    pub key: String,
}
impl InterpreterReusePolicy {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}
#[doc = "JS runtime options for user defined endpoints"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JsRuntimeOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_exception_details: Option<Boolean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_cached_interpreters: Option<Uint64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_execution_time_ms: Option<Uint64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_heap_bytes: Option<Uint64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_stack_bytes: Option<Uint64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_exception_details: Option<Boolean>,
}
impl JsRuntimeOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type LedgerEntries = Vec<LedgerEntry>;
#[doc = "An entry in the ledger."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LedgerEntry {
    #[doc = "Contents of the ledger entry."]
    pub contents: String,
    #[serde(rename = "collectionId", default, skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[doc = "A unique identifier for the state of the ledger. If returned as part of a LedgerEntry, it indicates the state from which the entry was read."]
    #[serde(rename = "transactionId", default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<TransactionId>,
}
impl LedgerEntry {
    pub fn new(contents: String) -> Self {
        Self {
            contents,
            collection_id: None,
            transaction_id: None,
        }
    }
}
#[doc = "An application claim derived from ledger entry data."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LedgerEntryClaim {
    #[doc = "Identifier of a collection."]
    #[serde(rename = "collectionId", default, skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    #[doc = "Contents of a ledger entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contents: Option<String>,
    #[doc = "Base64-encoded secret key."]
    #[serde(rename = "secretKey", default, skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    #[doc = "Represents the protocol to be used to compute the digest of a claim from the given claim data."]
    pub protocol: ApplicationClaimProtocol,
}
impl LedgerEntryClaim {
    pub fn new(protocol: ApplicationClaimProtocol) -> Self {
        Self {
            collection_id: None,
            contents: None,
            secret_key: None,
            protocol,
        }
    }
}
#[doc = "The result of querying for a ledger entry from an older transaction id. The ledger entry is available in the response only if the returned state is Ready."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LedgerQueryResult {
    #[doc = "State of a ledger query."]
    pub state: LedgerQueryState,
    #[doc = "An entry in the ledger."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry: Option<LedgerEntry>,
}
impl LedgerQueryResult {
    pub fn new(state: LedgerQueryState) -> Self {
        Self { state, entry: None }
    }
}
#[doc = "State of a ledger query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LedgerQueryState")]
pub enum LedgerQueryState {
    Loading,
    Ready,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LedgerQueryState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LedgerQueryState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LedgerQueryState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Loading => serializer.serialize_unit_variant("LedgerQueryState", 0u32, "Loading"),
            Self::Ready => serializer.serialize_unit_variant("LedgerQueryState", 1u32, "Ready"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Details about a Confidential Ledger user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LedgerUser {
    #[doc = "Represents an assignable role."]
    #[serde(rename = "assignedRole")]
    pub assigned_role: LedgerUserRole,
    #[doc = "Identifier for the user. This must either be an AAD object id or a certificate fingerprint."]
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
}
impl LedgerUser {
    pub fn new(assigned_role: LedgerUserRole) -> Self {
        Self {
            assigned_role,
            user_id: None,
        }
    }
}
#[doc = "Details about a Confidential Ledger user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LedgerUserMultipleRoles {
    #[doc = "Represents an assignable role."]
    #[serde(rename = "assignedRoles")]
    pub assigned_roles: LedgerUserMultipleRolesDef,
    #[doc = "Identifier for the user. This must either be an AAD object id or a certificate fingerprint."]
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
}
impl LedgerUserMultipleRoles {
    pub fn new(assigned_roles: LedgerUserMultipleRolesDef) -> Self {
        Self {
            assigned_roles,
            user_id: None,
        }
    }
}
pub type LedgerUserMultipleRolesDef = Vec<MultipleRolesList>;
#[doc = "Represents an assignable role."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "LedgerUserRole")]
pub enum LedgerUserRole {
    Administrator,
    Contributor,
    Reader,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for LedgerUserRole {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for LedgerUserRole {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for LedgerUserRole {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Administrator => serializer.serialize_unit_variant("LedgerUserRole", 0u32, "Administrator"),
            Self::Contributor => serializer.serialize_unit_variant("LedgerUserRole", 1u32, "Contributor"),
            Self::Reader => serializer.serialize_unit_variant("LedgerUserRole", 2u32, "Reader"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Returned as a result of a write to the Confidential Ledger, the transaction id in the response indicates when the write will become durable."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LedgerWriteResult {
    #[serde(rename = "collectionId")]
    pub collection_id: String,
}
impl LedgerWriteResult {
    pub fn new(collection_id: String) -> Self {
        Self { collection_id }
    }
}
pub type MerkleProof = Vec<MerkleProofElement>;
#[doc = "An item in the Merkle proof."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MerkleProofElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right: Option<String>,
}
impl MerkleProofElement {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metadata {
    #[doc = "A map of path to method endpoints for the path"]
    pub endpoints: Path,
}
impl Metadata {
    pub fn new(endpoints: Path) -> Self {
        Self { endpoints }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "Mode")]
pub enum Mode {
    #[serde(rename = "readwrite")]
    Readwrite,
    #[serde(rename = "readonly")]
    Readonly,
    #[serde(rename = "historical")]
    Historical,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for Mode {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for Mode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for Mode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Readwrite => serializer.serialize_unit_variant("Mode", 0u32, "readwrite"),
            Self::Readonly => serializer.serialize_unit_variant("Mode", 1u32, "readonly"),
            Self::Historical => serializer.serialize_unit_variant("Mode", 2u32, "historical"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModuleDef {
    pub module: String,
    pub name: String,
}
impl ModuleDef {
    pub fn new(module: String, name: String) -> Self {
        Self { module, name }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ModuleDefArray {}
impl ModuleDefArray {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents list of roles for single user"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "MultipleRolesList")]
pub enum MultipleRolesList {
    Administrator,
    Contributor,
    Reader,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for MultipleRolesList {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for MultipleRolesList {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for MultipleRolesList {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Administrator => serializer.serialize_unit_variant("MultipleRolesList", 0u32, "Administrator"),
            Self::Contributor => serializer.serialize_unit_variant("MultipleRolesList", 1u32, "Contributor"),
            Self::Reader => serializer.serialize_unit_variant("MultipleRolesList", 2u32, "Reader"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Paginated collections returned in response to a query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedCollections {
    pub collections: Vec<Collection>,
    #[doc = "Path from which to retrieve the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedCollections {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedCollections {
    pub fn new(collections: Vec<Collection>) -> Self {
        Self {
            collections,
            next_link: None,
        }
    }
}
#[doc = "Paginated ledger entries returned in response to a query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedLedgerEntries {
    #[doc = "State of a ledger query."]
    pub state: LedgerQueryState,
    #[doc = "Path from which to retrieve the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[doc = "Array of ledger entries."]
    pub entries: LedgerEntries,
}
impl azure_core::Continuable for PagedLedgerEntries {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedLedgerEntries {
    pub fn new(state: LedgerQueryState, entries: LedgerEntries) -> Self {
        Self {
            state,
            next_link: None,
            entries,
        }
    }
}
#[doc = "Paginated users returned in response to a query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedLedgerUsers {
    #[serde(
        rename = "ledgerUsers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ledger_users: Vec<LedgerUserMultipleRoles>,
    #[doc = "Path from which to retrieve the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedLedgerUsers {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedLedgerUsers {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Paginated users returned in response to a query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedUsers {
    #[serde(
        rename = "ledgerUsers",
        default,
        deserialize_with = "azure_core::util::deserialize_null_as_default",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ledger_users: Vec<LedgerUser>,
    #[doc = "Path from which to retrieve the next page of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PagedUsers {
    type Continuation = String;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_link.clone().filter(|value| !value.is_empty())
    }
}
impl PagedUsers {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type Pem = String;
pub type PemArray = Vec<Pem>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReceiptContents {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cert: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leaf: Option<String>,
    #[serde(rename = "leafComponents", default, skip_serializing_if = "Option::is_none")]
    pub leaf_components: Option<ReceiptLeafComponents>,
    #[serde(rename = "nodeId")]
    pub node_id: String,
    pub proof: ReceiptElementArray,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<String>,
    #[serde(rename = "serviceEndorsements", default, skip_serializing_if = "Option::is_none")]
    pub service_endorsements: Option<PemArray>,
    pub signature: String,
}
impl ReceiptContents {
    pub fn new(node_id: String, proof: ReceiptElementArray, signature: String) -> Self {
        Self {
            cert: None,
            leaf: None,
            leaf_components: None,
            node_id,
            proof,
            root: None,
            service_endorsements: None,
            signature,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReceiptElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right: Option<String>,
}
impl ReceiptElement {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type ReceiptElementArray = Vec<ReceiptElement>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReceiptLeafComponents {
    #[serde(rename = "claimsDigest", default, skip_serializing_if = "Option::is_none")]
    pub claims_digest: Option<String>,
    #[serde(rename = "commitEvidence", default, skip_serializing_if = "Option::is_none")]
    pub commit_evidence: Option<String>,
    #[serde(rename = "writeSetDigest", default, skip_serializing_if = "Option::is_none")]
    pub write_set_digest: Option<String>,
}
impl ReceiptLeafComponents {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "RedirectionStrategy")]
pub enum RedirectionStrategy {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "to_primary")]
    ToPrimary,
    #[serde(rename = "to_backup")]
    ToBackup,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for RedirectionStrategy {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for RedirectionStrategy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for RedirectionStrategy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::None => serializer.serialize_unit_variant("RedirectionStrategy", 0u32, "none"),
            Self::ToPrimary => serializer.serialize_unit_variant("RedirectionStrategy", 1u32, "to_primary"),
            Self::ToBackup => serializer.serialize_unit_variant("RedirectionStrategy", 2u32, "to_backup"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Definition for roles"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Role {
    #[doc = "name of the user defined role"]
    #[serde(rename = "roleName", default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[serde(rename = "roleActions", default, skip_serializing_if = "Option::is_none")]
    pub role_actions: Option<serde_json::Value>,
}
impl Role {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type RoleActionDef = String;
#[doc = "Object for assigning a role to a user."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignment {
    #[doc = "Represents an assignable role."]
    #[serde(rename = "roleName")]
    pub role_name: LedgerUserRole,
    #[doc = "Description of the role."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl RoleAssignment {
    pub fn new(role_name: LedgerUserRole) -> Self {
        Self {
            role_name,
            description: None,
        }
    }
}
pub type TransactionId = String;
#[doc = "A receipt certifying the transaction at the specified id."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionReceipt {
    #[doc = "List of application claims."]
    #[serde(rename = "applicationClaims", default, skip_serializing_if = "Option::is_none")]
    pub application_claims: Option<ApplicationClaims>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt: Option<ReceiptContents>,
    #[doc = "State of a ledger query."]
    pub state: LedgerQueryState,
    #[doc = "A unique identifier for the state of the ledger. If returned as part of a LedgerEntry, it indicates the state from which the entry was read."]
    #[serde(rename = "transactionId")]
    pub transaction_id: TransactionId,
}
impl TransactionReceipt {
    pub fn new(state: LedgerQueryState, transaction_id: TransactionId) -> Self {
        Self {
            application_claims: None,
            receipt: None,
            state,
            transaction_id,
        }
    }
}
#[doc = "Represents the state of the transaction."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(remote = "TransactionState")]
pub enum TransactionState {
    Committed,
    Pending,
    #[serde(skip_deserializing)]
    UnknownValue(String),
}
impl FromStr for TransactionState {
    type Err = value::Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}
impl<'de> Deserialize<'de> for TransactionState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or(Self::UnknownValue(s));
        Ok(deserialized)
    }
}
impl Serialize for TransactionState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Committed => serializer.serialize_unit_variant("TransactionState", 0u32, "Committed"),
            Self::Pending => serializer.serialize_unit_variant("TransactionState", 1u32, "Pending"),
            Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
        }
    }
}
#[doc = "Response returned to a query for the transaction status"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionStatus {
    #[doc = "Represents the state of the transaction."]
    pub state: TransactionState,
    #[doc = "A unique identifier for the state of the ledger. If returned as part of a LedgerEntry, it indicates the state from which the entry was read."]
    #[serde(rename = "transactionId")]
    pub transaction_id: TransactionId,
}
impl TransactionStatus {
    pub fn new(state: TransactionState, transaction_id: TransactionId) -> Self {
        Self { state, transaction_id }
    }
}
pub type UserId = String;
pub type Boolean = bool;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Json {}
impl Json {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type JsonArray = Vec<Json>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MethodToEndpointProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get: Option<EndpointProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub put: Option<EndpointProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch: Option<EndpointProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<EndpointProperties>,
}
impl MethodToEndpointProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A map of path to method endpoints for the path"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Path {}
impl Path {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type Uint64 = i64;
