// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLatestHeightRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLatestHeightResponse {
    /// Response version.
    #[prost(string, tag="1")]
    pub v: ::prost::alloc::string::String,
    /// Status of the response.
    #[prost(string, tag="2")]
    pub s: ::prost::alloc::string::String,
    /// Error message.
    #[prost(string, tag="3")]
    pub e: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub data: ::core::option::Option<LatestBlockHeight>,
}
/// Latest block height from event provider db
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LatestBlockHeight {
    #[prost(uint64, tag="1")]
    pub height: u64,
    #[prost(sint64, tag="2")]
    pub time: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamBlockEventsRequest {
    /// Select backend processor
    #[prost(string, tag="1")]
    pub backend: ::prost::alloc::string::String,
    #[prost(sint32, tag="2")]
    pub height: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamBlockEventsResponse {
    #[prost(message, repeated, tag="1")]
    pub blocks: ::prost::alloc::vec::Vec<Block>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(sint64, tag="1")]
    pub height: i64,
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
    /// Processed block events in the block
    #[prost(message, repeated, tag="3")]
    pub events: ::prost::alloc::vec::Vec<BlockEvent>,
    /// Indicates whether the block is the latest one available in the event provider
    #[prost(bool, tag="4")]
    pub in_sync: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockEvent {
    /// Event type
    #[prost(string, tag="1")]
    pub type_url: ::prost::alloc::string::String,
    /// Event data
    #[prost(bytes="vec", tag="2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// TX hash
    #[prost(bytes="vec", tag="3")]
    pub tx_hash: ::prost::alloc::vec::Vec<u8>,
    /// Set only if it's a BeginBlock or EndBlock event
    #[prost(string, tag="4")]
    pub mode: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockEventsRpcRequest {
    /// Select backend processor
    #[prost(string, tag="1")]
    pub backend: ::prost::alloc::string::String,
    #[prost(sint32, tag="2")]
    pub height: i32,
    #[prost(bool, tag="3")]
    pub human_readable: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockEventsRpcResponse {
    /// Response version.
    #[prost(string, tag="1")]
    pub v: ::prost::alloc::string::String,
    /// Status of the response.
    #[prost(string, tag="2")]
    pub s: ::prost::alloc::string::String,
    /// Error message.
    #[prost(string, tag="3")]
    pub e: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub data: ::core::option::Option<BlockEventsRpc>,
}
/// Processed block events for backend processor to consume
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockEventsRpc {
    /// Array of event types
    #[prost(string, repeated, tag="1")]
    pub types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Array of parsed events
    #[prost(bytes="vec", repeated, tag="2")]
    pub events: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// Map of event index - tx hash
    #[prost(map="sint32, bytes", tag="3")]
    pub tx_hashes: ::std::collections::HashMap<i32, ::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomEventsRpcRequest {
    /// Select backend processor
    #[prost(string, tag="1")]
    pub backend: ::prost::alloc::string::String,
    #[prost(sint32, tag="2")]
    pub height: i32,
    /// Specify custom events to get
    #[prost(string, tag="3")]
    pub events: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomEventsRpcResponse {
    /// Response version.
    #[prost(string, tag="1")]
    pub v: ::prost::alloc::string::String,
    /// Status of the response.
    #[prost(string, tag="2")]
    pub s: ::prost::alloc::string::String,
    /// Error message.
    #[prost(string, tag="3")]
    pub e: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub data: ::core::option::Option<BlockEventsRpc>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAbciBlockEventsRequest {
    #[prost(sint32, tag="1")]
    pub height: i32,
    /// Array of event types
    #[prost(string, repeated, tag="2")]
    pub event_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAbciBlockEventsResponse {
    /// Response version.
    #[prost(string, tag="1")]
    pub v: ::prost::alloc::string::String,
    /// Status of the response.
    #[prost(string, tag="2")]
    pub s: ::prost::alloc::string::String,
    /// Error message.
    #[prost(string, tag="3")]
    pub e: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub raw_block: ::core::option::Option<RawBlock>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawBlock {
    #[prost(sint64, tag="1")]
    pub height: i64,
    #[prost(string, tag="5")]
    pub block_time: ::prost::alloc::string::String,
    /// Block timestamp in UNIX millis.
    #[prost(sint64, tag="6")]
    pub block_timestamp: i64,
    #[prost(message, repeated, tag="2")]
    pub txs_results: ::prost::alloc::vec::Vec<AbciResponseDeliverTx>,
    #[prost(message, repeated, tag="3")]
    pub begin_block_events: ::prost::alloc::vec::Vec<AbciEvent>,
    #[prost(message, repeated, tag="4")]
    pub end_block_events: ::prost::alloc::vec::Vec<AbciEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbciResponseDeliverTx {
    #[prost(sint32, tag="1")]
    pub code: i32,
    #[prost(string, tag="2")]
    pub log: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub info: ::prost::alloc::string::String,
    #[prost(sint64, tag="4")]
    pub gas_wanted: i64,
    #[prost(sint64, tag="5")]
    pub gas_used: i64,
    #[prost(message, repeated, tag="6")]
    pub events: ::prost::alloc::vec::Vec<AbciEvent>,
    #[prost(string, tag="7")]
    pub codespace: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="8")]
    pub tx_hash: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbciEvent {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub attributes: ::prost::alloc::vec::Vec<AbciAttribute>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbciAttribute {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub value: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAbciBlockEventsAtHeightRequest {
    #[prost(sint32, tag="1")]
    pub height: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAbciBlockEventsAtHeightResponse {
    /// Response version.
    #[prost(string, tag="1")]
    pub v: ::prost::alloc::string::String,
    /// Status of the response.
    #[prost(string, tag="2")]
    pub s: ::prost::alloc::string::String,
    /// Error message.
    #[prost(string, tag="3")]
    pub e: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub raw_block: ::core::option::Option<RawBlock>,
}
include!("event_provider_api.tonic.rs");
// @@protoc_insertion_point(module)
