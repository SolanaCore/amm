use crate::errors::SolanaCoreError;
use num_rational::Ratio;

#[inline(always)]
pub fn lp_to_mint(
    deposit_token_0_amount: u64,
    deposit_token_1_amount: u64,
    token_0_amount: u64,
    token_1_amount: u64,
    lp_supply: u64,
) -> Result<(u64, u64, u64), SolanaCoreError> {
    // Ratios of deposit relative to pool reserves
    let token_0_ratio = Ratio::new(deposit_token_0_amount, token_0_amount);
    let token_1_ratio = Ratio::new(deposit_token_1_amount, token_1_amount);

    // Pool token pair ratio (x/y form)
    let pool_ratio = Ratio::new(token_0_amount, token_1_amount);

    if token_0_ratio > token_1_ratio {
        // Limit is token 1 deposit
        let lp_to_mint = (token_1_ratio * lp_supply).to_integer();
        // Max token 0 that can be used while keeping pool ratio
        let max_token_0 = (*pool_ratio.numer())
            .checked_mul(deposit_token_1_amount)
            .ok_or(SolanaCoreError::MathError)?
            / (*pool_ratio.denom());

        Ok((lp_to_mint, max_token_0, deposit_token_1_amount))
    } else {
        // Limit is token 0 deposit
        let lp_to_mint = (token_0_ratio * lp_supply).to_integer();

        // Max token 1 that can be used while keeping pool ratio
        let max_token_1 = deposit_token_0_amount
            .checked_mul(*pool_ratio.denom())
            .ok_or(SolanaCoreError::MathError)?
            / (*pool_ratio.numer());

        Ok((lp_to_mint, deposit_token_0_amount, max_token_1))
    }
}

#[inline(always)]
pub fn lp_to_burn(
    withdraw_token_0_amount: u64,
    withdraw_token_1_amount: u64,
    token_0_amount: u64,
    token_1_amount: u64,
    lp_supply: u64,
) -> (u64, u64, u64) {
    // Ratios of withdraw relative to pool reserves
    let token_0_ratio = Ratio::new(withdraw_token_0_amount, token_0_amount);
    let token_1_ratio = Ratio::new(withdraw_token_1_amount, token_1_amount);

    // Pool token pair ratio (x/y form)
    let pool_ratio = Ratio::new(token_0_amount, token_1_amount);

    if token_0_ratio > token_1_ratio {
        // Limit is token 1 withdraw
        let lp_to_burn = (token_1_ratio * lp_supply).to_integer();
        // Max token 0 that can be used while keeping pool ratio
        let max_token_0 = (*pool_ratio.numer())
            .checked_mul(withdraw_token_1_amount)
            .unwrap()
            / (*pool_ratio.denom());

        (lp_to_burn, max_token_0, withdraw_token_1_amount)
    } else {
        // Limit is token 0 withdraw
        let lp_to_burn = (token_0_ratio * lp_supply).to_integer();

        // Max token 1 that can be used while keeping pool ratio
        let max_token_1 = withdraw_token_0_amount
            .checked_mul(*pool_ratio.denom())
            .unwrap()
            / (*pool_ratio.numer());

        (lp_to_burn, withdraw_token_0_amount, max_token_1)
    }
}

//token_0_amount(arg) is exclusive of fees i.e token_0_amount - (token_0_amount * fees_bump/100)
// slippagem tolerance 
#[inline(always)]
pub fn calculate_token_out(token_0_amount: u64, expected_token_1_amount: u64, slippage_bps: u64, token_0_liquidity: u64, token_1_liquidity: u64) -> Result<u64, SolanaCoreError> {
    //formula = k = x*y
    let  k: u64 = token_0_liquidity.checked_mul(token_1_liquidity).ok_or(SolanaCoreError::OverFlowDetected)?;
    let updated_k: u64 = (token_0_amount + token_0_liquidity).checked_mul(expected_token_1_amount).ok_or(SolanaCoreError::OverFlowDetected).unwrap(); 
    let token_1_out = k.checked_div(updated_k);

    if token_1_out >= expected_token_1_amount.checked_sub(expected_token_1_amount.checked_sub(slippage_bps/100).expect("The the subtraction shouldn't cause underflow")) {
        return Err(SolanaCoreError::Slippage.into())
    } else {
        Ok(token_1_out.expect("Token_1_out should be slippage adjusted"))
    }

}