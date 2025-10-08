import { defineStore } from "pinia";
import { http } from "@/api/mockClient";
import type { PurchaseOrder } from "@/types/domain";

export const usePurchaseOrders = defineStore("purchaseOrders", {
  state: () => ({
    items: [] as PurchaseOrder[],
    loading: false,
    error: "" as string | null,
  }),
  actions: {
    async fetchAll() {
      this.loading = true;
      this.error = null;
      try {
        const r = await http.get<{
          results?: PurchaseOrder[];
          data?: PurchaseOrder[];
        }>("/api/purchase-orders");
        this.items = r.data.results ?? r.data.data ?? [];
      } catch (e: any) {
        this.error = e?.message ?? "Okänt fel";
      } finally {
        this.loading = false;
      }
    },
  },
});
