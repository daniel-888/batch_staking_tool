use crate::{error, id, state, utils};
use crate::structures::CreateStake;

use anchor_lang::{prelude::*};
use anchor_spl::token::{
    self,
    MintTo,
    Transfer,
    SetAuthority,
};

use spl_token::instruction::AuthorityType;

impl<'info> CreateStake<'info> {
    pub fn process(
        &mut self,
        go_live_date: i64,
        burn_fee: u64,
        upfront_fee: bool,
        collection_count: u64,
        reward_token_vault_bump: u8,
    ) -> Result<()> {
        self.staking_instance.authority = self.authority.key().clone();
        self.staking_instance.reward_token_mint = self.reward_token_mint.key().clone();
        self.staking_instance.reward_token_vault = self.reward_token_vault.key().clone();
        self.staking_instance.go_live_date = go_live_date;
        self.staking_instance.initialize_key = self.initialize_key.key().clone();
        self.staking_instance.burn_wallet = self.burn_wallet.key().clone();
        self.staking_instance.burn_fee = burn_fee;
        self.staking_instance.unstaking_wallet = self.unstaking_wallet.key().clone();

        // create vault reward token account to hold user's nft
        utils::sys_create_account(
            &self.authority.to_account_info(),
            &self.reward_token_vault.to_account_info(),
            self.rent_sysvar.minimum_balance(token::TokenAccount::LEN),
            token::TokenAccount::LEN,
            &token::Token::id(),
            &[
                utils::REWARD_VAULT.as_ref(),
                self.staking_instance.key().as_ref(),
                &[reward_token_vault_bump],
            ]
        )?;

        // initialize vault spl-token account
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = token::InitializeAccount {
            account: self.reward_token_vault.to_account_info(),
            mint: self.reward_token_mint.to_account_info(),
            authority: self.reward_token_vault.to_account_info(),
            rent: self.rent_sysvar.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &[]);
        token::initialize_account(cpi_ctx)?;

        // send to the service upfront fee if the creater want
        if upfront_fee {
            utils::sys_transfer_unchecked(
                &self.authority.to_account_info(),
                &self.unstaking_wallet.to_account_info(), 
                collection_count * 5 * utils::LAMPORTS_PER_MILISOL,
            )?;
        }
        
        
        // let cpi_program = self.token_program.to_account_info();
        // let cpi_accounts = SetAuthority {
        //     current_authority: self.authority.to_account_info(),
        //     account_or_mint: self.reward_token_mint.to_account_info(),
        // };
        // let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &[]);
        // token::set_authority(cpi_ctx, AuthorityType::MintTokens, Some(self.staking_instance.key()))?;

        Ok(())
    }
}