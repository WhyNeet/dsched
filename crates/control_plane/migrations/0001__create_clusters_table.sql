CREATE TYPE CLUSTER_STATUS AS ENUM ('disconnected', 'connected');

CREATE TABLE IF NOT EXISTS clusters (
  key VARCHAR PRIMARY KEY,
  status CLUSTER_STATUS NOT NULL,
  last_seen TIMESTAMPTZ,
  connected_at TIMESTAMPTZ,
  address VARCHAR
);
