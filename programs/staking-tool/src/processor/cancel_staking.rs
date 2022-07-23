use crate::{error, id, state, utils};
use crate::structures::{CancelStaking};
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

impl<'info> CancelStaking<'info> {
    pub fn process(
        &mut self,
        escrow_bump: u8,
        reward_token_vault_bump: u8,
    ) -> Result<()> {
        // if self.staking_pool.stake_type == StakeType::Locked && self.clock_sysvar.unix_timestamp < self.staking_pool.end_time.unwrap() {
        //     return Err(error::ErrorCode::StakingIsNotFinished.into());
        // }

        let staking_pool = &self.staking_pool;
        let income = utils::get_pending_reward(
            self.clock_sysvar.unix_timestamp, 
            staking_pool,
        );

        // transfer nft from escrow to receive account
        let staking_pool = &self.staking_pool.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            utils::ESCROW_PREFIX.as_ref(),
            self.staking_pool.authority.as_ref(),
            staking_pool.as_ref(),
            &[escrow_bump],
        ]];

        let cpi_accounts = Transfer {
            to: self.receive_nft_token_account.to_account_info(),
            from: self.escrow.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds.clone());
        token::transfer(cpi_ctx, 1)?;

        // mint rewards
        // let cpi_accounts = MintTo {
        //     mint: self.reward_token_mint.to_account_info(),
        //     to: self.receive_reward_token_account.to_account_info(),
        //     authority: self.staking_instance.to_account_info(),
        // };
        // let cpi_program = self.token_program.to_account_info();
        // let authority_seeds : &[&[u8]] = &[ 
        //     utils::STAKING_SEED.as_ref(),
        //     self.staking_instance.authority.as_ref(),
        //     self.staking_instance.initialize_key.as_ref(),
        //     &[staking_instance_bump],
        // ];
        // let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        let amount = if income == 0 {
                0
            } else {
                income
            };
        // token::mint_to(cpi_ctx.with_signer(&[&authority_seeds[..]]), amount)?;

        // utils::delete_account(
        //     &self.staking_pool.to_account_info(),
        //     &self.authority.to_account_info(),
        // )?;

        // Delete escrow account
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token::CloseAccount {
            account: self.escrow.to_account_info(),
            destination: self.authority.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        token::close_account(cpi_ctx)?;

        // transfer reward from vault to receive account
        let staking_instance = &self.staking_instance.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            utils::REWARD_VAULT.as_ref(),
            staking_instance.as_ref(),
            &[reward_token_vault_bump],
        ]];

        let cpi_accounts = Transfer {
            to: self.receive_reward_token_account.to_account_info(),
            from: self.reward_token_vault.to_account_info(),
            authority: self.reward_token_vault.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        token::transfer(cpi_ctx, amount)?;

        // transfer reward from receive account to burn account
        let cpi_accounts = Transfer {
            to: self.burn_token_account.to_account_info(),
            from: self.receive_reward_token_account.to_account_info(),
            authority: self.authority.to_account_info(),
        };
        let cpi_program = self.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, self.staking_instance.burn_fee)?;

        // transfer sol as unstaking fee if no upfront fee payment
        if !self.staking_instance.upfront_fee {
            utils::sys_transfer_unchecked(
                &self.authority.to_account_info(), 
                &self.unstaking_wallet.to_account_info(), 
                5 * utils::LAMPORTS_PER_MILISOL, 
            )?;
        }

        Ok(())
    }
}