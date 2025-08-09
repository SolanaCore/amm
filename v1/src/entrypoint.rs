#![allow(unexpected_cfgs)]

use crate::errors::{SolanaCoreError};

use pinocchio::{
    account_info::AccountInfo, nostd_panic_handler, msg, no_allocator, program_entrypoint,
    program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

// This is the entrypoint for the program.
// program_entrypoint!(process_instruction);
//Do not allocate memory.
no_allocator!();

// use inline(always) for performance
//Return: ProgramResult
// #[inline(always)]
// fn process_instruction(
//     _program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     instruction_data: &[u8],
// ){
//     // let (disc, data) = instruction_data
//     //     .split_first()
//     //     .ok_or(ProgramError::InvalidInstructionData)?;
// }

/*
While solana-program writes data to an AccountInfo struct that owns the data, Pinocchio’s AccountInfo struct is itself just a pointer to the underlying input data that represents the account. This reduces the amount of data needed to be copied, saving a lot of CUs ...
*/

/*
How does Pinocchio enable developers to optimize CUs?
Since the instruction processor receives references to pointers, developers using the Pinocchio library will notice that their logic rarely ever has ownership over the data they’re working with. 

This can easily be seen when attempting to access values on the AccountInfo. Reading the account’s public key with the key() method will return a reference to the Pubkey. This makes it cheaper to read account information throughout program execution and cheaper to mutate the account’s data.
*/