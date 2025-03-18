// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceRequest {
    /// Account address
    #[prost(string, tag="1")]
    pub account: ::prost::alloc::string::String,
    /// Symbol resolution. Possible resolutions are 1D,1W,1M
    #[prost(string, tag="2")]
    pub resolution: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceResponse {
    #[prost(message, optional, tag="1")]
    pub historical_balance: ::core::option::Option<HistoricalBalance>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistoricalBalance {
    /// Time, Unix timestamp (UTC)
    #[prost(sint32, repeated, tag="1")]
    pub t: ::prost::alloc::vec::Vec<i32>,
    /// Balance value
    #[prost(double, repeated, tag="2")]
    pub v: ::prost::alloc::vec::Vec<f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RpnlRequest {
    /// Account address
    #[prost(string, tag="1")]
    pub account: ::prost::alloc::string::String,
    /// Symbol resolution. Possible resolutions are 1D,1W,1M
    #[prost(string, tag="2")]
    pub resolution: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RpnlResponse {
    #[prost(message, optional, tag="1")]
    pub historical_rpnl: ::core::option::Option<HistoricalRpnl>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistoricalRpnl {
    /// Time, Unix timestamp (UTC)
    #[prost(sint32, repeated, tag="1")]
    pub t: ::prost::alloc::vec::Vec<i32>,
    /// Realized Profit and Loss value
    #[prost(double, repeated, tag="2")]
    pub v: ::prost::alloc::vec::Vec<f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumesRequest {
    /// Account address
    #[prost(string, tag="1")]
    pub account: ::prost::alloc::string::String,
    /// Symbol resolution. Possible resolutions are 1D,1W,1M
    #[prost(string, tag="2")]
    pub resolution: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumesResponse {
    #[prost(message, optional, tag="1")]
    pub historical_volumes: ::core::option::Option<HistoricalVolumes>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistoricalVolumes {
    /// Time, Unix timestamp (UTC)
    #[prost(sint32, repeated, tag="1")]
    pub t: ::prost::alloc::vec::Vec<i32>,
    /// Volume value
    #[prost(double, repeated, tag="2")]
    pub v: ::prost::alloc::vec::Vec<f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PnlLeaderboardRequest {
    /// Start date of the leaderboard period in unix time (ms)
    #[prost(sint64, tag="1")]
    pub start_date: i64,
    /// End date of the leaderboard period in unix time (ms)
    #[prost(sint64, tag="2")]
    pub end_date: i64,
    /// Number of leaderboard entries to return
    #[prost(sint32, tag="3")]
    pub limit: i32,
    /// Account address that's querying the leaderboard
    #[prost(string, tag="4")]
    pub account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PnlLeaderboardResponse {
    /// First date of snapshots used for the leaderboard period
    #[prost(string, tag="1")]
    pub first_date: ::prost::alloc::string::String,
    /// Last date of snapshots used for the leaderboard period
    #[prost(string, tag="2")]
    pub last_date: ::prost::alloc::string::String,
    /// Leaderboard entries
    #[prost(message, repeated, tag="3")]
    pub leaders: ::prost::alloc::vec::Vec<LeaderboardRow>,
    /// Leaderboard entry for the querying account
    #[prost(message, optional, tag="4")]
    pub account_row: ::core::option::Option<LeaderboardRow>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaderboardRow {
    /// Account address
    #[prost(string, tag="1")]
    pub account: ::prost::alloc::string::String,
    /// Realized profit and loss (USD)
    #[prost(double, tag="2")]
    pub pnl: f64,
    /// Trade volume (USD)
    #[prost(double, tag="3")]
    pub volume: f64,
    /// Rank in leaderboard
    #[prost(sint32, tag="4")]
    pub rank: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolLeaderboardRequest {
    /// Start date of the leaderboard period in unix time (ms)
    #[prost(sint64, tag="1")]
    pub start_date: i64,
    /// End date of the leaderboard period in unix time (ms)
    #[prost(sint64, tag="2")]
    pub end_date: i64,
    /// Number of leaderboard entries to return
    #[prost(sint32, tag="3")]
    pub limit: i32,
    /// Account address that's querying the leaderboard
    #[prost(string, tag="4")]
    pub account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolLeaderboardResponse {
    /// First date of snapshots used for the leaderboard period
    #[prost(string, tag="1")]
    pub first_date: ::prost::alloc::string::String,
    /// Last date of snapshots used for the leaderboard period
    #[prost(string, tag="2")]
    pub last_date: ::prost::alloc::string::String,
    /// Leaderboard entries
    #[prost(message, repeated, tag="3")]
    pub leaders: ::prost::alloc::vec::Vec<LeaderboardRow>,
    /// Leaderboard entry for the querying account
    #[prost(message, optional, tag="4")]
    pub account_row: ::core::option::Option<LeaderboardRow>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PnlLeaderboardFixedResolutionRequest {
    /// Leaderboard resolution. Possible resolutions are 1D,1W,1M,6M,ALL
    #[prost(string, tag="1")]
    pub resolution: ::prost::alloc::string::String,
    /// Number of leaderboard entries to return
    #[prost(sint32, tag="2")]
    pub limit: i32,
    /// Account address that's querying the leaderboard
    #[prost(string, tag="3")]
    pub account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PnlLeaderboardFixedResolutionResponse {
    /// First date of snapshots used for the leaderboard period
    #[prost(string, tag="1")]
    pub first_date: ::prost::alloc::string::String,
    /// Last date of snapshots used for the leaderboard period
    #[prost(string, tag="2")]
    pub last_date: ::prost::alloc::string::String,
    /// Leaderboard entries
    #[prost(message, repeated, tag="3")]
    pub leaders: ::prost::alloc::vec::Vec<LeaderboardRow>,
    /// Leaderboard entry for the querying account
    #[prost(message, optional, tag="4")]
    pub account_row: ::core::option::Option<LeaderboardRow>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolLeaderboardFixedResolutionRequest {
    /// Leaderboard resolution. Possible resolutions are 1D,1W,1M,6M,ALL
    #[prost(string, tag="1")]
    pub resolution: ::prost::alloc::string::String,
    /// Number of leaderboard entries to return
    #[prost(sint32, tag="2")]
    pub limit: i32,
    /// Account address that's querying the leaderboard
    #[prost(string, tag="3")]
    pub account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolLeaderboardFixedResolutionResponse {
    /// First date of snapshots used for the leaderboard period
    #[prost(string, tag="1")]
    pub first_date: ::prost::alloc::string::String,
    /// Last date of snapshots used for the leaderboard period
    #[prost(string, tag="2")]
    pub last_date: ::prost::alloc::string::String,
    /// Leaderboard entries
    #[prost(message, repeated, tag="3")]
    pub leaders: ::prost::alloc::vec::Vec<LeaderboardRow>,
    /// Leaderboard entry for the querying account
    #[prost(message, optional, tag="4")]
    pub account_row: ::core::option::Option<LeaderboardRow>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenomHoldersRequest {
    /// Denom address
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    /// Token for pagination
    #[prost(string, tag="2")]
    pub token: ::prost::alloc::string::String,
    #[prost(sint32, tag="3")]
    pub limit: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenomHoldersResponse {
    #[prost(message, repeated, tag="1")]
    pub holders: ::prost::alloc::vec::Vec<Holder>,
    /// Next tokens for pagination
    #[prost(string, repeated, tag="2")]
    pub next: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Total number of holders
    #[prost(sint32, tag="3")]
    pub total: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Holder {
    /// Account address for the holder
    #[prost(string, tag="1")]
    pub account_address: ::prost::alloc::string::String,
    /// The balance of the holder
    #[prost(string, tag="2")]
    pub balance: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistoricalTradesRequest {
    /// The starting block height that the trades must be equal or older than
    #[prost(uint64, tag="1")]
    pub from_block: u64,
    /// The ending block height that the trades must be equal or older than
    #[prost(uint64, tag="2")]
    pub end_block: u64,
    /// The starting timestamp in UNIX milliseconds that the trades must be equal or
    /// older than
    #[prost(sint64, tag="3")]
    pub from_time: i64,
    /// The ending timestamp in UNIX milliseconds that the trades must be equal or
    /// older than
    #[prost(sint64, tag="4")]
    pub end_time: i64,
    /// The number of trades to return per page
    #[prost(sint32, tag="5")]
    pub per_page: i32,
    /// Token for pagination
    #[prost(string, tag="6")]
    pub token: ::prost::alloc::string::String,
    /// Account address
    #[prost(string, tag="7")]
    pub account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistoricalTradesResponse {
    #[prost(message, repeated, tag="1")]
    pub trades: ::prost::alloc::vec::Vec<HistoricalTrade>,
    /// The last block height available in the service
    #[prost(uint64, tag="2")]
    pub last_height: u64,
    /// The timestamp of the last block available in the service
    #[prost(sint64, tag="3")]
    pub last_time: i64,
    /// Next token for pagination
    #[prost(string, repeated, tag="4")]
    pub next: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistoricalTrade {
    /// Account address
    #[prost(string, tag="1")]
    pub account: ::prost::alloc::string::String,
    /// The subaccountId that executed the trade
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// The ID of the market that this trade is in
    #[prost(string, tag="3")]
    pub market_id: ::prost::alloc::string::String,
    /// The direction the trade
    #[prost(string, tag="4")]
    pub trade_direction: ::prost::alloc::string::String,
    /// Price level at which trade has been executed
    #[prost(message, optional, tag="5")]
    pub price: ::core::option::Option<PriceLevel>,
    /// The fee associated with the trade (quote asset denom)
    #[prost(string, tag="6")]
    pub fee: ::prost::alloc::string::String,
    /// Timestamp of trade execution in UNIX millis
    #[prost(sint64, tag="7")]
    pub executed_at: i64,
    /// Block height of trade execution
    #[prost(uint64, tag="8")]
    pub executed_height: u64,
    /// Fee recipient address
    #[prost(string, tag="9")]
    pub fee_recipient: ::prost::alloc::string::String,
    /// Trade's execution side, maker/taker
    #[prost(string, tag="10")]
    pub execution_side: ::prost::alloc::string::String,
    /// USD value of the trade at the time of execution
    #[prost(string, tag="11")]
    pub usd_value: ::prost::alloc::string::String,
    /// A list of flag assigned to the trade
    #[prost(string, repeated, tag="12")]
    pub flags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Type of market
    #[prost(string, tag="13")]
    pub market_type: ::prost::alloc::string::String,
    /// Unique trade ID
    #[prost(string, tag="14")]
    pub trade_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceLevel {
    /// Price number of the price level.
    #[prost(string, tag="1")]
    pub price: ::prost::alloc::string::String,
    /// Quantity of the price level.
    #[prost(string, tag="2")]
    pub quantity: ::prost::alloc::string::String,
    /// Price level last updated timestamp in UNIX millis.
    #[prost(sint64, tag="3")]
    pub timestamp: i64,
}
include!("injective_archiver_rpc.tonic.rs");
// @@protoc_insertion_point(module)
