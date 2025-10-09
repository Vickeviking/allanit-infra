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
        @click="closeInvoiceModal"
      ></div>
      <div class="absolute inset-0 flex items-center justify-center p-4">
        <div class="bg-white rounded-lg shadow-xl w-full max-w-2xl">
          <div class="px-6 py-4 border-b border-gray-200">
            <h3 class="text-lg font-medium text-gray-900">
              {{ editingInvoice ? "Redigera faktura" : "Skapa ny faktura" }}
            </h3>
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
              @click="closeInvoiceModal"
              class="flex-1 px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
            >
              Avbryt
            </button>
            <button
              @click="createInvoice"
              class="flex-1 px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700"
            >
              {{ editingInvoice ? "Uppdatera faktura" : "Skapa faktura" }}
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

    <!-- Invoice Detail Modal -->
    <div
      v-if="viewingInvoice"
      class="fixed inset-0 z-50 overflow-hidden"
    >
      <div
        class="absolute inset-0 bg-gray-500 bg-opacity-75"
        @click="closeInvoiceView"
      ></div>
      <div class="absolute inset-0 flex items-center justify-center p-4">
        <div class="bg-white rounded-2xl shadow-2xl w-full max-w-4xl max-h-[90vh] overflow-hidden">
          <!-- Header -->
          <div class="px-8 py-6 border-b border-gray-200 bg-gradient-to-r from-blue-50 to-indigo-50">
            <div class="flex items-center justify-between">
              <div class="flex items-center space-x-4">
                <div class="p-3 bg-blue-100 rounded-xl">
                  <svg class="w-6 h-6 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M4 4a2 2 0 00-2 2v8a2 2 0 002 2h12a2 2 0 002-2V6a2 2 0 00-2-2H4zm2 6a2 2 0 114 0 2 2 0 01-4 0zm8 0a2 2 0 114 0 2 2 0 01-4 0z" clip-rule="evenodd"></path>
                  </svg>
                </div>
                <div>
                  <h3 class="text-2xl font-bold text-gray-900">{{ viewingInvoice.invoice_number }}</h3>
                  <p class="text-lg text-gray-600">{{ getCustomerName(viewingInvoice.customer_id) }}</p>
                  <div class="flex items-center space-x-2 mt-1">
                    <span
                      class="inline-flex px-3 py-1 text-sm font-medium rounded-full"
                      :class="getStatusColor(viewingInvoice.status)"
                    >
                      {{ getStatusLabel(viewingInvoice.status) }}
                    </span>
                  </div>
                </div>
              </div>
              <button
                @click="closeInvoiceView"
                class="p-3 text-gray-400 hover:text-gray-600 hover:bg-white/80 rounded-xl transition-all duration-200"
              >
                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                </svg>
              </button>
            </div>
          </div>

          <!-- Content -->
          <div class="flex-1 overflow-y-auto p-8">
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
              <!-- Left Column - Invoice Details -->
              <div class="space-y-6">
                <div class="bg-gradient-to-br from-gray-50 to-blue-50 rounded-xl p-6 border border-gray-200">
                  <h4 class="text-lg font-semibold text-gray-900 mb-4 flex items-center">
                    <svg class="w-5 h-5 mr-2 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M4 4a2 2 0 00-2 2v8a2 2 0 002 2h12a2 2 0 002-2V6a2 2 0 00-2-2H4zm2 6a2 2 0 114 0 2 2 0 01-4 0zm8 0a2 2 0 114 0 2 2 0 01-4 0z" clip-rule="evenodd"></path>
                    </svg>
                    Fakturainformation
                  </h4>
                  <div class="space-y-3">
                    <div class="flex justify-between">
                      <span class="text-gray-600">Fakturanummer:</span>
                      <span class="font-medium text-gray-900">{{ viewingInvoice.invoice_number }}</span>
                    </div>
                    <div class="flex justify-between">
                      <span class="text-gray-600">Belopp:</span>
                      <span class="font-bold text-gray-900 text-lg">{{ formatCurrency(viewingInvoice.amount) }}</span>
                    </div>
                    <div class="flex justify-between">
                      <span class="text-gray-600">Status:</span>
                      <span class="font-medium" :class="getStatusTextColor(viewingInvoice.status)">
                        {{ getStatusLabel(viewingInvoice.status) }}
                      </span>
                    </div>
                    <div class="flex justify-between">
                      <span class="text-gray-600">Förfallodatum:</span>
                      <span class="font-medium text-gray-900">{{ formatDate(viewingInvoice.due_date) }}</span>
                    </div>
                    <div class="flex justify-between">
                      <span class="text-gray-600">Skapad:</span>
                      <span class="font-medium text-gray-900">{{ formatDate(viewingInvoice.created_at) }}</span>
                    </div>
                  </div>
                </div>

                <div class="bg-gradient-to-br from-green-50 to-emerald-50 rounded-xl p-6 border border-gray-200">
                  <h4 class="text-lg font-semibold text-gray-900 mb-4 flex items-center">
                    <svg class="w-5 h-5 mr-2 text-green-600" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-6-3a2 2 0 11-4 0 2 2 0 014 0zM8 7a3 3 0 000 6h4a3 3 0 000-6H8z" clip-rule="evenodd"></path>
                    </svg>
                    Kundinformation
                  </h4>
                  <div class="space-y-3">
                    <div class="flex justify-between">
                      <span class="text-gray-600">Kund:</span>
                      <span class="font-medium text-gray-900">{{ getCustomerName(viewingInvoice.customer_id) }}</span>
                    </div>
                    <div class="flex justify-between">
                      <span class="text-gray-600">Kund-ID:</span>
                      <span class="font-medium text-gray-900">#{{ viewingInvoice.customer_id }}</span>
                    </div>
                  </div>
                </div>
              </div>

              <!-- Right Column - Description and Actions -->
              <div class="space-y-6">
                <div class="bg-gradient-to-br from-purple-50 to-pink-50 rounded-xl p-6 border border-gray-200">
                  <h4 class="text-lg font-semibold text-gray-900 mb-4 flex items-center">
                    <svg class="w-5 h-5 mr-2 text-purple-600" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M4 4a2 2 0 012-2h4.586A2 2 0 0112 2.586L15.414 6A2 2 0 0116 7.414V16a2 2 0 01-2 2H6a2 2 0 01-2-2V4zm2 6a1 1 0 011-1h6a1 1 0 110 2H7a1 1 0 01-1-1zm1 3a1 1 0 100 2h6a1 1 0 100-2H7z" clip-rule="evenodd"></path>
                    </svg>
                    Beskrivning
                  </h4>
                  <div class="bg-white rounded-lg p-4 border border-gray-200">
                    <p class="text-gray-700">{{ viewingInvoice.description || 'Ingen beskrivning angiven' }}</p>
                  </div>
                </div>

                <div class="bg-gradient-to-br from-orange-50 to-red-50 rounded-xl p-6 border border-gray-200">
                  <h4 class="text-lg font-semibold text-gray-900 mb-4 flex items-center">
                    <svg class="w-5 h-5 mr-2 text-orange-600" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M11.3 1.046A1 1 0 0112 2v5h4a1 1 0 01.82 1.573l-7 10A1 1 0 018 18v-5H4a1 1 0 01-.82-1.573l7-10a1 1 0 011.12-.38z" clip-rule="evenodd"></path>
                    </svg>
                    Snabbåtgärder
                  </h4>
                  <div class="space-y-3">
                    <button
                      @click="editInvoice(viewingInvoice); closeInvoiceView()"
                      class="w-full px-4 py-3 text-sm font-semibold text-white bg-blue-600 rounded-xl hover:bg-blue-700 transition-colors duration-200 flex items-center justify-center space-x-2"
                    >
                      <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                        <path d="M13.586 3.586a2 2 0 112.828 2.828l-.793.793-2.828-2.828.793-.793zM11.379 5.793L3 14.172V17h2.828l8.38-8.379-2.83-2.828z"></path>
                      </svg>
                      <span>Redigera faktura</span>
                    </button>
                    <button
                      @click="exportInvoiceToBL(viewingInvoice)"
                      class="w-full px-4 py-3 text-sm font-semibold text-white bg-green-600 rounded-xl hover:bg-green-700 transition-colors duration-200 flex items-center justify-center space-x-2"
                    >
                      <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd"></path>
                      </svg>
                      <span>Exportera till Björn Lunden</span>
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Actions -->
          <div class="px-8 py-6 border-t border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <div class="flex space-x-4">
              <button
                @click="closeInvoiceView"
                class="px-6 py-3 text-sm font-semibold text-gray-700 bg-white border border-gray-300 rounded-xl hover:bg-gray-50 hover:border-gray-400 transition-all duration-200 shadow-sm hover:shadow-md"
              >
                Stäng
              </button>
            </div>
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
const editingInvoice = ref<any>(null);
const viewingInvoice = ref<any>(null);

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

function getStatusTextColor(status: string): string {
  const colors = {
    draft: "text-gray-600",
    sent: "text-blue-600",
    paid: "text-green-600",
    overdue: "text-red-600",
  };
  return colors[status as keyof typeof colors] || "text-gray-600";
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
  viewingInvoice.value = invoice;
}

function editInvoice(invoice: any) {
  editingInvoice.value = invoice;
  invoiceForm.value = {
    customerId: invoice.customer_id.toString(),
    amount: invoice.amount.toString(),
    description: invoice.description || "",
    dueDate: invoice.due_date,
  };
  showCreateInvoiceModal.value = true;
}

function exportInvoiceToBL(invoice: any) {
  console.log("Export invoice to BL:", invoice);
  alert(`Faktura ${invoice.invoice_number} exporterad till Björn Lunden!`);
}

function closeInvoiceModal() {
  showCreateInvoiceModal.value = false;
  editingInvoice.value = null;
  invoiceForm.value = {
    customerId: "",
    amount: "",
    description: "",
    dueDate: "",
  };
}

function closeInvoiceView() {
  viewingInvoice.value = null;
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

  if (editingInvoice.value) {
    // Update existing invoice
    const index = invoices.value.findIndex(inv => inv.id === editingInvoice.value.id);
    if (index !== -1) {
      invoices.value[index] = {
        ...invoices.value[index],
        customer_id: parseInt(invoiceForm.value.customerId),
        amount: parseFloat(invoiceForm.value.amount),
        due_date: invoiceForm.value.dueDate,
        description: invoiceForm.value.description,
      };
      alert(`Faktura ${editingInvoice.value.invoice_number} uppdaterad!`);
    }
  } else {
    // Create new invoice
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
    alert(`Faktura ${invoice.invoice_number} skapad!`);
  }

  closeInvoiceModal();
}

onMounted(() => {
  loadData();
});
</script>

