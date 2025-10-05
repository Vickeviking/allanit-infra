# Seventimes Mock API

En lättviktsserver byggd med **FastAPI** och **PostgreSQL** som simulerar de viktigaste delarna av Seventime-systemet. Den används för testning, staging och integrationer mot framtida Rust-gatewayn i **Allanit-infra**.

---

## 🚀 Snabbstart

### 1. Kör via Docker

Från repo-roten (`allanit-infra/`):

```bash
docker compose up --build
```

Det startar:

- **Postgres** på port `5433` (inuti nätverket `db:5432`)
- **Mock-API** på port `4000`

### 2. Hälsokontroll

```bash
curl -s http://localhost:4000/health | jq .
```

Svar:

```json
{ "ok": true, "time": "2025-10-05T16:52:39.584888" }
```

---

## ⚙️ Miljövariabler

| Variabel                  | Exempelvärde                            | Beskrivning                                  |
| ------------------------- | --------------------------------------- | -------------------------------------------- |
| `PORT`                    | `4000`                                  | HTTP-port för FastAPI                        |
| `CLIENT_SECRET`           | `dev-secret`                            | Krävs i varje request-header                 |
| `DATABASE_URL`            | `postgresql://sev:sev@db:5432/sev_mock` | Postgres-anslutning                          |
| `STRICT_EMAIL_VALIDATION` | `false`                                 | (valfritt) aktiverar strikt e-postvalidering |

Alla sätts redan i `docker-compose.yml`.

---

## 🔐 Autentisering

Alla endpoints kräver en header:

```
Client-Secret: dev-secret
```

Exempel:

```bash
curl -H "Client-Secret: dev-secret" http://localhost:4000/api/v1/customers
```

---

## 🧠 Databasstruktur

Två huvudtabeller:

```sql
customers (
  id SERIAL PRIMARY KEY,
  external_id TEXT UNIQUE,
  name TEXT,
  email TEXT,
  phone TEXT,
  org_no TEXT,
  created_at timestamptz DEFAULT now()
)

purchase_orders (
  id SERIAL PRIMARY KEY,
  external_id TEXT UNIQUE,
  customer_id INT REFERENCES customers(id),
  status TEXT,
  description TEXT,
  amount NUMERIC(12,2),
  created_at timestamptz DEFAULT now()
)
```

---

## 🧩 API-endpoints

### 🔹 `GET /health`

Snabbkontroll att servern är igång.

### 🔹 `GET /api/v1/customers`

Lista alla kunder.

```bash
curl -H "Client-Secret: dev-secret" http://localhost:4000/api/v1/customers
```

**Svar:**

```json
{
  "results": [
    { "external_id": "C-100", "name": "Maskad Kund AB", ... },
    { "external_id": "C-101", "name": "Fikafabriken", ... }
  ]
}
```

---

### 🔹 `GET /api/v1/customers/{id_or_external}`

Hämta en kund via intern ID eller `external_id`.

```bash
curl -H "Client-Secret: dev-secret" http://localhost:4000/api/v1/customers/C-100
```

---

### 🔹 `POST /api/v1/customers`

Skapa eller uppdatera en kund (upsert).

**Body:**

```json
{
  "external_id": "C-102",
  "name": "Nytt AB",
  "email": "kontakt@nytt.example.test"
}
```

**Svar:**

```json
{
  "id": 3,
  "external_id": "C-102",
  "name": "Nytt AB",
  "email": "kontakt@nytt.example.test",
  "created_at": "2025-10-05T16:56:40.020097+00:00"
}
```

---

### 🔹 `GET /api/v1/purchase-orders`

Lista alla inköpsordrar (Purchase Orders) med kundobjekt inbäddat.

```bash
curl -H "Client-Secret: dev-secret" http://localhost:4000/api/v1/purchase-orders
```

**Svar:**

```json
{
  "results": [
    {
      "external_id": "PO-200",
      "status": "open",
      "amount": 3450.0,
      "customer": { "id": "C-100", "name": "Maskad Kund AB" }
    }
  ]
}
```

---

### 🔹 `GET /api/v1/purchase-orders/{id_or_external}`

Hämta en enskild order.

```bash
curl -H "Client-Secret: dev-secret" http://localhost:4000/api/v1/purchase-orders/PO-200
```

---

### 🔹 `POST /api/v1/purchase-orders`

Skapa eller uppdatera en order (upsert).
Kräver att `customer_external_id` finns i databasen.

**Body:**

```json
{
  "external_id": "PO-300",
  "customer_external_id": "C-102",
  "status": "open",
  "description": "Linjemålning",
  "amount": 1500
}
```

**Svar:**

```json
{
  "id": 3,
  "external_id": "PO-300",
  "customer_id": 3,
  "status": "open",
  "description": "Linjemålning",
  "amount": 1500.0,
  "created_at": "2025-10-05T16:56:51.264699+00:00"
}
```

---

## 🧰 Verktyg & test

Snabba testkommandon:

```bash
# Bygg och starta
docker compose up --build

# Röktester
bash scripts/test.sh

# Stoppa allt
docker compose down
```

---

## 🧾 Seedade testdata

Automatiskt inlästa vid första start:

| Customer   | Beskrivning          |
| ---------- | -------------------- |
| **C-100**  | Maskad Kund AB       |
| **C-101**  | Fikafabriken         |
| **PO-200** | Snöröjning kvarter A |
| **PO-201** | Fasadtvätt del B     |

---

## 🧱 Användningsområden

- **n8n integration:** Simulera Seventime-API utan att röra produktionsdata.
- **Dashboardutveckling:** Låtsas-data för offert, order och kundvy.
- **Rust-gateway:** Nästa steg är att bygga en Rust-tjänst som läser och synkar data härifrån.

---

## 🧩 Nästa steg

1. Lägg till fler endpoints (offert, arbetsorder, faktura).
2. Bygg Rust-gatewayn som ansluter hit.
3. Lägg på audit-logg för alla POST-requests.
4. Koppla in n8n och börja testa faktiska integrationsflöden.

---

**Författare:** Viktor @ Allanit
**Datum:** 2025-10-05
**Version:** 0.1.0 (Mock)
