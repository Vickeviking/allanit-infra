# Allanit Dashboard - API Reference

**Powered by Viktor Liljenberg**  
**© 2025 Allanit AB**

## API Architecture

The application uses a centralized API client (`src/api/apiClient.ts`) that automatically routes requests based on configuration:

- **Mock Mode** (`config.mockMode = true`): Uses local mock data
- **Production Mode** (`config.mockMode = false`): Calls real backend API

**Base URL**: Configured in `src/config.ts` (default: `http://localhost:8000`)

## Endpoint Summary

All endpoints follow REST conventions and return:
```typescript
{
  data: {
    results: T[]  // Array of resources
  }
}
```

## 🌐 Base URL
```
Development: http://localhost:8000/api
Production: https://api.allanit.com/api
```

---

## 🔐 Authentication

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/auth/login` | User login |
| POST | `/auth/logout` | User logout |
| GET | `/auth/me` | Get current user |

---

## 👥 Customers

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/customers` | List all customers |
| GET | `/customers/{id}` | Get customer by ID |
| POST | `/customers` | Create new customer |
| PUT | `/customers/{id}` | Update customer |
| DELETE | `/customers/{id}` | Delete customer |

**Customer Model:**
```typescript
interface Customer {
  id: number;
  name: string;
  email?: string;
  phone?: string;
  address?: string;
  subsidiary_id?: number;
  created_at: string;
  updated_at: string;
}
```

---

## 👨‍💼 Employees

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/employees` | List all employees |
| GET | `/employees/{id}` | Get employee by ID |
| POST | `/employees` | Create new employee |
| PUT | `/employees/{id}` | Update employee |
| DELETE | `/employees/{id}` | Delete employee |

**Employee Model:**
```typescript
interface Employee {
  id: number;
  name: string;
  email: string;
  phone?: string;
  role: string;
  subsidiary: string;
  status: "active" | "inactive" | "on_leave";
  image?: string;
  hireDate?: string;
  department?: string;
  address?: string;
  created_at: string;
  updated_at: string;
}
```

---

## 📦 Purchase Orders

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/purchase-orders` | List all orders |
| GET | `/purchase-orders/{id}` | Get order by ID |
| POST | `/purchase-orders` | Create new order |
| PUT | `/purchase-orders/{id}` | Update order |
| DELETE | `/purchase-orders/{id}` | Delete order |
| PUT | `/purchase-orders/{id}/assign` | Assign order to employee |
| PUT | `/purchase-orders/{id}/schedule` | Schedule order |
| PUT | `/purchase-orders/{id}/complete` | Mark order as completed |

**PurchaseOrder Model:**
```typescript
interface PurchaseOrder {
  id: number;
  external_id: string;
  customer_id: number | null;
  status: 'not_planned' | 'planned' | 'in_progress' | 'paused' | 
          'waiting_for_materials' | 'blocked' | 'in_review' | 
          'completed' | 'cancelled';
  description: string | null;
  amount: number;
  assigned_employee_id: number | null;
  supervisor_id: number | null;
  scheduled_date: string | null;
  completed_date: string | null;
  notes: string | null;
  company: string;
  priority: 'low' | 'medium' | 'high';
  dueDate?: string;
  created_at: string;
  updated_at: string;
}
```

---

## 💬 Order Comments

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/order-comments` | List all comments |
| GET | `/order-comments/{orderId}` | Get comments for order |
| POST | `/order-comments` | Create new comment |
| PUT | `/order-comments/{id}` | Update comment |
| DELETE | `/order-comments/{id}` | Delete comment |

**OrderComment Model:**
```typescript
interface OrderComment {
  id: string;
  orderId: number;
  employeeId: number;
  comment: string;
  createdAt: string;
  status?: string; // Optional status change with comment
}
```

---

## 🏭 Machines

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/machines` | List all machines |
| GET | `/machines/{id}` | Get machine by ID |
| POST | `/machines` | Create new machine |
| PUT | `/machines/{id}` | Update machine |
| DELETE | `/machines/{id}` | Delete machine |
| PUT | `/machines/{id}/assign` | Assign machine to employee |

**Machine Model:**
```typescript
interface Machine {
  id: number;
  name: string;
  type: string;
  model: string;
  status: "active" | "out_of_service" | "maintenance";
  nextServiceDate: string;
  responsibleEmployeeId: number;
  image: string;
  specifications: string;
  created_at: string;
  updated_at: string;
}
```

---

## 🔧 Machine Fault Reports

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/machine-fault-reports` | List all fault reports |
| GET | `/machine-fault-reports/{id}` | Get fault report by ID |
| POST | `/machine-fault-reports` | Create new fault report |
| PUT | `/machine-fault-reports/{id}` | Update fault report |
| PUT | `/machine-fault-reports/{id}/resolve` | Mark fault as resolved |

**MachineFaultReport Model:**
```typescript
interface MachineFaultReport {
  id: string;
  machineId: number;
  reportedBy: number; // employeeId
  title: string;
  description: string;
  severity: 'low' | 'medium' | 'high' | 'critical';
  status: 'reported' | 'acknowledged' | 'in_repair' | 'resolved';
  reportedAt: string;
  resolvedAt?: string;
  notes?: string;
}
```

---

## 📝 Machine Journals

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/machine-journals` | List all machine journals |
| GET | `/machine-journals/{machineId}` | Get journals for machine |
| POST | `/machine-journals` | Create new journal entry |
| PUT | `/machine-journals/{id}` | Update journal entry |
| DELETE | `/machine-journals/{id}` | Delete journal entry |

**MachineJournal Model:**
```typescript
interface MachineJournal {
  id: string;
  machineId: number;
  employeeId: number;
  title: string;
  body: string;
  category: 'maintenance' | 'repair' | 'inspection' | 'issue' | 'general';
  priority: 'low' | 'medium' | 'high';
  createdAt: string;
  updatedAt?: string;
}
```

---

## 📝 Employee Journals

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/journals` | List all employee journals |
| GET | `/journals/{employeeId}` | Get journals for employee |
| POST | `/journals` | Create new journal entry |
| PUT | `/journals/{id}` | Update journal entry |
| DELETE | `/journals/{id}` | Delete journal entry |

**Journal Model:**
```typescript
interface Journal {
  id: string;
  employeeId: string;
  title: string;
  body: string;
  createdAt: string;
  updatedAt?: string;
  author: string;
  tags: string[];
  category: 'work' | 'training' | 'maintenance' | 'customer' | 
           'safety' | 'equipment' | 'general';
  priority: 'low' | 'medium' | 'high';
  status: 'draft' | 'published' | 'archived';
  attachments?: string[];
}
```

---

## 💰 Invoices

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/invoices` | List all invoices |
| GET | `/invoices/{id}` | Get invoice by ID |
| POST | `/invoices` | Create new invoice |
| PUT | `/invoices/{id}` | Update invoice |
| DELETE | `/invoices/{id}` | Delete invoice |
| POST | `/invoices/{id}/export` | Export to Björn Lunden |

**Invoice Model:**
```typescript
interface Invoice {
  id: number;
  invoice_number: string;
  customer_id: number;
  amount: number;
  status: "draft" | "sent" | "paid" | "overdue";
  due_date: string;
  description?: string;
  created_at: string;
  updated_at: string;
}
```

---

## 🏢 Subsidiaries

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/subsidiaries` | List all subsidiaries |
| GET | `/subsidiaries/{id}` | Get subsidiary by ID |
| POST | `/subsidiaries` | Create new subsidiary |
| PUT | `/subsidiaries/{id}` | Update subsidiary |
| DELETE | `/subsidiaries/{id}` | Delete subsidiary |

**Subsidiary Model:**
```typescript
interface Subsidiary {
  id: number;
  name: string;
  address?: string;
  phone?: string;
  email?: string;
  organization_number?: string;
  created_at: string;
  updated_at: string;
}
```

---

## 📧 Email Templates

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/mail/templates` | List all templates |
| GET | `/mail/templates/{id}` | Get template by ID |
| POST | `/mail/templates` | Create new template |
| PUT | `/mail/templates/{id}` | Update template |
| DELETE | `/mail/templates/{id}` | Delete template |

**MailTemplate Model:**
```typescript
interface MailTemplate {
  id: string;
  name: string;
  subject: string;
  html: string;
  tags: string[];
  created_at: string;
  updated_at: string;
}
```

---

## 📤 Sent Emails

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/sent-emails` | List all sent emails |
| GET | `/sent-emails/{id}` | Get sent email by ID |
| POST | `/sent-emails` | Send new email |

**SentEmail Model:**
```typescript
interface SentEmail {
  id: string;
  campaign_name: string;
  template_name: string;
  to_count: number;
  delivered: number;
  opened: number;
  sent_at: string;
}
```

---

## 🎯 Segments

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/segments` | List all segments |
| GET | `/segments/{id}` | Get segment by ID |
| POST | `/segments` | Create new segment |
| PUT | `/segments/{id}` | Update segment |
| DELETE | `/segments/{id}` | Delete segment |

**Segment Model:**
```typescript
interface Segment {
  id: string;
  name: string;
  description?: string;
  count_hint: number;
  criteria: any; // JSON object with segment criteria
  created_at: string;
  updated_at: string;
}
```

---

## 📊 Reports

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/reports/revenue` | Revenue reports |
| GET | `/reports/orders` | Order completion reports |
| GET | `/reports/employees` | Employee performance reports |
| GET | `/reports/machines` | Machine utilization reports |
| GET | `/reports/export/{type}` | Export report (CSV/PDF) |

---

## 🔒 Authentication & Authorization

### JWT Token Structure
```typescript
interface JWTPayload {
  user_id: number;
  role: "administrator" | "employee";
  subsidiary: string;
  exp: number;
  iat: number;
}
```

### Role-Based Access Control
- **Administrators**: Full access to all endpoints
- **Employees**: Limited access to:
  - `/purchase-orders` (assigned orders only)
  - `/machines` (assigned machines only)
  - `/order-comments` (own comments)
  - `/journals` (own journals)

---

## 📝 Response Format

### Success Response
```typescript
interface ApiResponse<T> {
  data: T;
  message?: string;
  status: "success";
}
```

### Error Response
```typescript
interface ApiError {
  error: string;
  message: string;
  status: "error";
  code?: number;
}
```

### Paginated Response
```typescript
interface PaginatedResponse<T> {
  results: T[];
  total: number;
  page: number;
  per_page: number;
  total_pages: number;
}
```