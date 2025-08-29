use pinocchio::program_error::ProgramError;
use solana_program::{decode_error::DecodeError, msg, program_error::PrintProgramError};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, shank::ShankType, Error)]
pub enum SolanaCoreError {
    #[error("Invalid instruction data")]
    InvalidInstructionData,

    #[error("PDA mismatch")]
    PdaMismatch,

    #[error("Invalid account owner")]
    InvalidOwner,

    #[error("Math error")]
    MathError,

    #[error("Not enough account keys")]
    NotEnoughAccountKeys,

    #[error("Pool account not writable")]
    PoolAccountNotWritable,

    #[error("Signer required")]
    SignerRequired,

    #[error("Invalid account data")]
    InvalidAccountData,

    #[error("Account already initialized")]
    AccountAlreadyInitialized,

    #[error("Missing required signature")]
    MissingRequiredSignature,

    #[error("Overflow detected")]
    OverFlowDetected,

    #[error("Slippage error")]
    Slippage,
}

impl PrintProgramError for SolanaCoreError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<SolanaCoreError> for ProgramError {
    fn from(e: SolanaCoreError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for MetadataError {
    fn type_of() -> &'static str {
        "Metadata Error"
    }
}
