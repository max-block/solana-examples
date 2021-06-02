.PHONY: localnet client

localnet:
	solana-test-validator -r --ledger localnet/ledger

airdrop:
	solana airdrop 1 CD6To88A4KrApbnDUkHrwpjMY5ufgPpVQzm9rRX5d3ro -u localhost
	solana airdrop 1 9C8ARBpAqcmoDfqZTDtvB1JgZC7gjvcq48xRJoR7Wpeq -u localhost

deploy:
	cd program; cargo build-bpf
	solana program deploy program/target/deploy/program.so -u localhost --program-id localnet/program.json


client-cli: airdrop deploy
	cd client/cli; ./node_modules/.bin/ts-node main.ts


client-browser: airdrop deploy
	cd client/browser; npm start


