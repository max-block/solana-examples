.PHONY: localnet client

localnet-validator:
	solana-test-validator -r --ledger localnet/ledger

localnet-init:
	solana airdrop 1 CD6To88A4KrApbnDUkHrwpjMY5ufgPpVQzm9rRX5d3ro -u localhost
	solana airdrop 1 9C8ARBpAqcmoDfqZTDtvB1JgZC7gjvcq48xRJoR7Wpeq -u localhost

deploy:
	cd program; cargo build-bpf
	solana program deploy program/target/deploy/program.so -u localhost --program-id localnet/program.json


client-cli:
	cd client/cli; ./node_modules/.bin/ts-node main.ts


client-browser:
	cd client/browser; npm start


