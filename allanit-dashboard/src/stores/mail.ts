import { defineStore } from "pinia";
import { http } from "@/api/mockClient";

export interface MailTemplate {
  id: string;
  name: string;
  subject: string;
  tags: string[];
  html: string;
}

export const useMail = defineStore("mail", {
  state: () => ({
    templates: [] as MailTemplate[],
    loading: false,
    error: null as string | null,
  }),
  actions: {
    async fetchTemplates() {
      this.loading = true;
      this.error = null;
      try {
        const r = await http.get<{ results: MailTemplate[] }>(
          "/api/mail/templates",
        );
        this.templates = r.data.results ?? [];
      } catch (e: any) {
        this.error = e?.message ?? "Fel";
      } finally {
        this.loading = false;
      }
    },
  },
});
