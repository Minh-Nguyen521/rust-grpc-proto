// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamRequest {
    #[prost(message, optional, tag="1")]
    pub bank_balances_filter: ::core::option::Option<BankBalancesFilter>,
    #[prost(message, optional, tag="2")]
    pub subaccount_deposits_filter: ::core::option::Option<SubaccountDepositsFilter>,
    #[prost(message, optional, tag="3")]
    pub spot_trades_filter: ::core::option::Option<TradesFilter>,
    #[prost(message, optional, tag="4")]
    pub derivative_trades_filter: ::core::option::Option<TradesFilter>,
    #[prost(message, optional, tag="5")]
    pub spot_orders_filter: ::core::option::Option<OrdersFilter>,
    #[prost(message, optional, tag="6")]
    pub derivative_orders_filter: ::core::option::Option<OrdersFilter>,
    #[prost(message, optional, tag="7")]
    pub spot_orderbooks_filter: ::core::option::Option<OrderbookFilter>,
    #[prost(message, optional, tag="8")]
    pub derivative_orderbooks_filter: ::core::option::Option<OrderbookFilter>,
    #[prost(message, optional, tag="9")]
    pub positions_filter: ::core::option::Option<PositionsFilter>,
    #[prost(message, optional, tag="10")]
    pub oracle_price_filter: ::core::option::Option<OraclePriceFilter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamResponse {
    #[prost(uint64, tag="1")]
    pub block_height: u64,
    #[prost(int64, tag="2")]
    pub block_time: i64,
    #[prost(message, repeated, tag="3")]
    pub bank_balances: ::prost::alloc::vec::Vec<BankBalance>,
    #[prost(message, repeated, tag="4")]
    pub subaccount_deposits: ::prost::alloc::vec::Vec<SubaccountDeposits>,
    #[prost(message, repeated, tag="5")]
    pub spot_trades: ::prost::alloc::vec::Vec<SpotTrade>,
    #[prost(message, repeated, tag="6")]
    pub derivative_trades: ::prost::alloc::vec::Vec<DerivativeTrade>,
    #[prost(message, repeated, tag="7")]
    pub spot_orders: ::prost::alloc::vec::Vec<SpotOrderUpdate>,
    #[prost(message, repeated, tag="8")]
    pub derivative_orders: ::prost::alloc::vec::Vec<DerivativeOrderUpdate>,
    #[prost(message, repeated, tag="9")]
    pub spot_orderbook_updates: ::prost::alloc::vec::Vec<OrderbookUpdate>,
    #[prost(message, repeated, tag="10")]
    pub derivative_orderbook_updates: ::prost::alloc::vec::Vec<OrderbookUpdate>,
    #[prost(message, repeated, tag="11")]
    pub positions: ::prost::alloc::vec::Vec<Position>,
    #[prost(message, repeated, tag="12")]
    pub oracle_prices: ::prost::alloc::vec::Vec<OraclePrice>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderbookUpdate {
    #[prost(uint64, tag="1")]
    pub seq: u64,
    #[prost(message, optional, tag="2")]
    pub orderbook: ::core::option::Option<Orderbook>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Orderbook {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub buy_levels: ::prost::alloc::vec::Vec<super::super::exchange::v1beta1::Level>,
    #[prost(message, repeated, tag="3")]
    pub sell_levels: ::prost::alloc::vec::Vec<super::super::exchange::v1beta1::Level>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BankBalance {
    #[prost(string, tag="1")]
    pub account: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub balances: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountDeposits {
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub deposits: ::prost::alloc::vec::Vec<SubaccountDeposit>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountDeposit {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub deposit: ::core::option::Option<super::super::exchange::v1beta1::Deposit>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotOrderUpdate {
    #[prost(enumeration="OrderUpdateStatus", tag="1")]
    pub status: i32,
    #[prost(string, tag="2")]
    pub order_hash: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub cid: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub order: ::core::option::Option<SpotOrder>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotOrder {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub order: ::core::option::Option<super::super::exchange::v1beta1::SpotLimitOrder>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeOrderUpdate {
    #[prost(enumeration="OrderUpdateStatus", tag="1")]
    pub status: i32,
    #[prost(string, tag="2")]
    pub order_hash: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub cid: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub order: ::core::option::Option<DerivativeOrder>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeOrder {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub order: ::core::option::Option<super::super::exchange::v1beta1::DerivativeLimitOrder>,
    #[prost(bool, tag="3")]
    pub is_market: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub is_long: bool,
    #[prost(string, tag="4")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub entry_price: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub margin: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub cumulative_funding_entry: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OraclePrice {
    #[prost(string, tag="1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub r#type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotTrade {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub is_buy: bool,
    #[prost(string, tag="3")]
    pub execution_type: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub price: ::prost::alloc::string::String,
    /// bytes32 subaccount ID that executed the trade
    #[prost(string, tag="6")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub fee: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub order_hash: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub fee_recipient_address: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub cid: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub trade_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeTrade {
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub is_buy: bool,
    #[prost(string, tag="3")]
    pub execution_type: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub position_delta: ::core::option::Option<super::super::exchange::v1beta1::PositionDelta>,
    #[prost(string, tag="6")]
    pub payout: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub fee: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub order_hash: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub fee_recipient_address: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub cid: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub trade_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradesFilter {
    #[prost(string, repeated, tag="1")]
    pub subaccount_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsFilter {
    #[prost(string, repeated, tag="1")]
    pub subaccount_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrdersFilter {
    #[prost(string, repeated, tag="1")]
    pub subaccount_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="2")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderbookFilter {
    #[prost(string, repeated, tag="1")]
    pub market_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BankBalancesFilter {
    #[prost(string, repeated, tag="1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountDepositsFilter {
    #[prost(string, repeated, tag="1")]
    pub subaccount_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OraclePriceFilter {
    #[prost(string, repeated, tag="1")]
    pub symbol: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderUpdateStatus {
    Unspecified = 0,
    Booked = 1,
    Matched = 2,
    Cancelled = 3,
}
impl OrderUpdateStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderUpdateStatus::Unspecified => "Unspecified",
            OrderUpdateStatus::Booked => "Booked",
            OrderUpdateStatus::Matched => "Matched",
            OrderUpdateStatus::Cancelled => "Cancelled",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unspecified" => Some(Self::Unspecified),
            "Booked" => Some(Self::Booked),
            "Matched" => Some(Self::Matched),
            "Cancelled" => Some(Self::Cancelled),
            _ => None,
        }
    }
}
include!("injective.stream.v1beta1.tonic.rs");
// @@protoc_insertion_point(module)
