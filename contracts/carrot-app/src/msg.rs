use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Uint128, Uint64};

use crate::{
    contract::App,
    state::{AutocompoundRewardsConfig, Position},
};

// This is used for type safety and re-exporting the contract endpoint structs.
abstract_app::app_msg_types!(App, AppExecuteMsg, AppQueryMsg);

/// App instantiate message
#[cosmwasm_schema::cw_serde]
pub struct AppInstantiateMsg {
    /// Id of the pool used to get rewards
    pub pool_id: u64,
    /// Seconds to wait before autocompound is incentivized.
    pub autocompound_cooldown_seconds: Uint64,
    /// Configuration of rewards to the address who helped to execute autocompound
    pub autocompound_rewards_config: AutocompoundRewardsConfig,
    /// Create position with instantiation.
    /// Will not create position if omitted
    pub create_position: Option<CreatePositionMessage>,
}

#[cosmwasm_schema::cw_serde]
pub struct CreatePositionMessage {
    pub lower_tick: i64,
    pub upper_tick: i64,
    // Funds to use to deposit on the account
    pub funds: Vec<Coin>,
}

/// App execute messages
#[cosmwasm_schema::cw_serde]
#[cfg_attr(feature = "interface", derive(cw_orch::ExecuteFns))]
#[cfg_attr(feature = "interface", impl_into(ExecuteMsg))]
pub enum AppExecuteMsg {
    /// Create the initial liquidity position
    CreatePosition(CreatePositionMessage),
    /// Deposit funds onto the app
    Deposit { funds: Vec<Coin> },
    /// Partial withdraw of the funds available on the app
    Withdraw { amount: Uint128 },
    /// Withdraw everything that is on the app
    WithdrawAll {},
    /// Auto-compounds the pool rewards into the pool
    Autocompound {},
}

/// App query messages
#[cosmwasm_schema::cw_serde]
#[cfg_attr(feature = "interface", derive(cw_orch::QueryFns))]
#[cfg_attr(feature = "interface", impl_into(QueryMsg))]
#[derive(QueryResponses)]
pub enum AppQueryMsg {
    #[returns(crate::state::Config)]
    Config {},
    #[returns(AssetsBalanceResponse)]
    Balance {},
    /// Get the claimable rewards that the position has accumulated.
    /// Returns [`AvailableRewardsResponse`]
    #[returns(AvailableRewardsResponse)]
    AvailableRewards {},
    #[returns(PositionResponse)]
    Position {},
    /// Get the status of the compounding logic of the application
    /// Returns [`CompoundStatusResponse`]
    #[returns(CompoundStatusResponse)]
    CompoundStatus {},
}

#[cosmwasm_schema::cw_serde]
pub enum AppMigrateMsg {}

#[cosmwasm_schema::cw_serde]
pub struct BalanceResponse {
    pub balance: Vec<Coin>,
}
#[cosmwasm_schema::cw_serde]
pub struct AvailableRewardsResponse {
    pub available_rewards: Vec<Coin>,
}

#[cw_serde]
pub struct AssetsBalanceResponse {
    pub balances: Vec<Coin>,
    pub liquidity: String,
}

#[cw_serde]
pub struct PositionResponse {
    pub position: Option<Position>,
}

#[cw_serde]
pub struct CompoundStatusResponse {
    pub status: CompoundStatus,
    pub reward: Coin,
    // TODO: Contract can't query authz, should this check be done by bot instead?
    pub rewards_available: bool,
}

#[cw_serde]
/// Wether contract is ready for the compound
pub enum CompoundStatus {
    /// Contract is ready for the compound
    Ready {},
    /// How much seconds left for the next compound
    Cooldown(Uint64),
    /// No open position right now
    NoPosition {},
}

impl CompoundStatus {
    pub fn is_ready(&self) -> bool {
        matches!(self, Self::Ready {})
    }
}
