// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTradingStrategiesRequest {
    #[prost(string, tag="1")]
    pub state: ::prost::alloc::string::String,
    /// MarketId of the trading strategy
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    /// subaccount ID to filter by
    #[prost(string, tag="3")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Account address
    #[prost(string, tag="4")]
    pub account_address: ::prost::alloc::string::String,
    /// Indicates whether the trading strategy is pending execution
    #[prost(bool, tag="5")]
    pub pending_execution: bool,
    /// The starting timestamp in UNIX milliseconds for the creation time of the
    /// trading strategy
    #[prost(sint64, tag="6")]
    pub start_time: i64,
    /// The ending timestamp in UNIX milliseconds for the creation time of the
    /// trading strategy
    #[prost(sint64, tag="7")]
    pub end_time: i64,
    #[prost(sint32, tag="8")]
    pub limit: i32,
    #[prost(uint64, tag="9")]
    pub skip: u64,
    /// Filter by strategy type
    #[prost(string, repeated, tag="10")]
    pub strategy_type: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Filter by market type
    #[prost(string, tag="11")]
    pub market_type: ::prost::alloc::string::String,
    /// The last executed timestamp in UNIX milliseconds for the last executed time
    /// of the trading strategy
    #[prost(sint64, tag="12")]
    pub last_executed_time: i64,
    /// Include TVL in the response
    #[prost(bool, tag="13")]
    pub with_tvl: bool,
    /// Indicates whether the trading strategy is a trailing strategy
    #[prost(bool, tag="14")]
    pub is_trailing_strategy: bool,
    /// Indicates whether the trading strategy performance should be included in the
    /// response
    #[prost(bool, tag="15")]
    pub with_performance: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTradingStrategiesResponse {
    /// The trading strategies
    #[prost(message, repeated, tag="1")]
    pub strategies: ::prost::alloc::vec::Vec<TradingStrategy>,
    #[prost(message, optional, tag="2")]
    pub paging: ::core::option::Option<Paging>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingStrategy {
    #[prost(string, tag="1")]
    pub state: ::prost::alloc::string::String,
    /// MarketId of the trading strategy
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    /// subaccount ID of the trading strategy
    #[prost(string, tag="3")]
    pub subaccount_id: ::prost::alloc::string::String,
    /// Account address
    #[prost(string, tag="4")]
    pub account_address: ::prost::alloc::string::String,
    /// Contract address
    #[prost(string, tag="5")]
    pub contract_address: ::prost::alloc::string::String,
    /// Execution price of the trading strategy
    #[prost(string, tag="6")]
    pub execution_price: ::prost::alloc::string::String,
    /// Base quantity of the trading strategy
    #[prost(string, tag="7")]
    pub base_quantity: ::prost::alloc::string::String,
    /// Quote quantity of the trading strategy
    #[prost(string, tag="20")]
    pub quote_quantity: ::prost::alloc::string::String,
    /// Lower bound of the trading strategy
    #[prost(string, tag="8")]
    pub lower_bound: ::prost::alloc::string::String,
    /// Upper bound of the trading strategy
    #[prost(string, tag="9")]
    pub upper_bound: ::prost::alloc::string::String,
    /// Stop loss limit of the trading strategy
    #[prost(string, tag="10")]
    pub stop_loss: ::prost::alloc::string::String,
    /// Take profit limit of the trading strategy
    #[prost(string, tag="11")]
    pub take_profit: ::prost::alloc::string::String,
    /// Swap fee of the trading strategy
    #[prost(string, tag="12")]
    pub swap_fee: ::prost::alloc::string::String,
    /// Base deposit at the time of closing the trading strategy
    #[prost(string, tag="17")]
    pub base_deposit: ::prost::alloc::string::String,
    /// Quote deposit at the time of closing the trading strategy
    #[prost(string, tag="18")]
    pub quote_deposit: ::prost::alloc::string::String,
    /// Market mid price at the time of closing the trading strategy
    #[prost(string, tag="19")]
    pub market_mid_price: ::prost::alloc::string::String,
    /// Subscription quote quantity of the trading strategy
    #[prost(string, tag="21")]
    pub subscription_quote_quantity: ::prost::alloc::string::String,
    /// Subscription base quantity of the trading strategy
    #[prost(string, tag="22")]
    pub subscription_base_quantity: ::prost::alloc::string::String,
    /// Number of grid levels of the trading strategy
    #[prost(string, tag="23")]
    pub number_of_grid_levels: ::prost::alloc::string::String,
    /// Indicates whether the trading strategy should exit with quote only
    #[prost(bool, tag="24")]
    pub should_exit_with_quote_only: bool,
    /// Indicates the reason for stopping the trading strategy
    #[prost(string, tag="25")]
    pub stop_reason: ::prost::alloc::string::String,
    /// Indicates whether the trading strategy is pending execution
    #[prost(bool, tag="26")]
    pub pending_execution: bool,
    /// Block height when the strategy was created.
    #[prost(sint64, tag="13")]
    pub created_height: i64,
    /// Block height when the strategy was removed.
    #[prost(sint64, tag="14")]
    pub removed_height: i64,
    /// UpdatedAt timestamp in UNIX millis.
    #[prost(sint64, tag="15")]
    pub created_at: i64,
    /// UpdatedAt timestamp in UNIX millis.
    #[prost(sint64, tag="16")]
    pub updated_at: i64,
    /// Indicate how bot will convert funds (into base or quote or keep as is) after
    /// strategy ended
    #[prost(string, tag="27")]
    pub exit_type: ::prost::alloc::string::String,
    /// Exit config for stop loss
    #[prost(message, optional, tag="28")]
    pub stop_loss_config: ::core::option::Option<ExitConfig>,
    /// Exit config for take profit
    #[prost(message, optional, tag="29")]
    pub take_profit_config: ::core::option::Option<ExitConfig>,
    /// Strategy type: arithmetic, geometric...
    #[prost(string, tag="30")]
    pub strategy_type: ::prost::alloc::string::String,
    /// Version of the contract
    #[prost(string, tag="31")]
    pub contract_version: ::prost::alloc::string::String,
    /// Name of the contract
    #[prost(string, tag="32")]
    pub contract_name: ::prost::alloc::string::String,
    /// Type of the market
    #[prost(string, tag="33")]
    pub market_type: ::prost::alloc::string::String,
    /// lastExecutedAt timestamp in UNIX millis.
    #[prost(sint64, tag="34")]
    pub last_executed_at: i64,
    /// trailing up price
    #[prost(string, tag="35")]
    pub trail_up_price: ::prost::alloc::string::String,
    /// trailing down price
    #[prost(string, tag="36")]
    pub trail_down_price: ::prost::alloc::string::String,
    /// trailing up counter
    #[prost(sint64, tag="37")]
    pub trail_up_counter: i64,
    /// trailing down counter
    #[prost(sint64, tag="38")]
    pub trail_down_counter: i64,
    /// TVL of the trading strategy
    #[prost(string, tag="39")]
    pub tvl: ::prost::alloc::string::String,
    /// PnL of the trading strategy
    #[prost(string, tag="40")]
    pub pnl: ::prost::alloc::string::String,
    /// PnL percentage of the trading strategy
    #[prost(string, tag="41")]
    pub pnl_perc: ::prost::alloc::string::String,
    /// pnlUpdatedAt timestamp in UNIX millis.
    #[prost(sint64, tag="42")]
    pub pnl_updated_at: i64,
    /// Indicates the performance of the trading strategy
    #[prost(string, tag="43")]
    pub performance: ::prost::alloc::string::String,
    /// Return on investment of the trading strategy
    #[prost(string, tag="44")]
    pub roi: ::prost::alloc::string::String,
    /// Initial base price of the trading strategy
    #[prost(string, tag="45")]
    pub initial_base_price: ::prost::alloc::string::String,
    /// Initial quote price of the trading strategy
    #[prost(string, tag="46")]
    pub initial_quote_price: ::prost::alloc::string::String,
    /// Current base price of the trading strategy
    #[prost(string, tag="47")]
    pub current_base_price: ::prost::alloc::string::String,
    /// Current quote price of the trading strategy
    #[prost(string, tag="48")]
    pub current_quote_price: ::prost::alloc::string::String,
    /// Final base price of the trading strategy
    #[prost(string, tag="49")]
    pub final_base_price: ::prost::alloc::string::String,
    /// Final quote price of the trading strategy
    #[prost(string, tag="50")]
    pub final_quote_price: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExitConfig {
    /// strategy exit type (stopLoss/takeProfit)
    #[prost(string, tag="1")]
    pub exit_type: ::prost::alloc::string::String,
    /// strategy stopLoss/takeProfit price
    #[prost(string, tag="2")]
    pub exit_price: ::prost::alloc::string::String,
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
pub struct GetTradingStatsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTradingStatsResponse {
    /// Total of unique active trading strategies
    #[prost(uint64, tag="1")]
    pub active_trading_strategies: u64,
    /// Total number of created trading strategies
    #[prost(uint64, tag="2")]
    pub total_trading_strategies_created: u64,
    /// Total TVL of all active trading strategies
    #[prost(string, tag="3")]
    pub total_tvl: ::prost::alloc::string::String,
    /// Market stats
    #[prost(message, repeated, tag="4")]
    pub markets: ::prost::alloc::vec::Vec<Market>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Market {
    /// MarketId of the trading strategy
    #[prost(string, tag="1")]
    pub market_id: ::prost::alloc::string::String,
    /// Total of unique active trading strategies
    #[prost(uint64, tag="2")]
    pub active_trading_strategies: u64,
}
include!("injective_trading_rpc.tonic.rs");
// @@protoc_insertion_point(module)
