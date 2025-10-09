<template>
  <div class="max-w-screen-xl mx-auto px-4">
    <!-- Header -->
    <div class="flex items-center justify-between mb-8">
      <div class="flex items-center space-x-4">
        <div class="p-3 bg-purple-100 rounded-xl">
          <svg class="w-8 h-8 text-purple-600" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path>
          </svg>
        </div>
        <div>
          <h1 class="text-3xl font-bold text-gray-900">Utvecklarvy</h1>
          <p class="text-gray-600 mt-1">Systemloggar, debugging och utvecklingsverktyg</p>
        </div>
      </div>

      <div class="flex items-center space-x-4">
        <button
          @click="refreshLogs"
          class="px-6 py-3 text-sm font-semibold text-gray-700 bg-white border border-gray-300 rounded-xl hover:bg-gray-50 hover:border-gray-400 transition-all duration-200 shadow-sm hover:shadow-md flex items-center space-x-2"
        >
          <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clip-rule="evenodd"></path>
          </svg>
          <span>Uppdatera</span>
        </button>
        <button
          @click="clearLogs"
          class="px-6 py-3 text-sm font-semibold text-white bg-red-600 border border-transparent rounded-xl hover:bg-red-700 hover:shadow-lg transition-all duration-200 shadow-sm flex items-center space-x-2"
        >
          <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M9 2a1 1 0 000 2h2a1 1 0 100-2H9z" clip-rule="evenodd"></path>
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
          </svg>
          <span>Rensa loggar</span>
        </button>
      </div>
    </div>

    <!-- Stats Cards -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
      <div class="bg-white rounded-2xl shadow-sm border border-gray-200 p-6 hover:shadow-lg transition-shadow duration-200">
        <div class="text-center">
          <div class="text-3xl font-bold text-blue-600 mb-2">{{ totalLogs.toLocaleString() }}</div>
          <div class="text-sm text-gray-500 font-medium">Totalt loggar</div>
          <div class="text-xs text-gray-400 mt-1">Senaste 24h</div>
        </div>
      </div>
      <div class="bg-white rounded-2xl shadow-sm border border-gray-200 p-6 hover:shadow-lg transition-shadow duration-200">
        <div class="text-center">
          <div class="text-3xl font-bold text-red-600 mb-2">{{ errorCount.toLocaleString() }}</div>
          <div class="text-sm text-gray-500 font-medium">Fel</div>
          <div class="text-xs text-gray-400 mt-1">{{ Math.round((errorCount / totalLogs) * 100) }}% av totalt</div>
        </div>
      </div>
      <div class="bg-white rounded-2xl shadow-sm border border-gray-200 p-6 hover:shadow-lg transition-shadow duration-200">
        <div class="text-center">
          <div class="text-3xl font-bold text-yellow-600 mb-2">{{ warningCount.toLocaleString() }}</div>
          <div class="text-sm text-gray-500 font-medium">Varningar</div>
          <div class="text-xs text-gray-400 mt-1">{{ Math.round((warningCount / totalLogs) * 100) }}% av totalt</div>
        </div>
      </div>
      <div class="bg-white rounded-2xl shadow-sm border border-gray-200 p-6 hover:shadow-lg transition-shadow duration-200">
        <div class="text-center">
          <div class="text-3xl font-bold text-green-600 mb-2">{{ infoCount.toLocaleString() }}</div>
          <div class="text-sm text-gray-500 font-medium">Info</div>
          <div class="text-xs text-gray-400 mt-1">{{ Math.round((infoCount / totalLogs) * 100) }}% av totalt</div>
        </div>
      </div>
    </div>

    <!-- Filters -->
    <div class="bg-white rounded-2xl shadow-sm border border-gray-200 p-6 mb-6">
      <h3 class="text-lg font-semibold text-gray-900 mb-4">Filtrera loggar</h3>
      
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <!-- Level Filter -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Nivå</label>
          <select
            v-model="filters.level"
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-purple-500"
          >
            <option value="">Alla nivåer</option>
            <option value="ERROR">Error</option>
            <option value="WARN">Warning</option>
            <option value="INFO">Info</option>
            <option value="DEBUG">Debug</option>
          </select>
        </div>

        <!-- Module Filter -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Modul</label>
          <select
            v-model="filters.module"
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-purple-500"
          >
            <option value="">Alla moduler</option>
            <option value="Fetcher">Fetcher</option>
            <option value="Ingestor">Ingestor</option>
            <option value="DbWriter">DbWriter</option>
            <option value="DeadLetter">DeadLetter</option>
            <option value="CommandBus">CommandBus</option>
            <option value="Rocket">Rocket</option>
            <option value="EmailService">EmailService</option>
            <option value="AuthService">AuthService</option>
            <option value="PaymentService">PaymentService</option>
            <option value="NotificationService">NotificationService</option>
          </select>
        </div>

        <!-- Action Filter -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Åtgärd</label>
          <select
            v-model="filters.action"
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-purple-500"
          >
            <option value="">Alla åtgärder</option>
            <option value="FETCH">Fetch</option>
            <option value="UPSERT">Upsert</option>
            <option value="REPROCESS">Reprocess</option>
            <option value="ERROR">Error</option>
            <option value="SUCCESS">Success</option>
            <option value="RETRY">Retry</option>
            <option value="TIMEOUT">Timeout</option>
            <option value="VALIDATION">Validation</option>
            <option value="SEND">Send</option>
            <option value="RECEIVE">Receive</option>
            <option value="PROCESS">Process</option>
            <option value="ARCHIVE">Archive</option>
          </select>
        </div>

        <!-- Search -->
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Sök</label>
          <input
            v-model="filters.search"
            type="text"
            placeholder="Sök i meddelanden..."
            class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-purple-500"
          />
        </div>
      </div>

      <div class="flex justify-between items-center mt-4">
        <div class="text-sm text-gray-500">
          Visar {{ filteredLogs.length }} av {{ totalLogs }} loggar
        </div>
        <div class="flex space-x-2">
          <button
            @click="clearFilters"
            class="px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200 transition-colors"
          >
            Rensa filter
          </button>
          <button
            @click="exportLogs"
            class="px-4 py-2 text-sm font-medium text-white bg-purple-600 rounded-lg hover:bg-purple-700 transition-colors"
          >
            Exportera
          </button>
        </div>
      </div>
    </div>

    <!-- Logs Table -->
    <div class="bg-white rounded-2xl shadow-sm border border-gray-200 overflow-hidden">
      <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-purple-50 to-indigo-50">
        <div class="flex items-center space-x-3">
          <div class="p-2 bg-purple-100 rounded-lg">
            <svg class="w-5 h-5 text-purple-600" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path>
            </svg>
          </div>
          <h3 class="text-lg font-bold text-gray-900">
            Systemloggar
          </h3>
        </div>
      </div>

      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th class="px-6 py-3 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                Tid
              </th>
              <th class="px-6 py-3 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                Nivå
              </th>
              <th class="px-6 py-3 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                Modul
              </th>
              <th class="px-6 py-3 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                Åtgärd
              </th>
              <th class="px-6 py-3 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                Meddelande
              </th>
              <th class="px-6 py-3 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                Detaljer
              </th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr
              v-for="log in paginatedLogs"
              :key="log.id"
              class="hover:bg-gray-50 transition-colors duration-200"
            >
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                <div class="flex items-center space-x-2">
                  <svg class="w-4 h-4 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd"></path>
                  </svg>
                  <span>{{ formatTime(log.created_at) }}</span>
                </div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap">
                <span
                  class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-semibold"
                  :class="getLevelClass(log.level)"
                >
                  {{ log.level }}
                </span>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                <div class="flex items-center space-x-2">
                  <div class="p-1 bg-gray-100 rounded">
                    <svg class="w-3 h-3 text-gray-600" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path>
                    </svg>
                  </div>
                  <span>{{ log.module }}</span>
                </div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                <span class="font-mono text-xs bg-gray-100 px-2 py-1 rounded">{{ log.action }}</span>
              </td>
              <td class="px-6 py-4 text-sm text-gray-900 max-w-md">
                <div class="truncate">{{ log.custom_msg }}</div>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                <button
                  @click="toggleLogDetails(log.id)"
                  class="text-purple-600 hover:text-purple-800 font-medium"
                >
                  {{ expandedLogs.has(log.id) ? 'Dölj' : 'Visa' }}
                </button>
              </td>
            </tr>
            
            <!-- Expanded Details Row -->
            <tr
              v-for="log in paginatedLogs"
              :key="`details-${log.id}`"
              v-show="expandedLogs.has(log.id)"
              class="bg-gray-50"
            >
              <td colspan="6" class="px-6 py-4">
                <div class="bg-white rounded-lg p-4 border border-gray-200">
                  <h4 class="font-semibold text-gray-900 mb-3">Loggdetaljer</h4>
                  
                  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div>
                      <h5 class="text-sm font-medium text-gray-700 mb-2">Grundläggande information</h5>
                      <div class="space-y-2 text-sm">
                        <div class="flex justify-between">
                          <span class="text-gray-500">ID:</span>
                          <span class="font-mono">{{ log.id }}</span>
                        </div>
                        <div class="flex justify-between">
                          <span class="text-gray-500">Skapad:</span>
                          <span>{{ formatDateTime(log.created_at) }}</span>
                        </div>
                        <div class="flex justify-between">
                          <span class="text-gray-500">Expirerar:</span>
                          <span>{{ formatDateTime(log.expires_at) }}</span>
                        </div>
                      </div>
                    </div>
                    
                    <div v-if="log.details">
                      <h5 class="text-sm font-medium text-gray-700 mb-2">Detaljer</h5>
                      <div class="space-y-2 text-sm">
                        <div v-if="log.details.duration" class="flex justify-between">
                          <span class="text-gray-500">Varaktighet:</span>
                          <span>{{ log.details.duration }}ms</span>
                        </div>
                        <div v-if="log.details.records_affected" class="flex justify-between">
                          <span class="text-gray-500">Poster påverkade:</span>
                          <span>{{ log.details.records_affected }}</span>
                        </div>
                        <div v-if="log.details.error_code" class="flex justify-between">
                          <span class="text-gray-500">Felkod:</span>
                          <span class="font-mono text-red-600">{{ log.details.error_code }}</span>
                        </div>
                        <div v-if="log.details.retry_count" class="flex justify-between">
                          <span class="text-gray-500">Försök:</span>
                          <span>{{ log.details.retry_count }}</span>
                        </div>
                        <div v-if="log.details.user_id" class="flex justify-between">
                          <span class="text-gray-500">Användar-ID:</span>
                          <span class="font-mono">{{ log.details.user_id }}</span>
                        </div>
                        <div v-if="log.details.request_id" class="flex justify-between">
                          <span class="text-gray-500">Request-ID:</span>
                          <span class="font-mono text-xs">{{ log.details.request_id }}</span>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <!-- Pagination -->
      <div class="px-6 py-4 border-t border-gray-200 bg-gray-50">
        <div class="flex items-center justify-between">
          <div class="text-sm text-gray-500">
            Visar {{ (currentPage - 1) * itemsPerPage + 1 }} till {{ Math.min(currentPage * itemsPerPage, filteredLogs.length) }} av {{ filteredLogs.length }} loggar
          </div>
          <div class="flex space-x-2">
            <button
              @click="currentPage = Math.max(1, currentPage - 1)"
              :disabled="currentPage === 1"
              class="px-3 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Föregående
            </button>
            <span class="px-3 py-2 text-sm text-gray-700">
              Sida {{ currentPage }} av {{ totalPages }}
            </span>
            <button
              @click="currentPage = Math.min(totalPages, currentPage + 1)"
              :disabled="currentPage === totalPages"
              class="px-3 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              Nästa
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { http } from "@/api/mockClient";
import type { LogEntry } from "@/mock/logs";

const logs = ref<LogEntry[]>([]);
const expandedLogs = ref(new Set<number>());
const currentPage = ref(1);
const itemsPerPage = ref(20);

const filters = ref({
  level: "",
  module: "",
  action: "",
  search: ""
});

const filteredLogs = computed(() => {
  return logs.value.filter(log => {
    if (filters.value.level && log.level !== filters.value.level) return false;
    if (filters.value.module && log.module !== filters.value.module) return false;
    if (filters.value.action && log.action !== filters.value.action) return false;
    if (filters.value.search && !log.custom_msg.toLowerCase().includes(filters.value.search.toLowerCase())) return false;
    return true;
  });
});

const paginatedLogs = computed(() => {
  const start = (currentPage.value - 1) * itemsPerPage.value;
  const end = start + itemsPerPage.value;
  return filteredLogs.value.slice(start, end);
});

const totalPages = computed(() => {
  return Math.ceil(filteredLogs.value.length / itemsPerPage.value);
});

const totalLogs = computed(() => logs.value.length);
const errorCount = computed(() => logs.value.filter(log => log.level === 'ERROR').length);
const warningCount = computed(() => logs.value.filter(log => log.level === 'WARN').length);
const infoCount = computed(() => logs.value.filter(log => log.level === 'INFO').length);

function getLevelClass(level: string) {
  switch (level) {
    case 'ERROR':
      return 'bg-red-100 text-red-800';
    case 'WARN':
      return 'bg-yellow-100 text-yellow-800';
    case 'INFO':
      return 'bg-blue-100 text-blue-800';
    case 'DEBUG':
      return 'bg-gray-100 text-gray-800';
    default:
      return 'bg-gray-100 text-gray-800';
  }
}

function formatTime(dateString: string): string {
  return new Date(dateString).toLocaleTimeString("sv-SE");
}

function formatDateTime(dateString: string): string {
  return new Date(dateString).toLocaleString("sv-SE");
}

function toggleLogDetails(logId: number) {
  if (expandedLogs.value.has(logId)) {
    expandedLogs.value.delete(logId);
  } else {
    expandedLogs.value.add(logId);
  }
}

function clearFilters() {
  filters.value = {
    level: "",
    module: "",
    action: "",
    search: ""
  };
  currentPage.value = 1;
}

function refreshLogs() {
  loadLogs();
}

function clearLogs() {
  if (confirm("Är du säker på att du vill rensa alla loggar? Detta går inte att ångra.")) {
    logs.value = [];
    expandedLogs.value.clear();
    currentPage.value = 1;
  }
}

function exportLogs() {
  const dataStr = JSON.stringify(filteredLogs.value, null, 2);
  const dataBlob = new Blob([dataStr], { type: 'application/json' });
  const url = URL.createObjectURL(dataBlob);
  const link = document.createElement('a');
  link.href = url;
  link.download = `logs_${new Date().toISOString().split('T')[0]}.json`;
  link.click();
  URL.revokeObjectURL(url);
}

async function loadLogs() {
  try {
    const logsRes = await http.get("/api/logs");
    logs.value = logsRes.data.results || logsRes.data;
  } catch (error) {
    console.error("Error loading logs:", error);
  }
}

onMounted(() => {
  loadLogs();
});
</script>