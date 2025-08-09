use pinocchio::program_error::ProgramError;
use pinocchio::pubkey;
use pinocchio::pubkey::Pubkey;
use crate::errors::SolanaCoreError;
pub trait DataLen {
    const LEN: usize;
}

#[inline(always)]
pub unsafe fn load_acc_unchecked<T: DataLen>(bytes: &[u8]) -> Result<&T, SolanaCoreError> {
    if bytes.len() != T::LEN {
        return Err(SolanaCoreError::InvalidAccountData);
    }
    Ok(&*(bytes.as_ptr() as *const T))
}

#[inline(always)]
pub unsafe fn load_acc_mut_unchecked<T: DataLen>(bytes: &mut [u8]) -> Result<&mut T, SolanaCoreError> {
    if bytes.len() != T::LEN {
        return Err(SolanaCoreError::InvalidAccountData);
    }
    Ok(&mut *(bytes.as_mut_ptr() as *mut T))
}

#[inline(always)]
pub unsafe fn load_ix_data<T: DataLen>(bytes: &[u8]) -> Result<&T, SolanaCoreError> {
    if bytes.len() != T::LEN {
        return Err(SolanaCoreError::InvalidInstructionData.into());
    }
    Ok(&*(bytes.as_ptr() as *const T))
}

#[inline(always)]
pub unsafe fn to_bytes<T: DataLen>(data: &T) -> &[u8] {
    core::slice::from_raw_parts(data as *const T as *const u8, T::LEN)
}

#[inline(always)]
pub unsafe fn to_mut_bytes<T: DataLen>(data: &mut T) -> &mut [u8] {
    core::slice::from_raw_parts_mut(data as *mut T as *mut u8, T::LEN)
}

#[inline(always)]
pub fn validate_pda(seeds_and_bump:&[&[u8]], address: &Pubkey) -> Result<(), SolanaCoreError> {
    let derive_pda = pubkey::create_program_address(seeds_and_bump, &crate::ID).expect("The pda should match");
    //(DE)reference derive_pda to get it's value...
    if derive_pda != *address {
        return Err(SolanaCoreError::PdaMismatch.into());
    }else {
        Ok(())
    }
}