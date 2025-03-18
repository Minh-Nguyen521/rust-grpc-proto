// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatusRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatusResponse {
    /// Status of the response.
    #[prost(string, tag="1")]
    pub s: ::prost::alloc::string::String,
    /// Error message.
    #[prost(string, tag="2")]
    pub errmsg: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub data: ::core::option::Option<HealthStatus>,
    #[prost(string, tag="4")]
    pub status: ::prost::alloc::string::String,
}
/// Status defines the structure for health information
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthStatus {
    /// Block height from local mongo exchange db.
    #[prost(sint32, tag="1")]
    pub local_height: i32,
    /// block timestamp from local mongo exchange db.
    #[prost(sint32, tag="2")]
    pub local_timestamp: i32,
    /// block height from Horacle service.
    #[prost(sint32, tag="3")]
    pub horacle_height: i32,
    /// block timestamp from Horacle service.
    #[prost(sint32, tag="4")]
    pub horacle_timestamp: i32,
    /// Migration version of the database.
    #[prost(sint32, tag="5")]
    pub migration_last_version: i32,
    /// Block height from event provider service.
    #[prost(sint32, tag="6")]
    pub ep_height: i32,
    /// Block UNIX timestamp from event provider service.
    #[prost(sint32, tag="7")]
    pub ep_timestamp: i32,
}
include!("api.v1.tonic.rs");
// @@protoc_insertion_point(module)
