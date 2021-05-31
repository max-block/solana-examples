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

    /// Reset the counter. Only admin can do it.
    ///
    /// Accounts expected:
    ///
    /// 0. `[singer]` Admin of the counter
    /// 1. `[writable]` counter_account, PDA
    /// 2. `[]` settings_account, PDA
    Reset,

    /// Update settings for the counter. Only admin can do it
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` Admin of the counter
    /// 1. `[writable]` settings_account, PDA
    UpdateSettings { inc_step: u32, dec_step: u32 },
}

#[cfg(test)]
mod test {
    use borsh::BorshSerialize;

    use crate::instruction::CounterInstruction;

    #[test]
    fn test_serialization() {
        let data = CounterInstruction::UpdateSettings { inc_step: 19, dec_step: 99 }.try_to_vec().unwrap();
        assert_eq!(data, [3, 19, 0, 0, 0, 99, 0, 0, 0]);
    }
}

