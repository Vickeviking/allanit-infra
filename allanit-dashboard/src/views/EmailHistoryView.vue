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
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
      <div class="bg-white rounded-2xl shadow-sm border border-gray-200 p-6 hover:shadow-lg transition-shadow duration-200">
        <div class="text-center">
          <div class="text-3xl font-bold text-blue-600 mb-2">{{ totalSent.toLocaleString() }}</div>
          <div class="text-sm text-gray-500 font-medium">Totalt skickade</div>
          <div class="text-xs text-gray-400 mt-1">{{ sentEmails.length }} kampanjer</div>
        </div>
      </div>
      <div class="bg-white rounded-2xl shadow-sm border border-gray-200 p-6 hover:shadow-lg transition-shadow duration-200">
        <div class="text-center">
          <div class="text-3xl font-bold text-green-600 mb-2">{{ openRate }}%</div>
          <div class="text-sm text-gray-500 font-medium">Öppningsgrad</div>
          <div class="text-xs text-gray-400 mt-1">{{ totalOpened.toLocaleString() }} öppnade</div>
        </div>
      </div>
      <div class="bg-white rounded-2xl shadow-sm border border-gray-200 p-6 hover:shadow-lg transition-shadow duration-200">
        <div class="text-center">
          <div class="text-3xl font-bold text-purple-600 mb-2">{{ clickRate }}%</div>
          <div class="text-sm text-gray-500 font-medium">Klickgrad</div>
          <div class="text-xs text-gray-400 mt-1">{{ totalClicked.toLocaleString() }} klick</div>
        </div>
      </div>
      <div class="bg-white rounded-2xl shadow-sm border border-gray-200 p-6 hover:shadow-lg transition-shadow duration-200">
        <div class="text-center">
          <div class="text-3xl font-bold text-red-600 mb-2">{{ bounceRate }}%</div>
          <div class="text-sm text-gray-500 font-medium">Studsgrad</div>
          <div class="text-xs text-gray-400 mt-1">{{ totalBounced.toLocaleString() }} studsade</div>
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
                {{ email.to_count }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                <div class="flex items-center space-x-2">
                  <span class="font-medium">{{ email.delivered }}</span>
                  <span class="text-xs text-gray-500">({{ Math.round((email.delivered / email.to_count) * 100) }}%)</span>
                </div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                <div class="flex items-center space-x-2">
                  <span class="font-medium">{{ email.opened }}</span>
                  <span class="text-xs text-gray-500">({{ Math.round((email.opened / email.delivered) * 100) }}%)</span>
                </div>
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
  sentEmails.value.reduce((sum, email) => sum + email.to_count, 0),
);

const totalDelivered = computed(() =>
  sentEmails.value.reduce((sum, email) => sum + email.delivered, 0),
);

const totalOpened = computed(() =>
  sentEmails.value.reduce((sum, email) => sum + email.opened, 0),
);

const totalClicked = computed(() =>
  sentEmails.value.reduce((sum, email) => sum + email.clicked, 0),
);

const totalBounced = computed(() =>
  sentEmails.value.reduce((sum, email) => sum + email.bounced, 0),
);

const openRate = computed(() => {
  if (totalDelivered.value === 0) return 0;
  return Math.round((totalOpened.value / totalDelivered.value) * 100);
});

const clickRate = computed(() => {
  if (totalOpened.value === 0) return 0;
  return Math.round((totalClicked.value / totalOpened.value) * 100);
});

const bounceRate = computed(() => {
  if (totalSent.value === 0) return 0;
  return Math.round((totalBounced.value / totalSent.value) * 100);
});

const deliveryRate = computed(() => {
  if (totalSent.value === 0) return 0;
  return Math.round((totalDelivered.value / totalSent.value) * 100);
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
