// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotMarketHistoryRequest {
    /// Specify unique ticker to search
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    /// As an alternative is possible to provide a marketId
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    /// Symbol resolution. Possible resolutions are daily (D or 1D, 2D ... ), weekly
    /// (1W, 2W ...), monthly (1M, 2M...) and an intra-day resolution – minutes(1, 2
    /// ...).
    #[prost(string, tag="3")]
    pub resolution: ::prost::alloc::string::String,
    /// Unix timestamp (UTC) of the leftmost required bar, including from
    #[prost(sint32, tag="4")]
    pub from: i32,
    /// Unix timestamp (UTC) of the rightmost required bar, including to. It can be
    /// in the future. In this case, the rightmost required bar is the latest
    /// available bar.
    #[prost(sint32, tag="5")]
    pub to: i32,
    /// Number of bars (higher priority than from) starting with to. If countback is
    /// set, from should be ignored.
    #[prost(sint32, tag="6")]
    pub countback: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotMarketHistoryResponse {
    /// Bar time, Unix timestamp (UTC). Daily bars should only have the date part,
    /// time should be 0.
    #[prost(sint32, repeated, tag="1")]
    pub t: ::prost::alloc::vec::Vec<i32>,
    /// Open price.
    #[prost(double, repeated, tag="2")]
    pub o: ::prost::alloc::vec::Vec<f64>,
    /// High price.
    #[prost(double, repeated, tag="3")]
    pub h: ::prost::alloc::vec::Vec<f64>,
    /// Low price.
    #[prost(double, repeated, tag="4")]
    pub l: ::prost::alloc::vec::Vec<f64>,
    /// Close price.
    #[prost(double, repeated, tag="5")]
    pub c: ::prost::alloc::vec::Vec<f64>,
    /// Volume.
    #[prost(double, repeated, tag="6")]
    pub v: ::prost::alloc::vec::Vec<f64>,
    /// Status of the request.
    #[prost(string, tag="7")]
    pub s: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeMarketHistoryRequest {
    /// Specify unique ticker to search.
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    /// As an alternative is possible to provide a marketId
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    /// Symbol resolution. Possible resolutions are daily (D or 1D, 2D ... ), weekly
    /// (1W, 2W ...), monthly (1M, 2M...) and an intra-day resolution – minutes(1, 2
    /// ...).
    #[prost(string, tag="3")]
    pub resolution: ::prost::alloc::string::String,
    /// Unix timestamp (UTC) of the leftmost required bar, including from
    #[prost(sint32, tag="4")]
    pub from: i32,
    /// Unix timestamp (UTC) of the rightmost required bar, including to. It can be
    /// in the future. In this case, the rightmost required bar is the latest
    /// available bar.
    #[prost(sint32, tag="5")]
    pub to: i32,
    /// Number of bars (higher priority than from) starting with to. If countback is
    /// set, from should be ignored.
    #[prost(sint32, tag="6")]
    pub countback: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeMarketHistoryResponse {
    /// Bar time, Unix timestamp (UTC). Daily bars should only have the date part,
    /// time should be 0.
    #[prost(sint32, repeated, tag="1")]
    pub t: ::prost::alloc::vec::Vec<i32>,
    /// Open price.
    #[prost(double, repeated, tag="2")]
    pub o: ::prost::alloc::vec::Vec<f64>,
    /// High price.
    #[prost(double, repeated, tag="3")]
    pub h: ::prost::alloc::vec::Vec<f64>,
    /// Low price.
    #[prost(double, repeated, tag="4")]
    pub l: ::prost::alloc::vec::Vec<f64>,
    /// Close price.
    #[prost(double, repeated, tag="5")]
    pub c: ::prost::alloc::vec::Vec<f64>,
    /// Volume.
    #[prost(double, repeated, tag="6")]
    pub v: ::prost::alloc::vec::Vec<f64>,
    /// Status of the request.
    #[prost(string, tag="7")]
    pub s: ::prost::alloc::string::String,
}
include!("injective_chart_rpc.tonic.rs");
// @@protoc_insertion_point(module)
