use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum CounterError {
    #[error("Admin signature is required")]
    AdminRequired,

    #[error("Wrong counter PDA for this user")]
    WrongCounterPDA,

    #[error("Wrong settings PDA")]
    WrongSettingsPDA,
}

impl From<CounterError> for ProgramError {
    fn from(e: CounterError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
