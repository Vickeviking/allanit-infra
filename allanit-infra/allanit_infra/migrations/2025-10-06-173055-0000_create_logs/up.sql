CREATE TABLE logs (
  id          SERIAL PRIMARY KEY,
  created_at  TIMESTAMP NOT NULL,
  expires_at  TIMESTAMP NOT NULL,
  level       VARCHAR   NOT NULL,
  module      VARCHAR   NOT NULL,
  action      VARCHAR   NOT NULL,
  custom_msg  TEXT
);

CREATE INDEX idx_logs_expires_at ON logs (expires_at);
CREATE INDEX idx_logs_created_at ON logs (created_at);
CREATE INDEX idx_logs_action ON logs (action);

