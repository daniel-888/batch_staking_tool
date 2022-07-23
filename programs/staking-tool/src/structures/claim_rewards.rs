use crate::{utils};
use crate::state::{
    StakingInstance,
    StakingPool,
};

use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token, Mint};

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        constraint = staking_pool.authority == authority.key(),
        constraint = staking_pool.staking_instance == staking_instance.key(),
    )]
    pub staking_pool: Box<Account<'info, StakingPool>>,
    #[account(
        mut,
        seeds = [
            utils::STAKING_SEED.as_ref(),
            staking_instance.authority.as_ref(),
            staking_instance.initialize_key.as_ref(),
        ],
        bump,
    )]
    pub staking_instance: Account<'info, StakingInstance>,
    #[account(
        mut, 
        seeds = [
            utils::REWARD_VAULT.as_ref(),
            staking_instance.key().as_ref(),
        ],
        bump,
    )]
    pub reward_token_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = receive_reward_token_account.mint == staking_instance.reward_token_mint,
    )]
    pub receive_reward_token_account: Box<Account<'info, TokenAccount>> ,
    #[account(
        mut,
        constraint = reward_token_mint.key() == staking_instance.reward_token_mint,
    )]
    pub reward_token_mint: Box<Account<'info, Mint>>,
    // #[account(
    //     mut,
    //     seeds = [
    //         utils::ESCROW_PREFIX.as_ref(),
    //         authority.key().as_ref(),
    //         staking_pool.key().as_ref(),
    //     ],
    //     bump,
    // )]
    pub escrow: Box<Account<'info, TokenAccount>>,

    pub rent_sysvar: Sysvar<'info, Rent>,
    pub clock_sysvar: Sysvar<'info, Clock>,
    #[account(
        constraint = 
            token_program.key() == utils::TOKEN_PROGRAM_BYTES.parse::<Pubkey>().unwrap(),
    )]
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,

}