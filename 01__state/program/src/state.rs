use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Counter {
    /// Public key of the user who changed the
    pub last_user: [u8; 32],

    /// Value of the counter
    pub value: i64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Settings {
    /// Who can reset the counter and change the settings
    pub admin: [u8; 32],

    /// Step value for increment
    pub inc_step: u32,

    /// Step value for decrement
    pub dec_step: u32,
}
