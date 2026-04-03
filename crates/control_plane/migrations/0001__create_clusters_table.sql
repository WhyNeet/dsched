CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE CLUSTER_STATUS AS ENUM ('disconnected', 'connected');

CREATE TABLE IF NOT EXISTS clusters (
  id UUID NOT NULL,
  key VARCHAR NOT NULL,
  display_name VARCHAR NOT NULL,
  status CLUSTER_STATUS NOT NULL,
  last_seen TIMESTAMPTZ,
  connected_at TIMESTAMPTZ,
  address VARCHAR
);
