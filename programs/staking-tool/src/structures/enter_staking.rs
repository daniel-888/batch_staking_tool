use crate::{utils};
use crate::state::{
    StakingInstance,
    StakingPool,
};

use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint, Token};

#[derive(Accounts)]
pub struct EnterStaking<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub nft_token_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        constraint = nft_token_account.mint == nft_token_mint.key()
    )]
    pub nft_token_account: Box<Account<'info, TokenAccount>>,

    pub staking_instance: Account<'info, StakingInstance>,

    #[account(
        init,
        seeds = [
            utils::STAKING_POOL.as_ref(),
            authority.key().as_ref(),
            nft_token_mint.key().as_ref(),
        ],
        bump,
        payer = authority,
        space = 8 + core::mem::size_of::<StakingPool>(),
    )]
    pub staking_pool: Box<Account<'info, StakingPool>>,

    /// CHECK: it's alright
    #[account(
        mut,
        seeds = [
            utils::ESCROW_PREFIX.as_ref(),
            authority.key().as_ref(),
            staking_pool.key().as_ref(),
        ],
        bump,
        // payer = authority,
        // space = 8 + core::mem::size_of::<TokenAccount>(),
    )]
    pub escrow: UncheckedAccount<'info>,

    pub rent_sysvar: Sysvar<'info, Rent>,
    pub clock_sysvar: Sysvar<'info, Clock>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,

}