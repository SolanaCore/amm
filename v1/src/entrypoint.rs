#![allow(unexpected_cfgs)]

use crate::errors::SolanaCoreError;
use crate::instructions::{init_pool, liquidate_pool, deliquidate_pool, swap};

use pinocchio::{
    account_info::AccountInfo, 
    nostd_panic_handler, 
    no_allocator, 
    program_entrypoint,
    program_error::ProgramError, 
    pubkey::Pubkey, 
    ProgramResult,
};

use pinocchio::msg;

program_entrypoint!(process_instruction);

no_allocator!();

// Instruction discriminators
const INIT_POOL_DISCRIMINATOR: u8 = 0;
const LIQUIDATE_POOL_DISCRIMINATOR: u8 = 1;
const DELIQUIDATE_POOL_DISCRIMINATOR: u8 = 2;
const SWAP_DISCRIMINATOR: u8 = 3;

#[inline(always)]
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(SolanaCoreError::InvalidInstructionData)?;

    msg!("Processing instruction");

    match *discriminator {
        INIT_POOL_DISCRIMINATOR => {
            msg!("Instruction: InitPool");
            init_pool(accounts, data)
                .map_err(|e| {
                    msg!("InitPool failed");
                    ProgramError::from(e)
                })
        }
        LIQUIDATE_POOL_DISCRIMINATOR => {
            msg!("Instruction: LiquidatePool");
            liquidate_pool(accounts, data)
                .map_err(|e| {
                    msg!("LiquidatePool failed");
                    ProgramError::from(e)
                })
        }
        DELIQUIDATE_POOL_DISCRIMINATOR => {
            msg!("Instruction: DeliquidatePool");
            deliquidate_pool(accounts, data)
                .map_err(|e| {
                    msg!("DeliquidatePool failed");
                    ProgramError::from(e)
                })
        }
        SWAP_DISCRIMINATOR => {
            msg!("Instruction: Swap");
            swap(accounts, data)
                .map_err(|e| {
                    msg!("Swap failed");
                    ProgramError::from(e)
                })
        }
        _ => {
            msg!("Unknown instruction discriminator");
            Err(SolanaCoreError::InvalidInstructionData.into())
        }
    }
}
