export interface Customer {
  id: number;
  external_id: string;
  name: string;
  email: string | null;
  phone: string | null;
  org_no: string | null;
  created_at: string; // ISO
  updated_at: string; // ISO
}

export interface PurchaseOrder {
  id: number;
  external_id: string;
  customer_id: number | null;
  status: 'not_planned' | 'planned' | 'in_progress' | 'paused' | 'waiting_for_materials' | 'blocked' | 'in_review' | 'completed' | 'cancelled';
  description: string | null;
  amount: number;
  
  // New fields
  assigned_employee_id?: number;  // Assigned medarbetare
  supervisor_id?: number;          // Handledare/ansvarig
  scheduled_date?: string;         // Planerat datum
  completed_date?: string;         // Slutfört datum
  notes?: string;                  // Admin anteckningar
  company: 'allanit' | 'industrimålning';  // Vilket företag
  priority: 'low' | 'medium' | 'high';     // Prioritet
  dueDate?: string;               // Förfallodatum
  
  created_at: string;
  updated_at: string;
}

export interface OrderComment {
  id: string;
  orderId: number;
  employeeId: number;
  comment: string;
  createdAt: string;
  status?: string; // Optional status change with comment
}

export interface MachineFaultReport {
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

export interface EmailInbound {
  id: number;
  message_id: string;
  from_addr: string;
  to_addr: (string | null)[];
  subject: string | null;
  received_at: string;
  raw_source: string | null;
  parsed: unknown | null;
  status: "received" | "parsed" | "failed";
  created_at: string;
}

export interface InvoiceRaw {
  id: number;
  source_system: string;
  external_id: string | null;
  customer_external_id: string | null;
  email_id: number | null;
  payload: unknown;
  file_uri: string | null;
  ocr_text: string | null;
  status: "received" | "validated" | "exported" | "failed";
  received_at: string;
  created_at: string;
}

export interface LogEntry {
  id: number;
  created_at: string; // NaiveDateTime serialiseras som ISO av API:et
  expires_at: string;
  level: "debug" | "info" | "warn" | "error";
  module: string;
  action: string;
  custom_msg: string | null;
}

export interface User {
  id: number;
  username: string;
  email: string;
  created_at: string;
}

export type UserRole = 'administrator' | 'employee'

export interface AuthenticatedUser {
  id: number
  username: string
  name: string
  role: UserRole
  employeeId?: number
}

export interface Machine {
  id: number
  name: string
  model: string
  category: 'transport' | 'cleaning' | 'winter'
  image: string
  nextServiceDate: string
  responsibleEmployeeId: number
  status: 'operational' | 'maintenance' | 'out_of_service'
  purchaseDate: string
  hours?: number
  kilometers?: number
  location: string
  description?: string
  created_at: string
  updated_at: string
}

export type MachineJournal = {
  id: string
  machineId: string
  title: string
  body: string
  createdAt: string
  updatedAt?: string
  author: string
  tags: string[]
  category: 'maintenance' | 'repair' | 'inspection' | 'cleaning' | 'upgrade' | 'general'
  priority: 'low' | 'medium' | 'high'
  status: 'draft' | 'published' | 'archived'
  attachments?: string[]
}
