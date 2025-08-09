    use pinocchio_token::state::Mint;
    use crate::errors::SolanaCoreError;
    use pinocchio::account_info::AccountInfo;
    pub fn get_mint_supply(mint_info: &AccountInfo) -> Result<u64, SolanaCoreError> {
        let mint = Mint::from_account_info(mint_info).expect("");
        let mint_supply = mint.supply() as u64;
        Ok(mint_supply)
    }