# spl-examples
List of on-chain Solana programs which demonstrate different aspects of Solana architecture.

----

## 01__state
It's a counter program. Each user has his own counter. There is one settings account which only admin can manage.

*Topics:*
- Basics about Solana programming model   
- Serialization and deserialization instruction_data and state in Rust using `borsh`
- Serialization and deserialization instruction_data and state on the client side using `buffer-layout`
- Unit and functional tests for on-chain Solana programs
- Using Solana client `@solana/web3.js`
- Program Derived Addresses
- Create accounts inside on-chain programs
- Deploy on testnet and check it using Solana explorer

[Comments about this example on YouTube (in Russian)](https://www.youtube.com/watch?v=uUfhqHBoQpU)  

---

## 02__transfer-lamports
Transfer lamports inside a Solana on-chain program

_Topics:_
- Invoke system instructions inside on-chain programs
- Cross-Program Invocations
- Privilege extension
- Phantom and Sollet wallet integrations

YouTube link (in Russian): *work in progress*

---

## 03__transfer-tokens
Transfer and approve SPL tokens inside a Solana on-chain program

*Topics:*
- SPL Token
- SPL Associated Token Account Program
- javascript @solana/spl-token
- https://github.com/solana-labs/token-list

YouTube link (in Russian): *work in progress*

---


### TODO:
- Pseudo random
- Python Solana client
- Subscription on changes in Accounts on the client side    
- Compare Solana vs Ethereum