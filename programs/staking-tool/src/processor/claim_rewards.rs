use crate::{error, id, state, utils};
use crate::structures::{ClaimRewards};
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

impl<'info> ClaimRewards<'info> {
    pub fn process(
        &mut self,
        reward_token_vault_bump: u8,
    ) -> Result<()> {
        if self.clock_sysvar.unix_timestamp < self.staking_pool.start_time {
            return Err(error::ErrorCode::StakingNotYetStarted.into());
        } 

        let staking_pool = &mut self.staking_pool;
        let income = utils::get_pending_reward(
            self.clock_sysvar.unix_timestamp,
            staking_pool,
        );
        staking_pool.last_reward_time = self.clock_sysvar.unix_timestamp;
        // let income = match self.staking_pool.stake_type == StakeType::Locked 
        //     && self.clock_sysvar.unix_timestamp > self.staking_pool.end_time.unwrap() {
        //     false => self.staking_pool.reward_per_day
        //         .checked_mul((self.clock_sysvar.unix_timestamp as u64)
        //         .checked_sub(self.staking_pool.last_reward_time as u64)
        //         .unwrap()/(utils::time_period_of_one_day))
        //         .unwrap(),
        //     true => self.staking_pool.reward_per_day
        //         .checked_mul((self.staking_pool.end_time.unwrap() as u64)
        //         .checked_sub(self.staking_pool.last_reward_time as u64)
        //         .unwrap()/(utils::time_period_of_one_day))
        //         .unwrap(),
        // };

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

        // transfer reward from escrow to receive account
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

        Ok(())
    }
}