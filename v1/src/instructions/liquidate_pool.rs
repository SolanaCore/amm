use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    pubkey::Pubkey,
    sysvars::rent::Rent,
    ProgramResult,
};
use crate::utils::get_mint_supply;
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::instructions::{InitializeMint, TransferChecked, InitializeAccount, MintToChecked};
use pinocchio_associated_token_account::instructions::Create;
use crate::{
    errors::SolanaCoreError,
    utils::{
        checks::{load_ix_data, DataLen},
        load_acc_mut_unchecked, validate_pda,
        lp_to_mint
    },
    states::{
        Pool, LiquidatePool
    }
};

pub fn liquidate_pool(accounts: &[AccountInfo], data: &[u8]) -> Result<(), SolanaCoreError> {
    let [signer, pool, token_0_ata, token_1_ata, token_0_mint, token_1_mint, vault_0_ata, vault_1_ata, lp_mint, lp_user_ata, sysvar_rent_acc, system_program, token_program] = accounts else {
        return Err(SolanaCoreError::NotEnoughAccountKeys);
    };
    let mut pool_acc: Pool = unsafe { *load_acc_mut_unchecked::<Pool>(pool.borrow_mut_data_unchecked()).unwrap() };
    let ix_data: &LiquidatePool = unsafe { load_ix_data::<LiquidatePool>(data).unwrap() };

    let pda_bump_bytes = [pool_acc.pool_bump];
    let signer_seeds = [
        Seed::from(Pool::POOL_SEED.as_bytes()),
        Seed::from(pool_acc.token_0_mint.as_ref()),
        Seed::from(pool_acc.token_1_mint.as_ref()),
        Seed::from(&pda_bump_bytes[..]),
    ];

    let pool_signers = [Signer::from(&signer_seeds[..])];
    let signer_seed_slices: Vec<&[u8]> = signer_seeds.iter().map(|s| s.as_ref()).collect();
    let _ =validate_pda(&signer_seed_slices, pool.key());

    let lp_bump = [pool_acc.lp_bump];
    let lp_signer_seeds = [
        Seed::from(Pool::LP_SEED.as_bytes()),
        Seed::from(pool.key().as_ref()),
        Seed::from(&lp_bump),
    ];

    let lp_signer_seed_slices: Vec<&[u8]> = lp_signer_seeds.iter().map(|s| s.as_ref()).collect();
    let _ = validate_pda(&lp_signer_seed_slices, lp_mint.key());

    let (lp_to_mint, max_token_0, max_token_1) = lp_to_mint(
        ix_data.deposit_token_0_amount,
        ix_data.deposit_token_1_amount,
        pool_acc.token_0_amount,
        pool_acc.token_1_amount,
        get_mint_supply(lp_mint)?
    ).expect("Return lp_mint, max_token_0, max_token_1");

    // Mint LP tokens
    let _ = MintToChecked {
        mint: lp_mint,
        account: lp_user_ata,
        mint_authority: pool,
        amount: lp_to_mint,
        decimals: 9,
    }.invoke_signed(&pool_signers);

    // Transfer token_0 to vault_0
    let _ = TransferChecked {
        from: token_0_ata,
        mint: token_0_mint,
        to: vault_0_ata,
        authority: signer,
        amount: max_token_0,
        decimals: 9,
    }.invoke_signed(&pool_signers);

    // Transfer token_1 to vault_1
    let _ = TransferChecked {
        from: token_1_ata,
        mint: token_1_mint,
        to: vault_1_ata,
        authority: signer,
        amount: max_token_1,
        decimals: 9,
    }.invoke_signed(&pool_signers);

    // Update pool state
    pool_acc.token_0_amount += max_token_0;
    pool_acc.token_1_amount += max_token_1;

    Ok(())
}
