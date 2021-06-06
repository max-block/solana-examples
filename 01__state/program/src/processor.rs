use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::{rent::Rent, Sysvar};
use solana_program::{msg, system_instruction};

use crate::error::CounterError;
use crate::instruction::CounterInstruction;
use crate::state::{Counter, Settings};
use crate::{id, SETTINGS_SEED};

pub struct Processor;

impl Processor {
    pub fn process(_program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        msg!("counter: {:?}", input);
        let instruction = CounterInstruction::try_from_slice(input)?;
        match instruction {
            CounterInstruction::Inc => Self::process_inc(accounts),
            CounterInstruction::Dec => Self::process_dec(accounts),
            CounterInstruction::UpdateSettings { admin, inc_step, dec_step } => {
                Self::process_update_settings(accounts, admin, inc_step, dec_step)
            }
        }
    }

    fn process_inc(accounts: &[AccountInfo]) -> ProgramResult {
        msg!("process_inc");
        let acc_iter = &mut accounts.iter();
        let user_info = next_account_info(acc_iter)?;
        let counter_info = next_account_info(acc_iter)?;
        let settings_info = next_account_info(acc_iter)?;

        // Checks
        if !user_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !Counter::is_ok_counter_pubkey(user_info.key, counter_info.key) {
            return Err(CounterError::WrongCounterPDA.into());
        }
        if !Settings::is_ok_settings_pubkey(settings_info.key) {
            return Err(CounterError::WrongSettingsPDA.into());
        }

        let settings = Settings::try_from_slice(&settings_info.data.borrow())?;
        let mut counter = Counter::try_from_slice(&counter_info.data.borrow())?;

        counter.value += settings.inc_step as i64;
        counter.counter += 1;

        let _ = counter.serialize(&mut &mut counter_info.data.borrow_mut()[..]);
        msg!("process_inc: done");
        Ok(())
    }

    fn process_dec(accounts: &[AccountInfo]) -> ProgramResult {
        msg!("process_dec");
        let acc_iter = &mut accounts.iter();
        let user_info = next_account_info(acc_iter)?;
        let counter_info = next_account_info(acc_iter)?;
        let settings_info = next_account_info(acc_iter)?;

        // Checks
        if !user_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !Counter::is_ok_counter_pubkey(user_info.key, counter_info.key) {
            return Err(CounterError::WrongCounterPDA.into());
        }
        if !Settings::is_ok_settings_pubkey(settings_info.key) {
            return Err(CounterError::WrongSettingsPDA.into());
        }

        let settings = Settings::try_from_slice(&settings_info.data.borrow())?;
        let mut counter = Counter::try_from_slice(&counter_info.data.borrow())?;

        counter.value -= settings.dec_step as i64;
        counter.counter += 1;

        let _ = counter.serialize(&mut &mut counter_info.data.borrow_mut()[..]);
        msg!("process_dec: done");

        Ok(())
    }

    fn process_update_settings(
        accounts: &[AccountInfo],
        admin: [u8; 32],
        inc_step: u32,
        dec_step: u32,
    ) -> ProgramResult {
        msg!(
            "process_update_settings: admin={:?} inc_step={}, dec_step={}",
            admin,
            inc_step,
            dec_step
        );
        let acc_iter = &mut accounts.iter();
        let admin_info = next_account_info(acc_iter)?;
        let settings_info = next_account_info(acc_iter)?;
        let rent_info = next_account_info(acc_iter)?;
        let system_program_info = next_account_info(acc_iter)?;

        let (settings_pubkey, bump_seed) = Settings::get_settings_pubkey_with_bump();
        if settings_pubkey != *settings_info.key {
            return Err(ProgramError::InvalidArgument);
        }

        if !admin_info.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if settings_info.data_is_empty() {
            msg!("Creating settings account");
            let settings = Settings { admin: admin_info.key.to_bytes(), inc_step, dec_step };
            let space = settings.try_to_vec()?.len();
            let rent = &Rent::from_account_info(rent_info)?;
            let lamports = rent.minimum_balance(space);
            let signer_seeds: &[&[_]] = &[SETTINGS_SEED.as_bytes(), &[bump_seed]];
            invoke_signed(
                &system_instruction::create_account(
                    admin_info.key,
                    &settings_pubkey,
                    lamports,
                    space as u64,
                    &id(),
                ),
                &[admin_info.clone(), settings_info.clone(), system_program_info.clone()],
                &[&signer_seeds],
            )?;
        }

        let mut settings = Settings::try_from_slice(&settings_info.data.borrow())?;
        if settings.admin != admin_info.key.to_bytes() && settings.admin != [0; 32] {
            return Err(CounterError::AdminRequired.into());
        }
        settings.admin = admin;
        settings.inc_step = inc_step;
        settings.dec_step = dec_step;

        let _ = settings.serialize(&mut &mut settings_info.data.borrow_mut()[..]);
        msg!("process_update_settings: done");
        Ok(())
    }
}
