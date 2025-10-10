# Allanit Dashboard - Migration Guide

## 🚀 Snabbstart

### 1. Aktivera riktig backend
```bash
# I .env filen
VITE_USE_MOCK=false
VITE_API_URL=http://localhost:8000
```

### 2. Starta backend
```bash
cd backend
cargo run
```

### 3. Frontend använder automatiskt riktig API
Frontend växlar automatiskt till riktig backend när `VITE_USE_MOCK=false`.

---

## 🏗️ Arkitektur

### Frontend (Vue 3 + TypeScript)
- **Mock Client**: `/src/api/mockClient.ts` - Returnerar statisk data
- **HTTP Client**: `/src/api/http.ts` - Växlar mellan mock och riktig API
- **API Modules**: `/src/api/*.ts` - Specifika API-anrop per domän

### Backend (Rocket + Rust)
- **REST API**: JSON-baserat API med Rocket
- **Database**: PostgreSQL med Diesel ORM
- **Authentication**: JWT-baserad autentisering
- **CORS**: Konfigurerat för frontend-domän

---

## 🌐 API Endpoints

### Bas-URL
```
http://localhost:8000/api
```

### Autentisering
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/auth/login` | User login |
| POST | `/auth/logout` | User logout |
| GET | `/auth/me` | Get current user |

### Kunder
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/customers` | Lista alla kunder |
| GET | `/customers/{id}` | Hämta specifik kund |
| POST | `/customers` | Skapa ny kund |
| PUT | `/customers/{id}` | Uppdatera kund |
| DELETE | `/customers/{id}` | Ta bort kund |

### Medarbetare
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/employees` | Lista alla medarbetare |
| GET | `/employees/{id}` | Hämta specifik medarbetare |
| POST | `/employees` | Skapa ny medarbetare |
| PUT | `/employees/{id}` | Uppdatera medarbetare |
| DELETE | `/employees/{id}` | Ta bort medarbetare |

### Uppdrag
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/purchase-orders` | Lista alla uppdrag |
| GET | `/purchase-orders/{id}` | Hämta specifikt uppdrag |
| POST | `/purchase-orders` | Skapa nytt uppdrag |
| PUT | `/purchase-orders/{id}` | Uppdatera uppdrag |
| DELETE | `/purchase-orders/{id}` | Ta bort uppdrag |
| PUT | `/purchase-orders/{id}/assign` | Tilldela uppdrag |
| PUT | `/purchase-orders/{id}/schedule` | Schemalägg uppdrag |
| PUT | `/purchase-orders/{id}/complete` | Markera som slutfört |

### Maskiner
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/machines` | Lista alla maskiner |
| GET | `/machines/{id}` | Hämta specifik maskin |
| POST | `/machines` | Skapa ny maskin |
| PUT | `/machines/{id}` | Uppdatera maskin |
| DELETE | `/machines/{id}` | Ta bort maskin |
| PUT | `/machines/{id}/assign` | Tilldela maskin |

### Felanmälningar
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/machine-fault-reports` | Lista alla felanmälningar |
| GET | `/machine-fault-reports/{id}` | Hämta specifik felanmälan |
| POST | `/machine-fault-reports` | Skapa ny felanmälan |
| PUT | `/machine-fault-reports/{id}` | Uppdatera felanmälan |
| PUT | `/machine-fault-reports/{id}/resolve` | Markera som löst |

### Kommentarer
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/order-comments` | Lista alla kommentarer |
| GET | `/order-comments/{orderId}` | Hämta kommentarer för uppdrag |
| POST | `/order-comments` | Skapa ny kommentar |
| PUT | `/order-comments/{id}` | Uppdatera kommentar |
| DELETE | `/order-comments/{id}` | Ta bort kommentar |

### Journaler
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/journals` | Lista alla journaler |
| GET | `/journals/{employeeId}` | Hämta journaler för medarbetare |
| POST | `/journals` | Skapa ny journalpost |
| PUT | `/journals/{id}` | Uppdatera journalpost |
| DELETE | `/journals/{id}` | Ta bort journalpost |

### Fakturor
| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/invoices` | Lista alla fakturor |
| GET | `/invoices/{id}` | Hämta specifik faktura |
| POST | `/invoices` | Skapa ny faktura |
| PUT | `/invoices/{id}` | Uppdatera faktura |
| DELETE | `/invoices/{id}` | Ta bort faktura |
| POST | `/invoices/{id}/export` | Exportera till Björn Lunden |

---

## 🗄️ Database Schema

### Core Tables

**employees**
```sql
CREATE TABLE employees (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    phone VARCHAR(50),
    role VARCHAR(100) NOT NULL,
    subsidiary VARCHAR(100) NOT NULL,
    status VARCHAR(50) DEFAULT 'active',
    image VARCHAR(500),
    hire_date DATE,
    department VARCHAR(100),
    address TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
```

**customers**
```sql
CREATE TABLE customers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255),
    phone VARCHAR(50),
    address TEXT,
    subsidiary_id INTEGER REFERENCES subsidiaries(id),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
```

**purchase_orders**
```sql
CREATE TABLE purchase_orders (
    id SERIAL PRIMARY KEY,
    external_id VARCHAR(100) UNIQUE NOT NULL,
    customer_id INTEGER REFERENCES customers(id),
    status VARCHAR(50) DEFAULT 'not_planned',
    description TEXT,
    amount DECIMAL(10,2) NOT NULL,
    assigned_employee_id INTEGER REFERENCES employees(id),
    supervisor_id INTEGER REFERENCES employees(id),
    scheduled_date TIMESTAMP,
    completed_date TIMESTAMP,
    notes TEXT,
    company VARCHAR(100) NOT NULL,
    priority VARCHAR(20) DEFAULT 'medium',
    due_date DATE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
```

**machines**
```sql
CREATE TABLE machines (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    type VARCHAR(100) NOT NULL,
    model VARCHAR(100),
    status VARCHAR(50) DEFAULT 'active',
    next_service_date DATE,
    responsible_employee_id INTEGER REFERENCES employees(id),
    image VARCHAR(500),
    specifications TEXT,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);
```

**machine_fault_reports**
```sql
CREATE TABLE machine_fault_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    machine_id INTEGER REFERENCES machines(id),
    reported_by INTEGER REFERENCES employees(id),
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    severity VARCHAR(20) NOT NULL,
    status VARCHAR(20) DEFAULT 'reported',
    reported_at TIMESTAMP DEFAULT NOW(),
    resolved_at TIMESTAMP,
    notes TEXT
);
```

**order_comments**
```sql
CREATE TABLE order_comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id INTEGER REFERENCES purchase_orders(id),
    employee_id INTEGER REFERENCES employees(id),
    comment TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    status VARCHAR(50)
);
```

**journals**
```sql
CREATE TABLE journals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id INTEGER REFERENCES employees(id),
    title VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP,
    author VARCHAR(255) NOT NULL,
    tags TEXT[],
    category VARCHAR(50) NOT NULL,
    priority VARCHAR(20) DEFAULT 'medium',
    status VARCHAR(20) DEFAULT 'published',
    attachments TEXT[]
);
```

---

## 🔐 Authentication

### JWT Token Structure
```json
{
  "user_id": 1,
  "role": "administrator",
  "subsidiary": "Allanit Service AB",
  "exp": 1640995200,
  "iat": 1640908800
}
```

### Role-Based Access Control
- **Administrators**: Full access to all endpoints
- **Employees**: Limited access to assigned resources only

---

## 🚀 Deployment

### Development
```bash
# Backend
cd backend
cargo run

# Frontend
npm run dev
```

### Production
```bash
# Build frontend
VITE_USE_MOCK=false npm run build

# Deploy backend
cargo build --release
./target/release/allanit-backend
```

---

## 🔧 Configuration

### Environment Variables

**Frontend (.env)**
```bash
VITE_USE_MOCK=false
VITE_API_URL=https://api.allanit.com
```

**Backend (.env)**
```bash
DATABASE_URL=postgresql://username:password@localhost/allanit_db
JWT_SECRET=your-super-secret-jwt-key-here
ROCKET_PORT=8000
ROCKET_ADDRESS=0.0.0.0
```

---

## 📊 Monitoring

### Health Checks
- **API Health**: `GET /api/health`
- **Database Health**: `GET /api/health/db`
- **System Status**: `GET /api/health/system`

### Logging
- **Application Logs**: Structured JSON logging
- **Error Tracking**: Comprehensive error logging
- **Performance Metrics**: Response time monitoring

---

*Denna guide ger dig allt du behöver för att migrera från mock-data till riktig backend.*