INSERT INTO customers (external_id, name, email, phone, org_no)
VALUES
  ('C-100','Maskad Kund AB','maskad.kund@example.test','0700000000','556000-0000'),
  ('C-101','Fikafabriken','kontakt@fikafabriken.example.test','0700000001','559000-0001')
ON CONFLICT (external_id) DO NOTHING;

INSERT INTO purchase_orders (external_id, customer_id, status, description, amount)
SELECT 'PO-200', c.id, 'open', 'Snöröjning kvarter A', 3450.00
FROM customers c WHERE c.external_id = 'C-100'
ON CONFLICT (external_id) DO NOTHING;

INSERT INTO purchase_orders (external_id, customer_id, status, description, amount)
SELECT 'PO-201', c.id, 'completed', 'Fasadtvätt del B', 2200.00
FROM customers c WHERE c.external_id = 'C-101'
ON CONFLICT (external_id) DO NOTHING;
