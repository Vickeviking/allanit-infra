import { defineStore } from "pinia";
import { http } from "@/api/mockClient";
import type { Customer } from "@/types/domain";

export const useCustomers = defineStore("customers", {
  state: () => ({
    items: [] as Customer[],
    loading: false,
    error: "" as string | null,
  }),
  actions: {
    async fetchAll() {
      this.loading = true;
      this.error = null;
      try {
        const r = await http.get<{ results?: Customer[]; data?: Customer[] }>(
          "/api/customers",
        );
        this.items = r.data.results ?? r.data.data ?? [];
      } catch (e: any) {
        this.error = e?.message ?? "Okänt fel";
      } finally {
        this.loading = false;
      }
    },
  },
});
