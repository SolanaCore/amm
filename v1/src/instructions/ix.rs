use crate::states::{
    InitPool
};
use shank::ShankInstruction;
#[derive(ShankInstruction)]
pub enum SolanaCoreInstruction {
    
    #[account(0,name = "Signer")]
    #[account(1, signer,  name = "Pool Account", desc = "")]
    #[account(2, name = "Token_0_ATA", desc = "")]
    #[account(3, name = "Token_1_ATA", desc = "")]
    #[account(4, name = "Token_0 Mint", desc = "")]
    #[account(5, name = "Token_1 Mint", desc = "")]
    #[account(6, name = "Vault_0 ATA", desc = "")]
    #[account(7, name = "Vault_1 ATA", desc = "")]
    #[account(8, name = "LP Mint", desc = "")]
    #[account(9, name = "LP_USER_ATA", desc = "")]
    #[account(10, name = "rent_sysvar", desc = "")]
    #[account(11, name = "system_program", desc = "")]
    #[account(12, name = "token_program", desc = "")]
    InitPool,
    // Liquidate_pool
    #[account(0,name = "Signer")]
    #[account(1, signer,  name = "Pool Account", desc = "")]
    #[account(2, name = "Token_0_ATA", desc = "")]
    #[account(3, name = "Token_1_ATA", desc = "")]
    #[account(4, name = "Token_0 Mint", desc = "")]
    #[account(5, name = "Token_1 Mint", desc = "")]
    #[account(6, name = "Vault_0 ATA", desc = "")]
    #[account(7, name = "Vault_1 ATA", desc = "")]
    #[account(8, name = "LP Mint", desc = "")]
    #[account(9, name = "LP_USER_ATA", desc = "")]
    #[account(10, name = "rent_sysvar", desc = "")]
    #[account(11, name = "system_program", desc = "")]
    #[account(12, name = "token_program", desc = "")]
    LiquidatePool,
    // Deliquidate_pool
    #[account(0,name = "Signer")]
    #[account(1, signer,  name = "Pool Account", desc = "")]
    #[account(2, name = "Token_0_ATA", desc = "")]
    #[account(3, name = "Token_1_ATA", desc = "")]
    #[account(4, name = "Token_0 Mint", desc = "")]
    #[account(5, name = "Token_1 Mint", desc = "")]
    #[account(6, name = "Vault_0 ATA", desc = "")]
    #[account(7, name = "Vault_1 ATA", desc = "")]
    #[account(8, name = "LP Mint", desc = "")]
    #[account(9, name = "LP_USER_ATA", desc = "")]
    #[account(10, name = "rent_sysvar", desc = "")]
    #[account(11, name = "system_program", desc = "")]
    #[account(12, name = "token_program", desc = "")]
    DeliquidatePool
    //SWAP
}