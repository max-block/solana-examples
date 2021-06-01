use borsh::{BorshDeserialize, BorshSerialize};

/// Each user has his own counter account.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Counter {
    /// Increment this field every time by 1
    pub counter: u32,

    /// Value of the counter
    pub value: i64,
}

/// There is only one settings account. All counter accounts use it.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Settings {
    /// Only admin can change this account
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
        let data = Settings { admin: [7_u8; 32], inc_step: 19, dec_step: 99 }.try_to_vec().unwrap();
        assert_eq!(
            data,
            [
                7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 19, 0, 0, 0, 99,
                0, 0, 0
            ]
        );
    }
}
