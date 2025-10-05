import os
from typing import Optional, List, Dict, Any
from datetime import datetime
import psycopg
from fastapi import FastAPI, Header, HTTPException
from pydantic import BaseModel, Field, constr

PORT = int(os.getenv("PORT", "4000"))
CLIENT_SECRET = os.getenv("CLIENT_SECRET", "")
DATABASE_URL = os.getenv("DATABASE_URL", "")

if not CLIENT_SECRET:
    raise RuntimeError("CLIENT_SECRET is required")
if not DATABASE_URL:
    raise RuntimeError("DATABASE_URL is required")

app = FastAPI(title="seventimes-mock", version="0.1.0")

def check_auth(secret: Optional[str]):
    if secret != CLIENT_SECRET:
        raise HTTPException(status_code=401, detail="unauthorized")

def now_iso() -> str:
    return datetime.utcnow().isoformat()

def db_conn():
    # Nya psycopg ansluter per anrop. Enkelt och stabilt för mock.
    return psycopg.connect(DATABASE_URL, autocommit=True)

# Pydantic-modeller
class CustomerIn(BaseModel):
    external_id: str = Field(min_length=1)
    name: str = Field(min_length=1)
    email: Optional[constr(pattern=r"^[^@\s]+@[^@\s]+\.[^@\s]+$")] = None
    phone: Optional[str] = None
    org_no: Optional[str] = None

class PurchaseOrderIn(BaseModel):
    external_id: str = Field(min_length=1)
    customer_external_id: str = Field(min_length=1)
    status: Optional[str] = Field(default="open", pattern="^(open|completed|cancelled)$")
    description: Optional[str] = None
    amount: Optional[float] = Field(default=0.0, ge=0)

@app.get("/health")
def health():
    return {"ok": True, "time": now_iso()}

# Customers
@app.get("/api/v1/customers")
def list_customers(client_secret: Optional[str] = Header(default=None), Client_Secret: Optional[str] = Header(default=None)):
    check_auth(client_secret or Client_Secret)
    with db_conn() as conn, conn.cursor() as cur:
        cur.execute("""SELECT id, external_id, name, email, phone, org_no, created_at
                       FROM customers ORDER BY id""")
        rows = [dict(zip([c.name for c in cur.description], r)) for r in cur.fetchall()]
    return {"results": rows}

@app.get("/api/v1/customers/{id_or_external}")
def get_customer(id_or_external: str, client_secret: Optional[str] = Header(default=None), Client_Secret: Optional[str] = Header(default=None)):
    check_auth(client_secret or Client_Secret)
    with db_conn() as conn, conn.cursor() as cur:
        cur.execute("""SELECT id, external_id, name, email, phone, org_no, created_at
                       FROM customers
                       WHERE external_id = %s OR id::text = %s""", (id_or_external, id_or_external))
        row = cur.fetchone()
        if not row:
            raise HTTPException(status_code=404, detail="not_found")
        return dict(zip([c.name for c in cur.description], row))

@app.post("/api/v1/customers", status_code=201)
def upsert_customer(payload: CustomerIn, client_secret: Optional[str] = Header(default=None), Client_Secret: Optional[str] = Header(default=None)):
    check_auth(client_secret or Client_Secret)
    with db_conn() as conn, conn.cursor() as cur:
        cur.execute(
            """
            INSERT INTO customers (external_id, name, email, phone, org_no)
            VALUES (%s, %s, %s, %s, %s)
            ON CONFLICT (external_id)
            DO UPDATE SET name=EXCLUDED.name, email=EXCLUDED.email, phone=EXCLUDED.phone, org_no=EXCLUDED.org_no
            RETURNING id, external_id, name, email, phone, org_no, created_at
            """,
            (payload.external_id, payload.name, payload.email, payload.phone, payload.org_no)
        )
        row = cur.fetchone()
        return dict(zip([c.name for c in cur.description], row))

# Purchase Orders
@app.get("/api/v1/purchase-orders")
def list_pos(client_secret: Optional[str] = Header(default=None), Client_Secret: Optional[str] = Header(default=None)):
    check_auth(client_secret or Client_Secret)
    with db_conn() as conn, conn.cursor() as cur:
        cur.execute(
            """
            SELECT po.id, po.external_id, po.status, po.description, po.amount, po.created_at,
                   json_build_object('id', c.external_id, 'name', c.name) AS customer
            FROM purchase_orders po
            LEFT JOIN customers c ON c.id = po.customer_id
            ORDER BY po.id
            """
        )
        rows = [dict(zip([c.name for c in cur.description], r)) for r in cur.fetchall()]
    return {"results": rows}

@app.get("/api/v1/purchase-orders/{id_or_external}")
def get_po(id_or_external: str, client_secret: Optional[str] = Header(default=None), Client_Secret: Optional[str] = Header(default=None)):
    check_auth(client_secret or Client_Secret)
    with db_conn() as conn, conn.cursor() as cur:
        cur.execute("""SELECT id, external_id, customer_id, status, description, amount, created_at
                       FROM purchase_orders
                       WHERE external_id = %s OR id::text = %s""", (id_or_external, id_or_external))
        row = cur.fetchone()
        if not row:
            raise HTTPException(status_code=404, detail="not_found")
        return dict(zip([c.name for c in cur.description], row))

@app.post("/api/v1/purchase-orders", status_code=201)
def upsert_po(payload: PurchaseOrderIn, client_secret: Optional[str] = Header(default=None), Client_Secret: Optional[str] = Header(default=None)):
    check_auth(client_secret or Client_Secret)
    with db_conn() as conn, conn.cursor() as cur:
        # Slå upp kunden
        cur.execute("SELECT id FROM customers WHERE external_id = %s", (payload.customer_external_id,))
        c = cur.fetchone()
        if not c:
            raise HTTPException(status_code=400, detail="unknown_customer")
        customer_id = c[0]
        cur.execute(
            """
            INSERT INTO purchase_orders (external_id, customer_id, status, description, amount)
            VALUES (%s, %s, %s, %s, %s)
            ON CONFLICT (external_id)
            DO UPDATE SET customer_id=EXCLUDED.customer_id, status=EXCLUDED.status,
                          description=EXCLUDED.description, amount=EXCLUDED.amount
            RETURNING id, external_id, customer_id, status, description, amount, created_at
            """,
            (payload.external_id, customer_id, payload.status or "open", payload.description, payload.amount or 0.0)
        )
        row = cur.fetchone()
        return dict(zip([c.name for c in cur.description], row))
