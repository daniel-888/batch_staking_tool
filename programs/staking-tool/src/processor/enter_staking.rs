use crate::{error, id, state, utils};
use crate::structures::{EnterStaking};
use crate::state::{
    StakeStatus,
    StakeType,
};

use anchor_lang::{prelude::*};
use anchor_spl::token::{
    self,
    MintTo,
    Transfer,
    SetAuthority,
};

impl<'info> EnterStaking<'info> {
    pub fn process(
        &mut self,
        escrow_bump: u8,
        duration: Option<i64>,
        is_flexible: bool,
        reward_per_day: u64,
        rarity: u32,
    ) -> Result<()> {
        // if self.staking_instance.go_live_date >= self.clock_sysvar.unix_timestamp {
        //     return Err(error::ErrorCode::StakingNotYetStarted.into());
        // }
        if !is_flexible {
            if None == duration {
                return Err(error::ErrorCode::DurationMissing.into());
            }
        }
        self.staking_pool.authority = self.authority.key().clone();
        self.staking_pool.nft_token_mint = self.nft_token_mint.key().clone();
        self.staking_pool.start_time = self.clock_sysvar.unix_timestamp.clone();
        self.staking_pool.end_time = match is_flexible {
            true => None,
            false => match duration {
                Some(time) => Some(self.clock_sysvar.unix_timestamp + time),
                None => None,
                }
        };
        self.staking_pool.stake_type = match is_flexible {
            true => StakeType::Flexible,
            false => StakeType::Locked,
        };
        self.staking_pool.last_reward_time = self.clock_sysvar.unix_timestamp.clone();
        self.staking_pool.reward_per_day = reward_per_day;
        self.staking_pool.rarity = rarity;
        self.staking_pool.state = StakeStatus::Started;
        self.staking_pool.reward_token_mint = self.staking_instance.reward_token_mint.key().clone();
        self.staking_pool.staking_instance = self.staking_instance.key().clone();

        // create escrow token account to hold user's nft
        utils::sys_create_account(
            &self.authority.to_account_info(),
            &self.escrow.to_account_info(),
            self.rent_sysvar.minimum_balance(token::TokenAccount::LEN),
            token::TokenAccount::LEN,
            &token::Token::id(),
            &[
                utils::ESCROW_PREFIX.as_ref(),
                self.authority.key().as_ref(),
                self.staking_pool.key().as_ref(),
                &[escrow_bump],
            ]
        )?;

        // initialize escrow spl-token account
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token::InitializeAccount {
            account: self.escrow.to_account_info(),
            mint: self.nft_token_mint.to_account_info(),
            authority: self.escrow.to_account_info(),
            rent: self.rent_sysvar.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &[]);
        token::initialize_account(cpi_ctx)?;

        // transfer nft to spl_token escrow
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token::Transfer {
            from: self.nft_token_account.to_account_info(),
            to: self.escrow.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &[]);
        token::transfer(cpi_ctx, 1)?;

        Ok(())

    }
}