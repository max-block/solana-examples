use std::convert::TryInto;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let alice = next_account_info(account_info_iter)?;
    let bob = next_account_info(account_info_iter)?;
    // the third account is SystemProgram. Don't forget it

    let amount = input
        .get(..8)
        .and_then(|slice| slice.try_into().ok())
        .map(u64::from_le_bytes)
        .ok_or(ProgramError::InvalidInstructionData)?;

    invoke(
        &system_instruction::transfer(alice.key, bob.key, amount),
        &[alice.clone(), bob.clone()],
    )?;
    msg!("transfer {} lamports from {:?} to {:?}", amount, alice.key, bob.key);
    Ok(())
}
