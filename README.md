# Getting Started

1. Generate certs (`openssl` required)
  - `make bootstrap_ca` to generate the Certificate Authority
  - `make bootstrap_server` to generate the server certificate and key, signed with the CA -- this is for the MQTT broker
  - `make bootstrap_client` to generate the client certificate and key, signed with the CA -- this is for the Oracle

2. Run the stack with `docker compose up -d`
3. Send a test message with `make send_message`
