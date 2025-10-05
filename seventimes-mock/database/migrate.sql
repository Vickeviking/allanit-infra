CREATE TABLE IF NOT EXISTS customers (
  id SERIAL PRIMARY KEY,
  external_id TEXT UNIQUE,
  name TEXT NOT NULL,
  email TEXT,
  phone TEXT,
  org_no TEXT,
  created_at timestamptz DEFAULT now()
);

CREATE TABLE IF NOT EXISTS purchase_orders (
  id SERIAL PRIMARY KEY,
  external_id TEXT UNIQUE,
  customer_id INT REFERENCES customers(id) ON DELETE SET NULL,
  status TEXT NOT NULL DEFAULT 'open',
  description TEXT,
  amount NUMERIC(12,2) NOT NULL DEFAULT 0,
  created_at timestamptz DEFAULT now()
);
