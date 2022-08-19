use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use std::convert::TryInto;

use crate::error::MovieReviewError;
use crate::instruction::MovieReviewInstruction;
use crate::state::MovieAccountState;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = MovieReviewInstruction::unpack(instruction_data)?;

    match instruction {
        MovieReviewInstruction::AddMovieReview {
            title,
            rating,
            review,
        } => add_movie_review(program_id, accounts, title, rating, review),
        MovieReviewInstruction::UpdateMovieReview {
            title,
            rating,
            review,
        } => update_movie_review(program_id, accounts, title, rating, review),
    }
}

pub fn add_movie_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: u8,
    review: String,
) -> ProgramResult {
    msg!("adding movie review...");

    msg!("title: {}", title);
    msg!("rating: {}", rating);
    msg!("review: {}", review);

    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    if pda_account.owner != program_id {
        msg!("illegal pda owner");
        return Err(ProgramError::IllegalOwner);
    }

    if !initializer.is_signer {
        msg!("missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), title.as_bytes().as_ref()],
        program_id,
    );
    if pda != *pda_account.key {
        msg!("invalid seeds for PDA");
        return Err(MovieReviewError::InvalidPDA.into());
    }

    if rating > 5 || rating < 1 {
        msg!("rating should be between 1 & 5, both inclusive");
        return Err(MovieReviewError::InvalidRating.into());
    }

    let account_len: usize = 1000;

    if MovieAccountState::get_account_size(title.clone(), review.clone()) > account_len {
        msg!("data length is greater than 1000 bytes");
        return Err(MovieReviewError::InvalidDataLength.into());
    }

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    msg!("creating MovieReview PDA account...");
    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[
            initializer.clone(),
            pda_account.clone(),
            system_program.clone(),
        ],
        &[&[
            initializer.key.as_ref(),
            title.as_bytes().as_ref(),
            &[bump_seed],
        ]],
    )?;
    msg!("PDA created: {}", pda);

    msg!("unpacking state account");
    let mut account_data =
        try_from_slice_unchecked::<MovieAccountState>(&pda_account.data.borrow()).unwrap();
    msg!("borrowed account data");

    msg!("check ing if MovieAccount PDA is already initialized");
    if account_data.is_initialized() {
        msg!("MovieAccount PDA already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    account_data.discriminator = MovieAccountState::DISCRIMINATOR.to_string();
    account_data.title = title;
    account_data.reviewer = *initializer.key;
    account_data.rating = rating;
    account_data.review = review;
    account_data.is_initialized = true;

    msg!("serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("state account serialized");

    Ok(())
}

pub fn update_movie_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: u8,
    review: String,
) -> ProgramResult {
    msg!("updating movie review...");

    // loop through the accounts that should be present
    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;

    // check if initializer has actually signed the tx
    if !initializer.is_signer {
        msg!("initializer hasn't signed the tx");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // find the pda account linked to the review that needs to be updated
    let (pda, _bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), title.as_bytes().as_ref()],
        program_id,
    );

    // verify the found pda matches the passed in pda
    if pda != *pda_account.key {
        msg!("incorrect account passed");
        return Err(MovieReviewError::InvalidPDA.into());
    }

    // deserialize pda data
    msg!("unpacking state account");
    let mut account_data =
        try_from_slice_unchecked::<MovieAccountState>(&pda_account.data.borrow())?;
    msg!("borrowed account data");

    // make sure pda is initialized
    if !account_data.is_initialized() {
        msg!("account is not initialized");
        return Err(MovieReviewError::UninitializedAccount.into());
    }

    if rating > 5 || rating < 1 {
        msg!("rating should be between 1 & 5, both inclusive");
        return Err(MovieReviewError::InvalidRating.into());
    }

    let account_len: usize = 1000;

    if MovieAccountState::get_account_size(title.clone(), review.clone()) > account_len {
        msg!("data length is greater than 1000 bytes");
        return Err(MovieReviewError::InvalidDataLength.into());
    }

    // update data
    account_data.rating = rating;
    account_data.review = review;

    // serialize pda data
    msg!("serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("state account serialized");

    Ok(())
}
