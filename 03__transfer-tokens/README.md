# Transfer and approve SPL tokens inside a Solana on-chain program

*Topics:*
- SPL Token
- SPL Associated Token Account Program
- javascript @solana/spl-token
- The community maintained Solana token registry


[Comments about this example on YouTube (in Russian)](https://www.youtube.com/watch?v=JwjZLwjY-4w)

### Instruction
```rust
/// 0. `[signer]` from authority
/// 1. `[writable]` from_token
/// 2. `[writable]` to_token
/// 3. `[]` token program
pub enum Instruction {
    Transfer { amount: u64 },
    Approve { amount: u64 },
}
```

### Links:
- https://spl.solana.com/token
- https://spl.solana.com/associated-token-account
- https://github.com/solana-labs/token-list
- https://docs.rs/spl-token/3.1.1/spl_token/index.html
- https://docs.rs/spl-associated-token-account/1.0.2/spl_associated_token_account/  
- https://github.com/solana-labs/solana-program-library/tree/master/token/js

### Usage:
```
# Run a localnet cluster (don't stop it)
$ make localnet-validator

# Init localnet (create token, token accounts, mint)
$ make localnet-init

# Deploy the Solana on-chain program
$ make deploy

# Run the client
$ make client

# Check the current state
# make localnet-state 
```

### Localnet keys:
```
program_id: Cf2FH5TEV6T511C4nJDyuyuaVc34vDA66rmmkwquyWeM
token: CZyEKArwVYSKkv9im3grGNXmggbPfS8YGUovBnzoKQ4s  
alice: CD6To88A4KrApbnDUkHrwpjMY5ufgPpVQzm9rRX5d3ro
alice token account: G6GTsFAnYP1PaNc1g36SF4iuEiosfTZZCWWdnCNxxA8d  
bob: 9C8ARBpAqcmoDfqZTDtvB1JgZC7gjvcq48xRJoR7Wpeq
bob token account: 82SyqQyffa3yeUbuCfcfrHGg3LjdKr919aUqVT7uXkez
carol: 2EheLY8aWQcKPsSXpTs7teiwtBume8gLRaFQzB4HHxJP
carol token account: Eau9odNYvqtAFmAVJx2wcH358gd4PAeshF1X8VEyMHWb
```

