// src/types/seventime.ts
export interface STCustomer {
  external_id: string;
  name: string;
  email?: string | null;
  phone?: string | null;
  org_no?: string | null;
  created_at?: string | null;
}

export interface STPurchaseOrder {
  external_id: string;
  customer_external_id: string;
  status: string;
  description?: string | null;
  amount: number;
  created_at?: string | null;
}
