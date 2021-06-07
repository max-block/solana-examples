use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub enum Instruction {
    Transfer { amount: u64 },
    Approve { amount: u64 },
}

/// Accounts expected:
/// 0. `[signer]` from authority
/// 1. `[writable]` from_token
/// 2. `[writable]` to_token
/// 3. `[]` token program
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    msg!("input: {:?}", input);
    let acc_iter = &mut accounts.iter();
    let from_info = next_account_info(acc_iter)?;
    let from_token_info = next_account_info(acc_iter)?;
    let to_token_info = next_account_info(acc_iter)?;
    let token_info = next_account_info(acc_iter)?;
    // It's a good idea to check all accounts in a real app...

    match Instruction::try_from_slice(input)? {
        Instruction::Transfer { amount } => {
            msg!("transfer: {}", amount);
            let ix = spl_token::instruction::transfer(
                token_info.key,
                from_token_info.key,
                to_token_info.key,
                from_info.key,
                &[from_info.key],
                amount,
            )?;
            invoke(
                &ix,
                &[
                    from_token_info.clone(),
                    to_token_info.clone(),
                    from_info.clone(),
                    token_info.clone(),
                ],
            )?;
            msg!(
                "transfer from {} to {} amount {}: done",
                from_token_info.key,
                to_token_info.key,
                amount
            );
        }
        Instruction::Approve { amount } => {
            msg!("approve: {}", amount);
            let ix = spl_token::instruction::approve(
                token_info.key,
                from_token_info.key,
                to_token_info.key,
                from_info.key,
                &[from_info.key],
                amount,
            )?;
            invoke(
                &ix,
                &[
                    from_token_info.clone(),
                    to_token_info.clone(),
                    from_info.clone(),
                    token_info.clone(),
                ],
            )?;
            msg!(
                "approve from {} to {} amount {}: done",
                from_token_info.key,
                to_token_info.key,
                amount
            );
        }
    }

    Ok(())
}
