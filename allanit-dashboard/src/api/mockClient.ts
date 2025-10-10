import customers from "@/mock/customers.ts";
import orders from "@/mock/purchase_orders.ts";
import journals from "@/mock/journals.ts";
import employees from "@/mock/employees.ts";
import subsidiaries from "@/mock/subsidiaries.ts";
import sentEmails from "@/mock/sent_emails.ts";
import templates from "@/mock/mail_templates.ts";
import segments from "@/mock/segments.ts";
import contacts from "@/mock/contacts.ts";
import notes from "@/mock/notes.ts";
import campaigns from "@/mock/campaigns.ts";
import sends from "@/mock/campaign_sends.ts";
import logs from "@/mock/logs.ts";
import syncHistory from "@/mock/sync_history.ts";
import invoices from "@/mock/invoices.ts";
import machines from "@/mock/machines.ts";
import machineJournals from "@/mock/machineJournals.ts";
import orderComments from "@/mock/orderComments.ts";
import machineFaultReports from "@/mock/machineFaultReports.ts";

type Resp<T> = { data: T };

export const http = {
  async get<T = any>(url: string): Promise<Resp<T>> {
    if (url.startsWith("/api/customers")) {
      return { data: { results: customers } as T };
    }
    if (url.startsWith("/api/purchase-orders")) {
      return { data: { results: orders } as T };
    }
    if (url.startsWith("/api/mail/templates")) {
      return { data: { results: templates } as T };
    }
    if (url.startsWith("/api/segments")) {
      return { data: { results: segments } as T };
    }
    if (url.startsWith("/api/contacts")) {
      return { data: { results: contacts } as T };
    }
    if (url.startsWith("/api/journals/employee")) {
      return { data: { results: journals.employee } as T };
    }
    if (url.startsWith("/api/journals/site")) {
      return { data: { results: journals.site } as T };
    }
    if (url.startsWith("/api/notes")) {
      return { data: { results: notes } as T };
    }
    if (url.startsWith("/api/campaigns/sends")) {
      return { data: { results: sends } as T };
    }
    if (url.startsWith("/api/campaigns")) {
      return { data: { results: campaigns } as T };
    }
    if (url.startsWith("/api/logs")) {
      return { data: { results: logs } as T };
    }
    if (url.startsWith("/api/sync/history")) {
      return { data: { results: syncHistory } as T };
    }
    if (url.startsWith("/api/employees")) {
      return { data: { results: employees } as T };
    }
    if (url.startsWith("/api/invoices")) {
      return { data: { results: invoices } as T };
    }
    if (url.startsWith("/api/sent-emails")) {
      return { data: { results: sentEmails } as T };
    }
    if (url.startsWith("/api/subsidiaries")) {
      return { data: { results: subsidiaries } as T };
    }
    if (url.startsWith("/api/machines")) {
      return { data: { results: machines } as T };
    }
        if (url.startsWith("/api/journals/machine")) {
          return { data: { results: machineJournals.machine } as T };
        }
        if (url.startsWith("/api/order-comments")) {
          return { data: { results: orderComments } as T };
        }
        if (url.startsWith("/api/machine-fault-reports")) {
          return { data: { results: machineFaultReports } as T };
        }
    if (url.startsWith("/api/mail/templates")) {
      return { data: { results: templates } as T };
    }
    if (url.startsWith("/api/segments")) {
      return { data: { results: segments } as T };
    }
    if (url.startsWith("/api/contacts")) {
      return { data: { results: contacts } as T };
    }
    if (url.startsWith("/api/notes")) {
      return { data: { results: notes } as T };
    }
    if (url.startsWith("/api/campaigns/sends")) {
      return { data: { results: sends } as T };
    }
    if (url.startsWith("/api/campaigns")) {
      return { data: { results: campaigns } as T };
    }
    if (url.startsWith("/api/logs")) {
      return { data: { results: logs } as T };
    }
    if (url.startsWith("/api/sync/history")) {
      return { data: { results: syncHistory } as T };
    }
    if (url.startsWith("/api/invoices")) {
      return { data: { results: invoices } as T };
    }
    return { data: { results: [] } as T };
  },

  async post<T = any>(url: string, body?: any): Promise<Resp<T>> {
    // "Skapa kund", "Skicka kampanj", "Lägg journal" kan appendas till in-memory/localStorage
    // och returneras som skapad resurs i mockläge
    // # finns ej än i backend bara
    return { data: { ok: true, body } as T };
  },

  async patch<T = any>(_url: string, _body?: any): Promise<Resp<T>> {
    return { data: { ok: true } as T };
  },

  async delete<T = any>(_url: string): Promise<Resp<T>> {
    return { data: { ok: true } as T };
  },
};
