// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioRequest {
    /// Account address
    #[prost(string, tag="1")]
    pub account_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioResponse {
    /// The portfolio of this account
    #[prost(message, optional, tag="1")]
    pub portfolio: ::core::option::Option<AccountPortfolio>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountPortfolio {
    /// The account's portfolio value in USD.
    #[prost(string, tag="1")]
    pub portfolio_value: ::prost::alloc::string::String,
    /// The account's available balance value in USD.
    #[prost(string, tag="2")]
    pub available_balance: ::prost::alloc::string::String,
    /// The account's locked balance value in USD.
    #[prost(string, tag="3")]
    pub locked_balance: ::prost::alloc::string::String,
    /// The account's total unrealized PnL value in USD.
    #[prost(string, tag="4")]
    pub unrealized_pnl: ::prost::alloc::string::String,
    /// List of all subaccounts' portfolio
    #[prost(message, repeated, tag="5")]
    pub subaccounts: ::prost::alloc::vec::Vec<SubaccountPortfolio>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountPortfolio {
    /// The ID of this subaccount
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// The subaccount's available balance value in USD.
    #[prost(string, tag="2")]
    pub available_balance: ::prost::alloc::string::String,
    /// The subaccount's locked balance value in USD.
    #[prost(string, tag="3")]
    pub locked_balance: ::prost::alloc::string::String,
    /// The subaccount's total unrealized PnL value in USD.
    #[prost(string, tag="4")]
    pub unrealized_pnl: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderStatesRequest {
    #[prost(string, repeated, tag="1")]
    pub spot_order_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="2")]
    pub derivative_order_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderStatesResponse {
    /// List of the spot order state records
    #[prost(message, repeated, tag="1")]
    pub spot_order_states: ::prost::alloc::vec::Vec<OrderStateRecord>,
    /// List of the derivative order state records
    #[prost(message, repeated, tag="2")]
    pub derivative_order_states: ::prost::alloc::vec::Vec<OrderStateRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderStateRecord {
    /// Hash of the order
    #[prost(string, tag="1")]
    pub order_hash: ::prost::alloc::string::String,
    /// The subaccountId that this order belongs to
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// The Market ID of the order
    #[prost(string, tag="3")]
    pub market_id: ::prost::alloc::string::String,
    /// The type of the order
    #[prost(string, tag="4")]
    pub order_type: ::prost::alloc::string::String,
    /// The side of the order
    #[prost(string, tag="5")]
    pub order_side: ::prost::alloc::string::String,
    /// The state (status) of the order
    #[prost(string, tag="6")]
    pub state: ::prost::alloc::string::String,
    /// The filled quantity of the order
    #[prost(string, tag="7")]
    pub quantity_filled: ::prost::alloc::string::String,
    /// The filled quantity of the order
    #[prost(string, tag="8")]
    pub quantity_remaining: ::prost::alloc::string::String,
    /// Order committed timestamp in UNIX millis.
    #[prost(sint64, tag="9")]
    pub created_at: i64,
    /// Order updated timestamp in UNIX millis.
    #[prost(sint64, tag="10")]
    pub updated_at: i64,
    /// Order prices
    #[prost(string, tag="11")]
    pub price: ::prost::alloc::string::String,
    /// Margin for derivative order
    #[prost(string, tag="12")]
    pub margin: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountsListRequest {
    /// Account address, the subaccounts owner
    #[prost(string, tag="1")]
    pub account_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountsListResponse {
    #[prost(string, repeated, tag="1")]
    pub subaccounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountBalancesListRequest {
    /// SubaccountId of the trader we want to get the trades from
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Filter balances by denoms. If not set, the balances of all the denoms for
    /// the subaccount are provided.
    #[prost(string, repeated, tag="2")]
    pub denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountBalancesListResponse {
    /// List of subaccount balances
    #[prost(message, repeated, tag="1")]
    pub balances: ::prost::alloc::vec::Vec<SubaccountBalance>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountBalance {
    /// Related subaccount ID
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Account address, owner of this subaccount
    #[prost(string, tag="2")]
    pub account_address: ::prost::alloc::string::String,
    /// Coin denom on the chain.
    #[prost(string, tag="3")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
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
pub struct SubaccountBalanceEndpointRequest {
    /// SubaccountId of the trader we want to get the trades from
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Specify denom to get balance
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountBalanceEndpointResponse {
    /// Subaccount balance
    #[prost(message, optional, tag="1")]
    pub balance: ::core::option::Option<SubaccountBalance>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamSubaccountBalanceRequest {
    /// SubaccountId of the trader we want to get the trades from
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Filter balances by denoms. If not set, the balances of all the denoms for
    /// the subaccount are provided.
    #[prost(string, repeated, tag="2")]
    pub denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamSubaccountBalanceResponse {
    /// Subaccount balance
    #[prost(message, optional, tag="1")]
    pub balance: ::core::option::Option<SubaccountBalance>,
    /// Operation timestamp in UNIX millis.
    #[prost(sint64, tag="2")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountHistoryRequest {
    /// SubaccountId of the trader we want to get the history from
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Filter history by denom
    #[prost(string, tag="2")]
    pub denom: ::prost::alloc::string::String,
    /// Filter history by transfer type
    #[prost(string, repeated, tag="3")]
    pub transfer_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Skip will skip the first n item from the result
    #[prost(uint64, tag="4")]
    pub skip: u64,
    /// Limit is used to specify the maximum number of items to be returned
    #[prost(sint32, tag="5")]
    pub limit: i32,
    /// Upper bound of account transfer history's executedAt
    #[prost(sint64, tag="6")]
    pub end_time: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountHistoryResponse {
    /// List of subaccount transfers
    #[prost(message, repeated, tag="1")]
    pub transfers: ::prost::alloc::vec::Vec<SubaccountBalanceTransfer>,
    #[prost(message, optional, tag="2")]
    pub paging: ::core::option::Option<Paging>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountBalanceTransfer {
    /// Type of the subaccount balance transfer
    #[prost(string, tag="1")]
    pub transfer_type: ::prost::alloc::string::String,
    /// Subaccount ID of the sending side
    #[prost(string, tag="2")]
    pub src_subaccount_id: ::prost::alloc::string::String,
    /// Account address of the sending side
    #[prost(string, tag="3")]
    pub src_account_address: ::prost::alloc::string::String,
    /// Subaccount ID of the receiving side
    #[prost(string, tag="4")]
    pub dst_subaccount_id: ::prost::alloc::string::String,
    /// Account address of the receiving side
    #[prost(string, tag="5")]
    pub dst_account_address: ::prost::alloc::string::String,
    /// Coin amount of the transfer
    #[prost(message, optional, tag="6")]
    pub amount: ::core::option::Option<CosmosCoin>,
    /// Timestamp of the transfer in UNIX millis
    #[prost(sint64, tag="7")]
    pub executed_at: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CosmosCoin {
    /// Coin denominator
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    /// Coin amount (big int)
    #[prost(string, tag="2")]
    pub amount: ::prost::alloc::string::String,
}
/// Paging defines the structure for required params for handling pagination
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Paging {
    /// total number of txs saved in database
    #[prost(sint64, tag="1")]
    pub total: i64,
    /// can be either block height or index num
    #[prost(sint32, tag="2")]
    pub from: i32,
    /// can be either block height or index num
    #[prost(sint32, tag="3")]
    pub to: i32,
    /// count entries by subaccount, serving some places on helix
    #[prost(sint64, tag="4")]
    pub count_by_subaccount: i64,
    /// array of tokens to navigate to the next pages
    #[prost(string, repeated, tag="5")]
    pub next: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountOrderSummaryRequest {
    /// SubaccountId of the trader we want to get the summary from
    #[prost(string, tag="1")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// MarketId is limiting order summary to specific market only
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    /// Filter by direction of the orders
    #[prost(string, tag="3")]
    pub order_direction: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountOrderSummaryResponse {
    /// Total count of subaccount's spot orders in given market and direction
    #[prost(sint64, tag="1")]
    pub spot_orders_total: i64,
    /// Total count of subaccount's derivative orders in given market and direction
    #[prost(sint64, tag="2")]
    pub derivative_orders_total: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardsRequest {
    /// The distribution epoch sequence number. -1 for latest.
    #[prost(sint64, tag="1")]
    pub epoch: i64,
    /// Account address for the rewards distribution
    #[prost(string, tag="2")]
    pub account_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardsResponse {
    /// The trading rewards distributed
    #[prost(message, repeated, tag="1")]
    pub rewards: ::prost::alloc::vec::Vec<Reward>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reward {
    /// Account address
    #[prost(string, tag="1")]
    pub account_address: ::prost::alloc::string::String,
    /// Reward coins distributed
    #[prost(message, repeated, tag="2")]
    pub rewards: ::prost::alloc::vec::Vec<Coin>,
    /// Rewards distribution timestamp in UNIX millis.
    #[prost(sint64, tag="3")]
    pub distributed_at: i64,
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
pub struct StreamAccountDataRequest {
    /// account address
    #[prost(string, tag="1")]
    pub account_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamAccountDataResponse {
    #[prost(message, optional, tag="1")]
    pub subaccount_balance: ::core::option::Option<SubaccountBalanceResult>,
    #[prost(message, optional, tag="2")]
    pub position: ::core::option::Option<PositionsResult>,
    #[prost(message, optional, tag="3")]
    pub trade: ::core::option::Option<TradeResult>,
    #[prost(message, optional, tag="4")]
    pub order: ::core::option::Option<OrderResult>,
    #[prost(message, optional, tag="5")]
    pub order_history: ::core::option::Option<OrderHistoryResult>,
    #[prost(message, optional, tag="6")]
    pub funding_payment: ::core::option::Option<FundingPaymentResult>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountBalanceResult {
    /// Subaccount balance
    #[prost(message, optional, tag="1")]
    pub balance: ::core::option::Option<SubaccountBalance>,
    /// Operation timestamp in UNIX millis.
    #[prost(sint64, tag="2")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsResult {
    /// Updated derivative Position
    #[prost(message, optional, tag="1")]
    pub position: ::core::option::Option<Position>,
    /// Operation timestamp in UNIX millis.
    #[prost(sint64, tag="2")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
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
    /// Position updated timestamp in UNIX millis.
    #[prost(sint64, tag="10")]
    pub updated_at: i64,
    /// Position created timestamp in UNIX millis.
    #[prost(sint64, tag="11")]
    pub created_at: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeResult {
    /// Executed trades update type
    #[prost(string, tag="3")]
    pub operation_type: ::prost::alloc::string::String,
    /// Operation timestamp in UNIX millis.
    #[prost(sint64, tag="4")]
    pub timestamp: i64,
    #[prost(oneof="trade_result::Trade", tags="1, 2")]
    pub trade: ::core::option::Option<trade_result::Trade>,
}
/// Nested message and enum types in `TradeResult`.
pub mod trade_result {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Trade {
        /// New spot market trade
        #[prost(message, tag="1")]
        SpotTrade(super::SpotTrade),
        /// New derivative market trade
        #[prost(message, tag="2")]
        DerivativeTrade(super::DerivativeTrade),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotTrade {
    /// Maker order hash.
    #[prost(string, tag="1")]
    pub order_hash: ::prost::alloc::string::String,
    /// The subaccountId that executed the trade
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// The ID of the market that this trade is in
    #[prost(string, tag="3")]
    pub market_id: ::prost::alloc::string::String,
    /// The execution type of the trade
    #[prost(string, tag="4")]
    pub trade_execution_type: ::prost::alloc::string::String,
    /// The direction the trade
    #[prost(string, tag="5")]
    pub trade_direction: ::prost::alloc::string::String,
    /// Price level at which trade has been executed
    #[prost(message, optional, tag="6")]
    pub price: ::core::option::Option<PriceLevel>,
    /// The fee associated with the trade (quote asset denom)
    #[prost(string, tag="7")]
    pub fee: ::prost::alloc::string::String,
    /// Timestamp of trade execution in UNIX millis
    #[prost(sint64, tag="8")]
    pub executed_at: i64,
    /// Fee recipient address
    #[prost(string, tag="9")]
    pub fee_recipient: ::prost::alloc::string::String,
    /// A unique string that helps differentiate between trades
    #[prost(string, tag="10")]
    pub trade_id: ::prost::alloc::string::String,
    /// Trade's execution side, marker/taker
    #[prost(string, tag="11")]
    pub execution_side: ::prost::alloc::string::String,
    /// Custom client order ID
    #[prost(string, tag="12")]
    pub cid: ::prost::alloc::string::String,
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeTrade {
    /// Order hash.
    #[prost(string, tag="1")]
    pub order_hash: ::prost::alloc::string::String,
    /// The subaccountId that executed the trade
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// The ID of the market that this trade is in
    #[prost(string, tag="3")]
    pub market_id: ::prost::alloc::string::String,
    /// The execution type of the trade
    #[prost(string, tag="4")]
    pub trade_execution_type: ::prost::alloc::string::String,
    /// True if the trade is a liquidation
    #[prost(bool, tag="5")]
    pub is_liquidation: bool,
    /// Position Delta from the trade
    #[prost(message, optional, tag="6")]
    pub position_delta: ::core::option::Option<PositionDelta>,
    /// The payout associated with the trade
    #[prost(string, tag="7")]
    pub payout: ::prost::alloc::string::String,
    /// The fee associated with the trade
    #[prost(string, tag="8")]
    pub fee: ::prost::alloc::string::String,
    /// Timestamp of trade execution in UNIX millis
    #[prost(sint64, tag="9")]
    pub executed_at: i64,
    /// Fee recipient address
    #[prost(string, tag="10")]
    pub fee_recipient: ::prost::alloc::string::String,
    /// A unique string that helps differentiate between trades
    #[prost(string, tag="11")]
    pub trade_id: ::prost::alloc::string::String,
    /// Trade's execution side, marker/taker
    #[prost(string, tag="12")]
    pub execution_side: ::prost::alloc::string::String,
    /// Custom client order ID
    #[prost(string, tag="13")]
    pub cid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionDelta {
    /// The direction the trade
    #[prost(string, tag="1")]
    pub trade_direction: ::prost::alloc::string::String,
    /// Execution Price of the trade.
    #[prost(string, tag="2")]
    pub execution_price: ::prost::alloc::string::String,
    /// Execution Quantity of the trade.
    #[prost(string, tag="3")]
    pub execution_quantity: ::prost::alloc::string::String,
    /// Execution Margin of the trade.
    #[prost(string, tag="4")]
    pub execution_margin: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderResult {
    /// Executed orders update type
    #[prost(string, tag="3")]
    pub operation_type: ::prost::alloc::string::String,
    /// Operation timestamp in UNIX millis.
    #[prost(sint64, tag="4")]
    pub timestamp: i64,
    #[prost(oneof="order_result::Order", tags="1, 2")]
    pub order: ::core::option::Option<order_result::Order>,
}
/// Nested message and enum types in `OrderResult`.
pub mod order_result {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Order {
        /// Updated spot market order
        #[prost(message, tag="1")]
        SpotOrder(super::SpotLimitOrder),
        /// Updated derivative market order
        #[prost(message, tag="2")]
        DerivativeOrder(super::DerivativeLimitOrder),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotLimitOrder {
    /// Hash of the order
    #[prost(string, tag="1")]
    pub order_hash: ::prost::alloc::string::String,
    /// The side of the order
    #[prost(string, tag="2")]
    pub order_side: ::prost::alloc::string::String,
    /// Spot Market ID is keccak265(baseDenom + quoteDenom)
    #[prost(string, tag="3")]
    pub market_id: ::prost::alloc::string::String,
    /// The subaccountId that this order belongs to
    #[prost(string, tag="4")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Price of the order
    #[prost(string, tag="5")]
    pub price: ::prost::alloc::string::String,
    /// Quantity of the order
    #[prost(string, tag="6")]
    pub quantity: ::prost::alloc::string::String,
    /// The amount of the quantity remaining unfilled
    #[prost(string, tag="7")]
    pub unfilled_quantity: ::prost::alloc::string::String,
    /// Trigger price is the trigger price used by stop/take orders. 0 if the
    /// trigger price is not set.
    #[prost(string, tag="8")]
    pub trigger_price: ::prost::alloc::string::String,
    /// Fee recipient address
    #[prost(string, tag="9")]
    pub fee_recipient: ::prost::alloc::string::String,
    /// Order state
    #[prost(string, tag="10")]
    pub state: ::prost::alloc::string::String,
    /// Order committed timestamp in UNIX millis.
    #[prost(sint64, tag="11")]
    pub created_at: i64,
    /// Order updated timestamp in UNIX millis.
    #[prost(sint64, tag="12")]
    pub updated_at: i64,
    /// Transaction Hash where order is created. Not all orders have this field
    #[prost(string, tag="13")]
    pub tx_hash: ::prost::alloc::string::String,
    /// Custom client order ID
    #[prost(string, tag="14")]
    pub cid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeLimitOrder {
    /// Hash of the order
    #[prost(string, tag="1")]
    pub order_hash: ::prost::alloc::string::String,
    /// The side of the order
    #[prost(string, tag="2")]
    pub order_side: ::prost::alloc::string::String,
    /// Derivative Market ID
    #[prost(string, tag="3")]
    pub market_id: ::prost::alloc::string::String,
    /// The subaccountId that this order belongs to
    #[prost(string, tag="4")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// True if the order is a reduce-only order
    #[prost(bool, tag="5")]
    pub is_reduce_only: bool,
    /// Margin of the order
    #[prost(string, tag="6")]
    pub margin: ::prost::alloc::string::String,
    /// Price of the order
    #[prost(string, tag="7")]
    pub price: ::prost::alloc::string::String,
    /// Quantity of the order
    #[prost(string, tag="8")]
    pub quantity: ::prost::alloc::string::String,
    /// The amount of the quantity remaining unfilled
    #[prost(string, tag="9")]
    pub unfilled_quantity: ::prost::alloc::string::String,
    /// Trigger price is the trigger price used by stop/take orders
    #[prost(string, tag="10")]
    pub trigger_price: ::prost::alloc::string::String,
    /// Fee recipient address
    #[prost(string, tag="11")]
    pub fee_recipient: ::prost::alloc::string::String,
    /// Order state
    #[prost(string, tag="12")]
    pub state: ::prost::alloc::string::String,
    /// Order committed timestamp in UNIX millis.
    #[prost(sint64, tag="13")]
    pub created_at: i64,
    /// Order updated timestamp in UNIX millis.
    #[prost(sint64, tag="14")]
    pub updated_at: i64,
    /// Order number of subaccount
    #[prost(sint64, tag="15")]
    pub order_number: i64,
    /// Order type
    #[prost(string, tag="16")]
    pub order_type: ::prost::alloc::string::String,
    /// Order type
    #[prost(bool, tag="17")]
    pub is_conditional: bool,
    /// Trigger timestamp, only exists for conditional orders
    #[prost(uint64, tag="18")]
    pub trigger_at: u64,
    /// OrderHash of order that is triggered by this conditional order
    #[prost(string, tag="19")]
    pub placed_order_hash: ::prost::alloc::string::String,
    /// Execution type of conditional order
    #[prost(string, tag="20")]
    pub execution_type: ::prost::alloc::string::String,
    /// Transaction Hash where order is created. Not all orders have this field
    #[prost(string, tag="21")]
    pub tx_hash: ::prost::alloc::string::String,
    /// Custom client order ID
    #[prost(string, tag="22")]
    pub cid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderHistoryResult {
    /// Order update type
    #[prost(string, tag="3")]
    pub operation_type: ::prost::alloc::string::String,
    /// Operation timestamp in UNIX millis.
    #[prost(sint64, tag="4")]
    pub timestamp: i64,
    #[prost(oneof="order_history_result::OrderHistory", tags="1, 2")]
    pub order_history: ::core::option::Option<order_history_result::OrderHistory>,
}
/// Nested message and enum types in `OrderHistoryResult`.
pub mod order_history_result {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OrderHistory {
        /// Spot order history
        #[prost(message, tag="1")]
        SpotOrderHistory(super::SpotOrderHistory),
        /// Derivative order history
        #[prost(message, tag="2")]
        DerivativeOrderHistory(super::DerivativeOrderHistory),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpotOrderHistory {
    /// Hash of the order
    #[prost(string, tag="1")]
    pub order_hash: ::prost::alloc::string::String,
    /// Spot Market ID is keccak265(baseDenom + quoteDenom)
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    /// active state of the order
    #[prost(bool, tag="3")]
    pub is_active: bool,
    /// The subaccountId that this order belongs to
    #[prost(string, tag="4")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// The execution type
    #[prost(string, tag="5")]
    pub execution_type: ::prost::alloc::string::String,
    /// The side of the order
    #[prost(string, tag="6")]
    pub order_type: ::prost::alloc::string::String,
    /// Price of the order
    #[prost(string, tag="7")]
    pub price: ::prost::alloc::string::String,
    /// Trigger price
    #[prost(string, tag="8")]
    pub trigger_price: ::prost::alloc::string::String,
    /// Quantity of the order
    #[prost(string, tag="9")]
    pub quantity: ::prost::alloc::string::String,
    /// Filled amount
    #[prost(string, tag="10")]
    pub filled_quantity: ::prost::alloc::string::String,
    /// Order state
    #[prost(string, tag="11")]
    pub state: ::prost::alloc::string::String,
    /// Order committed timestamp in UNIX millis.
    #[prost(sint64, tag="12")]
    pub created_at: i64,
    /// Order updated timestamp in UNIX millis.
    #[prost(sint64, tag="13")]
    pub updated_at: i64,
    /// Order direction (order side)
    #[prost(string, tag="14")]
    pub direction: ::prost::alloc::string::String,
    /// Transaction Hash where order is created. Not all orders have this field
    #[prost(string, tag="15")]
    pub tx_hash: ::prost::alloc::string::String,
    /// Custom client order ID
    #[prost(string, tag="16")]
    pub cid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DerivativeOrderHistory {
    /// Hash of the order
    #[prost(string, tag="1")]
    pub order_hash: ::prost::alloc::string::String,
    /// Spot Market ID is keccak265(baseDenom + quoteDenom)
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    /// active state of the order
    #[prost(bool, tag="3")]
    pub is_active: bool,
    /// The subaccountId that this order belongs to
    #[prost(string, tag="4")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// The execution type
    #[prost(string, tag="5")]
    pub execution_type: ::prost::alloc::string::String,
    /// The side of the order
    #[prost(string, tag="6")]
    pub order_type: ::prost::alloc::string::String,
    /// Price of the order
    #[prost(string, tag="7")]
    pub price: ::prost::alloc::string::String,
    /// Trigger price
    #[prost(string, tag="8")]
    pub trigger_price: ::prost::alloc::string::String,
    /// Quantity of the order
    #[prost(string, tag="9")]
    pub quantity: ::prost::alloc::string::String,
    /// Filled amount
    #[prost(string, tag="10")]
    pub filled_quantity: ::prost::alloc::string::String,
    /// Order state
    #[prost(string, tag="11")]
    pub state: ::prost::alloc::string::String,
    /// Order committed timestamp in UNIX millis.
    #[prost(sint64, tag="12")]
    pub created_at: i64,
    /// Order updated timestamp in UNIX millis.
    #[prost(sint64, tag="13")]
    pub updated_at: i64,
    /// True if an order is reduce only
    #[prost(bool, tag="14")]
    pub is_reduce_only: bool,
    /// Order direction (order side)
    #[prost(string, tag="15")]
    pub direction: ::prost::alloc::string::String,
    /// True if this is conditional order, otherwise false
    #[prost(bool, tag="16")]
    pub is_conditional: bool,
    /// Trigger timestamp in unix milli
    #[prost(uint64, tag="17")]
    pub trigger_at: u64,
    /// Order hash placed when this triggers
    #[prost(string, tag="18")]
    pub placed_order_hash: ::prost::alloc::string::String,
    /// Order's margin
    #[prost(string, tag="19")]
    pub margin: ::prost::alloc::string::String,
    /// Transaction Hash where order is created. Not all orders have this field
    #[prost(string, tag="20")]
    pub tx_hash: ::prost::alloc::string::String,
    /// Custom client order ID
    #[prost(string, tag="21")]
    pub cid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundingPaymentResult {
    /// Funding payments of the account
    #[prost(message, optional, tag="1")]
    pub funding_payments: ::core::option::Option<FundingPayment>,
    /// Funding payments type
    #[prost(string, tag="2")]
    pub operation_type: ::prost::alloc::string::String,
    /// Operation timestamp in UNIX millis.
    #[prost(sint64, tag="4")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FundingPayment {
    /// Derivative Market ID
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// The subaccountId that the position belongs to
    #[prost(string, tag="2")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Amount of the funding payment
    #[prost(string, tag="3")]
    pub amount: ::prost::alloc::string::String,
    /// Timestamp of funding payment in UNIX millis
    #[prost(sint64, tag="4")]
    pub timestamp: i64,
}
include!("injective_accounts_rpc.tonic.rs");
// @@protoc_insertion_point(module)
