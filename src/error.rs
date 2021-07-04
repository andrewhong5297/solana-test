use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

/* not sure what this means

Also, make sure to add the new error variant inside error.rs and adjust the use statement:

From

use crate::instruction::EscrowInstruction;
To

use crate::{instruction::EscrowInstruction, error::EscrowError};

*/