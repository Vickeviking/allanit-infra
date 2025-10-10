# Allanit Dashboard - Project Documentation

**Powered by Viktor Liljenberg**  
**© 2025 Allanit AB. All rights reserved.**

## Project Structure

### Root Directory
- `/src` - Application source code
- `/public` - Static assets (images, fonts)
- `/migration` - Migration and API documentation
- `/dist` - Production build output
- `vite.config.ts` - Vite configuration
- `tailwind.config.js` - Tailwind CSS configuration
- `tsconfig.json` - TypeScript configuration

### Source Code Structure (`/src`)

#### API Layer (`/src/api`)
- `apiClient.ts` - **Centralized API client with mock/real mode switching**
- `mockClient.ts` - Mock HTTP client for development
- `http.ts` - HTTP utilities
- `campaigns.ts` - Campaign management API
- `clients.ts` - Client management API
- `contacts.ts` - Contact management API
- `journals.ts` - Journal/log API
- `machines.ts` - Machine management API
- `machineJournals.ts` - Machine journal API
- `notes.ts` - Notes API
- `segments.ts` - Customer segment API

#### Components (`/src/components`)
- `Shell.vue` - **Main layout with responsive navigation**
- `/ui` - Reusable UI components:
  - `DataCard.vue` - Data display card
  - `DataTable.vue` - Table component
  - `DetailDrawer.vue` - Slide-out details panel
  - `FilterBar.vue` - Filter toolbar
  - `JournalCard.vue` - Journal entry card
  - `KpiCard.vue` - KPI metric card
  - `MachineJournalCard.vue` - Machine-specific journal
  - `Modal.vue` - Modal dialog
  - `OrderCard.vue` - Order display card
  - `OrderAssignmentModal.vue` - Order assignment dialog
  - `EmployeeOrdersModal.vue` - Employee order management
  - `TagSelector.vue` - Tag selection component

#### Mock Data (`/src/mock`)
All mock data for development:
- `campaigns.ts`, `campaign_sends.ts` - Marketing campaigns
- `contacts.ts`, `customers.ts` - Customer data
- `employees.ts` - Employee records
- `invoices.ts` - Invoice data
- `journals.ts` - Site and employee journals
- `logs.ts`, `sync_history.ts` - System logs
- `machines.ts`, `machineJournals.ts` - Equipment data
- `machineFaultReports.ts` - Fault reporting
- `mail_templates.ts`, `sent_emails.ts` - Email system
- `notes.ts` - Notes and comments
- `orderComments.ts` - Order comment history
- `purchase_orders.ts` - Order/job data
- `segments.ts` - Customer segments
- `subsidiaries.ts` - Company subsidiaries

#### Views (`/src/views`)
- `HomeView.vue` - **Dashboard (admin/employee specific)**
- `LoginView.vue` - Authentication with quick login
- `OrdersView.vue` - **Admin order management with pagination**
- `MyOrdersView.vue` - **Employee order view with pagination**
- `MaskinparkView.vue` - **Machine management with fault reporting**
- `EmployeesView.vue` - Employee directory
- `CustomersView.vue` - Customer management
- `SubsidiariesView.vue` - Subsidiary management
- `InvoicesView.vue` - Invoice tracking
- `EmailManagementView.vue` - Email campaign manager
- `EmailHistoryView.vue` - Sent email history
- `CampaignsView.vue` - Campaign management
- `DeadLettersView.vue` - Failed email tracking
- `LogsView.vue` - System logs
- `SyncView.vue` - Data synchronization
- `DeveloperView.vue` - Developer tools
- `SettingsView.vue` - Application settings

#### Other Directories
- `/router` - Vue Router configuration with RBAC
- `/stores` - Pinia state management
- `/types` - TypeScript type definitions
- `config.ts` - **Environment configuration (mock mode toggle)**

### Public Assets (`/public`)
- `/medarbetare` - Employee profile images
- `/maskiner` - Machine/equipment images
- `/updrag` - Order type images
- `allanit-logo.png` - Company logo
- `LogoDesign.png` - Login page logo
- `favicon.png` - Browser favicon
- `Generiskbostadsrätt.png` - Default order image

### Configuration Files
- `mockMode: true/false` in `src/config.ts` - Switch between mock and real API

## API Endpoints (Backend Implementation Ready)

See [API.md](./API.md) for complete endpoint documentation.

All API calls route through `src/api/apiClient.ts` which automatically switches between:
- Mock data (when `config.mockMode = true`)
- Real backend API (when `config.mockMode = false`)

**Base URL**: Configured in `src/config.ts` (`apiUrl: 'http://localhost:8000'`)

## Features

See [FUNCTIONALITIES.md](./FUNCTIONALITIES.md) for complete feature list.

## Development Mode

```bash
npm run dev
```

Toggle mock mode in `src/config.ts`:
```typescript
export const config = {
  mockMode: true,  // Set to false to use real backend
  apiUrl: 'http://localhost:8000'
}
```

## Quick Start

1. **Install dependencies**: `npm install`
2. **Start development server**: `npm run dev`
3. **Login as admin**: Use "johan johan" or "tobias tobias"
4. **Login as employee**: Use "alfons alfons", "janus janus", etc.
5. **Switch to real API**: Set `mockMode: false` in `src/config.ts`

## Architecture Overview

### Frontend Stack
- **Vue 3** with Composition API
- **TypeScript** for type safety
- **Tailwind CSS** for styling
- **Vite** for build tooling
- **Vue Router** for navigation
- **Pinia** for state management

### Key Features
- **Role-Based Access Control (RBAC)**
- **Responsive Design** with mobile hamburger menu
- **Pagination** throughout (6 items per page)
- **Real-time Updates** via API integration
- **Mock Data** for development
- **Comprehensive Error Handling**

### API Integration
- **Centralized API Client** with automatic mock/real switching
- **RESTful Endpoints** with consistent response format
- **Type-safe** API calls with TypeScript
- **Error Handling** and loading states

## Documentation Files

- **[API.md](./API.md)** - Complete API endpoint documentation
- **[FUNCTIONALITIES.md](./FUNCTIONALITIES.md)** - Feature overview and user flows
- **[TECHNICAL.md](./TECHNICAL.md)** - Technical implementation details
- **[MARKETING.md](./MARKETING.md)** - Business value and key features
- **[SWITCH.md](./SWITCH.md)** - Mock/real data switching guide

---

**Powered by Viktor Liljenberg**  
**© 2025 Allanit AB. All rights reserved.**