use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum CounterInstruction {
    /// Increment the counter
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` user who inc the counter
    /// 1. `[writable]` counter_account, PDA
    /// 2. `[]` settings_account, PDA
    Inc,

    /// Decrement the counter
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` user who inc the counter
    /// 2. `[writable]` counter_account, PDA
    /// 2. `[]` settings_account, PDA
    Dec,

    /// Update settings for the counter. Only admin can do it
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` Admin of the counter
    /// 1. `[writable]` settings_account, PDA
    /// 2. `[]` Rent sysvar
    /// 3. `[]` System program
    UpdateSettings { admin: [u8; 32], inc_step: u32, dec_step: u32 },
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use borsh::BorshSerialize;
    use solana_program::pubkey::Pubkey;

    use crate::instruction::CounterInstruction;

    #[test]
    fn test_serialization() {
        let data = CounterInstruction::UpdateSettings {
            admin: Pubkey::from_str("EG7uy9FCe4AxL9AavEA1nXDfo2AoBo1ZtBCV224hmoub").unwrap().to_bytes(),
            inc_step: 19,
            dec_step: 99,
        }
        .try_to_vec()
        .unwrap();
        assert_eq!(
            data,
            [
                2, 197, 7, 117, 129, 90, 151, 178, 48, 248, 208, 199, 5, 17, 134, 51, 183, 155, 153, 209, 86, 177, 138, 127, 133,
                1, 191, 178, 128, 179, 23, 157, 98, 19, 0, 0, 0, 99, 0, 0, 0
            ]
        );
    }
}
