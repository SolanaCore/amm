use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    pubkey::Pubkey,
    sysvars::rent::Rent,
    ProgramResult,
};
use pinocchio_system::instructions::{CreateAccount};
use pinocchio_token::instructions::{InitializeMint, TransferChecked, InitializeAccount, BurnChecked};
use pinocchio_associated_token_account::instructions::Create;

//We use Create - It creates for an ata for the wallet address and token mint, If it doesn't already exist.
//Return an err if the acc exist

use crate::{
    errors::SolanaCoreError,
    utils::{
        checks::{load_ix_data, DataLen},
        load_acc_mut_unchecked, validate_pda,
        lp_to_burn, get_mint_supply
    },
    states::{
        Pool, DeliquidatePool
    }
};

pub fn deliquidate_pool(accounts: &[AccountInfo], data: &[u8]) -> Result<(), SolanaCoreError> {
    let [signer, pool, token_0_ata, token_1_ata, token_0_mint, token_1_mint, vault_0_ata ,vault_1_ata, lp_mint, lp_user_ata,  sysvar_rent_acc, system_program, token_program] = accounts else {
        return Err(SolanaCoreError::NotEnoughAccountKeys.into());
    };
    if !signer.is_signer() {
        return Err(SolanaCoreError::SignerRequired.into());
    }
    if !pool.is_writable() {
        return Err(SolanaCoreError::PoolAccountNotWritable.into());
    }
    let  pool_acc: &mut Pool = unsafe { load_acc_mut_unchecked::<Pool>(pool.borrow_mut_data_unchecked()).unwrap()};  
    let  ix_data: &DeliquidatePool = unsafe { load_ix_data::<DeliquidatePool>(data)}.unwrap();

    let pda_bump_bytes = [pool_acc.pool_bump];
    // signer seeds
let signer_seeds = [
    Seed::from(Pool::POOL_SEED.as_bytes()),
    Seed::from(&pool_acc.token_0_mint),
    Seed::from(&pool_acc.token_1_mint),
    Seed::from(&pda_bump_bytes[..]),
];
    let pool_signers = [Signer::from(&signer_seeds[..])];
// convert to Vec<&[u8]>
let signer_seed_slices: Vec<&[u8]> = signer_seeds.iter()
    .map(|s| s.as_ref()) // calls AsRef<[u8]>
    .collect();

// now it's &[&[u8]]
let signer_seed_slices: &[&[u8]] = &signer_seed_slices;

    validate_pda(signer_seed_slices as &[&[u8]], pool.key());

    let lp_bump = [pool_acc.lp_bump];
    let lp_signer_seeds = [
        Seed::from(Pool::LP_SEED.as_bytes()),
        Seed::from(pool.key()),
        Seed::from(&lp_bump),
    ];

    
     // convert to Vec<&[u8]>
    let lp_signer_seed_slices: Vec<&[u8]> = lp_signer_seeds.iter()
    .map(|s| s.as_ref()) // calls AsRef<[u8]>
    .collect();

    // now it's &[&[u8]]
    let lp_signer_seed_slices: &[&[u8]] = &lp_signer_seed_slices;

    validate_pda(lp_signer_seed_slices as &[&[u8]], lp_mint.key());

    // check the ratio in which they are withdrawing token
    let (lp_to_burn, max_token_0, max_token_1)= lp_to_burn(ix_data.withdraw_token_0_amount, ix_data.withdraw_token_1_amount, pool_acc.token_0_amount, pool_acc.token_1_amount, get_mint_supply(lp_mint).unwrap());

    // burn lp_token
    BurnChecked {
        account: lp_user_ata, 
        mint: lp_mint, 
        authority: pool, 
        amount: lp_to_burn, 
        decimals: 9
    }.invoke_signed(&pool_signers);

    // transfer token_0 to user
    TransferChecked {
        from: vault_0_ata,
        mint: token_0_mint,
        to: token_0_ata,
        authority: signer,
        amount: max_token_0,
        decimals: 9,
    }.invoke_signed(&pool_signers);

    // transfer token_1 to user
    TransferChecked {
        from: vault_1_ata,
        mint: token_1_mint,
        to: token_1_ata,
        authority: signer,
        amount: max_token_1,    
        decimals: 9,
    }.invoke_signed(&pool_signers);

    pool_acc.token_0_amount -= max_token_0;
    pool_acc.token_1_amount -= max_token_1;
    Ok(())
}