// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenHoldersRequest {
    /// Denom of the token
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    /// Cursor for pagination
    #[prost(string, tag="2")]
    pub cursor: ::prost::alloc::string::String,
    #[prost(sint32, tag="3")]
    pub limit: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenHoldersResponse {
    #[prost(message, repeated, tag="1")]
    pub holders: ::prost::alloc::vec::Vec<Holder>,
    /// Next cursors for pagination
    #[prost(string, repeated, tag="2")]
    pub next_cursors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
pub struct AccountPortfolioRequest {
    /// Account address
    #[prost(string, tag="1")]
    pub account_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountPortfolioResponse {
    /// The portfolio of this account
    #[prost(message, optional, tag="1")]
    pub portfolio: ::core::option::Option<Portfolio>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Portfolio {
    /// The account's portfolio address
    #[prost(string, tag="1")]
    pub account_address: ::prost::alloc::string::String,
    /// Account available bank balances
    #[prost(message, repeated, tag="2")]
    pub bank_balances: ::prost::alloc::vec::Vec<Coin>,
    /// Subaccounts list
    #[prost(message, repeated, tag="3")]
    pub subaccounts: ::prost::alloc::vec::Vec<SubaccountBalanceV2>,
    /// All positions for all subaccounts, with unrealized PNL
    #[prost(message, repeated, tag="4")]
    pub positions_with_upnl: ::prost::alloc::vec::Vec<PositionsWithUpnl>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Coin {
    /// Denom of the coin
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountBalanceV2 {
    /// Related subaccount ID
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Coin denom on the chain.
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub deposit: ::core::option::Option<SubaccountDeposit>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountDeposit {
    #[prost(string, tag="1")]
    pub total_balance: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub available_balance: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsWithUpnl {
    #[prost(message, optional, tag="1")]
    pub position: ::core::option::Option<DerivativePosition>,
    /// Unrealized PNL
    #[prost(string, tag="2")]
    pub unrealized_pnl: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativePosition {
    /// Ticker of the derivative market
    #[prost(string, tag="1")]
    pub ticker: ::prost::alloc::string::String,
    /// Derivative Market ID
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    /// The subaccountId that the position belongs to
    #[prost(string, tag="3")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Direction of the position
    #[prost(string, tag="4")]
    pub direction: ::prost::alloc::string::String,
    /// Quantity of the position
    #[prost(string, tag="5")]
    pub quantity: ::prost::alloc::string::String,
    /// Price of the position
    #[prost(string, tag="6")]
    pub entry_price: ::prost::alloc::string::String,
    /// Margin of the position
    #[prost(string, tag="7")]
    pub margin: ::prost::alloc::string::String,
    /// LiquidationPrice of the position
    #[prost(string, tag="8")]
    pub liquidation_price: ::prost::alloc::string::String,
    /// MarkPrice of the position
    #[prost(string, tag="9")]
    pub mark_price: ::prost::alloc::string::String,
    /// Aggregate Quantity of the Reduce Only orders associated with the position
    #[prost(string, tag="11")]
    pub aggregate_reduce_only_quantity: ::prost::alloc::string::String,
    /// Position updated timestamp in UNIX millis.
    #[prost(sint64, tag="12")]
    pub updated_at: i64,
    /// Position created timestamp in UNIX millis.
    #[prost(sint64, tag="13")]
    pub created_at: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountPortfolioBalancesRequest {
    /// Account address
    #[prost(string, tag="1")]
    pub account_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountPortfolioBalancesResponse {
    /// The portfolio balances of this account
    #[prost(message, optional, tag="1")]
    pub portfolio: ::core::option::Option<PortfolioBalances>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioBalances {
    /// The account's portfolio address
    #[prost(string, tag="1")]
    pub account_address: ::prost::alloc::string::String,
    /// Account available bank balances
    #[prost(message, repeated, tag="2")]
    pub bank_balances: ::prost::alloc::vec::Vec<Coin>,
    /// Subaccounts list
    #[prost(message, repeated, tag="3")]
    pub subaccounts: ::prost::alloc::vec::Vec<SubaccountBalanceV2>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamAccountPortfolioRequest {
    /// The account's portfolio address
    #[prost(string, tag="1")]
    pub account_address: ::prost::alloc::string::String,
    /// Related subaccount ID
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub r#type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamAccountPortfolioResponse {
    /// type of portfolio entry
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    /// denom of portfolio entry
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
    /// amount of portfolio entry
    #[prost(string, tag="3")]
    pub amount: ::prost::alloc::string::String,
    /// subaccount id of portfolio entry
    #[prost(string, tag="4")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Operation timestamp in UNIX millis.
    #[prost(sint64, tag="5")]
    pub timestamp: i64,
}
include!("injective_portfolio_rpc.tonic.rs");
// @@protoc_insertion_point(module)
