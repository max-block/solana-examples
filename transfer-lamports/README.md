# Transfer lamports inside a Solana on-chain program 

_Topics:_
- Invoke system instructions inside on-chain programs
- Cross-Program Invocations
- Privilege extension
- Phantom and Sollet wallet integrations

[Comments about this example on YouTube (in Russian)](https://www.youtube.com/watch?v=sl8zY6bturs)

### Instruction
```
Accounts expected:
    0. `[signer, writable]` Debit lamports from this account
    1. `[writable]` Credit lamports to this accoun
    2. `[]` System program
    
Instruction data:
    amount: u64    
```

### Usage
```
# Run a localnet cluster (don't stop it)
$ just localnet-validator

# Aidrop localnet accounts
$ just localnet-init

# Deploy the Solana on-chain program
$ just deploy

# Run  cli client
$ just client-cli 

# Run a browser client
$ just client-browser
```

### Localnet keys:
```
program_id: Cf2FH5TEV6T511C4nJDyuyuaVc34vDA66rmmkwquyWeM
alice: CD6To88A4KrApbnDUkHrwpjMY5ufgPpVQzm9rRX5d3ro
bob: 9C8ARBpAqcmoDfqZTDtvB1JgZC7gjvcq48xRJoR7Wpeq
```

### Links:
- https://docs.solana.com/developing/programming-model/calling-between-programs
- https://github.com/project-serum/sol-wallet-adapter
- https://docs.phantom.app


