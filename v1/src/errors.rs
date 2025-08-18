use pinocchio::program_error::ProgramError;

#[derive(Debug, Clone, PartialEq, shank::ShankType)]
pub enum SolanaCoreError {
    InvalidInstructionData,
    PdaMismatch,
    InvalidOwner,
    MathError,
    NotEnoughAccountKeys,
    PoolAccountNotWritable,
    SignerRequired,
    InvalidAccountData,
    AccountAlreadyInitialized,
    MissingRequiredSignature,
    OverFlowDetected,
    Slippage,
}

impl From<SolanaCoreError> for ProgramError {
    fn from(e: SolanaCoreError) -> Self {
        Self::Custom(e as u32)
    }
}

impl From<ProgramError> for SolanaCoreError {
    fn from(_e: ProgramError) -> Self {
        // For simplicity, we'll map all ProgramErrors to InvalidInstructionData
        SolanaCoreError::InvalidInstructionData
    }
}
