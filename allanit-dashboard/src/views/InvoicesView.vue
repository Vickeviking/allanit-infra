<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-gray-900">Fakturor</h1>
        <p class="text-gray-600">
          Hantera fakturor och export till Björn Lunden
        </p>
      </div>
      <div class="flex items-center space-x-3">
        <button
          @click="exportToBjornLunden"
          class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
        >
          Exportera till Björn Lunden
        </button>

        <button
          @click="showCreateInvoiceModal = true"
          class="px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700"
        >
          Skapa faktura
        </button>
      </div>
    </div>

    <!-- Filters -->
    <div class="bg-white rounded-lg shadow-sm border p-6">
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1"
            >Status</label
          >
          <select
            v-model="filters.status"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
          >
            <option value="">Alla status</option>
            <option value="draft">Utkast</option>
            <option value="sent">Skickad</option>
            <option value="paid">Betalad</option>
            <option value="overdue">Förfallen</option>
          </select>
        </div>
        <div class="pl-4">
          <label class="block text-sm font-medium text-gray-700 mb-1"
            >Kund</label
          >
          <select
            v-model="filters.customerId"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
          >
            <option value="">Alla kunder</option>
            <option
              v-for="customer in customers"
              :key="customer.id"
              :value="customer.id"
            >
              {{ customer.name }}
            </option>
          </select>
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1"
            >Från datum</label
          >
          <input
            v-model="filters.dateFrom"
            type="date"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
          />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1"
            >Till datum</label
          >
          <input
            v-model="filters.dateTo"
            type="date"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
          />
        </div>
      </div>
    </div>

    <!-- Invoice Table -->
    <div class="bg-white rounded-lg shadow-sm border">
      <div class="px-6 py-4 border-b border-gray-200">
        <div class="flex items-center justify-between">
          <h3 class="text-lg font-medium text-gray-900">Fakturor</h3>
          <div class="flex items-center space-x-2">
            <span class="text-sm text-gray-600"
              >{{ filteredInvoices.length }} fakturor</span
            >
            <span class="text-sm font-medium text-gray-900">
              Totalt: {{ formatCurrency(totalAmount) }}
            </span>
          </div>
        </div>
      </div>

      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Fakturanr
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Kund
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Belopp
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Status
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Förfallodatum
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Skapad
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Åtgärder
              </th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr
              v-for="invoice in filteredInvoices"
              :key="invoice.id"
              class="hover:bg-gray-50"
            >
              <td
                class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900"
              >
                {{ invoice.invoice_number }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                {{ getCustomerName(invoice.customer_id) }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                {{ formatCurrency(invoice.amount) }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <span
                  class="inline-flex px-2 py-1 text-xs font-medium rounded-full"
                  :class="getStatusColor(invoice.status)"
                >
                  {{ getStatusLabel(invoice.status) }}
                </span>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                {{ formatDate(invoice.due_date) }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                {{ formatDate(invoice.created_at) }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                <div class="flex space-x-2">
                  <button
                    @click="viewInvoice(invoice)"
                    class="text-blue-600 hover:text-blue-800"
                  >
                    Visa
                  </button>
                  <button
                    @click="editInvoice(invoice)"
                    class="text-gray-600 hover:text-gray-800"
                  >
                    Redigera
                  </button>
                  <button
                    @click="exportInvoiceToBL(invoice)"
                    class="text-green-600 hover:text-green-800"
                  >
                    Exportera till BL
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Create Invoice Modal -->
    <div
      v-if="showCreateInvoiceModal"
      class="fixed inset-0 z-50 overflow-hidden"
    >
      <div
        class="absolute inset-0 bg-gray-500 bg-opacity-75"
        @click="showCreateInvoiceModal = false"
      ></div>
      <div class="absolute inset-0 flex items-center justify-center p-4">
        <div class="bg-white rounded-lg shadow-xl w-full max-w-2xl">
          <div class="px-6 py-4 border-b border-gray-200">
            <h3 class="text-lg font-medium text-gray-900">Skapa ny faktura</h3>
          </div>

          <div class="px-6 py-4 space-y-4">
            <div class="grid grid-cols-2 gap-4">
              <div class="pl-4">
                <label class="block text-sm font-medium text-gray-700 mb-1"
                  >Kund</label
                >
                <select
                  v-model="invoiceForm.customerId"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
                >
                  <option value="">Välj kund...</option>
                  <option
                    v-for="customer in customers"
                    :key="customer.id"
                    :value="customer.id"
                  >
                    {{ customer.name }}
                  </option>
                </select>
              </div>

              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1"
                  >Belopp</label
                >
                <input
                  v-model="invoiceForm.amount"
                  type="number"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
                />
              </div>
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1"
                >Beskrivning</label
              >
              <input
                v-model="invoiceForm.description"
                type="text"
                class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
              />
            </div>

            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1"
                >Förfallodatum</label
              >
              <input
                v-model="invoiceForm.dueDate"
                type="date"
                class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
              />
            </div>
          </div>

          <div class="px-6 py-4 border-t border-gray-200 flex space-x-3">
            <button
              @click="showCreateInvoiceModal = false"
              class="flex-1 px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
            >
              Avbryt
            </button>
            <button
              @click="createInvoice"
              class="flex-1 px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700"
            >
              Skapa faktura
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Export Status Modal -->
    <div v-if="showExportStatus" class="fixed inset-0 z-50 overflow-hidden">
      <div
        class="absolute inset-0 bg-gray-500 bg-opacity-75"
        @click="showExportStatus = false"
      ></div>
      <div class="absolute inset-0 flex items-center justify-center p-4">
        <div class="bg-white rounded-lg shadow-xl w-full max-w-md">
          <div class="px-6 py-4 border-b border-gray-200">
            <h3 class="text-lg font-medium text-gray-900">
              Export till Björn Lunden
            </h3>
          </div>

          <div class="px-6 py-4">
            <div class="text-center">
              <div
                class="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-4 p-2"
              >
                <CheckIcon
                  class="w-8 h-8 text-green-600"
                  style="margin-top: 4px"
                />
              </div>
              <h4 class="text-lg font-medium text-gray-900 mb-2">
                Export slutförd!
              </h4>
              <p class="text-sm text-gray-600 mb-4">
                {{ exportCount }} fakturor har exporterats till Björn Lunden
                Administration.
              </p>
              <p class="text-xs text-gray-500">
                Filen har sparats lokalt: invoices_export_{{
                  new Date().toISOString().split("T")[0]
                }}.csv
              </p>
            </div>
          </div>

          <div class="px-6 py-4 border-t border-gray-200">
            <button
              @click="showExportStatus = false"
              class="w-full px-4 py-2 text-sm font-medium text-white bg-green-600 border border-transparent rounded-md hover:bg-green-700"
            >
              Stäng
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { CheckIcon } from "@heroicons/vue/24/outline";
import { http } from "@/api/mockClient";

const invoices = ref<any[]>([]);
const customers = ref<any[]>([]);
const showCreateInvoiceModal = ref(false);
const showExportStatus = ref(false);
const exportCount = ref(0);

const filters = ref({
  status: "",
  customerId: "",
  dateFrom: "",
  dateTo: "",
});

const invoiceForm = ref({
  customerId: "",
  amount: "",
  description: "",
  dueDate: "",
});

const filteredInvoices = computed(() => {
  let filtered = invoices.value;

  if (filters.value.status) {
    filtered = filtered.filter(
      (invoice) => invoice.status === filters.value.status,
    );
  }

  if (filters.value.customerId) {
    filtered = filtered.filter(
      (invoice) => invoice.customer_id === parseInt(filters.value.customerId),
    );
  }

  if (filters.value.dateFrom) {
    filtered = filtered.filter(
      (invoice) =>
        new Date(invoice.created_at) >= new Date(filters.value.dateFrom),
    );
  }

  if (filters.value.dateTo) {
    filtered = filtered.filter(
      (invoice) =>
        new Date(invoice.created_at) <= new Date(filters.value.dateTo),
    );
  }

  return filtered;
});

const totalAmount = computed(() => {
  return filteredInvoices.value.reduce(
    (sum, invoice) => sum + invoice.amount,
    0,
  );
});

async function loadData() {
  try {
    const [invoicesRes, customersRes] = await Promise.all([
      http.get("/api/invoices"),
      http.get("/api/customers"),
    ]);

    invoices.value = invoicesRes.data.results;
    customers.value = customersRes.data.results;
  } catch (error) {
    console.error("Error loading data:", error);
  }
}

function getCustomerName(customerId: number): string {
  const customer = customers.value.find((c) => c.id === customerId);
  return customer?.name || "Okänd kund";
}

function getStatusColor(status: string): string {
  const colors = {
    draft: "bg-gray-100 text-gray-800",
    sent: "bg-blue-100 text-blue-800",
    paid: "bg-green-100 text-green-800",
    overdue: "bg-red-100 text-red-800",
  };
  return colors[status as keyof typeof colors] || "bg-gray-100 text-gray-800";
}

function getStatusLabel(status: string): string {
  const labels = {
    draft: "Utkast",
    sent: "Skickad",
    paid: "Betalad",
    overdue: "Förfallen",
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

function viewInvoice(invoice: any) {
  console.log("View invoice:", invoice);
}

function editInvoice(invoice: any) {
  console.log("Edit invoice:", invoice);
}

function exportInvoiceToBL(invoice: any) {
  console.log("Export invoice to BL:", invoice);
  alert(`Faktura ${invoice.invoice_number} exporterad till Björn Lunden!`);
}

function exportToBjornLunden() {
  const invoicesToExport = filteredInvoices.value.filter(
    (invoice) => invoice.status !== "draft",
  );
  exportCount.value = invoicesToExport.length;

  if (exportCount.value === 0) {
    alert("Inga fakturor att exportera. Välj fakturor som inte är utkast.");
    return;
  }

  // Simulate export
  console.log("Exporting to Björn Lunden:", invoicesToExport);
  showExportStatus.value = true;
}

function createInvoice() {
  if (!invoiceForm.value.customerId || !invoiceForm.value.amount) {
    alert("Vänligen fyll i alla obligatoriska fält.");
    return;
  }

  const invoice = {
    id: Date.now(),
    invoice_number: `INV-2025-${String(invoices.value.length + 1).padStart(3, "0")}`,
    customer_id: parseInt(invoiceForm.value.customerId),
    amount: parseFloat(invoiceForm.value.amount),
    status: "draft",
    due_date:
      invoiceForm.value.dueDate ||
      new Date(Date.now() + 30 * 24 * 60 * 60 * 1000)
        .toISOString()
        .split("T")[0],
    created_at: new Date().toISOString(),
    description: invoiceForm.value.description,
  };

  invoices.value.unshift(invoice);
  showCreateInvoiceModal.value = false;

  // Reset form
  invoiceForm.value = {
    customerId: "",
    amount: "",
    description: "",
    dueDate: "",
  };

  alert(`Faktura ${invoice.invoice_number} skapad!`);
}

onMounted(() => {
  loadData();
});
</script>

