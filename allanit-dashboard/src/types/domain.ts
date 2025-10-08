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
  status: string;
  description: string | null;
  amount: number;
  created_at: string; // ISO
  updated_at: string; // ISO
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
