pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

pub const COUNTER_SEED: &str = "seed";
pub const SETTINGS_SEED: &str = "settings";
