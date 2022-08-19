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
}

#[derive(BorshDeserialize)]
struct MovieReviewPayload {
    title: String,
    rating: u8,
    review: String,
}

impl MovieReviewInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => {
                let payload = MovieReviewPayload::try_from_slice(rest).unwrap();
                Self::AddMovieReview {
                    title: payload.title,
                    rating: payload.rating,
                    review: payload.review,
                }
            }
            1 => {
                let payload = MovieReviewPayload::try_from_slice(rest).unwrap();
                Self::UpdateMovieReview {
                    title: payload.title,
                    rating: payload.rating,
                    review: payload.review,
                }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
