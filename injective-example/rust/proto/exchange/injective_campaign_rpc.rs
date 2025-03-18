// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RankingRequest {
    /// Campaign ID
    #[prost(string, tag="1")]
    pub campaign_id: ::prost::alloc::string::String,
    /// MarketId of the campaign
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    /// Account address
    #[prost(string, tag="3")]
    pub account_address: ::prost::alloc::string::String,
    #[prost(sint32, tag="4")]
    pub limit: i32,
    #[prost(uint64, tag="5")]
    pub skip: u64,
    /// Contract address that manages the round and reward
    #[prost(string, tag="6")]
    pub contract_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RankingResponse {
    /// The campaign information
    #[prost(message, optional, tag="1")]
    pub campaign: ::core::option::Option<Campaign>,
    /// The campaign users
    #[prost(message, repeated, tag="2")]
    pub users: ::prost::alloc::vec::Vec<CampaignUser>,
    #[prost(message, optional, tag="3")]
    pub paging: ::core::option::Option<Paging>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Campaign {
    #[prost(string, tag="1")]
    pub campaign_id: ::prost::alloc::string::String,
    /// MarketId of the trading strategy
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    /// Total campaign score
    #[prost(string, tag="4")]
    pub total_score: ::prost::alloc::string::String,
    /// Last time the campaign score has been updated.
    #[prost(sint64, tag="5")]
    pub last_updated: i64,
    /// Campaign start date in UNIX millis.
    #[prost(sint64, tag="6")]
    pub start_date: i64,
    /// Campaign end date in UNIX millis.
    #[prost(sint64, tag="7")]
    pub end_date: i64,
    /// Whether the campaign rewards can be claimed.
    #[prost(bool, tag="8")]
    pub is_claimable: bool,
    /// Campaigns round ID
    #[prost(sint32, tag="9")]
    pub round_id: i32,
    /// Contract address that controls this campaign
    #[prost(string, tag="10")]
    pub manager_contract: ::prost::alloc::string::String,
    /// Reward tokens of this campaign
    #[prost(message, repeated, tag="11")]
    pub rewards: ::prost::alloc::vec::Vec<Coin>,
    /// Total user score if accountAddress is passed, this is useful to estimate
    /// account's reward
    #[prost(string, tag="12")]
    pub user_score: ::prost::alloc::string::String,
    /// Return true if user claimed the reward of this campaign
    #[prost(bool, tag="13")]
    pub user_claimed: bool,
    /// Suffix of the subaccount that eligible for volume score
    #[prost(string, tag="14")]
    pub subaccount_id_suffix: ::prost::alloc::string::String,
    /// Contract that manage users reward
    #[prost(string, tag="15")]
    pub reward_contract: ::prost::alloc::string::String,
    /// Version of reward contract, UI use this to determine the message that need
    /// to be sent
    #[prost(string, tag="16")]
    pub version: ::prost::alloc::string::String,
    /// Campaign type
    #[prost(string, tag="17")]
    pub r#type: ::prost::alloc::string::String,
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
pub struct CampaignUser {
    #[prost(string, tag="1")]
    pub campaign_id: ::prost::alloc::string::String,
    /// MarketId of the trading strategy
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    /// Account address
    #[prost(string, tag="3")]
    pub account_address: ::prost::alloc::string::String,
    /// Campaign score
    #[prost(string, tag="4")]
    pub score: ::prost::alloc::string::String,
    /// Whether the distribution contract has been updated with the latest score
    #[prost(bool, tag="5")]
    pub contract_updated: bool,
    /// Block height when the score has been updated.
    #[prost(sint64, tag="6")]
    pub block_height: i64,
    /// Block time timestamp in UNIX millis.
    #[prost(sint64, tag="7")]
    pub block_time: i64,
    /// Amount swapped but only count base denom of the market
    #[prost(string, tag="8")]
    pub purchased_amount: ::prost::alloc::string::String,
    /// True if this user is updated to be in Galxe Campain list, only eligible
    /// address are added
    #[prost(bool, tag="9")]
    pub galxe_updated: bool,
    /// True if this user claimed the reward
    #[prost(bool, tag="10")]
    pub reward_claimed: bool,
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
pub struct CampaignsRequest {
    /// Round ID, if not specified, it will return latest roundId
    #[prost(sint64, tag="1")]
    pub round_id: i64,
    /// Address of login account, if not specified it will return no user rewards
    #[prost(string, tag="2")]
    pub account_address: ::prost::alloc::string::String,
    /// This will return campaign x where x.roundId <= toRoundId. Useful for listing
    /// multiple rounds
    #[prost(sint32, tag="3")]
    pub to_round_id: i32,
    /// Contract address that manages the round and reward
    #[prost(string, tag="4")]
    pub contract_address: ::prost::alloc::string::String,
    /// Campaign type
    #[prost(string, tag="5")]
    pub r#type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignsResponse {
    #[prost(message, repeated, tag="1")]
    pub campaigns: ::prost::alloc::vec::Vec<Campaign>,
    #[prost(message, repeated, tag="2")]
    pub accumulated_rewards: ::prost::alloc::vec::Vec<Coin>,
    #[prost(sint32, tag="3")]
    pub reward_count: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignsV2Request {
    /// Campaign type
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    /// Whether the campaign is active.
    /// Deprecated: use status instead.
    #[prost(bool, tag="2")]
    pub active: bool,
    /// Limit number of returned campaigns
    #[prost(sint32, tag="3")]
    pub limit: i32,
    /// Cursor for pagination
    #[prost(string, tag="4")]
    pub cursor: ::prost::alloc::string::String,
    /// Filter campaigns by start date greater than this value in milliseconds
    #[prost(sint64, tag="5")]
    pub from_start_date: i64,
    /// Filter campaigns by start date lower than this value in milliseconds
    #[prost(sint64, tag="6")]
    pub to_start_date: i64,
    /// Filter campaigns by end date greater than this value in milliseconds
    #[prost(sint64, tag="7")]
    pub from_end_date: i64,
    /// Filter campaigns by end date lower than this value in milliseconds
    #[prost(sint64, tag="8")]
    pub to_end_date: i64,
    /// Filter campaigns by status
    #[prost(string, tag="9")]
    pub status: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignsV2Response {
    #[prost(message, repeated, tag="1")]
    pub campaigns: ::prost::alloc::vec::Vec<CampaignV2>,
    #[prost(string, tag="2")]
    pub cursor: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignV2 {
    #[prost(string, tag="1")]
    pub campaign_id: ::prost::alloc::string::String,
    /// MarketId of the trading strategy
    #[prost(string, tag="2")]
    pub market_id: ::prost::alloc::string::String,
    /// Total campaign score
    #[prost(string, tag="4")]
    pub total_score: ::prost::alloc::string::String,
    /// Campaign creation date in UNIX millis.
    #[prost(sint64, tag="5")]
    pub created_at: i64,
    /// Campaign last update date in UNIX millis.
    #[prost(sint64, tag="6")]
    pub updated_at: i64,
    /// Campaign start date in UNIX millis.
    #[prost(sint64, tag="7")]
    pub start_date: i64,
    /// Campaign end date in UNIX millis.
    #[prost(sint64, tag="8")]
    pub end_date: i64,
    /// Whether the campaign rewards can be claimed.
    #[prost(bool, tag="9")]
    pub is_claimable: bool,
    /// Campaigns round ID
    #[prost(sint32, tag="10")]
    pub round_id: i32,
    /// Contract address that controls this campaign
    #[prost(string, tag="11")]
    pub manager_contract: ::prost::alloc::string::String,
    /// Reward tokens of this campaign
    #[prost(message, repeated, tag="12")]
    pub rewards: ::prost::alloc::vec::Vec<Coin>,
    /// Suffix of the subaccount that eligible for volume score
    #[prost(string, tag="13")]
    pub subaccount_id_suffix: ::prost::alloc::string::String,
    /// Contract that manage users reward
    #[prost(string, tag="14")]
    pub reward_contract: ::prost::alloc::string::String,
    /// Campaign type
    #[prost(string, tag="15")]
    pub r#type: ::prost::alloc::string::String,
    /// Version of reward contract, UI use this to determine the message that need
    /// to be sent
    #[prost(string, tag="16")]
    pub version: ::prost::alloc::string::String,
    /// Campaign name
    #[prost(string, tag="17")]
    pub name: ::prost::alloc::string::String,
    /// Campaign description
    #[prost(string, tag="18")]
    pub description: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGuildsRequest {
    /// Campaign contract address
    #[prost(string, tag="1")]
    pub campaign_contract: ::prost::alloc::string::String,
    /// Limit number of returned guilds
    #[prost(sint32, tag="2")]
    pub limit: i32,
    /// Skip some first guilds in the list for next page
    #[prost(sint32, tag="3")]
    pub skip: i32,
    /// Sort by some metrics
    #[prost(string, tag="4")]
    pub sort_by: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGuildsResponse {
    #[prost(message, repeated, tag="1")]
    pub guilds: ::prost::alloc::vec::Vec<Guild>,
    #[prost(message, optional, tag="2")]
    pub paging: ::core::option::Option<Paging>,
    /// Snapshot updated at time in UNIX milli
    #[prost(sint64, tag="3")]
    pub updated_at: i64,
    /// Summary of the campaign
    #[prost(message, optional, tag="4")]
    pub campaign_summary: ::core::option::Option<CampaignSummary>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Guild {
    #[prost(string, tag="1")]
    pub campaign_contract: ::prost::alloc::string::String,
    /// Guild ID
    #[prost(string, tag="2")]
    pub guild_id: ::prost::alloc::string::String,
    /// Guild's master address
    #[prost(string, tag="3")]
    pub master_address: ::prost::alloc::string::String,
    /// Guild creation date (in UNIX milliseconds)
    #[prost(sint64, tag="4")]
    pub created_at: i64,
    /// Average TVL score
    #[prost(string, tag="5")]
    pub tvl_score: ::prost::alloc::string::String,
    /// Total volume score
    #[prost(string, tag="6")]
    pub volume_score: ::prost::alloc::string::String,
    /// guild's rank by volume
    #[prost(sint32, tag="7")]
    pub rank_by_volume: i32,
    /// guild's rank by TVL
    #[prost(sint32, tag="8")]
    pub rank_by_tvl: i32,
    /// guild's logo, at the moment it supports numberic string (i.e '1', '2' and so
    /// on) not a random URL because of front end limitation
    #[prost(string, tag="9")]
    pub logo: ::prost::alloc::string::String,
    /// guild's total TVL
    #[prost(string, tag="10")]
    pub total_tvl: ::prost::alloc::string::String,
    /// Snapshot updated at time in UNIX milli
    #[prost(sint64, tag="11")]
    pub updated_at: i64,
    /// Guild name
    #[prost(string, tag="14")]
    pub name: ::prost::alloc::string::String,
    /// Active status of guild, true when master total tvl meets the minimum
    /// requirements
    #[prost(bool, tag="13")]
    pub is_active: bool,
    /// Master balance (in current campaigns denom)
    #[prost(string, tag="15")]
    pub master_balance: ::prost::alloc::string::String,
    /// Guild description, set by master of the guild
    #[prost(string, tag="16")]
    pub description: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignSummary {
    /// Campaign id
    #[prost(string, tag="1")]
    pub campaign_id: ::prost::alloc::string::String,
    /// Guild manager contract address
    #[prost(string, tag="2")]
    pub campaign_contract: ::prost::alloc::string::String,
    /// Number of guild in the campaign
    #[prost(sint32, tag="3")]
    pub total_guilds_count: i32,
    /// Total TVL
    #[prost(string, tag="4")]
    pub total_tvl: ::prost::alloc::string::String,
    /// Sum average TVL of all guilds
    #[prost(string, tag="5")]
    pub total_average_tvl: ::prost::alloc::string::String,
    /// Total volume across all guilds (in market quote denom, often USDT)
    #[prost(string, tag="6")]
    pub total_volume: ::prost::alloc::string::String,
    /// Snapshot updated at time in UNIX milli
    #[prost(sint64, tag="7")]
    pub updated_at: i64,
    /// Total member joined the campaign (include guild masters)
    #[prost(sint32, tag="8")]
    pub total_members_count: i32,
    /// Campaign start time
    #[prost(sint64, tag="9")]
    pub start_time: i64,
    /// Campaign end time
    #[prost(sint64, tag="10")]
    pub end_time: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGuildMembersRequest {
    /// Campaign contract address
    #[prost(string, tag="1")]
    pub campaign_contract: ::prost::alloc::string::String,
    /// ID of guild, inside campaign
    #[prost(string, tag="2")]
    pub guild_id: ::prost::alloc::string::String,
    /// Limit number of returned guild members
    #[prost(sint32, tag="3")]
    pub limit: i32,
    /// Skip some first guild members in the list for next page
    #[prost(sint32, tag="4")]
    pub skip: i32,
    /// whether to include guild summary info, it's better to use this in terms of
    /// latency, instead of sending 2 requests we just need once
    #[prost(bool, tag="5")]
    pub include_guild_info: bool,
    /// Sort by some metrics
    #[prost(string, tag="6")]
    pub sort_by: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGuildMembersResponse {
    #[prost(message, repeated, tag="1")]
    pub members: ::prost::alloc::vec::Vec<GuildMember>,
    #[prost(message, optional, tag="2")]
    pub paging: ::core::option::Option<Paging>,
    #[prost(message, optional, tag="3")]
    pub guild_info: ::core::option::Option<Guild>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuildMember {
    /// Guild manager contract address
    #[prost(string, tag="1")]
    pub campaign_contract: ::prost::alloc::string::String,
    /// Guild ID
    #[prost(string, tag="2")]
    pub guild_id: ::prost::alloc::string::String,
    /// Guild member address
    #[prost(string, tag="3")]
    pub address: ::prost::alloc::string::String,
    /// Guild enrollment date (in UNIX milliseconds)
    #[prost(sint64, tag="4")]
    pub joined_at: i64,
    /// Average TVL score
    #[prost(string, tag="5")]
    pub tvl_score: ::prost::alloc::string::String,
    /// Total volume score
    #[prost(string, tag="6")]
    pub volume_score: ::prost::alloc::string::String,
    /// Total volume score
    #[prost(string, tag="7")]
    pub total_tvl: ::prost::alloc::string::String,
    /// Volume percentage out of guilds total volume
    #[prost(double, tag="8")]
    pub volume_score_percentage: f64,
    /// TVL percentage out of guilds total TVL score
    #[prost(double, tag="9")]
    pub tvl_score_percentage: f64,
    /// Rewards for volume campaign (amount+denom)
    #[prost(message, repeated, tag="10")]
    pub tvl_reward: ::prost::alloc::vec::Vec<Coin>,
    /// Rewards for TVL campaign (amount+denom)
    #[prost(message, repeated, tag="11")]
    pub volume_reward: ::prost::alloc::vec::Vec<Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildMemberRequest {
    /// Campaign contract address
    #[prost(string, tag="1")]
    pub campaign_contract: ::prost::alloc::string::String,
    /// User address
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuildMemberResponse {
    #[prost(message, optional, tag="1")]
    pub info: ::core::option::Option<GuildMember>,
}
include!("injective_campaign_rpc.tonic.rs");
// @@protoc_insertion_point(module)
