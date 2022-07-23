use crate::{utils};
use crate::state::StakingInstance;

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct CreateStake<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        constraint = reward_token_mint
            .mint_authority
            .unwrap() == authority.key(),
    )]
    pub reward_token_mint: Box<Account<'info, Mint>>,

    #[account(
        mut, 
        seeds = [
            utils::REWARD_VAULT.as_ref(),
            staking_instance.key().as_ref(),
        ],
        bump,
    )]
    /// CHECK: it'a alright
    pub reward_token_vault: UncheckedAccount<'info>,

    #[account(
        init,
        seeds = [
            utils::STAKING_SEED.as_ref(),
            authority.key().as_ref(),
            initialize_key.key().as_ref(),
        ],
        bump,
        payer = authority,
        space = 8 + core::mem::size_of::<StakingInstance>(),
    )]
    pub staking_instance: Box<Account<'info, StakingInstance>>,

    /// CHECK: it's alright
    pub initialize_key: UncheckedAccount<'info>,

    /// CHECK: it's alright
    pub burn_wallet: UncheckedAccount<'info>,
    #[account(
        mut,
        constraint = reward_token_mint.key() == burn_token_account.mint,
        constraint = burn_token_account.owner == burn_wallet.key(),
    )]
    pub burn_token_account: Box<Account<'info,TokenAccount>>,

    #[account(mut)]
    /// CHECK: it's alright
    pub unstaking_wallet: UncheckedAccount<'info>,

    pub rent_sysvar: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}