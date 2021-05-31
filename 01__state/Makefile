.PHONY: localnet client

localnet:
	solana-test-validator -r --ledger localnet/ledger

airdrop:
	solana airdrop 1 EG7uy9FCe4AxL9AavEA1nXDfo2AoBo1ZtBCV224hmoub -u localhost
	solana airdrop 1 FKr2pLkJXFpnJf2sUtStVwDiQPq61rKngtXyhLw8SQbF -u localhost

deploy:
	cd program; cargo build-bpf
	solana program deploy program/target/deploy/counter.so -u localhost --program-id localnet/program.json


client:
	cd client; ./node_modules/.bin/ts-node src/main.ts


