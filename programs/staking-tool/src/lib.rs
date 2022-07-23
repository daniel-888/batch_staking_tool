pub mod state;
pub mod utils;
pub mod structures;
pub mod processor;
pub mod error;

use structures::{
    create_stake::*,
    enter_staking::*,
    claim_rewards::*,
    cancel_staking::*,
};

use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

declare_id!("5t5JnxNKbUXgbECee468FCxghPxWnRyBni1Lso2fj8nQ");

#[program]
pub mod staking_tool {
    use super::*;

    pub fn create_stake(
        _ctx: Context<CreateStake>, 
        go_live_date: i64,
        burn_fee: u64,
        collection_count: u64,
        upfront_fee: bool,
    ) -> Result<()> {
        _ctx.accounts.process(
            go_live_date,
            burn_fee,
            upfront_fee,
            collection_count,
            *_ctx.bumps.get("reward_token_vault").unwrap(),
        )
    }

    pub fn enter_staking(
        _ctx: Context<EnterStaking>, 
        is_flexible: bool, 
        duration: Option<i64>,
        reward_per_day: u64,
        rarity: u32,
    ) -> Result<()> {
        _ctx.accounts.process(
            *_ctx.bumps.get("escrow").unwrap(),
            duration,
            is_flexible,
            reward_per_day,
            rarity,
        )
    }

    pub fn claim_rewards(
        _ctx: Context<ClaimRewards>,
    ) -> Result<()> {
        _ctx.accounts.process(
            *_ctx.bumps.get("reward_token_vault").unwrap(),
        )
    }

    pub fn cancel_staking(
        _ctx: Context<CancelStaking>,
    ) -> Result<()> {
        _ctx.accounts.process(
            *_ctx.bumps.get("escrow").unwrap(),
            *_ctx.bumps.get("reward_token_vault").unwrap(),
        )
    }
}
