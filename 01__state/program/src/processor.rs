use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::error::CounterError;
use crate::instruction::CounterInstruction;
use crate::state::{Counter, Settings};
use crate::{COUNTER_SEED, SETTINGS_SEED};

pub struct Processor;

impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        msg!("counter: {:?}", input);
        let instruction = CounterInstruction::try_from_slice(input)?;
        match instruction {
            CounterInstruction::Inc => Self::process_inc(program_id, accounts),
            CounterInstruction::Dec => Self::process_dec(program_id, accounts),
            CounterInstruction::UpdateSettings { inc_step, dec_step } => {
                Self::process_update_settings(program_id, accounts, inc_step, dec_step)
            }
        }
    }

    fn process_inc(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        msg!("process_inc");
        let acc_iter = &mut accounts.iter();
        let user_account = next_account_info(acc_iter)?;
        let counter_account = next_account_info(acc_iter)?;
        let settings_account = next_account_info(acc_iter)?;

        // Check accounts
        if !user_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if *counter_account.key != Pubkey::create_with_seed(user_account.key, COUNTER_SEED, program_id)? {
            return Err(CounterError::WrongCounterPDA.into());
        }
        Processor::check_settings_pda(program_id, settings_account.key)?;

        let settings = Settings::try_from_slice(&settings_account.data.borrow())?;
        let mut counter = Counter::try_from_slice(&counter_account.data.borrow())?;

        counter.value += settings.inc_step as i64;
        counter.counter += 1;

        let _ = counter.serialize(&mut &mut counter_account.data.borrow_mut()[..]);

        Ok(())
    }

    fn process_dec(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        msg!("process_dec");
        let acc_iter = &mut accounts.iter();
        let user_account = next_account_info(acc_iter)?;
        let counter_account = next_account_info(acc_iter)?;
        let settings_account = next_account_info(acc_iter)?;

        // Check accounts
        if !user_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if *counter_account.key != Pubkey::create_with_seed(user_account.key, COUNTER_SEED, program_id)? {
            return Err(CounterError::WrongCounterPDA.into());
        }
        Processor::check_settings_pda(program_id, settings_account.key)?;

        let settings = Settings::try_from_slice(&settings_account.data.borrow())?;
        let mut counter = Counter::try_from_slice(&counter_account.data.borrow())?;

        counter.value -= settings.dec_step as i64;
        counter.counter += 1;

        let _ = counter.serialize(&mut &mut counter_account.data.borrow_mut()[..]);

        Ok(())
    }

    fn process_update_settings(program_id: &Pubkey, accounts: &[AccountInfo], inc_step: u32, dec_step: u32) -> ProgramResult {
        msg!("process_update_settings: inc_step={}, dec_step={}", inc_step, dec_step);
        let acc_iter = &mut accounts.iter();
        let admin_account = next_account_info(acc_iter)?;
        let settings_account = next_account_info(acc_iter)?;

        Processor::check_settings_pda(program_id, settings_account.key)?;

        // Check that admin do it
        if !admin_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        let settings = Settings::try_from_slice(&settings_account.data.borrow())?;
        if settings.admin != admin_account.key.to_bytes() && settings.admin != [0; 32] {
            return Err(CounterError::AdminRequired.into());
        }

        let settings = Settings { admin: admin_account.key.to_bytes(), inc_step, dec_step };
        let _ = settings.serialize(&mut &mut settings_account.data.borrow_mut()[..]);
        Ok(())
    }

    fn check_settings_pda(program_id: &Pubkey, settings_pubkey: &Pubkey) -> ProgramResult {
        if *settings_pubkey != Pubkey::create_program_address(&[SETTINGS_SEED.as_bytes()], program_id)? {
            return Err(CounterError::WrongSettingsPDA.into());
        }
        Ok(())
    }
}
