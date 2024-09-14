# Directory for certificates
CERT_DIR = ./dev/mosquitto/certs

# Make sure the certs directory exists
certs_dir:
	@mkdir -p $(CERT_DIR)

# Generate CA key with RSA 3072-bit for better security
generate_ca_key: certs_dir
	@echo "Generating CA key"
	@openssl genpkey -algorithm RSA -out $(CERT_DIR)/ca.key -pkeyopt rsa_keygen_bits:3072

# Generate CA certificate
generate_ca_cert: generate_ca_key
	@echo "Generating CA certificate"
	@openssl req -x509 -new -nodes -key $(CERT_DIR)/ca.key -sha256 -days 3650 -out $(CERT_DIR)/ca.crt -subj "/C=US/ST=CA/L=City/O=TestCA/OU=TestCA/CN=TestCA"

# Bootstrap the CA generation consecutively
bootstrap_ca: generate_ca_key generate_ca_cert

# Generate server key with RSA 3072-bit
generate_server_key: certs_dir
	@echo "Generating server key"
	@openssl genpkey -algorithm RSA -out $(CERT_DIR)/server.key -pkeyopt rsa_keygen_bits:3072

# Generate server CSR
generate_server_csr: generate_server_key
	@echo "Generating server CSR"
	@openssl req -new -key $(CERT_DIR)/server.key -out $(CERT_DIR)/server.csr -subj "/C=US/ST=CA/L=City/O=Test/OU=Test/CN=server"

# Create extension file for subjectAltName
create_server_extfile:
	@echo "Creating server extension file"
	@echo "subjectAltName = DNS:*, DNS:localhost, IP:192.168.1.79, IP:127.0.0.1\nkeyUsage = digitalSignature,keyEncipherment\nextendedKeyUsage = serverAuth" > /tmp/server_extfile.conf

# Sign server certificate
sign_server_cert: generate_server_csr create_server_extfile
	@echo "Signing server certificate"
	@openssl x509 -req -in $(CERT_DIR)/server.csr -CA $(CERT_DIR)/ca.crt -CAkey $(CERT_DIR)/ca.key -CAcreateserial -out $(CERT_DIR)/server.crt -days 365 -sha256 -extfile /tmp/server_extfile.conf
	@rm /tmp/server_extfile.conf

# Bootstrap server certificate generation
bootstrap_server: generate_server_key generate_server_csr sign_server_cert

# Generate client key with RSA 3072-bit
generate_client_key: certs_dir
	@echo "Generating client key"
	@openssl genpkey -algorithm RSA -out $(CERT_DIR)/client.key -pkeyopt rsa_keygen_bits:3072

# Generate client CSR
generate_client_csr: generate_client_key
	@echo "Generating client CSR"
	@openssl req -new -key $(CERT_DIR)/client.key -out $(CERT_DIR)/client.csr -subj "/C=US/ST=CA/L=City/O=Test/OU=Test/CN=client"

# Create client extension file for keyUsage and extendedKeyUsage
create_client_extfile:
	@echo "Creating client extension file"
	@echo "keyUsage = digitalSignature,keyEncipherment\nextendedKeyUsage = clientAuth" > /tmp/client_extfile.conf

# Sign client certificate
sign_client_cert: generate_client_csr create_client_extfile
	@echo "Signing client certificate"
	@openssl x509 -req -in $(CERT_DIR)/client.csr -CA $(CERT_DIR)/ca.crt -CAkey $(CERT_DIR)/ca.key -CAcreateserial -out $(CERT_DIR)/client.crt -days 365 -sha256 -extfile /tmp/client_extfile.conf
	@rm /tmp/client_extfile.conf

# Bootstrap client certificate generation
bootstrap_client: generate_client_key generate_client_csr sign_client_cert

# Bootstrap oracle client
bootstrap_oracle: 
	@echo "Generating oracle client certificate"
	@openssl genpkey -algorithm RSA -out $(CERT_DIR)/oracle.key -pkeyopt rsa_keygen_bits:3072
	@openssl req -new -key $(CERT_DIR)/oracle.key -out $(CERT_DIR)/oracle.csr -subj "/C=US/ST=CA/L=City/O=Oracle/OU=Oracle/CN=oracle"
	@openssl x509 -req -in $(CERT_DIR)/oracle.csr -CA $(CERT_DIR)/ca.crt -CAkey $(CERT_DIR)/ca.key -CAcreateserial -out $(CERT_DIR)/oracle.crt -days 365 -sha256


# Install dependencies for testing
install_test_deps:
	@echo "Installing testing dependencies"
	@brew install mosquitto
	@docker compose up -d

# Send a message to the broker using TLS v1.3
send_message:
	@echo "Sending message with TLS v1.3"
	@mosquitto_pub -h localhost -p 8883 --cafile $(CERT_DIR)/ca.crt --cert $(CERT_DIR)/client.crt --key $(CERT_DIR)/client.key -t "skypal/drone" -m "Hello, world" --tls-version tlsv1.3

# Clean up generated certs
clean:
	@echo "Cleaning up certificates"
	@rm -rf $(CERT_DIR)/*.crt $(CERT_DIR)/*.csr $(CERT_DIR)/*.key $(CERT_DIR)/*.srl


#############################################
# Solana Commands 													#
#############################################

solana_devnet:
	@solana config set --url https://api.devnet.solana.com

solana_localnet:
	@solana config set --url http://localhost:8899

solana_create_wallet:
	@mkdir -p ./dev/solana
	@solana-keygen new --force --outfile ./dev/solana/keypair.json

# May have to go to the faucet to get some SOL manually
solana_fund_wallet:
	@solana airdrop 1 ./dev/solana/keypair.json 

solana_create_token:
	@spl-token create-token --config ./dev/solana/cli-config.yml --url https://api.devnet.solana.com > ./dev/solana/token_mint_address

TOKEN_ADDRESS=`grep "Address:" ./dev/solana/token_mint_address | sed 's/Address:[[:space:]]*//'`

solana_create_token_account:
	@spl-token create-account $(TOKEN_ADDRESS) --config ./dev/solana/cli-config.yml > ./dev/solana/token_account_address

TOKEN_ACCOUNT_ADDRESS=`grep "Creating account" ./dev/solana/token_account_address | sed 's/Creating account[[:space:]]*//'`

solana_mint_token:
	@spl-token mint $(TOKEN_ADDRESS) 1000 $(TOKEN_ACCOUNT_ADDRESS) --config ./dev/solana/cli-config.yml
