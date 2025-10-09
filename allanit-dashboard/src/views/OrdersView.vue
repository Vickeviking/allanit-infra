<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-gray-900">Ordrar</h1>
        <p class="text-gray-600">Hantera köpordrar och deras status</p>
      </div>
      <div class="flex items-center space-x-3">
        <button
          @click="exportOrders"
          class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
        >
          Exportera CSV
        </button>

        <button
          @click="exportToBL"
          class="px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700"
        >
          Exportera till BL
        </button>
      </div>
    </div>

    <!-- Filters -->
    <div class="bg-white rounded-lg shadow-sm border p-6">
      <FilterBar
        :show-date-range="true"
        :select-filters="selectFilters"
        @filter-change="handleFilterChange"
      >
        <template #actions>
          <button
            @click="clearAllFilters"
            class="px-3 py-1 text-sm text-gray-600 hover:text-gray-800"
          >
            Rensa alla
          </button>
        </template>
      </FilterBar>
    </div>

    <!-- Table -->
    <DataTable
      title="Ordrar"
      :items="filteredOrders"
      :columns="columns"
      :loading="ordersStore.loading"
      :error="ordersStore.error"
      selectable
      :selected-items="selectedOrders"
      show-filters
      @select-all="handleSelectAll"
      @select-item="handleSelectItem"
      @row-click="openOrderDetail"
      @sort="handleSort"
    >
      <template #filters>
        <div class="flex items-center space-x-4">
          <div class="text-sm text-gray-600">
            {{ filteredOrders.length }} av {{ ordersStore.items.length }} ordrar
          </div>
        </div>
      </template>

      <template #cell-id="{ item }">
        <div class="flex items-center space-x-2">
          <ShoppingBagIcon class="w-4 h-4 text-gray-400" />
          <span class="text-sm font-medium text-gray-900">{{
            item.external_id
          }}</span>
        </div>
      </template>

      <template #cell-customer="{ item }">
        <div>
          <p class="text-sm font-medium text-gray-900">
            {{ getCustomerName(item.customer_id) }}
          </p>
          <p class="text-xs text-gray-500">ID: {{ item.customer_id }}</p>
        </div>
      </template>

      <template #cell-amount="{ item }">
        <span class="text-sm font-medium text-gray-900">{{
          formatCurrency(item.amount)
        }}</span>
      </template>

      <template #cell-status="{ item }">
        <span
          class="inline-flex px-2 py-1 text-xs font-medium rounded-full"
          :class="getStatusColor(item.status)"
        >
          {{ getStatusLabel(item.status) }}
        </span>
      </template>

      <template #header-actions>
        <div
          v-if="selectedOrders.length > 0"
          class="flex items-center space-x-2"
        >
          <span class="text-sm text-gray-600"
            >{{ selectedOrders.length }} valda</span
          >
          <button
            @click="bulkExport"
            class="px-3 py-1 text-sm bg-blue-600 text-white rounded hover:bg-blue-700"
          >
            Exportera valda
          </button>
        </div>
      </template>

      <template #empty-actions>
        <button
          @click="createOrder"
          class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
        >
          Skapa första ordern
        </button>
      </template>
    </DataTable>

    <!-- Order Detail Drawer -->
    <div v-if="selectedOrder" class="fixed inset-0 z-50 overflow-hidden">
      <div
        class="absolute inset-0 bg-gray-500 bg-opacity-75"
        @click="closeOrderDetail"
      ></div>
      <div class="absolute right-0 top-0 h-full w-96 bg-white shadow-xl">
        <DetailDrawer
          :title="`Order ${selectedOrder.external_id}`"
          :tabs="orderTabs"
          :actions="orderActions"
          @close="closeOrderDetail"
          @action="handleOrderAction"
        >
          <template #tab-summary>
            <div class="space-y-4">
              <div>
                <h4 class="text-sm font-medium text-gray-900 mb-2">
                  Orderinformation
                </h4>
                <div class="space-y-2">
                  <div class="flex items-center justify-between">
                    <span class="text-sm text-gray-600">Status:</span>
                    <span
                      class="inline-flex px-2 py-1 text-xs font-medium rounded-full"
                      :class="getStatusColor(selectedOrder.status)"
                    >
                      {{ getStatusLabel(selectedOrder.status) }}
                    </span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-sm text-gray-600">Belopp:</span>
                    <span class="text-sm font-medium">{{
                      formatCurrency(selectedOrder.amount)
                    }}</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-sm text-gray-600">Skapad:</span>
                    <span class="text-sm">{{
                      formatDate(selectedOrder.created_at)
                    }}</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-sm text-gray-600">Uppdaterad:</span>
                    <span class="text-sm">{{
                      formatDate(selectedOrder.updated_at)
                    }}</span>
                  </div>
                </div>
              </div>

              <div>
                <h4 class="text-sm font-medium text-gray-900 mb-2">
                  Beskrivning
                </h4>
                <p class="text-sm text-gray-600">
                  {{ selectedOrder.description || "Ingen beskrivning" }}
                </p>
              </div>
            </div>
          </template>

          <template #tab-timeline>
            <div class="space-y-4">
              <div class="space-y-3">
                <div class="flex items-start space-x-3">
                  <div class="w-2 h-2 bg-blue-500 rounded-full mt-2"></div>
                  <div>
                    <p class="text-sm font-medium text-gray-900">
                      Order skapad
                    </p>
                    <p class="text-xs text-gray-500">
                      {{ formatDate(selectedOrder.created_at) }}
                    </p>
                  </div>
                </div>
                <div class="flex items-start space-x-3">
                  <div class="w-2 h-2 bg-yellow-500 rounded-full mt-2"></div>
                  <div>
                    <p class="text-sm font-medium text-gray-900">
                      Status uppdaterad
                    </p>
                    <p class="text-xs text-gray-500">
                      {{ formatDate(selectedOrder.updated_at) }}
                    </p>
                  </div>
                </div>
              </div>
            </div>
          </template>

          <template #tab-customer>
            <div v-if="orderCustomer" class="space-y-4">
              <div>
                <h4 class="text-sm font-medium text-gray-900 mb-2">
                  Kundinformation
                </h4>
                <div class="space-y-2">
                  <div class="flex items-center space-x-2">
                    <span class="text-sm text-gray-600">Namn:</span>
                    <span class="text-sm">{{ orderCustomer.name }}</span>
                  </div>
                  <div class="flex items-center space-x-2">
                    <span class="text-sm text-gray-600">Email:</span>
                    <span class="text-sm">{{
                      orderCustomer.email || "Ej angiven"
                    }}</span>
                  </div>
                  <div class="flex items-center space-x-2">
                    <span class="text-sm text-gray-600">Telefon:</span>
                    <span class="text-sm">{{
                      orderCustomer.phone || "Ej angiven"
                    }}</span>
                  </div>
                </div>
              </div>
              <button
                @click="viewCustomer"
                class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
              >
                Visa kunddetaljer
              </button>
            </div>
          </template>

          <template #tab-export>
            <div class="space-y-4">
              <div>
                <h4 class="text-sm font-medium text-gray-900 mb-2">
                  Exportera order
                </h4>
                <p class="text-sm text-gray-600 mb-4">
                  Välj format för export av denna order.
                </p>
              </div>
              <div class="space-y-3">
                <button
                  @click="exportOrderCSV"
                  class="w-full px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
                >
                  Exportera som CSV
                </button>
                <button
                  @click="exportOrderBL"
                  class="w-full px-4 py-2 text-sm font-medium text-white bg-green-600 border border-transparent rounded-md hover:bg-green-700"
                >
                  Exportera till BL Administration
                </button>
              </div>
            </div>
          </template>
        </DetailDrawer>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { ShoppingBagIcon } from "@heroicons/vue/24/outline";
import DataTable from "@/components/ui/DataTable.vue";
import FilterBar from "@/components/ui/FilterBar.vue";
import DetailDrawer from "@/components/ui/DetailDrawer.vue";
import { usePurchaseOrders } from "@/stores/purchaseOrders";
import { useCustomers } from "@/stores/customers";
import type { PurchaseOrder } from "@/types/domain";

const ordersStore = usePurchaseOrders();
const customersStore = useCustomers();

const selectedOrder = ref<PurchaseOrder | null>(null);
const selectedOrders = ref<PurchaseOrder[]>([]);
const filters = ref<Record<string, any>>({});

const selectFilters = [
  {
    key: "status",
    label: "Status",
    options: [
      { value: "open", label: "Öppna" },
      { value: "closed", label: "Stängda" },
      { value: "cancelled", label: "Avbrutna" },
    ],
  },
  {
    key: "customer",
    label: "Kund",
    options: customersStore.items.map((c) => ({
      value: c.id.toString(),
      label: c.name,
    })),
  },
];

const columns = [
  { key: "id", label: "ID", sortable: true },
  { key: "customer", label: "Kund", sortable: true },
  { key: "amount", label: "Belopp", sortable: true },
  { key: "status", label: "Status", sortable: true },
  { key: "created_at", label: "Skapad", sortable: true },
  { key: "updated_at", label: "Uppdaterad", sortable: true },
];

const orderTabs = [
  { key: "summary", label: "Sammanfattning", icon: "div" },
  { key: "timeline", label: "Tidslinje", icon: "div" },
  { key: "customer", label: "Relaterad kund", icon: "div" },
  { key: "export", label: "Exportera till BL", icon: "div" },
];

const orderActions = [
  { key: "edit", label: "Redigera", variant: "secondary" as const },
  { key: "export", label: "Exportera", variant: "primary" as const },
];

const filteredOrders = computed(() => {
  let orders = ordersStore.items;

  if (filters.value.search) {
    const search = filters.value.search.toLowerCase();
    orders = orders.filter(
      (o) =>
        o.external_id.toLowerCase().includes(search) ||
        o.description?.toLowerCase().includes(search),
    );
  }

  if (filters.value.status) {
    orders = orders.filter((o) => o.status === filters.value.status);
  }

  if (filters.value.customer) {
    orders = orders.filter(
      (o) => o.customer_id === parseInt(filters.value.customer),
    );
  }

  if (filters.value.dateFrom) {
    orders = orders.filter(
      (o) => new Date(o.created_at) >= new Date(filters.value.dateFrom),
    );
  }

  if (filters.value.dateTo) {
    orders = orders.filter(
      (o) => new Date(o.created_at) <= new Date(filters.value.dateTo),
    );
  }

  return orders;
});

const orderCustomer = computed(() => {
  if (!selectedOrder.value) return null;
  return customersStore.items.find(
    (c) => c.id === selectedOrder.value!.customer_id,
  );
});

function handleFilterChange(newFilters: Record<string, any>) {
  filters.value = newFilters;
}

function clearAllFilters() {
  filters.value = {};
}

function handleSelectAll(checked: boolean) {
  if (checked) {
    selectedOrders.value = [...filteredOrders.value];
  } else {
    selectedOrders.value = [];
  }
}

function handleSelectItem(item: PurchaseOrder, checked: boolean) {
  if (checked) {
    selectedOrders.value.push(item);
  } else {
    const index = selectedOrders.value.findIndex((o) => o.id === item.id);
    if (index > -1) {
      selectedOrders.value.splice(index, 1);
    }
  }
}

function openOrderDetail(order: PurchaseOrder) {
  selectedOrder.value = order;
}

function closeOrderDetail() {
  selectedOrder.value = null;
}

function handleOrderAction(action: string) {
  if (action === "edit") {
    console.log("Edit order");
  } else if (action === "export") {
    console.log("Export order");
  }
}

function getCustomerName(customerId: number | null): string {
  if (!customerId) return "Okänd kund";
  const customer = customersStore.items.find((c) => c.id === customerId);
  return customer?.name || "Okänd kund";
}

function getStatusColor(status: string): string {
  const colors = {
    open: "bg-yellow-100 text-yellow-800",
    closed: "bg-green-100 text-green-800",
    cancelled: "bg-red-100 text-red-800",
  };
  return colors[status as keyof typeof colors] || "bg-gray-100 text-gray-800";
}

function getStatusLabel(status: string): string {
  const labels = {
    open: "Öppen",
    closed: "Stängd",
    cancelled: "Avbruten",
  };
  return labels[status as keyof typeof labels] || status;
}

function formatCurrency(amount: number): string {
  return new Intl.NumberFormat("sv-SE", {
    style: "currency",
    currency: "SEK",
  }).format(amount);
}

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString("sv-SE");
}

function exportOrders() {
  console.log("Exporting all orders");
}

function exportToBL() {
  console.log("Exporting to BL Administration");
}

function bulkExport() {
  console.log("Bulk export:", selectedOrders.value);
}

function createOrder() {
  console.log("Create new order");
}

function viewCustomer() {
  if (orderCustomer.value) {
    // Navigate to customer detail
    console.log("View customer:", orderCustomer.value);
  }
}

function exportOrderCSV() {
  console.log("Export order as CSV:", selectedOrder.value);
}

function exportOrderBL() {
  console.log("Export order to BL:", selectedOrder.value);
}

function handleSort(key: string) {
  console.log("Sort by:", key);
}

// Load data on mount
onMounted(async () => {
  await ordersStore.fetchAll();
  await customersStore.fetchAll();
});
</script>
