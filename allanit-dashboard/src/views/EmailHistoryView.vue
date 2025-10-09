<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-gray-900">E-posthistorik</h1>
        <p class="text-gray-600">Visa skickade e-postutskick och statistik</p>
      </div>
      <div class="flex items-center space-x-3">
        <button
          @click="refreshData"
          class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
        >
          Uppdatera
        </button>
      </div>
    </div>

    <!-- Stats Cards -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-6">
      <div class="bg-white rounded-lg shadow-sm border p-6">
        <div class="text-center">
          <div class="text-2xl font-bold text-gray-900">{{ totalSent }}</div>
          <div class="text-sm text-gray-500">Totalt skickade</div>
        </div>
      </div>
      <div class="bg-white rounded-lg shadow-sm border p-6">
        <div class="text-center">
          <div class="text-2xl font-bold text-gray-900">
            {{ totalDelivered }}
          </div>
          <div class="text-sm text-gray-500">Levererade</div>
        </div>
      </div>
      <div class="bg-white rounded-lg shadow-sm border p-6">
        <div class="text-center">
          <div class="text-2xl font-bold text-gray-900">{{ totalOpened }}</div>
          <div class="text-sm text-gray-500">Öppnade</div>
        </div>
      </div>
      <div class="bg-white rounded-lg shadow-sm border p-6">
        <div class="text-center">
          <div class="text-2xl font-bold text-gray-900">{{ openRate }}%</div>
          <div class="text-sm text-gray-500">Öppningsgrad</div>
        </div>
      </div>
    </div>

    <!-- Email History Table -->
    <div class="bg-white rounded-lg shadow-sm border">
      <div class="px-6 py-4 border-b border-gray-200">
        <h3 class="text-lg font-medium text-gray-900">
          Skickade e-postutskick
        </h3>
      </div>

      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Kampanj
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Mall
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Mottagare
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Levererade
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Öppnade
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                Skickad
              </th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr
              v-for="email in sentEmails"
              :key="email.id"
              class="hover:bg-gray-50"
            >
              <td
                class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900"
              >
                {{ email.campaign_name }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                {{ email.template_name }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                {{ email.recipient_count }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                {{ email.delivered_count }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                {{ email.opened_count }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                {{ formatDate(email.sent_at) }}
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { http } from "@/api/mockClient";

const sentEmails = ref<any[]>([]);

const totalSent = computed(() =>
  sentEmails.value.reduce((sum, email) => sum + email.recipient_count, 0),
);

const totalDelivered = computed(() =>
  sentEmails.value.reduce((sum, email) => sum + email.delivered_count, 0),
);

const totalOpened = computed(() =>
  sentEmails.value.reduce((sum, email) => sum + email.opened_count, 0),
);

const openRate = computed(() => {
  if (totalDelivered.value === 0) return 0;
  return Math.round((totalOpened.value / totalDelivered.value) * 100);
});

async function loadData() {
  try {
    const emailsRes = await http.get("/api/sent-emails");
    sentEmails.value = emailsRes.data.results;
  } catch (error) {
    console.error("Error loading data:", error);
  }
}

function refreshData() {
  loadData();
}

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString("sv-SE");
}

onMounted(() => {
  loadData();
});
</script>
