CREATE TABLE IF NOT EXISTS machine_payout (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  machine_id UUID REFERENCES machine(id) NOT NULL,
  amount BIGINT NOT NULL,
  wallet_destination VARCHAR NOT NULL,
  token_account VARCHAR,
  status VARCHAR NOT NULL DEFAULT('pending'),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  modified_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
