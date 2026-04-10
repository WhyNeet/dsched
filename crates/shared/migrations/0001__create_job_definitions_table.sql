CREATE TABLE IF NOT EXISTS job_definitions (
  id UUID PRIMARY KEY,
  type VARCHAR NOT NULL,
  payload JSONB NOT NULL,
  schedule_type VARCHAR NOT NULL,
  schedule VARCHAR,
  max_retries INTEGER NOT NULL,
  next_run_at TIMESTAMPTZ,
  last_triggered_at TIMESTAMPTZ,
  is_enabled BOOLEAN NOT NULL,
  created_at TIMESTAMPTZ NOT NULL
);
