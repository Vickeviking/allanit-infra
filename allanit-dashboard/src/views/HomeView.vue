<template>
  <div class="space-y-8">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-3xl font-bold text-gray-900">Översikt</h1>
        <p class="text-gray-600 mt-2">
          Välkommen till Allanit Dashboard - Snabb överblick över verksamheten
        </p>
      </div>
      <div class="flex items-center space-x-3">
        <button
          @click="refreshData"
          class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-xl hover:bg-gray-50 hover:border-gray-400 transition-all duration-200 shadow-sm hover:shadow-md flex items-center space-x-2"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path>
          </svg>
          <span>Uppdatera</span>
        </button>
      </div>
    </div>

    <!-- Key Metrics Dashboard -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
      <div class="bg-gradient-to-br from-blue-50 to-indigo-100 rounded-2xl p-6 border border-blue-200 shadow-sm hover:shadow-md transition-shadow duration-200">
        <div class="flex items-center justify-between mb-4">
          <div class="p-3 bg-blue-100 rounded-xl">
            <UserGroupIcon class="w-6 h-6 text-blue-600" />
          </div>
          <div class="text-right">
            <div class="text-2xl font-bold text-blue-900">{{ totalCustomers }}</div>
            <div class="text-sm text-blue-600">Aktiva kunder</div>
          </div>
        </div>
        <div class="flex items-center text-sm text-blue-700">
          <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M5.293 7.707a1 1 0 010-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 01-1.414 1.414L11 5.414V17a1 1 0 11-2 0V5.414L6.707 7.707a1 1 0 01-1.414 0z" clip-rule="evenodd"></path>
          </svg>
          <span class="font-medium">+2 denna månad</span>
        </div>
      </div>

      <div class="bg-gradient-to-br from-green-50 to-emerald-100 rounded-2xl p-6 border border-green-200 shadow-sm hover:shadow-md transition-shadow duration-200">
        <div class="flex items-center justify-between mb-4">
          <div class="p-3 bg-green-100 rounded-xl">
            <ShoppingBagIcon class="w-6 h-6 text-green-600" />
          </div>
          <div class="text-right">
            <div class="text-2xl font-bold text-green-900">{{ totalActiveOrders }}</div>
            <div class="text-sm text-green-600">Aktiva uppdrag</div>
          </div>
        </div>
        <div class="flex items-center text-sm text-green-700">
          <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M5.293 7.707a1 1 0 010-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 01-1.414 1.414L11 5.414V17a1 1 0 11-2 0V5.414L6.707 7.707a1 1 0 01-1.414 0z" clip-rule="evenodd"></path>
          </svg>
          <span class="font-medium">+1 denna vecka</span>
        </div>
      </div>

      <div class="bg-gradient-to-br from-purple-50 to-pink-100 rounded-2xl p-6 border border-purple-200 shadow-sm hover:shadow-md transition-shadow duration-200">
        <div class="flex items-center justify-between mb-4">
          <div class="p-3 bg-purple-100 rounded-xl">
            <CurrencyDollarIcon class="w-6 h-6 text-purple-600" />
          </div>
          <div class="text-right">
            <div class="text-2xl font-bold text-purple-900">{{ formatCurrency(totalRevenue) }}</div>
            <div class="text-sm text-purple-600">Månadsomsättning</div>
          </div>
        </div>
        <div class="flex items-center text-sm text-purple-700">
          <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M5.293 7.707a1 1 0 010-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 01-1.414 1.414L11 5.414V17a1 1 0 11-2 0V5.414L6.707 7.707a1 1 0 01-1.414 0z" clip-rule="evenodd"></path>
          </svg>
          <span class="font-medium">+12% från förra månaden</span>
        </div>
      </div>

      <div class="bg-gradient-to-br from-orange-50 to-red-100 rounded-2xl p-6 border border-orange-200 shadow-sm hover:shadow-md transition-shadow duration-200">
        <div class="flex items-center justify-between mb-4">
          <div class="p-3 bg-orange-100 rounded-xl">
            <UserIcon class="w-6 h-6 text-orange-600" />
          </div>
          <div class="text-right">
            <div class="text-2xl font-bold text-orange-900">{{ totalEmployees }}</div>
            <div class="text-sm text-orange-600">Medarbetare</div>
          </div>
        </div>
        <div class="flex items-center text-sm text-orange-700">
          <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-8.293l-3-3a1 1 0 00-1.414 0l-3 3a1 1 0 001.414 1.414L9 9.414V13a1 1 0 102 0V9.414l1.293 1.293a1 1 0 001.414-1.414z" clip-rule="evenodd"></path>
          </svg>
          <span class="font-medium">Stabil</span>
        </div>
      </div>
    </div>

    <!-- Quick Actions Grid -->
    <div class="bg-white rounded-2xl shadow-sm border border-gray-200 overflow-hidden">
      <div class="px-8 py-6 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-blue-50">
        <h3 class="text-xl font-bold text-gray-900 flex items-center">
          <svg class="w-6 h-6 mr-3 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M11.3 1.046A1 1 0 0112 2v5h4a1 1 0 01.82 1.573l-7 10A1 1 0 018 18v-5H4a1 1 0 01-.82-1.573l7-10a1 1 0 011.12-.38z" clip-rule="evenodd"></path>
          </svg>
          Snabbåtgärder
        </h3>
        <p class="text-sm text-gray-600 mt-1">Här kan du snabbt komma åt de vanligaste funktionerna</p>
      </div>
      
      <div class="p-8">
        <div class="grid grid-cols-2 md:grid-cols-4 lg:grid-cols-6 gap-6">
          <button
            @click="$router.push('/email-management')"
            class="group flex flex-col items-center p-6 text-gray-600 hover:text-blue-600 hover:bg-blue-50 rounded-2xl transition-all duration-200 border border-transparent hover:border-blue-200 hover:shadow-md"
          >
            <div class="p-4 bg-blue-100 rounded-2xl mb-3 group-hover:bg-blue-200 transition-colors duration-200">
              <MegaphoneIcon class="w-8 h-8 text-blue-600" />
            </div>
            <span class="text-sm font-semibold text-center">E-posthantering</span>
            <span class="text-xs text-gray-500 text-center mt-1">Skicka utskick</span>
          </button>

          <button
            @click="$router.push('/invoices')"
            class="group flex flex-col items-center p-6 text-gray-600 hover:text-green-600 hover:bg-green-50 rounded-2xl transition-all duration-200 border border-transparent hover:border-green-200 hover:shadow-md"
          >
            <div class="p-4 bg-green-100 rounded-2xl mb-3 group-hover:bg-green-200 transition-colors duration-200">
              <DocumentTextIcon class="w-8 h-8 text-green-600" />
            </div>
            <span class="text-sm font-semibold text-center">Fakturor</span>
            <span class="text-xs text-gray-500 text-center mt-1">Skapa & hantera</span>
          </button>

          <button
            @click="$router.push('/employees')"
            class="group flex flex-col items-center p-6 text-gray-600 hover:text-purple-600 hover:bg-purple-50 rounded-2xl transition-all duration-200 border border-transparent hover:border-purple-200 hover:shadow-md"
          >
            <div class="p-4 bg-purple-100 rounded-2xl mb-3 group-hover:bg-purple-200 transition-colors duration-200">
              <UserIcon class="w-8 h-8 text-purple-600" />
            </div>
            <span class="text-sm font-semibold text-center">Medarbetare</span>
            <span class="text-xs text-gray-500 text-center mt-1">Hantera team</span>
          </button>

          <button
            @click="$router.push('/customers')"
            class="group flex flex-col items-center p-6 text-gray-600 hover:text-orange-600 hover:bg-orange-50 rounded-2xl transition-all duration-200 border border-transparent hover:border-orange-200 hover:shadow-md"
          >
            <div class="p-4 bg-orange-100 rounded-2xl mb-3 group-hover:bg-orange-200 transition-colors duration-200">
              <UserGroupIcon class="w-8 h-8 text-orange-600" />
            </div>
            <span class="text-sm font-semibold text-center">Kunder</span>
            <span class="text-xs text-gray-500 text-center mt-1">Kundregister</span>
          </button>

          <button
            @click="$router.push('/orders')"
            class="group flex flex-col items-center p-6 text-gray-600 hover:text-indigo-600 hover:bg-indigo-50 rounded-2xl transition-all duration-200 border border-transparent hover:border-indigo-200 hover:shadow-md"
          >
            <div class="p-4 bg-indigo-100 rounded-2xl mb-3 group-hover:bg-indigo-200 transition-colors duration-200">
              <ShoppingBagIcon class="w-8 h-8 text-indigo-600" />
            </div>
            <span class="text-sm font-semibold text-center">Uppdrag</span>
            <span class="text-xs text-gray-500 text-center mt-1">Beställningar</span>
          </button>

          <button
            @click="$router.push('/subsidiaries')"
            class="group flex flex-col items-center p-6 text-gray-600 hover:text-pink-600 hover:bg-pink-50 rounded-2xl transition-all duration-200 border border-transparent hover:border-pink-200 hover:shadow-md"
          >
            <div class="p-4 bg-pink-100 rounded-2xl mb-3 group-hover:bg-pink-200 transition-colors duration-200">
              <BuildingOfficeIcon class="w-8 h-8 text-pink-600" />
            </div>
            <span class="text-sm font-semibold text-center">Dotterbolag</span>
            <span class="text-xs text-gray-500 text-center mt-1">Bolagsinfo</span>
          </button>
        </div>
      </div>
    </div>

    <!-- Recent Activity -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
      <!-- Recent Orders -->
      <div class="bg-white rounded-2xl shadow-sm border border-gray-200 overflow-hidden">
        <div class="px-8 py-6 border-b border-gray-200 bg-gradient-to-r from-blue-50 to-indigo-50">
          <h3 class="text-xl font-bold text-gray-900 flex items-center">
            <svg class="w-6 h-6 mr-3 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M10 2L3 7v11a1 1 0 001 1h12a1 1 0 001-1V7l-7-5zM8 15a1 1 0 11-2 0 1 1 0 012 0zm4 0a1 1 0 11-2 0 1 1 0 012 0z" clip-rule="evenodd"></path>
            </svg>
            Senaste uppdrag
          </h3>
        </div>
        <div class="p-8">
          <div v-if="loading" class="space-y-4">
            <div v-for="i in 3" :key="i" class="animate-pulse">
              <div class="h-16 bg-gray-200 rounded-xl"></div>
            </div>
          </div>
          <div v-else class="space-y-4">
            <div
              v-for="order in recentOrders"
              :key="order.id"
              class="flex items-center justify-between p-4 bg-gradient-to-r from-gray-50 to-blue-50 rounded-xl border border-gray-200 hover:shadow-sm transition-shadow duration-200"
            >
              <div class="flex items-center space-x-4">
                <img
                  :src="getOrderImage(order.description)"
                  :alt="order.description"
                  class="w-12 h-12 rounded-xl object-cover shadow-sm"
                />
                <div>
                  <div class="font-semibold text-gray-900">{{ order.description }}</div>
                  <div class="text-sm text-gray-600">{{ order.customer_name }}</div>
                </div>
              </div>
              <div class="text-right">
                <div class="font-bold text-gray-900">{{ formatCurrency(order.amount) }}</div>
                <div class="text-sm text-gray-500">{{ formatDate(order.created_at) }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Recent Emails -->
      <div class="bg-white rounded-2xl shadow-sm border border-gray-200 overflow-hidden">
        <div class="px-8 py-6 border-b border-gray-200 bg-gradient-to-r from-green-50 to-emerald-50">
          <h3 class="text-xl font-bold text-gray-900 flex items-center">
            <svg class="w-6 h-6 mr-3 text-green-600" fill="currentColor" viewBox="0 0 20 20">
              <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z"></path>
              <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z"></path>
            </svg>
            Senaste e-postutskick
          </h3>
        </div>
        <div class="p-8">
          <div v-if="loading" class="space-y-4">
            <div v-for="i in 3" :key="i" class="animate-pulse">
              <div class="h-16 bg-gray-200 rounded-xl"></div>
            </div>
          </div>
          <div v-else class="space-y-4">
            <div
              v-for="email in recentEmails"
              :key="email.id"
              class="flex items-center justify-between p-4 bg-gradient-to-r from-gray-50 to-green-50 rounded-xl border border-gray-200 hover:shadow-sm transition-shadow duration-200"
            >
              <div>
                <div class="font-semibold text-gray-900">{{ email.campaign_name }}</div>
                <div class="text-sm text-gray-600">{{ email.template_name }}</div>
              </div>
              <div class="text-right">
                <div class="font-bold text-gray-900">{{ email.recipient_count }} mottagare</div>
                <div class="text-sm text-gray-500">{{ formatDate(email.sent_at) }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import {
  UserGroupIcon,
  ShoppingBagIcon,
  CurrencyDollarIcon,
  UserIcon,
  MegaphoneIcon,
  DocumentTextIcon,
} from "@heroicons/vue/24/outline";
import KpiCard from "@/components/ui/KpiCard.vue";
import DataCard from "@/components/ui/DataCard.vue";
import { http } from "@/api/mockClient";

const loading = ref(false);
const subsidiaries = ref<any[]>([]);
const customers = ref<any[]>([]);
const orders = ref<any[]>([]);
const employees = ref<any[]>([]);
const sentEmails = ref<any[]>([]);

const totalCustomers = computed(() => customers.value.length);
const totalActiveOrders = computed(
  () =>
    orders.value.filter(
      (o) => o.status === "open" || o.status === "in_progress",
    ).length,
);
const totalRevenue = computed(() =>
  subsidiaries.value.reduce((sum, s) => sum + s.monthly_revenue, 0),
);
const totalEmployees = computed(() =>
  subsidiaries.value.reduce((sum, s) => sum + s.employees, 0),
);

const recentOrders = computed(() =>
  orders.value
    .sort(
      (a, b) =>
        new Date(b.created_at).getTime() - new Date(a.created_at).getTime(),
    )
    .slice(0, 5),
);

const recentEmails = computed(() =>
  sentEmails.value
    .sort(
      (a, b) => new Date(b.sent_at).getTime() - new Date(a.sent_at).getTime(),
    )
    .slice(0, 5),
);

async function loadData() {
  loading.value = true;
  try {
    const [subsidiariesRes, customersRes, ordersRes, employeesRes, emailsRes] =
      await Promise.all([
        http.get("/api/subsidiaries"),
        http.get("/api/customers"),
        http.get("/api/purchase-orders"),
        http.get("/api/employees"),
        http.get("/api/sent-emails"),
      ]);

    subsidiaries.value = subsidiariesRes.data.results;
    customers.value = customersRes.data.results;
    orders.value = ordersRes.data.results;
    employees.value = employeesRes.data.results;
    sentEmails.value = emailsRes.data.results;
  } catch (error) {
    console.error("Error loading data:", error);
  } finally {
    loading.value = false;
  }
}

function refreshData() {
  loadData();
}

function formatCurrency(amount: number): string {
  return new Intl.NumberFormat("sv-SE", {
    style: "currency",
    currency: "SEK",
    minimumFractionDigits: 0,
  }).format(amount);
}

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString("sv-SE");
}

function getOrderImage(description: string): string {
  if (
    description.toLowerCase().includes("snöröjning") ||
    description.toLowerCase().includes("vinter")
  ) {
    return "/src/resources/updrag/Snöröjning.png";
  } else if (description.toLowerCase().includes("miljörum")) {
    return "/src/resources/updrag/Miljörum.png";
  }
  return "/src/resources/Generiskbostadsrätt.png";
}

onMounted(() => {
  loadData();
});
</script>

