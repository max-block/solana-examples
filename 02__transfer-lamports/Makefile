.PHONY: client

localnet:
	solana-test-validator -r

build:
	cd program; cargo build-bpf

deploy: build
	solana program deploy program/target/deploy/program.so -u localhost --program-id keys/program.json

airdrop:
	solana airdrop 1 CD6To88A4KrApbnDUkHrwpjMY5ufgPpVQzm9rRX5d3ro -u localhost
	solana airdrop 1 9C8ARBpAqcmoDfqZTDtvB1JgZC7gjvcq48xRJoR7Wpeq -u localhost


cli:
	cd client/cli; ./node_modules/.bin/ts-node main.ts

browser:
	cd client/browser; npm start