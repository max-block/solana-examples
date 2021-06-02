.PHONY: localnet client

localnet-validator:
	solana-test-validator -r --ledger localnet/ledger

localnet-init:
	solana airdrop 1 CD6To88A4KrApbnDUkHrwpjMY5ufgPpVQzm9rRX5d3ro -u localhost
	spl-token create-token localnet/token.json
	spl-token create-account CZyEKArwVYSKkv9im3grGNXmggbPfS8YGUovBnzoKQ4s --owner localnet/alice.json
	spl-token create-account CZyEKArwVYSKkv9im3grGNXmggbPfS8YGUovBnzoKQ4s --owner localnet/bob.json
	spl-token mint CZyEKArwVYSKkv9im3grGNXmggbPfS8YGUovBnzoKQ4s 1000000 G6GTsFAnYP1PaNc1g36SF4iuEiosfTZZCWWdnCNxxA8d

deploy:
	cd program; cargo build-bpf
	solana program deploy program/target/deploy/program.so -u localhost --program-id localnet/program.json


client: localnet-init deploy
	cd client/; ./node_modules/.bin/ts-node main.ts


