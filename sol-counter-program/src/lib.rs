use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Instructions the program understands.
/// Borsh layout: 1 byte variant tag + 4 byte little-endian u32 value.
///   Increment(5) -> [0, 5, 0, 0, 0]
///   Decrement(2) -> [1, 2, 0, 0, 0]
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub enum InstructionType {
    Increment(u32),
    Decrement(u32),
}

/// Data stored inside the counter account (4 bytes).
#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub struct CounterAccount {
    pub count: u32,
}

entrypoint!(counter_contract);

pub fn counter_contract(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let acc = next_account_info(accounts_iter)?;

    // Only accounts owned by this program may be modified by it.
    if acc.owner != program_id {
        msg!("Counter account is not owned by this program");
        return Err(ProgramError::IncorrectProgramId);
    }
    if !acc.is_writable {
        msg!("Counter account must be writable");
        return Err(ProgramError::InvalidAccountData);
    }

    let instruction = InstructionType::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    let mut counter = CounterAccount::deserialize(&mut &acc.data.borrow()[..])
        .map_err(|_| ProgramError::InvalidAccountData)?;

    match instruction {
        InstructionType::Increment(value) => {
            counter.count = counter
                .count
                .checked_add(value)
                .ok_or(ProgramError::ArithmeticOverflow)?;
            msg!("Incremented by {}", value);
        }
        InstructionType::Decrement(value) => {
            counter.count = counter
                .count
                .checked_sub(value)
                .ok_or(ProgramError::ArithmeticOverflow)?;
            msg!("Decremented by {}", value);
        }
    }

    counter
        .serialize(&mut &mut acc.data.borrow_mut()[..])
        .map_err(|_| ProgramError::AccountDataTooSmall)?;
    msg!("Counter is now {}", counter.count);

    Ok(())
}
