CREATE TABLE IF NOT EXISTS customers (
  id           BIGSERIAL PRIMARY KEY,
  external_id  TEXT UNIQUE NOT NULL,
  name         TEXT NOT NULL,
  email        TEXT,
  phone        TEXT,
  org_no       TEXT,
  created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_customers_external_id ON customers (external_id);

-- Purchase orders
CREATE TABLE IF NOT EXISTS purchase_orders (
  id           BIGSERIAL PRIMARY KEY,
  external_id  TEXT UNIQUE NOT NULL,
  customer_id  BIGINT REFERENCES customers(id) ON DELETE SET NULL,
  status       TEXT NOT NULL,
  description  TEXT,
  amount       DOUBLE PRECISION NOT NULL DEFAULT 0,
  created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
  updated_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_po_external_id ON purchase_orders (external_id);
CREATE INDEX IF NOT EXISTS idx_po_customer_id ON purchase_orders (customer_id);

-- Inbound e-mail
CREATE TABLE IF NOT EXISTS emails_inbound (
  id            BIGSERIAL PRIMARY KEY,
  message_id    TEXT UNIQUE NOT NULL,
  from_addr     TEXT NOT NULL,
  to_addr       TEXT[] NOT NULL,
  subject       TEXT,
  received_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
  raw_source    TEXT,
  parsed        JSONB,
  status        TEXT NOT NULL DEFAULT 'received',
  created_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_emails_message_id ON emails_inbound (message_id);
CREATE INDEX IF NOT EXISTS idx_emails_received_at ON emails_inbound (received_at);

-- Råfakturor
CREATE TABLE IF NOT EXISTS invoices_raw (
  id                     BIGSERIAL PRIMARY KEY,
  source_system          TEXT NOT NULL,
  external_id            TEXT,
  customer_external_id   TEXT,
  email_id               BIGINT REFERENCES emails_inbound(id) ON DELETE SET NULL,
  payload                JSONB NOT NULL,
  file_uri               TEXT,
  ocr_text               TEXT,
  status                 TEXT NOT NULL DEFAULT 'received',
  received_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
  created_at             TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS idx_invoices_external_id ON invoices_raw (external_id);
CREATE INDEX IF NOT EXISTS idx_invoices_received_at ON invoices_raw (received_at);

-- Systemlogg
CREATE TABLE IF NOT EXISTS system_logs (
  id        BIGSERIAL PRIMARY KEY,
  ts        TIMESTAMPTZ NOT NULL DEFAULT now(),
  module    TEXT NOT NULL,
  level     TEXT NOT NULL,
  action    TEXT NOT NULL,
  message   TEXT,
  meta      JSONB
);
CREATE INDEX IF NOT EXISTS idx_logs_ts ON system_logs (ts);
CREATE INDEX IF NOT EXISTS idx_logs_level ON system_logs (level);
CREATE INDEX IF NOT EXISTS idx_logs_module ON system_logs (module);
