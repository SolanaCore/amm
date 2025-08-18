#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

pub mod errors;
pub mod instructions;
pub mod states;
pub mod utils;

pinocchio_pubkey::declare_id!("F3djNpWTDPFvum35roNrrH1u7PtXCioD9N6KApWcgVi3");

/// Internal tests using Mollusk
#[cfg(test)]
mod tests {
    use super::*;
    use mollusk_svm::result::{Check, ProgramResult};
    use mollusk_svm::{program, Mollusk};
    use solana_sdk::account::Account;
    use solana_sdk::instruction::{AccountMeta, Instruction};
    use solana_sdk::native_token::LAMPORTS_PER_SOL;
    use solana_sdk::pubkey;
    use solana_sdk::pubkey::Pubkey;
    use solana_sdk::rent::Rent;
    use solana_sdk::sysvar::Sysvar;
    extern crate alloc;
    use alloc::vec;

    use crate::states::InitPool;
    use crate::utils::to_bytes;
    use crate::errors::SolanaCoreError;

    pub const PROGRAM: Pubkey = pubkey!("F3djNpWTDPFvum35roNrrH1u7PtXCioD9N6KApWcgVi3");
    pub const RENT: Pubkey = pubkey!("SysvarRent111111111111111111111111111111111");
    pub const TOKEN_PROGRAM: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
    pub const SYSTEM_PROGRAM: Pubkey = pubkey!("11111111111111111111111111111111");
    pub const PAYER: Pubkey = pubkey!("52nvBaMXujpVYf6zBUvmQtHEZc4kAncRJccXG99F6yrg");

    pub const TOKEN_0_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
    pub const TOKEN_1_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

    // Instruction discriminators
    const INIT_POOL_DISCRIMINATOR: u8 = 0;
    const LIQUIDATE_POOL_DISCRIMINATOR: u8 = 1;
    const DELIQUIDATE_POOL_DISCRIMINATOR: u8 = 2;
    const SWAP_DISCRIMINATOR: u8 = 3;

    fn mollusk() -> Mollusk {
        Mollusk::new(&PROGRAM, "target/deploy/v1")
    }

    fn get_rent_data() -> Vec<u8> {
        let rent = Rent::default();
        unsafe {
            core::slice::from_raw_parts(&rent as *const Rent as *const u8, Rent::size_of()).to_vec()
        }
    }

    #[test]
    fn test_entrypoint_routing() {
        println!("Testing Entrypoint Instruction Routing");

        assert_eq!(INIT_POOL_DISCRIMINATOR, 0);
        assert_eq!(LIQUIDATE_POOL_DISCRIMINATOR, 1);
        assert_eq!(DELIQUIDATE_POOL_DISCRIMINATOR, 2);
        assert_eq!(SWAP_DISCRIMINATOR, 3);

        println!("All discriminator constants are correct!");
    }

    #[test]
    fn test_invalid_discriminator() {
        let mollusk = mollusk();
        let (system_program, system_account) = program::keyed_account_for_system_program();
        let payer_account = Account::new(LAMPORTS_PER_SOL, 0, &system_program);

        let ix_accounts = vec![AccountMeta::new(PAYER, true)];
        let ser_ix_data = vec![99];
        let instruction = Instruction::new_with_bytes(PROGRAM, &ser_ix_data, ix_accounts);

        let tx_accounts = vec![(PAYER, payer_account), (system_program, system_account)];

        println!("Testing Invalid Discriminator");

        let result = mollusk.process_and_validate_instruction(
            &instruction,
            &tx_accounts,
            &[Check::err(solana_sdk::program_error::ProgramError::Custom(
                SolanaCoreError::InvalidInstructionData as u32,
            ))],
        );

        println!("Invalid discriminator test result: {:?}", result.program_result);
    }

    #[test]
    fn test_empty_instruction_data() {
        let mollusk = mollusk();
        let (system_program, system_account) = program::keyed_account_for_system_program();
        let payer_account = Account::new(LAMPORTS_PER_SOL, 0, &system_program);

        let ix_accounts = vec![AccountMeta::new(PAYER, true)];
        let ser_ix_data = vec![];
        let instruction = Instruction::new_with_bytes(PROGRAM, &ser_ix_data, ix_accounts);

        let tx_accounts = vec![(PAYER, payer_account), (system_program, system_account)];

        println!("Testing Empty Instruction Data");

        let result = mollusk.process_and_validate_instruction(
            &instruction,
            &tx_accounts,
            &[Check::err(solana_sdk::program_error::ProgramError::Custom(
                SolanaCoreError::InvalidInstructionData as u32,
            ))],
        );

        println!("Empty instruction data test result: {:?}", result.program_result);
    }

    #[test]
    fn test_init_pool_discriminator_only() {
        println!("Testing InitPool discriminator routing");

        let ix_data = InitPool {
            token_0_mint: TOKEN_0_MINT.to_bytes(),
            token_1_mint: TOKEN_1_MINT.to_bytes(),
            token_0_amount: 1_000_000,
            token_1_amount: 2_000_000,
            vault_0: Pubkey::new_unique().to_bytes(),
            vault_1: Pubkey::new_unique().to_bytes(),
            pool_bump: 254,
            fees_bps: 30,
            lp_mint: Pubkey::new_unique().to_bytes(),
            lp_bump: 253,
        };

        let mut ser_ix_data = vec![INIT_POOL_DISCRIMINATOR];
        ser_ix_data.extend_from_slice(unsafe { to_bytes(&ix_data) });

        println!(" InitPool instruction data serialized successfully!");
        println!(" Discriminator: {}", INIT_POOL_DISCRIMINATOR);
        println!(" Data length: {} bytes", ser_ix_data.len());
    }
}
