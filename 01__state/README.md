# Manage state in a Solana on-chain program
It's a counter program. Each user has his own counter. There is one settings account which only admin can manage.


_Topics:_
- Basics about Solana programming model   
- Serialization and deserialization instruction_data and state in Rust using `borsh`
- Serialization and deserialization instruction_data and state on the client side using `buffer-layout`
- Unit and functional tests for on-chain Solana programs
- Using Solana client `@solana/web3.js`
- Program Derived Addresses
- Create accounts inside on-chain programs
- Deploy on testnet and check it using Solana explorer

YouTube link (in Russian): *work in progress*  


### Instruction
```rust
pub enum CounterInstruction {
    /// Increment a counter.
    /// Accounts:
    /// 0. `[signer]` owner of a counter
    /// 1. `[writable]` counter_account, PDA
    /// 2. `[]` settings_account, PDA
    Inc,

    /// Decrement a counter.
    /// Accounts:
    /// 0. `[signer]` owner of a counter
    /// 2. `[writable]` counter_account, PDA
    /// 2. `[]` settings_account, PDA
    Dec,

    /// Update settings for counters. Only admin can do it.
    /// Accounts:
    /// 0. `[signer, writable]` Admin of counters
    /// 1. `[writable]` settings_account, PDA
    /// 2. `[]` Rent sysvar
    /// 3. `[]` System program
    UpdateSettings { admin: [u8; 32], inc_step: u32, dec_step: u32 },
}
```

### State
```rust
/// Each user has his own counter account.
pub struct Counter {
    /// Increment this field every time by 1
    pub counter: u32,

    /// Value of a counter
    pub value: i64,
}


/// There is only one settings account. All counter accounts use it
pub struct Settings {
    /// Only admin can change this account
    pub admin: [u8; 32],

    /// Step value for increment
    pub inc_step: u32,

    /// Step value for decrement
    pub dec_step: u32,
}
```


### Accounts:
```
program: 9onZvMzqAFzSHJrLNVWfqLRFFQ5ZCGzNXB4PBxmp6z5Y
admin: EG7uy9FCe4AxL9AavEA1nXDfo2AoBo1ZtBCV224hmoub
user: FKr2pLkJXFpnJf2sUtStVwDiQPq61rKngtXyhLw8SQbF
settings: 4voA9ct4uAJuBVLNfoaPiU1VgpatMpGKRLHfvP8CZ147
counter: 9JVaomeo7Ps8D41whGLkz1c1wzWGfKpk62Mopnf3B274
```


### Usage:
```
# Run a localnet cluster (don't stop it)
$ make localnet-validator

# Aidrop localnet accounts
$ make localnet-init

# Deploy the Solana on-chain program
$ make localnet-deploy

# Run the client
$ make client 
```

### Links:
- https://docs.solana.com/developing/programming-model/overview
- https://borsh.io
- https://github.com/pabigot/buffer-layout
- https://explorer.solana.com/address/9onZvMzqAFzSHJrLNVWfqLRFFQ5ZCGzNXB4PBxmp6z5Y?cluster=testnet