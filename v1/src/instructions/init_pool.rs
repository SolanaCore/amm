use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    pubkey::Pubkey,
    sysvars::rent::Rent,
    ProgramResult,
    msg,
};
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::instructions::{InitializeMint, TransferChecked, MintToChecked};
use pinocchio_associated_token_account::instructions::Create;

use crate::{
    errors::SolanaCoreError,
    utils::{
        checks::{load_ix_data, DataLen},
        load_acc_mut_unchecked, validate_pda
    },
    states::{
        Pool, InitPool
    }
};

pub fn init_pool(accounts: &[AccountInfo], data: &[u8]) -> Result<(), SolanaCoreError> {
    let [signer, pool, token_0_ata, token_1_ata, token_0_mint, token_1_mint, vault_0_ata, vault_1_ata, lp_mint, lp_user_ata, sysvar_rent_acc, system_program, token_program] = accounts else {
        return Err(SolanaCoreError::NotEnoughAccountKeys);
    };   

    if !signer.is_signer() {
        return Err(SolanaCoreError::MissingRequiredSignature);
    }

    let rent = match Rent::from_account_info(sysvar_rent_acc) {
        Ok(rent) => rent,
        Err(_) => return Err(SolanaCoreError::InvalidAccountData),
    };

    let ix_data = unsafe { load_ix_data::<InitPool>(data) }?;

    let pda_bump_bytes = [ix_data.pool_bump];

    let signer_seeds = [
        Seed::from(Pool::POOL_SEED.as_bytes()),
        Seed::from(&ix_data.token_0_mint),
        Seed::from(&ix_data.token_1_mint),
        Seed::from(&pda_bump_bytes),
    ];

    let pool_signers = [Signer::from(&signer_seeds[..])];

    let signer_seed_slices: Vec<&[u8]> = signer_seeds.iter()
        .map(|s| s.as_ref())
        .collect();

    let signer_seed_slices_: &[&[u8]] = &signer_seed_slices;
    
    validate_pda(signer_seed_slices_, pool.key())?;

    let lp_bump = [ix_data.lp_bump];
    let lp_signer_seeds = [
        Seed::from(Pool::LP_SEED.as_bytes()),
        Seed::from(pool.key()),
        Seed::from(&lp_bump),
    ];
    
    let lp_signer_seed_slices: Vec<&[u8]> = lp_signer_seeds.iter()
        .map(|s| s.as_ref())
        .collect();

    let lp_signer_seed_slices: &[&[u8]] = &lp_signer_seed_slices;
    
    validate_pda(lp_signer_seed_slices, lp_mint.key())?;

    msg!("Creating pool account");
    
    match (CreateAccount {
        from: signer,
        to: pool,
        space: Pool::LEN as u64,
        owner: &crate::ID,
        lamports: rent.minimum_balance(Pool::LEN),
    }).invoke_signed(pool_signers.as_slice()) {
        Ok(_) => {},
        Err(_) => return Err(SolanaCoreError::InvalidInstructionData),
    }

    msg!("Creating vault ATAs");

    match (Create {
        funding_account: signer,
        account: vault_0_ata,
        wallet: pool,
        mint: token_0_mint,
        system_program,
        token_program,
    }).invoke() {
        Ok(_) => {},
        Err(_) => return Err(SolanaCoreError::InvalidInstructionData),
    }

    match (Create {
        funding_account: signer,
        account: vault_1_ata,
        wallet: pool,
        mint: token_1_mint,
        system_program,
        token_program,
    }).invoke() {
        Ok(_) => {},
        Err(_) => return Err(SolanaCoreError::InvalidInstructionData),
    }

    msg!("Transferring initial tokens");

    match (TransferChecked {
        from: token_0_ata,
        mint: token_0_mint,
        to: vault_0_ata,
        authority: signer,
        amount: ix_data.token_0_amount,
        decimals: 9
    }).invoke() {
        Ok(_) => {},
        Err(_) => return Err(SolanaCoreError::InvalidInstructionData),
    }

    match (TransferChecked {
        from: token_1_ata,
        mint: token_1_mint,
        to: vault_1_ata,
        authority: signer,
        amount: ix_data.token_1_amount,
        decimals: 9,
    }).invoke() {
        Ok(_) => {},
        Err(_) => return Err(SolanaCoreError::InvalidInstructionData),
    }

    msg!("Initializing LP mint");

    match (InitializeMint {
        mint: lp_mint,
        rent_sysvar: sysvar_rent_acc,
        decimals: 9,
        mint_authority: pool.key(),
        freeze_authority: Some(pool.key()),
    }).invoke() {
        Ok(_) => {},
        Err(_) => return Err(SolanaCoreError::InvalidInstructionData),
    }

    // Calculate LP tokens to mint: L = sqrt(x * y)
    let lp_to_mint = ((ix_data.token_0_amount as f64) * (ix_data.token_1_amount as f64)).sqrt() as u64;

    msg!("Minting LP tokens");

    match (MintToChecked {
        mint: lp_mint,
        account: lp_user_ata,
        mint_authority: pool,
        amount: lp_to_mint,
        decimals: 9,
    }).invoke_signed(pool_signers.as_slice()) {
        Ok(_) => {},
        Err(_) => return Err(SolanaCoreError::InvalidInstructionData),
    }

    msg!("Initializing pool state");

    match Pool::init_pool(pool, ix_data) {
        Ok(_) => {},
        Err(_) => return Err(SolanaCoreError::InvalidInstructionData),
    }
    
    msg!("Pool initialized successfully");
    Ok(())
}
