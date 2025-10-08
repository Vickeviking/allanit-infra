<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-gray-900">Loggar</h1>
        <p class="text-gray-600">Systemloggar och aktivitetshistorik</p>
      </div>
      <div class="flex items-center space-x-3">
        <button
          @click="refreshLogs"
          :disabled="loading"
          class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50"
        >
          {{ loading ? 'Laddar...' : 'Uppdatera' }}
        </button>
        <button
          @click="exportLogs"
          class="px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700"
        >
          Exportera
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

    <!-- Logs List -->
    <div class="bg-white rounded-lg shadow-sm border">
      <div class="px-6 py-4 border-b border-gray-200">
        <div class="flex items-center justify-between">
          <h3 class="text-lg font-medium text-gray-900">Loggar</h3>
          <div class="flex items-center space-x-4">
            <div class="text-sm text-gray-600">
              {{ filteredLogs.length }} loggar
            </div>
            <div class="flex items-center space-x-2">
              <button
                @click="toggleAutoRefresh"
                class="px-3 py-1 text-sm rounded-md"
                :class="autoRefresh ? 'bg-green-100 text-green-800' : 'bg-gray-100 text-gray-800'"
              >
                {{ autoRefresh ? 'Auto-uppdatering på' : 'Auto-uppdatering av' }}
              </button>
            </div>
          </div>
        </div>
      </div>

      <div class="max-h-96 overflow-y-auto">
        <div v-if="loading && logs.length === 0" class="p-6">
          <div class="space-y-3">
            <div v-for="i in 5" :key="i" class="animate-pulse">
              <div class="h-4 bg-gray-200 rounded w-3/4"></div>
            </div>
          </div>
        </div>

        <div v-else-if="filteredLogs.length === 0" class="p-6 text-center">
          <DocumentTextIcon class="w-12 h-12 text-gray-400 mx-auto mb-4" />
          <h3 class="text-lg font-medium text-gray-900 mb-2">Inga loggar</h3>
          <p class="text-gray-600">Inga loggar matchar dina filter.</p>
        </div>

        <div v-else class="divide-y divide-gray-200">
          <div
            v-for="log in filteredLogs"
            :key="log.id"
            class="p-4 hover:bg-gray-50 cursor-pointer"
            @click="openLogDetail(log)"
          >
            <div class="flex items-start space-x-3">
              <div
                class="w-2 h-2 rounded-full mt-2 flex-shrink-0"
                :class="getLogLevelColor(log.level)"
              ></div>
              <div class="flex-1 min-w-0">
                <div class="flex items-center justify-between">
                  <div class="flex items-center space-x-2">
                    <span class="text-sm font-medium text-gray-900">{{ log.module }}</span>
                    <span class="text-sm text-gray-500">•</span>
                    <span class="text-sm text-gray-600">{{ log.action }}</span>
                  </div>
                  <span class="text-xs text-gray-500">{{ formatTime(log.created_at) }}</span>
                </div>
                <p v-if="log.custom_msg" class="text-sm text-gray-600 mt-1">
                  {{ log.custom_msg }}
                </p>
                <div class="mt-2 flex items-center space-x-4 text-xs text-gray-500">
                  <span>ID: {{ log.id }}</span>
                  <span>Nivå: {{ log.level }}</span>
                  <button
                    @click.stop="viewRelatedEntity(log)"
                    class="text-blue-600 hover:text-blue-800"
                  >
                    Visa relaterad entitet
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Load More -->
      <div v-if="hasMoreLogs" class="px-6 py-4 border-t border-gray-200">
        <button
          @click="loadMoreLogs"
          :disabled="loadingMore"
          class="w-full px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50"
        >
          {{ loadingMore ? 'Laddar...' : 'Ladda fler loggar' }}
        </button>
      </div>
    </div>

    <!-- Log Detail Drawer -->
    <div
      v-if="selectedLog"
      class="fixed inset-0 z-50 overflow-hidden"
    >
      <div class="absolute inset-0 bg-gray-500 bg-opacity-75" @click="closeLogDetail"></div>
      <div class="absolute right-0 top-0 h-full w-96 bg-white shadow-xl">
        <DetailDrawer
          :title="`Logg ${selectedLog.id}`"
          :tabs="logTabs"
          :actions="logActions"
          @close="closeLogDetail"
          @action="handleLogAction"
        >
          <template #tab-summary>
            <div class="space-y-4">
              <div>
                <h4 class="text-sm font-medium text-gray-900 mb-2">Logginformation</h4>
                <div class="space-y-2">
                  <div class="flex items-center justify-between">
                    <span class="text-sm text-gray-600">Nivå:</span>
                    <span
                      class="inline-flex px-2 py-1 text-xs font-medium rounded-full"
                      :class="getLogLevelColor(selectedLog.level)"
                    >
                      {{ selectedLog.level.toUpperCase() }}
                    </span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-sm text-gray-600">Modul:</span>
                    <span class="text-sm">{{ selectedLog.module }}</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-sm text-gray-600">Åtgärd:</span>
                    <span class="text-sm">{{ selectedLog.action }}</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-sm text-gray-600">Skapad:</span>
                    <span class="text-sm">{{ formatDateTime(selectedLog.created_at) }}</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-sm text-gray-600">Expirerar:</span>
                    <span class="text-sm">{{ formatDateTime(selectedLog.expires_at) }}</span>
                  </div>
                </div>
              </div>

              <div v-if="selectedLog.custom_msg">
                <h4 class="text-sm font-medium text-gray-900 mb-2">Meddelande</h4>
                <p class="text-sm text-gray-600">{{ selectedLog.custom_msg }}</p>
              </div>
            </div>
          </template>

          <template #tab-context>
            <div class="space-y-4">
              <div>
                <h4 class="text-sm font-medium text-gray-900 mb-2">Kontext</h4>
                <div class="bg-gray-50 rounded-lg p-4">
                  <pre class="text-xs text-gray-600 whitespace-pre-wrap">{{ JSON.stringify({
                    id: selectedLog.id,
                    level: selectedLog.level,
                    module: selectedLog.module,
                    action: selectedLog.action,
                    timestamp: selectedLog.created_at
                  }, null, 2) }}</pre>
                </div>
              </div>
            </div>
          </template>

          <template #tab-related>
            <div class="space-y-4">
              <div>
                <h4 class="text-sm font-medium text-gray-900 mb-2">Relaterade entiteter</h4>
                <div class="space-y-3">
                  <button
                    @click="viewRelatedCustomer"
                    class="w-full px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
                  >
                    Visa relaterad kund
                  </button>
                  <button
                    @click="viewRelatedOrder"
                    class="w-full px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
                  >
                    Visa relaterad order
                  </button>
                  <button
                    @click="viewRelatedSync"
                    class="w-full px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
                  >
                    Visa relaterad synk
                  </button>
                </div>
              </div>
            </div>
          </template>
        </DetailDrawer>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { DocumentTextIcon } from '@heroicons/vue/24/outline'
import FilterBar from '@/components/ui/FilterBar.vue'
import DetailDrawer from '@/components/ui/DetailDrawer.vue'
import type { LogEntry } from '@/types/domain'

const loading = ref(false)
const loadingMore = ref(false)
const autoRefresh = ref(false)
const selectedLog = ref<LogEntry | null>(null)
const filters = ref<Record<string, any>>({})
const logs = ref<LogEntry[]>([])
const hasMoreLogs = ref(true)

const selectFilters = [
  {
    key: 'level',
    label: 'Nivå',
    options: [
      { value: 'debug', label: 'Debug' },
      { value: 'info', label: 'Info' },
      { value: 'warn', label: 'Varning' },
      { value: 'error', label: 'Fel' }
    ]
  },
  {
    key: 'module',
    label: 'Modul',
    options: [
      { value: 'sync', label: 'Sync' },
      { value: 'email', label: 'Email' },
      { value: 'order', label: 'Order' },
      { value: 'customer', label: 'Customer' },
      { value: 'api', label: 'API' }
    ]
  }
]

const logTabs = [
  { key: 'summary', label: 'Sammanfattning', icon: 'div' },
  { key: 'context', label: 'Kontext', icon: 'div' },
  { key: 'related', label: 'Relaterat', icon: 'div' }
]

const logActions = [
  { key: 'export', label: 'Exportera', variant: 'primary' as const }
]

const filteredLogs = computed(() => {
  let filtered = logs.value

  if (filters.value.search) {
    const search = filters.value.search.toLowerCase()
    filtered = filtered.filter(log => 
      log.module.toLowerCase().includes(search) ||
      log.action.toLowerCase().includes(search) ||
      log.custom_msg?.toLowerCase().includes(search)
    )
  }

  if (filters.value.level) {
    filtered = filtered.filter(log => log.level === filters.value.level)
  }

  if (filters.value.module) {
    filtered = filtered.filter(log => log.module === filters.value.module)
  }

  if (filters.value.dateFrom) {
    filtered = filtered.filter(log => new Date(log.created_at) >= new Date(filters.value.dateFrom))
  }

  if (filters.value.dateTo) {
    filtered = filtered.filter(log => new Date(log.created_at) <= new Date(filters.value.dateTo))
  }

  return filtered.sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
})

function handleFilterChange(newFilters: Record<string, any>) {
  filters.value = newFilters
}

function clearAllFilters() {
  filters.value = {}
}

function getLogLevelColor(level: string): string {
  const colors = {
    debug: 'bg-gray-400',
    info: 'bg-blue-400',
    warn: 'bg-yellow-400',
    error: 'bg-red-400'
  }
  return colors[level as keyof typeof colors] || 'bg-gray-400'
}

function formatTime(dateString: string): string {
  const date = new Date(dateString)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffMins = Math.floor(diffMs / 60000)
  
  if (diffMins < 1) return 'Nu'
  if (diffMins < 60) return `${diffMins}m sedan`
  if (diffMins < 1440) return `${Math.floor(diffMins / 60)}h sedan`
  return date.toLocaleDateString('sv-SE')
}

function formatDateTime(dateString: string): string {
  return new Date(dateString).toLocaleString('sv-SE')
}

function openLogDetail(log: LogEntry) {
  selectedLog.value = log
}

function closeLogDetail() {
  selectedLog.value = null
}

function handleLogAction(action: string) {
  if (action === 'export') {
    console.log('Export log:', selectedLog.value)
  }
}

function viewRelatedEntity(log: LogEntry) {
  // Navigate to related entity based on log context
  console.log('View related entity for log:', log)
}

function viewRelatedCustomer() {
  console.log('View related customer')
}

function viewRelatedOrder() {
  console.log('View related order')
}

function viewRelatedSync() {
  console.log('View related sync')
}

function refreshLogs() {
  loading.value = true
  // Simulate API call
  setTimeout(() => {
    loadMockLogs()
    loading.value = false
  }, 1000)
}

function loadMoreLogs() {
  loadingMore.value = true
  // Simulate loading more logs
  setTimeout(() => {
    const newLogs = generateMockLogs(10)
    logs.value.push(...newLogs)
    loadingMore.value = false
    hasMoreLogs.value = logs.value.length < 100 // Mock limit
  }, 1000)
}

function toggleAutoRefresh() {
  autoRefresh.value = !autoRefresh.value
  if (autoRefresh.value) {
    const interval = setInterval(() => {
      if (!autoRefresh.value) {
        clearInterval(interval)
        return
      }
      refreshLogs()
    }, 30000) // Refresh every 30 seconds
  }
}

function exportLogs() {
  console.log('Exporting logs')
}

function loadMockLogs() {
  logs.value = generateMockLogs(20)
}

function generateMockLogs(count: number): LogEntry[] {
  const levels = ['debug', 'info', 'warn', 'error']
  const modules = ['sync', 'email', 'order', 'customer', 'api']
  const actions = ['create', 'update', 'delete', 'import', 'export', 'validate', 'process']
  const messages = [
    'Operation completed successfully',
    'Validation failed',
    'Import started',
    'Export completed',
    'Customer created',
    'Order updated',
    'Sync in progress',
    'Email sent',
    'API call failed',
    'Data processed'
  ]

  return Array.from({ length: count }, (_, i) => ({
    id: logs.value.length + i + 1,
    created_at: new Date(Date.now() - Math.random() * 7 * 24 * 60 * 60 * 1000).toISOString(),
    expires_at: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString(),
    level: levels[Math.floor(Math.random() * levels.length)] as any,
    module: modules[Math.floor(Math.random() * modules.length)],
    action: actions[Math.floor(Math.random() * actions.length)],
    custom_msg: Math.random() > 0.3 ? messages[Math.floor(Math.random() * messages.length)] : null
  }))
}

// Load initial logs
onMounted(() => {
  loadMockLogs()
})
</script>
