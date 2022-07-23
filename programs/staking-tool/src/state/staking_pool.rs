use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum StakeStatus {
    Started,
    Finished,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq, Copy)]
pub enum StakeType {
    Flexible,
    Locked,
}

#[account]
#[derive(Copy, Debug)]
pub struct StakingPool {
    pub authority: Pubkey,
    pub nft_token_mint: Pubkey,
    pub stake_type: StakeType,
    pub start_time: i64,
    pub end_time: Option<i64>,
    pub last_reward_time: i64,
    pub reward_per_day: u64,
    pub rarity: u32,
    pub state: StakeStatus,
    pub reward_token_mint: Pubkey,
    pub staking_instance: Pubkey,
}