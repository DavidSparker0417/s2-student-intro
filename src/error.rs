use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StudentIntroError {
  #[error("Data length exceeds limit")]
  InvalidDataLength,
  #[error("Derived PDA is not match with passed in PDA")]
  InvalidPDA,
}

impl From<StudentIntroError> for ProgramError {
    fn from(e: StudentIntroError) -> Self {
        ProgramError::Custom(e as u32)
    }
}