# Getting Started

1. Generate certs (`openssl` required)
  - `make bootstrap_ca` to generate the Certificate Authority
  - `make bootstrap_server` to generate the server certificate and key, signed with the CA -- this is for the MQTT broker
  - `make bootstrap_client` to generate the client certificate and key, signed with the CA -- this is for the Oracle

2. Run the stack with `docker compose up -d`
3. Send a test message with `make send_message`

## Setting up with Solana's Devnet

1. `make solana_devnet`
2. `make solana_create_wallet`
3. `make solana_fund_wallet` or go to https://faucet.solana.com and do this for `devnet` manually; sometimes the CLI doesn't work
4. `make solana_create_token`
5. `make solana_create_token_account`
6. `make solana_mint_token` -- optional

You'll modify `oracle.yml` with the proper `mint_address`
