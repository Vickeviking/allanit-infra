# Allanit Dashboard - Teknisk Arkitektur

## 🏗️ Systemarkitektur

### Frontend (Vue 3 + TypeScript)
```
src/
├── api/                    # API abstraction layer
│   ├── http.ts            # HTTP client med mock/real switch
│   ├── mockClient.ts      # Mock data implementation
│   ├── customers.ts       # Customer API calls
│   ├── employees.ts       # Employee API calls
│   ├── machines.ts        # Machine API calls
│   ├── purchaseOrders.ts  # Order API calls
│   └── ...
├── components/            # Reusable UI components
│   ├── ui/               # Base UI components
│   ├── Shell.vue         # Main layout component
│   ├── OrderCard.vue     # Order display component
│   └── ...
├── views/                # Page components
│   ├── HomeView.vue      # Dashboard overview
│   ├── EmployeesView.vue # Employee management
│   ├── OrdersView.vue    # Order management (admin)
│   ├── MyOrdersView.vue  # Employee orders
│   ├── MaskinparkView.vue # Machine management
│   └── ...
├── stores/               # Pinia state management
├── types/                # TypeScript type definitions
│   └── domain.ts         # Core domain models
├── router/               # Vue Router configuration
└── mock/                 # Mock data files
```

### Backend (Rocket + Rust)
```
src/
├── main.rs               # Application entry point
├── routes/               # API route handlers
│   ├── auth.rs          # Authentication routes
│   ├── customers.rs      # Customer routes
│   ├── employees.rs      # Employee routes
│   ├── machines.rs       # Machine routes
│   ├── orders.rs         # Order routes
│   └── ...
├── models/               # Database models
│   ├── customer.rs       # Customer model
│   ├── employee.rs       # Employee model
│   ├── machine.rs        # Machine model
│   └── ...
├── schema/               # Database schema
├── middleware/           # Custom middleware
│   ├── auth.rs          # Authentication middleware
│   └── cors.rs          # CORS configuration
└── config/               # Configuration
    ├── database.rs       # Database configuration
    └── settings.rs       # Application settings
```

---

## 🔧 Mock till Real Data Migration

### Environment Configuration

**Frontend (.env)**
```bash
# Mock mode (development)
VITE_USE_MOCK=true

# Real API mode (production)
VITE_USE_MOCK=false
VITE_API_URL=http://localhost:8000
```

**Backend (.env)**
```bash
DATABASE_URL=postgresql://username:password@localhost/allanit_db
JWT_SECRET=your-super-secret-jwt-key-here
ROCKET_PORT=8000
ROCKET_ADDRESS=0.0.0.0
```

### HTTP Client Architecture

The frontend uses a unified HTTP client that automatically switches between mock and real data based on environment configuration. This allows for:

- **Zero-code migration**: Same frontend code works with both mock and real data
- **Development flexibility**: Test with mock data during development
- **Gradual backend implementation**: Implement endpoints one by one
- **Easy testing**: Switch between modes without code changes

---

## 🗄️ Database Schema

### Core Tables

**employees**
- id, name, email, phone, role, subsidiary, status, image, hire_date, department, address
- Indexes on subsidiary, status, role

**customers**
- id, name, email, phone, address, subsidiary_id
- Foreign key to subsidiaries

**purchase_orders**
- id, external_id, customer_id, status, description, amount, assigned_employee_id, supervisor_id, scheduled_date, completed_date, notes, company, priority, due_date
- Indexes on status, assigned_employee_id, company, priority

**machines**
- id, name, type, model, status, next_service_date, responsible_employee_id, image, specifications
- Foreign key to employees

**machine_fault_reports**
- id, machine_id, reported_by, title, description, severity, status, reported_at, resolved_at, notes
- Foreign key to machines and employees

**order_comments**
- id, order_id, employee_id, comment, created_at, status
- Foreign key to purchase_orders and employees

**journals**
- id, employee_id, title, body, created_at, updated_at, author, tags, category, priority, status, attachments
- Foreign key to employees

**invoices**
- id, invoice_number, customer_id, amount, status, due_date, description
- Foreign key to customers

**subsidiaries**
- id, name, address, phone, email, organization_number

---

## 🔐 Authentication & Authorization

### JWT Token Structure
- **user_id**: Employee ID
- **role**: "administrator" | "employee"
- **subsidiary**: Company affiliation
- **exp**: Expiration timestamp
- **iat**: Issued at timestamp

### Role-Based Access Control (RBAC)

**Administrators** have full access to:
- All employee management functions
- All order management and assignment
- All machine management
- All customer management
- All invoice management
- System administration

**Employees** have limited access to:
- Own assigned orders only
- Own assigned machines only
- Own journal entries
- Own order comments
- Broadcast messages

### Route Protection
- Frontend route guards check user role and permissions
- Backend middleware validates JWT tokens and role permissions
- API endpoints filter data based on user role

---

## 🎨 Frontend Architecture

### Component Structure
- **Views**: Top-level page components
- **Components**: Reusable UI components
- **Stores**: Pinia state management for global state
- **Types**: TypeScript interfaces for type safety

### State Management
- **User Store**: Current user information and authentication state
- **Navigation Store**: Sidebar state and active routes
- **Modal Store**: Global modal state management

### Routing
- **Route Guards**: Authentication and role-based access control
- **Meta Fields**: Role requirements for each route
- **Dynamic Routes**: Parameterized routes for detail views

---

## 🚀 Backend Architecture

### Rocket Framework
- **Async/Await**: Modern Rust async programming
- **JSON Serialization**: Serde for request/response handling
- **Database Integration**: Diesel ORM for PostgreSQL
- **Middleware**: Custom authentication and CORS middleware

### API Design
- **RESTful Endpoints**: Standard HTTP methods and status codes
- **Consistent Response Format**: Unified API response structure
- **Error Handling**: Comprehensive error responses
- **Pagination**: Standard pagination for list endpoints

### Database Integration
- **Connection Pooling**: Efficient database connection management
- **Transactions**: ACID compliance for data integrity
- **Migrations**: Version-controlled database schema changes
- **Indexes**: Optimized query performance

---

## 📊 Performance Considerations

### Frontend Optimization
- **Code Splitting**: Lazy loading of route components
- **Tree Shaking**: Eliminate unused code
- **Asset Optimization**: Compressed images and fonts
- **Caching**: Browser caching for static assets

### Backend Optimization
- **Database Indexing**: Optimized queries with proper indexes
- **Connection Pooling**: Efficient database connections
- **Caching**: Redis for frequently accessed data
- **Compression**: Gzip compression for API responses

### API Optimization
- **Pagination**: Limit data transfer with paginated responses
- **Filtering**: Server-side filtering to reduce data transfer
- **Compression**: Compressed responses for large datasets
- **Caching Headers**: Appropriate cache headers for static data

---

## 🔒 Security Implementation

### Authentication Security
- **JWT Tokens**: Secure token-based authentication
- **Token Expiration**: Short-lived tokens with refresh mechanism
- **Password Hashing**: bcrypt for secure password storage
- **Rate Limiting**: Prevent brute force attacks

### API Security
- **CORS Configuration**: Proper cross-origin resource sharing
- **Input Validation**: Server-side validation of all inputs
- **SQL Injection Prevention**: Parameterized queries
- **XSS Protection**: Input sanitization and output encoding

### Data Security
- **Encryption**: Sensitive data encryption at rest
- **Access Logging**: Comprehensive audit trails
- **Backup Security**: Encrypted database backups
- **Environment Variables**: Secure configuration management

---

## 🧪 Testing Strategy

### Frontend Testing
- **Unit Tests**: Component and utility function testing
- **Integration Tests**: API integration testing
- **E2E Tests**: Full user workflow testing
- **Mock Data Testing**: Comprehensive mock data coverage

### Backend Testing
- **Unit Tests**: Individual function and method testing
- **Integration Tests**: Database and API endpoint testing
- **Load Testing**: Performance under high load
- **Security Testing**: Vulnerability and penetration testing

---

## 📈 Monitoring & Logging

### Application Monitoring
- **Health Checks**: API endpoint health monitoring
- **Performance Metrics**: Response time and throughput monitoring
- **Error Tracking**: Comprehensive error logging and alerting
- **User Analytics**: Usage patterns and performance metrics

### Database Monitoring
- **Query Performance**: Slow query identification and optimization
- **Connection Monitoring**: Database connection pool monitoring
- **Storage Monitoring**: Database size and growth monitoring
- **Backup Monitoring**: Automated backup verification

---

## 🚀 Deployment Architecture

### Development Environment
- **Local Development**: Docker containers for consistent environment
- **Hot Reloading**: Fast development iteration
- **Mock Data**: Full mock data for frontend development
- **Debug Tools**: Comprehensive debugging and logging

### Production Environment
- **Containerized Deployment**: Docker containers for scalability
- **Load Balancing**: Multiple backend instances
- **Database Clustering**: High availability database setup
- **CDN Integration**: Global content delivery network

### CI/CD Pipeline
- **Automated Testing**: Comprehensive test suite execution
- **Code Quality**: Linting and code quality checks
- **Security Scanning**: Automated security vulnerability scanning
- **Automated Deployment**: Zero-downtime deployment process

---

*Denna arkitektur ger Allanit Dashboard en robust, skalbar och säker plattform för att hantera både service- och industrimålningsverksamheten.*