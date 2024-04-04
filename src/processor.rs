use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::{
    error::StudentIntroError, instruction::StudentIntroInstruction, state::StudentIntroState,
};

pub fn my_try_from_slice_unchecked<T: borsh::BorshDeserialize>(
    data: &[u8],
) -> Result<T, ProgramError> {
    let mut data_mut = data;
    match T::deserialize(&mut data_mut) {
        Ok(result) => Ok(result),
        Err(_) => Err(ProgramError::InvalidInstructionData),
    }
}

pub fn process_instrcution(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    msg!("Processing instruction...");
    let instruction = StudentIntroInstruction::unpack(data)?;
    match instruction {
        StudentIntroInstruction::AddIntro { name, message } => {
            add_intro(program_id, accounts, name, message)
        }
        StudentIntroInstruction::UpdateIntro { name, message } => {
            update_intro(program_id, accounts, name, message)
        }
    }
}

pub fn add_intro(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    message: String,
) -> ProgramResult {
    msg!("Add introduction ...");

    // check parameters
    msg!("Name: {}", name);
    msg!("Message: {}", message);
    let total_len = 1 + (4 + name.len()) + (4 + message.len());
    msg!("DataLen : {}", total_len);
    if total_len > 1000 {
        return Err(StudentIntroError::InvalidDataLength.into());
    }

    // Extract accounts
    let account_info_iter = &mut accounts.iter();
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    // check accounts
    msg!("Signer : {}", initializer.key);
    if !initializer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    msg!("PDA account : {}", pda_account.key);
    // Derive PDA
    let (pda, bump_seed) = Pubkey::find_program_address(&[initializer.key.as_ref()], program_id);
    if pda != *pda_account.key {
        return Err(StudentIntroError::InvalidPDA.into());
    }
    // creat account
    // calculate lamports to create account
    let account_size = 1000;
    let rent = Rent::get()?;
    let lamports_for_account = rent.minimum_balance(account_size);
    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            &pda,
            lamports_for_account,
            account_size.try_into().unwrap(),
            program_id,
        ),
        &[
            initializer.clone(),
            pda_account.clone(),
            system_program.clone(),
        ],
        &[&[initializer.key.as_ref(), &[bump_seed]]],
    )?;
    msg!("PDA created! {}", pda);
    // account data
    let mut account_data =
        my_try_from_slice_unchecked::<StudentIntroState>(&pda_account.data.borrow()).unwrap();

    // check if account is alread initialized
    if account_data.is_initialized() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }
    account_data.name = name;
    account_data.message = message;
    account_data.is_initialized = true;
    // serialize
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    Ok(())
}

pub fn update_intro(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    message: String,
) -> ProgramResult {
    msg!("Updating introduction...");
    // check for parameters
    msg!("Name: {}", name);
    msg!("Message: {}", message);
    let total_len = 1 + (4 + name.len()) + (4 + message.len());
    let max_account_len = 1000;
    if total_len > max_account_len {
        return Err(StudentIntroError::InvalidDataLength.into());
    }
    // Extract accounts
    let account_info_iter = &mut accounts.iter();
    let payer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    // check accounts
    msg!("Payer : {}", payer.key);
    msg!("PDA account passed in. {}", pda_account.key);
    msg!("System Program: {}", system_program.key);
    if !payer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if pda_account.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }
    // Get account data
    let mut account_data =
        my_try_from_slice_unchecked::<StudentIntroState>(&pda_account.data.borrow()).unwrap();
    if !account_data.is_initialized {
        return Err(ProgramError::UninitializedAccount);
    }
    msg!("Updating...");
    msg!(
        "Original data. name = {}, message = {}",
        account_data.name,
        account_data.message
    );
    account_data.name = name;
    account_data.message = message;
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("Successfully updated.");
    Ok(())
}
