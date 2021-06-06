use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::{id, COUNTER_SEED, SETTINGS_SEED};

/// Each user has his own counter account.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Counter {
    /// Increment this field every time by 1
    pub counter: u32,

    /// Value of a counter
    pub value: i64,
}

impl Counter {
    pub fn get_counter_pubkey(user: &Pubkey) -> Pubkey {
        Pubkey::create_with_seed(user, COUNTER_SEED, &id()).unwrap()
    }

    pub fn is_ok_counter_pubkey(user: &Pubkey, counter: &Pubkey) -> bool {
        counter.to_bytes() == Self::get_counter_pubkey(user).to_bytes()
    }
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

impl Settings {
    pub fn get_settings_pubkey_with_bump() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SETTINGS_SEED.as_bytes()], &id())
    }

    pub fn get_settings_pubkey() -> Pubkey {
        let (pubkey, _) = Self::get_settings_pubkey_with_bump();
        pubkey
    }

    pub fn is_ok_settings_pubkey(settings_pubkey: &Pubkey) -> bool {
        let (pubkey, _) = Self::get_settings_pubkey_with_bump();
        pubkey.to_bytes() == settings_pubkey.to_bytes()
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use borsh::BorshSerialize;

    use crate::state::*;

    #[test]
    fn test_serialization() {
        let data = Settings { admin: [7_u8; 32], inc_step: 19, dec_step: 99 }.try_to_vec().unwrap();
        assert_eq!(
            data,
            [
                7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
                7, 7, 7, 7, 19, 0, 0, 0, 99, 0, 0, 0
            ]
        );
    }

    #[test]
    fn test_get_settings_address_with_seed() {
        let (address, bump) = Settings::get_settings_pubkey_with_bump();
        assert_eq!(
            address,
            Pubkey::from_str("4voA9ct4uAJuBVLNfoaPiU1VgpatMpGKRLHfvP8CZ147").unwrap()
        );
        assert_eq!(bump, 255);
    }

    #[test]
    fn test_get_counter_pubkey() {
        let pubkey = Counter::get_counter_pubkey(
            &Pubkey::from_str("FKr2pLkJXFpnJf2sUtStVwDiQPq61rKngtXyhLw8SQbF").unwrap(),
        );
        assert_eq!(
            pubkey,
            Pubkey::from_str("9JVaomeo7Ps8D41whGLkz1c1wzWGfKpk62Mopnf3B274").unwrap()
        );
    }
}
