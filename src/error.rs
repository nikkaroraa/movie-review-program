use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MovieReviewError {
    #[error("account not initialized yet")]
    UninitializedAccount,

    #[error("PDA derived does not equal PDA passed in")]
    InvalidPDA,

    #[error("input data exceeds max length")]
    InvalidDataLength,

    #[error("rating should be between 1 & 5, both inclusive")]
    InvalidRating,

    #[error("incorrect account")]
    IncorrectAccount,

    #[error("amount overflow")]
    AmountOverflow,
}

impl From<MovieReviewError> for ProgramError {
    fn from(e: MovieReviewError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
