use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    pubkey::Pubkey,
    sysvars::rent::Rent,
    ProgramResult,
};
use pinocchio_system::instructions::{CreateAccount};
use pinocchio_token::instructions::{InitializeMint, TransferChecked, InitializeAccount, MintToChecked};
use pinocchio_associated_token_account::instructions::Create;
//We use Create - It creates for an ata for the wallet address and token mint, If it doesn't already exist.
//Return an err if the acc exist

use crate::{
    errors::SolanaCoreError,
    utils::{
        checks::{load_ix_data, DataLen},
        load_acc_mut_unchecked,
        validate_pda, calculate_token_out
    },
    states::{
        Swap, Pool, InitPool
    }
};

pub fn swap(accounts: &[AccountInfo], data: &[u8]) -> Result<(), SolanaCoreError> {
    let [signer, pool, token_0_ata, token_1_ata, token_0_mint, token_1_mint, vault_0_ata ,vault_1_ata, system_program, token_program] = accounts else {
        return Err(SolanaCoreError::NotEnoughAccountKeys.into());
    };
    let pool_acc: &mut Pool = unsafe { load_acc_mut_unchecked::<Pool>(pool.borrow_mut_data_unchecked())}.unwrap();  
    let pda_bump_bytes = [pool_acc.pool_bump];
    let signer_seeds = [
        Seed::from(Pool::POOL_SEED.as_bytes()),
        Seed::from(pool_acc.token_0_mint.as_ref()),
        Seed::from(pool_acc.token_1_mint.as_ref()),
        Seed::from(&pda_bump_bytes[..]),
    ];

    let pool_signers = [Signer::from(&signer_seeds[..])];
    let signer_seed_slices: Vec<&[u8]> = signer_seeds.iter().map(|s| s.as_ref()).collect();
    validate_pda(&signer_seed_slices, pool.key());

    let ix_data: &Swap = unsafe { load_ix_data::<Swap>(data).unwrap()};

    if !signer.is_signer() {
        return Err(SolanaCoreError::SignerRequired.into())
    }
    let fees_token_0 = ix_data.token_0_amount * (pool_acc.fees_bps / 100);

    //transfer token_0 -> vault
    TransferChecked {
        from: token_0_ata,
        mint: token_0_mint,
        to: vault_0_ata,
        authority: signer,
        amount: ix_data.token_0_amount,
        decimals: 9,
    }.invoke();
    //Slippage_bps feature also 
    let token_1_out = calculate_token_out(ix_data.token_0_amount - (ix_data.token_0_amount * (pool_acc.fees_bps/100)), ix_data.expected_token_1_amount, ix_data.slippage_bps, pool_acc.token_0_amount, pool_acc.token_1_amount).unwrap(); 
    //vault_1 to token_1
    TransferChecked {
        from: vault_1_ata,
        mint: token_1_mint,
        to: token_1_ata,
        authority: pool,
        amount: token_1_out,
        decimals: 9,
    }.invoke_signed(&pool_signers);

    //update pool_acc
    pool_acc.token_0_amount += ix_data.token_0_amount;
    pool_acc.token_1_amount -= token_1_out;
    Ok(())
}