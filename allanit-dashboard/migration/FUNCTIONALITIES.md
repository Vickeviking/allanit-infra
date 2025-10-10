# Allanit Dashboard - Functionalities

**Powered by Viktor Liljenberg**  
**© 2025 Allanit AB. All rights reserved.**

## Overview

A comprehensive business management dashboard for Allanit AB, managing multiple subsidiaries with role-based access control.

## User Roles & Permissions

### Administrator
Full system access including:
- Dashboard with KPIs and analytics
- Order management and assignment
- Employee management
- Customer and invoice management
- Machine/equipment management
- Email campaigns and marketing
- System logs and sync management
- Subsidiary oversight

### Employee (Medarbetare)
Limited access including:
- Personal dashboard with assigned tasks
- View and update assigned orders
- Machine responsibility management
- Submit fault reports
- Personal journal entries
- View company information

## Core Features

### 1. Order Management (Uppdrag)

**Admin Features**:
- View all orders across subsidiaries
- Filter by status, employee, company, priority, due date
- **Pagination**: 6 orders per page
- Assign orders to employees
- Set priority levels (low, medium, high)
- Track due dates and deadlines
- "Recent Events": Latest comments and status changes
- "Needs Attention": Auto-highlighted urgent orders
  - Overdue orders
  - Blocked orders
  - Unassigned orders
  - High priority near deadline

**Employee Features**:
- View personal assigned orders
- **Pagination**: 6 orders per page
- Update order status: planned, in_progress, paused, waiting_for_materials, blocked, in_review, completed, cancelled
- Add comments to orders
- View order history and comment thread
- Status summary dashboard

**Order Statuses**:
- `not_planned` - Not yet scheduled
- `planned` - Scheduled
- `in_progress` - Active work
- `paused` - Temporarily stopped
- `waiting_for_materials` - Blocked by materials
- `blocked` - Other blockers
- `in_review` - Under review
- `completed` - Finished
- `cancelled` - Cancelled

### 2. Machine Management (Maskinpark)

**All Users**:
- View all company machines
- Machine details: model, category, hours, location
- Service schedule tracking
- Responsible employee assignment

**Employee Features**:
- "Mina Maskiner": Machines under responsibility
- Visual distinction for owned machines
- **Fault Reporting** (Felanmälan):
  - Submit fault reports
  - Set severity level
  - Auto-update machine status to "out_of_service"
  - Track fault history
- Add machine journal entries

**Machine Data**:
- Purchase date and history
- Operating hours
- Next service date
- Current status (operational, maintenance, out_of_service)
- Responsible employee
- Location tracking

### 3. Employee Management (Medarbetare)

**Admin Only**:
- Employee directory
- View employee details:
  - Contact information
  - Hire date
  - Department
  - Address
  - Subsidiary assignment
- Employee journals
- Track employment status

### 4. Customer Management (Kunder)

**Admin Only**:
- Customer database
- Contact information
- Order history per customer
- Customer notes
- Segmentation

### 5. Dashboard

**Admin Dashboard**:
- Total customers count
- Active orders count
- Total revenue tracking
- Employee count
- Recent orders list
- Quick actions grid
- Activity feed

**Employee Dashboard**:
- My assigned orders count
- Completed tasks this week
- My machines count
- Recent assigned orders
- Machines under responsibility
- Recent activity feed
- Quick action shortcuts

### 6. Email & Marketing

**Admin Only**:
- Email campaign management
- Template library
- Customer segmentation
- Email history tracking
- Dead letter queue
- Campaign analytics

### 7. System Features

**Navigation**:
- Responsive design with mobile hamburger menu
- Sidebar auto-collapses on mobile
- Grouped navigation sections (Overview, Communication, Administration, System)
- Role-based menu filtering

**Authentication**:
- Quick login for development
- Role-based access control (RBAC)
- Route guards
- User session management

**UI/UX**:
- Tailwind CSS design system
- Heroicons icon library
- Responsive grid layouts
- Mobile-first design
- Collapsible sections
- Pagination throughout
- Status badges and indicators
- Modal dialogs and drawers

### 8. Subsidiary Management

**Admin Only**:
- Manage multiple subsidiaries:
  - Allanit Service AB (VD: Johan Liljenberg)
  - Industrimålning Stockholm AB (Staff: Knut, Niklas, Zamdall, Åke)
- Employee assignment per subsidiary
- Company-specific filtering

### 9. Invoices & Finance

**Admin Only**:
- Invoice tracking
- Financial overview
- Customer invoicing
- Payment status

### 10. Logs & Sync

**Admin Only**:
- System activity logs
- Data synchronization tracking
- Sync history
- Error monitoring

## Technical Features

### API Architecture
- Centralized API client with mock/production mode toggle
- Automatic routing based on `config.mockMode`
- All endpoints documented and ready for backend
- Consistent response format

### Data Management
- Mock data for development
- Type-safe TypeScript interfaces
- Pinia state management
- Vue Router with guards

### Responsive Design
- Mobile hamburger menu
- Tablet/desktop layouts
- Touch-friendly interfaces
- Accessible navigation

### Code Quality
- Modular component architecture
- Reusable UI components
- Type safety throughout
- Clean separation of concerns

## Development Notes

**Mock Mode Configuration**:
```typescript
// src/config.ts
export const config = {
  mockMode: true,  // Toggle for mock/real API
  apiUrl: 'http://localhost:8000'
}
```

**Quick Login Users** (Development):
- Johan Liljenberg (Administrator) - "johan johan"
- Tobias Högberg (Administrator) - "tobias tobias"
- Alfons (Employee) - "alfons alfons"
- Janus (Employee) - "janus janus"
- Knut Rogerson (Employee) - "knut knut"
- Niklas Danielsson (Employee) - "niklas niklas"
- Zamdall Gröndal (Employee) - "zamdall zamdall"
- Åke Jäger (Employee) - "ake ake"

## User Flows

### Admin Workflow
1. Login → Admin Dashboard
2. View "Needs Attention" orders
3. Assign orders to employees
4. Monitor progress via "Recent Events"
5. Manage employees and customers
6. Review system logs and sync status

### Employee Workflow
1. Login → Employee Dashboard
2. View assigned orders
3. Update order status and add comments
4. Check "Mina Maskiner" for machine responsibilities
5. Submit fault reports if needed
6. Add journal entries for completed work

### Machine Management Flow
1. View all machines in Maskinpark
2. Check service schedules
3. Report faults with severity levels
4. Track machine history and journals
5. Update machine status

## Data Models

### Core Entities
- **Users**: Employees with roles and permissions
- **Orders**: Jobs with status, priority, and assignments
- **Machines**: Equipment with service schedules and responsibility
- **Customers**: Client information and order history
- **Comments**: Order-specific communication
- **Fault Reports**: Machine issue tracking
- **Journals**: Activity logs and notes

### Key Relationships
- Users → Orders (assignment)
- Users → Machines (responsibility)
- Orders → Comments (communication)
- Machines → Fault Reports (issues)
- Orders → Customers (client relationship)

---

**Powered by Viktor Liljenberg**  
**© 2025 Allanit AB. All rights reserved.**