use pinocchio::pubkey::Pubkey;
use crate::utils::DataLen;
// Init_pool
    pub struct InitPool{
        pub token_0_mint: Pubkey,
        pub token_1_mint: Pubkey,
        pub token_0_amount: u64,
        pub token_1_amount: u64,
        pub vault_0: Pubkey,
        pub vault_1: Pubkey,
        pub pool_bump: u8,
        pub fees_bps: u64,
        pub lp_mint:Pubkey,
        pub lp_bump: u8
    }

    impl DataLen for InitPool {
    const LEN: usize = core::mem::size_of::<InitPool>();
    }


// Liquidate_pool
pub struct LiquidatePool {
    pub pool_key: Pubkey,
    pub deposit_token_0_amount: u64,
    pub deposit_token_1_amount: u64,
}
 
impl DataLen for LiquidatePool {
    const LEN: usize = core::mem::size_of::<LiquidatePool>();
    }

// DeLiquidate_pool
pub struct DeliquidatePool {
    pub pool_key: Pubkey,
    pub withdraw_token_0_amount:u64,
    pub withdraw_token_1_amount:u64,  
}


impl DataLen for DeliquidatePool {
    const LEN: usize = core::mem::size_of::<DeliquidatePool>();
    }

    pub struct Swap {
        pub token_0_amount:u64,
        pub expected_token_1_amount: u64,
        pub slippage_bps:u64
    }

    
impl DataLen for Swap {
    
    const LEN: usize = core::mem::size_of::<Swap>();
}