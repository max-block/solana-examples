.PHONY: localnet client

localnet-validator:
	solana-test-validator -r --ledger localnet/ledger

localnet-init:
	solana airdrop 1 CD6To88A4KrApbnDUkHrwpjMY5ufgPpVQzm9rRX5d3ro -u localhost
	solana airdrop 1 CD6To88A4KrApbnDUkHrwpjMY5ufgPpVQzm9rRX5d3ro -u localhost
	solana airdrop 1 2EheLY8aWQcKPsSXpTs7teiwtBume8gLRaFQzB4HHxJP -u localhost
	spl-token create-token localnet/token.json
	spl-token create-account CZyEKArwVYSKkv9im3grGNXmggbPfS8YGUovBnzoKQ4s --owner localnet/alice.json
	spl-token create-account CZyEKArwVYSKkv9im3grGNXmggbPfS8YGUovBnzoKQ4s --owner localnet/bob.json
	spl-token mint CZyEKArwVYSKkv9im3grGNXmggbPfS8YGUovBnzoKQ4s 300 G6GTsFAnYP1PaNc1g36SF4iuEiosfTZZCWWdnCNxxA8d # mint to alice 300 SPL

localnet-state:
	spl-token account-info --address G6GTsFAnYP1PaNc1g36SF4iuEiosfTZZCWWdnCNxxA8d # alice
	spl-token account-info --address 82SyqQyffa3yeUbuCfcfrHGg3LjdKr919aUqVT7uXkez # bob
	spl-token account-info --address Eau9odNYvqtAFmAVJx2wcH358gd4PAeshF1X8VEyMHWb # carol

deploy:
	cd program; cargo build-bpf
	solana program deploy program/target/deploy/program.so -u localhost --program-id localnet/program.json


client:
	cd client/; ./node_modules/.bin/ts-node main.ts


