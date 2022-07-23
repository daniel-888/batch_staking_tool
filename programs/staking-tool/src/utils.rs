use crate::state::{staking_pool::*};
use crate::utils;

use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke_signed, program::invoke, system_instruction},
};

pub static STAKING_SEED : &[u8] = b"staking_instance";
pub static STAKING_POOL : &[u8] = b"staking_pool";
pub static ESCROW_PREFIX : &[u8] = b"escrow";
pub static REWARD_VAULT: &[u8] = b"reward_vault";

pub static TOKEN_PROGRAM_BYTES: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
pub const LAMPORTS_PER_SOL : u64 = 1_000_000_000;
pub const LAMPORTS_PER_MILISOL : u64 = 1_000_000;

pub const TIME_PERIOD_OF_ONE_DAY : u64 = 10;

// wrapper of 'create_account' instruction from 'system_program' program
#[inline(always)]
pub fn sys_create_account<'a>(
    from: &AccountInfo<'a>,
    to: &AccountInfo<'a>,
    lamports: u64,
    space: usize,
    owner: &Pubkey,
    signer_seeds: &[&[u8]],
) -> Result<()> {
    invoke_signed(
        &system_instruction::create_account(from.key, to.key, lamports, space as u64, owner),
        &[from.clone(), to.clone()],
        &[&signer_seeds],
    )?;

    Ok(())
}

/// Delete `target` account, transfer all lamports to `receiver`.
#[inline(always)]
pub fn delete_account<'a>(target: &AccountInfo<'a>, receiver: &AccountInfo<'a>) -> Result<()> {
    let mut target_lamports = target.try_borrow_mut_lamports()?;
    let mut receiver_lamports = receiver.try_borrow_mut_lamports()?;

    **receiver_lamports += **target_lamports;
    **target_lamports = 0;

    Ok(())
}

#[inline(always)]
pub fn get_pending_reward<'a>(
    current_timestamp: i64,
    staking_pool: &StakingPool,
) -> u64 {
    let income = match staking_pool.stake_type == StakeType::Locked 
        && current_timestamp > staking_pool.end_time.unwrap() {
        false => staking_pool.reward_per_day
            .checked_mul((current_timestamp as u64)
            .checked_sub(staking_pool.last_reward_time as u64)
            .unwrap()/(utils::TIME_PERIOD_OF_ONE_DAY))
            .unwrap(),
        true => staking_pool.reward_per_day
            .checked_mul((staking_pool.end_time.unwrap() as u64)
            .checked_sub(staking_pool.last_reward_time as u64)
            .unwrap()/(utils::TIME_PERIOD_OF_ONE_DAY))
            .unwrap(),
    };
    income.checked_mul(staking_pool.rarity as u64).unwrap() / 1000u64
}

// wrapper of transfer instructin from system_program program
#[inline(always)]
pub fn sys_transfer<'a>(
    from: &AccountInfo<'a>,
    to: &AccountInfo<'a>,
    lamports: u64,
    signer_seeds: &[&[u8]],
) -> Result<()> {
    invoke_signed(
        &system_instruction::transfer(from.key, to.key, lamports), 
        &[from.clone(), to.clone()],
        &[&signer_seeds],
    )?;

    Ok(())
}

#[inline(always)]
pub fn sys_transfer_unchecked<'a>(
    from: &AccountInfo<'a>,
    to: &AccountInfo<'a>,
    lamports: u64,
) -> Result<()> {
    invoke(
        &system_instruction::transfer(from.key, to.key, lamports), 
        &[from.clone(), to.clone()],
    )?;

    Ok(())
}