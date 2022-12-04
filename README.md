# Solana smart contract for integrity validation of Kontent.ai content item variants

This project serves for storing accounts containing [Kontent.ai's](https://kontent.ai/) content item variant metadata that are used for data integrity validation. The smart contract is build on [Anchor](https://www.anchor-lang.com/).

Relates with these repositories:
- https://github.com/Radimersky/Content-item-variant-signature-service
- https://github.com/Radimersky/Client-interface-for-Kontent.ai-data-integrity-validation

## How to run localy

Make sure you’re on the localnet.
- solana config set --url localhost

And check your Anchor.toml file.

Run the tests.
- anchor test

Build, deploy and start a local ledger.
- anchor localnet

Or

- solana-test-validator

Then

- anchor build
- anchor deploy

## How to run on devnet

- solana config set --url devnet

And update your Anchor.toml file to devnet.

Airdrop yourself some money if necessary.

- solana airdrop 5

Build and deploy to devnet.

- anchor build
- anchor deploy
