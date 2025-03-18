// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PingResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionResponse {
    /// injective-exchange code version.
    #[prost(string, tag="1")]
    pub version: ::prost::alloc::string::String,
    /// Additional build meta info.
    #[prost(map="string, string", tag="2")]
    pub build: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoRequest {
    /// Provide current system UNIX timestamp in millis
    #[prost(sint64, tag="1")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoResponse {
    /// The original timestamp value in millis.
    #[prost(sint64, tag="1")]
    pub timestamp: i64,
    /// UNIX time on the server in millis.
    #[prost(sint64, tag="2")]
    pub server_time: i64,
    /// injective-exchange code version.
    #[prost(string, tag="3")]
    pub version: ::prost::alloc::string::String,
    /// Additional build meta info.
    #[prost(map="string, string", tag="4")]
    pub build: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Server's location region
    #[prost(string, tag="5")]
    pub region: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamKeepaliveRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamKeepaliveResponse {
    /// Server event
    #[prost(string, tag="1")]
    pub event: ::prost::alloc::string::String,
    /// New conection endpoint for the gRPC API
    #[prost(string, tag="2")]
    pub new_endpoint: ::prost::alloc::string::String,
    /// Operation timestamp in UNIX millis.
    #[prost(sint64, tag="3")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenMetadataRequest {
    #[prost(string, repeated, tag="1")]
    pub denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenMetadataResponse {
    /// tokens and their metadata list
    #[prost(message, repeated, tag="1")]
    pub tokens: ::prost::alloc::vec::Vec<TokenMetadataElement>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenMetadataElement {
    /// Token's Ethereum address, not all token have this information
    #[prost(string, tag="1")]
    pub ethereum_address: ::prost::alloc::string::String,
    /// Token's CoinGecko id for price references
    #[prost(string, tag="2")]
    pub coingecko_id: ::prost::alloc::string::String,
    /// Token's denom on injective chain
    #[prost(string, tag="3")]
    pub denom: ::prost::alloc::string::String,
    /// Token name
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    /// Token symbol
    #[prost(string, tag="5")]
    pub symbol: ::prost::alloc::string::String,
    /// Number of decimal places used to represent the token's smallest unit
    #[prost(sint32, tag="6")]
    pub decimals: i32,
    /// Token logo URL
    #[prost(string, tag="7")]
    pub logo: ::prost::alloc::string::String,
}
include!("injective_meta_rpc.tonic.rs");
// @@protoc_insertion_point(module)
