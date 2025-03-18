// @generated
/// Params defines the parameters for the permissions module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(uint64, tag="1")]
    pub wasm_hook_query_max_gas: u64,
}
/// Namespace defines a permissions namespace
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Namespace {
    /// tokenfactory denom to which this namespace applies to
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    /// address of smart contract to apply code-based restrictions
    #[prost(string, tag="2")]
    pub contract_hook: ::prost::alloc::string::String,
    /// permissions for each role
    #[prost(message, repeated, tag="3")]
    pub role_permissions: ::prost::alloc::vec::Vec<Role>,
    /// roles for each actor
    #[prost(message, repeated, tag="4")]
    pub actor_roles: ::prost::alloc::vec::Vec<ActorRoles>,
    ///   managers for each role
    #[prost(message, repeated, tag="5")]
    pub role_managers: ::prost::alloc::vec::Vec<RoleManager>,
    /// status for each policy
    #[prost(message, repeated, tag="6")]
    pub policy_statuses: ::prost::alloc::vec::Vec<PolicyStatus>,
    /// capabilities for each manager for each policy
    #[prost(message, repeated, tag="7")]
    pub policy_manager_capabilities: ::prost::alloc::vec::Vec<PolicyManagerCapability>,
}
/// AddressRoles defines roles for an actor
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActorRoles {
    #[prost(string, tag="1")]
    pub actor: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// RoleActors defines actors for a role
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleActors {
    #[prost(string, tag="1")]
    pub role: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub actors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// RoleManager defines roles for a manager address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleManager {
    #[prost(string, tag="1")]
    pub manager: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PolicyStatus defines the status of a policy
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyStatus {
    #[prost(enumeration="Action", tag="1")]
    pub action: i32,
    #[prost(bool, tag="2")]
    pub is_disabled: bool,
    #[prost(bool, tag="3")]
    pub is_sealed: bool,
}
/// Role is only used for storage
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Role {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub role_id: u32,
    #[prost(uint32, tag="3")]
    pub permissions: u32,
}
/// PolicyManagerCapability defines the capabilities of a manager for a policy
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyManagerCapability {
    #[prost(string, tag="1")]
    pub manager: ::prost::alloc::string::String,
    #[prost(enumeration="Action", tag="2")]
    pub action: i32,
    #[prost(bool, tag="3")]
    pub can_disable: bool,
    #[prost(bool, tag="4")]
    pub can_seal: bool,
}
/// used in storage
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoleIDs {
    #[prost(uint32, repeated, tag="1")]
    pub role_ids: ::prost::alloc::vec::Vec<u32>,
}
/// AddressVoucher is used to represent a voucher for a specific address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressVoucher {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub voucher: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// each Action enum value should be a power of two
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Action {
    /// 0 is reserved for ACTION_UNSPECIFIED
    Unspecified = 0,
    /// 1 is reserved for MINT
    Mint = 1,
    /// 2 is reserved for RECEIVE
    Receive = 2,
    /// 4 is reserved for BURN
    Burn = 4,
    /// 8 is reserved for SEND
    Send = 8,
    /// 16 is reserved for SUPER_BURN
    SuperBurn = 16,
    //
    // MANAGER ACTIONS BELOW
    //

    /// 2^27 is reserved for MODIFY_POLICY_MANAGERS
    ///
    /// 2^27 or 134217728
    ModifyPolicyManagers = 134217728,
    /// 2^28 is reserved for MODIFY_CONTRACT_HOOK
    ///
    /// 2^28 or 268435456
    ModifyContractHook = 268435456,
    /// 2^29 is reserved for MODIFY_ROLE_PERMISSIONS
    ///
    /// 2^29 or 536870912
    ModifyRolePermissions = 536870912,
    /// 2^30 is reserved for MODIFY_ROLE_MANAGERS
    ///
    /// 2^30 or 1073741824
    ModifyRoleManagers = 1073741824,
}
impl Action {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Action::Unspecified => "UNSPECIFIED",
            Action::Mint => "MINT",
            Action::Receive => "RECEIVE",
            Action::Burn => "BURN",
            Action::Send => "SEND",
            Action::SuperBurn => "SUPER_BURN",
            Action::ModifyPolicyManagers => "MODIFY_POLICY_MANAGERS",
            Action::ModifyContractHook => "MODIFY_CONTRACT_HOOK",
            Action::ModifyRolePermissions => "MODIFY_ROLE_PERMISSIONS",
            Action::ModifyRoleManagers => "MODIFY_ROLE_MANAGERS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED" => Some(Self::Unspecified),
            "MINT" => Some(Self::Mint),
            "RECEIVE" => Some(Self::Receive),
            "BURN" => Some(Self::Burn),
            "SEND" => Some(Self::Send),
            "SUPER_BURN" => Some(Self::SuperBurn),
            "MODIFY_POLICY_MANAGERS" => Some(Self::ModifyPolicyManagers),
            "MODIFY_CONTRACT_HOOK" => Some(Self::ModifyContractHook),
            "MODIFY_ROLE_PERMISSIONS" => Some(Self::ModifyRolePermissions),
            "MODIFY_ROLE_MANAGERS" => Some(Self::ModifyRoleManagers),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the permissions parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<Params>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateNamespace {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub namespace: ::core::option::Option<Namespace>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateNamespaceResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateNamespace {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// denom whose namespace updates are to be applied
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
    /// address of smart contract to apply code-based restrictions
    #[prost(message, optional, tag="3")]
    pub contract_hook: ::core::option::Option<msg_update_namespace::SetContractHook>,
    /// role permissions to update
    #[prost(message, repeated, tag="4")]
    pub role_permissions: ::prost::alloc::vec::Vec<Role>,
    ///   role managers to update
    #[prost(message, repeated, tag="5")]
    pub role_managers: ::prost::alloc::vec::Vec<RoleManager>,
    /// policy statuses to update
    #[prost(message, repeated, tag="6")]
    pub policy_statuses: ::prost::alloc::vec::Vec<PolicyStatus>,
    /// policy manager capabilities to update
    #[prost(message, repeated, tag="7")]
    pub policy_manager_capabilities: ::prost::alloc::vec::Vec<PolicyManagerCapability>,
}
/// Nested message and enum types in `MsgUpdateNamespace`.
pub mod msg_update_namespace {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SetContractHook {
        #[prost(string, tag="1")]
        pub new_value: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateNamespaceResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateActorRoles {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// namespace denom to which this updates are applied
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
    /// roles to add for given actors
    #[prost(message, repeated, tag="3")]
    pub role_actors_to_add: ::prost::alloc::vec::Vec<RoleActors>,
    /// roles to revoke from given actors
    #[prost(message, repeated, tag="5")]
    pub role_actors_to_revoke: ::prost::alloc::vec::Vec<RoleActors>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateActorRolesResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClaimVoucher {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClaimVoucherResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSetVoucher {
    #[prost(string, tag="1")]
    pub addr: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub voucher: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// GenesisState defines the permissions module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag="2")]
    pub namespaces: ::prost::alloc::vec::Vec<Namespace>,
    #[prost(message, repeated, tag="3")]
    pub vouchers: ::prost::alloc::vec::Vec<AddressVoucher>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryNamespaceDenomsRequest is the request type for the Query/NamespaceDenoms RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNamespaceDenomsRequest {
}
/// QueryNamespaceDenomsResponse is the response type for the Query/NamespaceDenoms
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNamespaceDenomsResponse {
    #[prost(string, repeated, tag="1")]
    pub denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryNamespacesRequest is the request type for the Query/Namespaces RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNamespacesRequest {
}
/// QueryNamespacesResponse is the response type for the Query/Namespaces
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNamespacesResponse {
    #[prost(message, repeated, tag="1")]
    pub namespaces: ::prost::alloc::vec::Vec<Namespace>,
}
/// QueryNamespaceRequest is the request type for the
/// Query/Namespace RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNamespaceRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryNamespaceResponse is the response type for the
/// Query/NamespaceByDenom RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNamespaceResponse {
    #[prost(message, optional, tag="1")]
    pub namespace: ::core::option::Option<Namespace>,
}
/// QueryAddressesByRoleRequest is the request type for the Query/AddressesByRole
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryActorsByRoleRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub role: ::prost::alloc::string::String,
}
/// QueryAddressesByRoleResponse is the response type for the
/// Query/AddressesByRole RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryActorsByRoleResponse {
    #[prost(string, repeated, tag="1")]
    pub actors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryRolesByActorRequest is the request type for the
/// Query/RolesByActor RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRolesByActorRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub actor: ::prost::alloc::string::String,
}
/// QueryRolesByActorResponse is the response type for the
/// Query/RolesByActor RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRolesByActorResponse {
    #[prost(string, repeated, tag="1")]
    pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryRoleManagersRequest is the request type for the Query/RoleManagers
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRoleManagersRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryRoleManagersResponse is the response type for the
/// Query/RoleManagers RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRoleManagersResponse {
    #[prost(message, repeated, tag="1")]
    pub role_managers: ::prost::alloc::vec::Vec<RoleManager>,
}
/// QueryRoleManagerRequest is the request type for the Query/RoleManager
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRoleManagerRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub manager: ::prost::alloc::string::String,
}
/// QueryRoleManagerResponse is the response type for the
/// Query/RoleManager RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRoleManagerResponse {
    #[prost(message, optional, tag="1")]
    pub role_manager: ::core::option::Option<RoleManager>,
}
/// QueryPolicyStatusesRequest is the request type for the Query/PolicyStatuses
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPolicyStatusesRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryRoleManagerResponse is the response type for the
/// Query/RoleManager RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPolicyStatusesResponse {
    #[prost(message, repeated, tag="1")]
    pub policy_statuses: ::prost::alloc::vec::Vec<PolicyStatus>,
}
/// QueryPolicyManagerCapabilitiesRequest is the request type for the Query/PolicyManagerCapabilities
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPolicyManagerCapabilitiesRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryPolicyManagerCapabilitiesResponse is the response type for the
/// Query/PolicyManagerCapabilities RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPolicyManagerCapabilitiesResponse {
    #[prost(message, repeated, tag="1")]
    pub policy_manager_capabilities: ::prost::alloc::vec::Vec<PolicyManagerCapability>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVouchersRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVouchersResponse {
    #[prost(message, repeated, tag="1")]
    pub vouchers: ::prost::alloc::vec::Vec<AddressVoucher>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVoucherRequest {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryVoucherResponse {
    #[prost(message, optional, tag="1")]
    pub voucher: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryModuleStateRequest is the request type for the Query/PermissionsModuleState
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleStateRequest {
}
/// QueryModuleStateResponse is the response type for the Query/PermissionsModuleState
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleStateResponse {
    #[prost(message, optional, tag="1")]
    pub state: ::core::option::Option<GenesisState>,
}
include!("injective.permissions.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
