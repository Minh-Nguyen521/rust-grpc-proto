// @generated
/// DenomAuthorityMetadata specifies metadata for addresses that have specific
/// capabilities over a token factory denom. Right now there is only one Admin
/// permission, but is planned to be extended to the future.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenomAuthorityMetadata {
    /// Can be empty for no admin, or a valid injective address
    #[prost(string, tag="1")]
    pub admin: ::prost::alloc::string::String,
    /// true if the admin can burn tokens from other addresses
    #[prost(bool, tag="2")]
    pub admin_burn_allowed: bool,
}
/// Params defines the parameters for the tokenfactory module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(message, repeated, tag="1")]
    pub denom_creation_fee: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgCreateDenom defines the message structure for the CreateDenom gRPC service
/// method. It allows an account to create a new denom. It requires a sender
/// address and a sub denomination. The (sender_address, sub_denomination) tuple
/// must be unique and cannot be re-used.
///
/// The resulting denom created is defined as
/// <factory/{creatorAddress}/{subdenom}>. The resulting denom's admin is
/// originally set to be the creator, but this can be changed later. The token
/// denom does not indicate the current admin.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateDenom {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// subdenom can be up to 44 "alphanumeric" characters long.
    #[prost(string, tag="2")]
    pub subdenom: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint32, tag="5")]
    pub decimals: u32,
    /// true if admins are allowed to burn tokens from other addresses
    #[prost(bool, tag="6")]
    pub allow_admin_burn: bool,
}
/// MsgCreateDenomResponse is the return value of MsgCreateDenom
/// It returns the full string of the newly created denom
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateDenomResponse {
    #[prost(string, tag="1")]
    pub new_token_denom: ::prost::alloc::string::String,
}
/// MsgMint is the sdk.Msg type for allowing an admin account or other permitted accounts to mint
/// more of a token.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMint {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag="3")]
    pub receiver: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMintResponse {
}
/// MsgBurn is the sdk.Msg type for allowing an admin account to burn
/// a token.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBurn {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag="3")]
    pub burn_from_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBurnResponse {
}
/// MsgChangeAdmin is the sdk.Msg type for allowing an admin account to reassign
/// adminship of a denom to a new account
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChangeAdmin {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub new_admin: ::prost::alloc::string::String,
}
/// MsgChangeAdminResponse defines the response structure for an executed
/// MsgChangeAdmin message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgChangeAdminResponse {
}
/// MsgSetDenomMetadata is the sdk.Msg type for allowing an admin account to set
/// the denom's bank metadata
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetDenomMetadata {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<super::super::super::cosmos::bank::v1beta1::Metadata>,
    #[prost(message, optional, tag="3")]
    pub admin_burn_disabled: ::core::option::Option<msg_set_denom_metadata::AdminBurnDisabled>,
}
/// Nested message and enum types in `MsgSetDenomMetadata`.
pub mod msg_set_denom_metadata {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AdminBurnDisabled {
        /// true if the admin burn capability should be disabled
        #[prost(bool, tag="1")]
        pub should_disable: bool,
    }
}
/// MsgSetDenomMetadataResponse defines the response structure for an executed
/// MsgSetDenomMetadata message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetDenomMetadataResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address of the governance account.
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the tokenfactory parameters to update.
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
pub struct EventCreateDenom {
    #[prost(string, tag="1")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMint {
    #[prost(string, tag="1")]
    pub minter: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag="3")]
    pub receiver: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBurn {
    #[prost(string, tag="1")]
    pub burner: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag="3")]
    pub burn_from: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventChangeAdmin {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub new_admin_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSetDenomMetadata {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<super::super::super::cosmos::bank::v1beta1::Metadata>,
}
/// GenesisState defines the tokenfactory module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag="2")]
    pub factory_denoms: ::prost::alloc::vec::Vec<GenesisDenom>,
}
/// GenesisDenom defines a tokenfactory denom that is defined within genesis
/// state. The structure contains DenomAuthorityMetadata which defines the
/// denom's admin.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisDenom {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub authority_metadata: ::core::option::Option<DenomAuthorityMetadata>,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint32, tag="5")]
    pub decimals: u32,
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
/// QueryDenomAuthorityMetadataRequest defines the request structure for the
/// DenomAuthorityMetadata gRPC query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomAuthorityMetadataRequest {
    #[prost(string, tag="1")]
    pub creator: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub sub_denom: ::prost::alloc::string::String,
}
/// QueryDenomAuthorityMetadataResponse defines the response structure for the
/// DenomAuthorityMetadata gRPC query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomAuthorityMetadataResponse {
    #[prost(message, optional, tag="1")]
    pub authority_metadata: ::core::option::Option<DenomAuthorityMetadata>,
}
/// QueryDenomsFromCreatorRequest defines the request structure for the
/// DenomsFromCreator gRPC query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomsFromCreatorRequest {
    #[prost(string, tag="1")]
    pub creator: ::prost::alloc::string::String,
}
/// QueryDenomsFromCreatorRequest defines the response structure for the
/// DenomsFromCreator gRPC query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomsFromCreatorResponse {
    #[prost(string, repeated, tag="1")]
    pub denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryModuleStateRequest is the request type for the
/// Query/TokenfactoryModuleState RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleStateRequest {
}
/// QueryModuleStateResponse is the response type for the
/// Query/TokenfactoryModuleState RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleStateResponse {
    #[prost(message, optional, tag="1")]
    pub state: ::core::option::Option<GenesisState>,
}
include!("injective.tokenfactory.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
