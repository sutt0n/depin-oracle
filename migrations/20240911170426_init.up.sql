CREATE TABLE IF NOT EXISTS drone (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  machine_id UUID REFERENCES machine(id) NOT NULL,
  serial_number VARCHAR NOT NULL,
  created TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  latitude FLOAT NOT NULL,
  longitude FLOAT NOT NULL,
  altitude FLOAT NOT NULL,
  x_speed FLOAT NOT NULL,
  y_speed FLOAT NOT NULL,
  yaw FLOAT NOT NULL,
  pilot_latitude FLOAT NOT NULL,
  pilot_longitude FLOAT NOT NULL,
  home_latitude FLOAT NOT NULL,
  home_longitude FLOAT NOT NULL
);

CREATE TABLE IF NOT EXISTS machine (
  id UUID PRIMARY KEY NOT NULL,
  latitude FLOAT NOT NULL,
  longitude FLOAT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  last_seen TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS drone_payout (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  drone_id UUID REFERENCES drone(id) NOT NULL,
  amount FLOAT NOT NULL,
  destination VARCHAR NOT NULL,
  status VARCHAR NOT NULL DEFAULT('pending'),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  modified_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS machine_address (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  machine_id UUID REFERENCES machine(id) NOT NULL,
  address VARCHAR NOT NULL,
  status VARCHAR NOT NULL DEFAULT('active'),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  modified_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
