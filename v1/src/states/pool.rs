use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{self, Pubkey},
    ProgramResult,
};
use shank::ShankAccount;
use crate::utils::validate_pda;
use crate::{utils::{load_acc_mut_unchecked, DataLen}, errors::SolanaCoreError, states::InitPool};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, ShankAccount)]
pub struct Pool {
    //token_mint
    pub token_0_mint: Pubkey,
    pub token_1_mint: Pubkey,

    //token_amount
    pub token_0_amount: u64,
    pub token_1_amount: u64,

    //vault
    pub vault_0: Pubkey,
    pub vault_1: Pubkey,

    //bump for the pool(program derived address)
    pub pool_bump: u8,

    //fees(unit: bps)
    // eg. 50 bps = 0.05 %(50/100)
    pub fees_bps: u64,

    //The mint address of the lp_token
    pub lp_mint: Pubkey,

    pub lp_bump: u8,
}

impl DataLen for Pool {
    const LEN: usize = core::mem::size_of::<Pool>();
}

impl Pool {
    //pool_seed
    pub const POOL_SEED: &'static str = "pool";
    //lp_seed
    pub const LP_SEED: &'static str = "lp";

    #[inline(always)]
    pub fn init_pool(pool: &AccountInfo, ix_data: &InitPool) -> ProgramResult {
        let pool_acc = unsafe { 
            match load_acc_mut_unchecked::<Pool>(pool.borrow_mut_data_unchecked()) {
                Ok(acc) => acc,
                Err(_) => return Err(SolanaCoreError::InvalidAccountData.into()),
            }
        };

        let fees_bps_bytes = ix_data.fees_bps.to_le_bytes();
        let pool_seeds: &[&[u8]] = &[
            Self::POOL_SEED.as_bytes(),
            ix_data.token_0_mint.as_ref(),
            ix_data.token_1_mint.as_ref(),
            &fees_bps_bytes,
            &[ix_data.pool_bump],
        ];
        
        match validate_pda(pool_seeds, pool.key()) {
            Ok(_) => {},
            Err(_) => return Err(SolanaCoreError::PdaMismatch.into()),
        }

        pool_acc.token_0_mint = ix_data.token_0_mint;
        pool_acc.token_1_mint = ix_data.token_1_mint;
        pool_acc.token_0_amount = ix_data.token_0_amount;
        pool_acc.token_1_amount = ix_data.token_1_amount;
        pool_acc.vault_0 = ix_data.vault_0;
        pool_acc.vault_1 = ix_data.vault_1;
        pool_acc.pool_bump = ix_data.pool_bump;
        
        if ix_data.fees_bps > 500 {
            return Err(SolanaCoreError::InvalidInstructionData.into());
        }
        pool_acc.fees_bps = ix_data.fees_bps;
        
        // lp_mint(is_pda)
        let lp_seeds = &[Self::LP_SEED.as_bytes(), pool.key().as_ref(), &[ix_data.lp_bump]];
        match validate_pda(lp_seeds, &ix_data.lp_mint) {
            Ok(_) => {},
            Err(_) => return Err(SolanaCoreError::PdaMismatch.into()),
        }
        
        pool_acc.lp_mint = ix_data.lp_mint;
        pool_acc.lp_bump = ix_data.lp_bump;
        
        Ok(())
    }
}
