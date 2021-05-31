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

#[cfg(test)]
mod test {
    use borsh::BorshSerialize;

    use crate::state::*;

    #[test]
    fn test_serialization() {
        let data = Counter { last_user: [7_u8; 32], value: 9_999_999_999 }.try_to_vec().unwrap();
        assert_eq!(data, [7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 255, 227, 11, 84, 2, 0, 0, 0]);
    }
}
