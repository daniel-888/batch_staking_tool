use anchor_lang::prelude::*;

#[account]
#[derive(Copy, Default)]
pub struct StakingInstance {
    pub authority: Pubkey,
    pub initialize_key: Pubkey,
    pub reward_token_mint: Pubkey,
    pub reward_token_vault: Pubkey,
    pub burn_wallet: Pubkey,
    pub burn_fee: u64,
    pub go_live_date: i64,
    pub upfront_fee: bool,
    pub unstaking_wallet: Pubkey,
}