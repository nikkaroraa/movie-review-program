use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum MovieReviewInstruction {
    AddMovieReview {
        title: String,
        rating: u8,
        review: String,
    },
    UpdateMovieReview {
        title: String,
        rating: u8,
        review: String,
    },
    DeleteMovieReview {
        title: String,
    },
}

#[derive(BorshDeserialize)]
struct AddMovieReviewPayload {
    title: String,
    rating: u8,
    review: String,
}

#[derive(BorshDeserialize)]
struct UpdateMovieReviewPayload {
    title: String,
    rating: u8,
    review: String,
}

#[derive(BorshDeserialize)]
struct DeleteMovieReviewPayload {
    title: String,
}

impl MovieReviewInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => {
                let payload = AddMovieReviewPayload::try_from_slice(rest).unwrap();
                Self::AddMovieReview {
                    title: payload.title,
                    rating: payload.rating,
                    review: payload.review,
                }
            }
            1 => {
                let payload = UpdateMovieReviewPayload::try_from_slice(rest).unwrap();
                Self::UpdateMovieReview {
                    title: payload.title,
                    rating: payload.rating,
                    review: payload.review,
                }
            }
            2 => {
                let payload = DeleteMovieReviewPayload::try_from_slice(rest).unwrap();
                Self::DeleteMovieReview {
                    title: payload.title,
                }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
