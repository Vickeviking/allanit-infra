<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-gray-900">Dead Letters</h1>
        <p class="text-gray-600">Hantera misslyckade meddelanden och fel</p>
      </div>
      <div class="flex items-center space-x-3">
        <button
          @click="refreshDeadLetters"
          :disabled="loading"
          class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50"
        >
          {{ loading ? 'Laddar...' : 'Uppdatera' }}
        </button>
        <button
          v-if="selectedDeadLetters.length > 0"
          @click="reprocessSelected"
          :disabled="reprocessing"
          class="px-4 py-2 text-sm font-medium text-white bg-orange-600 border border-transparent rounded-md hover:bg-orange-700 disabled:opacity-50"
        >
          {{ reprocessing ? 'Reprocessar...' : `Reprocessa ${selectedDeadLetters.length} valda` }}
        </button>
      </div>
    </div>

    <!-- Summary Cards -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-6">
      <div class="bg-white rounded-lg shadow-sm border p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <ExclamationTriangleIcon class="w-8 h-8 text-red-500" />
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-600">Totalt fel</p>
            <p class="text-2xl font-semibold text-gray-900">{{ deadLetters.length }}</p>
          </div>
        </div>
      </div>

      <div class="bg-white rounded-lg shadow-sm border p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <ClockIcon class="w-8 h-8 text-yellow-500" />
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-600">Väntar på reprocess</p>
            <p class="text-2xl font-semibold text-gray-900">{{ pendingReprocess }}</p>
          </div>
        </div>
      </div>

      <div class="bg-white rounded-lg shadow-sm border p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <CheckCircleIcon class="w-8 h-8 text-green-500" />
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-600">Reprocessade idag</p>
            <p class="text-2xl font-semibold text-gray-900">{{ reprocessedToday }}</p>
          </div>
        </div>
      </div>

      <div class="bg-white rounded-lg shadow-sm border p-6">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <XCircleIcon class="w-8 h-8 text-gray-500" />
          </div>
          <div class="ml-4">
            <p class="text-sm font-medium text-gray-600">Permanent fel</p>
            <p class="text-2xl font-semibold text-gray-900">{{ permanentFailures }}</p>
          </div>
        </div>
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

    <!-- Dead Letters Table -->
    <DataTable
      title="Dead Letters"
      :items="filteredDeadLetters"
      :columns="columns"
      :loading="loading"
      :error="error"
      selectable
      :selected-items="selectedDeadLetters"
      show-filters
      @select-all="handleSelectAll"
      @select-item="handleSelectItem"
      @row-click="openDeadLetterDetail"
      @sort="handleSort"
    >
      <template #filters>
        <div class="flex items-center space-x-4">
          <div class="text-sm text-gray-600">
            {{ filteredDeadLetters.length }} av {{ deadLetters.length }} fel
          </div>
        </div>
      </template>

      <template #cell-error-type="{ item }">
        <div class="flex items-center space-x-2">
          <ExclamationTriangleIcon class="w-4 h-4 text-red-500" />
          <span class="text-sm font-medium text-gray-900">{{ item.errorType }}</span>
        </div>
      </template>

      <template #cell-source="{ item }">
        <span class="text-sm text-gray-600">{{ item.source }}</span>
      </template>

      <template #cell-attempts="{ item }">
        <div class="flex items-center space-x-2">
          <span class="text-sm">{{ item.attempts }}</span>
          <span
            class="inline-flex px-2 py-1 text-xs font-medium rounded-full"
            :class="getAttemptStatusColor(item.attempts)"
          >
            {{ getAttemptStatus(item.attempts) }}
          </span>
        </div>
      </template>

      <template #cell-last-error="{ item }">
        <div>
          <p class="text-sm text-gray-900">{{ item.lastError }}</p>
          <p class="text-xs text-gray-500">{{ formatTime(item.lastErrorAt) }}</p>
        </div>
      </template>

      <template #header-actions>
        <div v-if="selectedDeadLetters.length > 0" class="flex items-center space-x-2">
          <span class="text-sm text-gray-600">{{ selectedDeadLetters.length }} valda</span>
          <button
            @click="reprocessSelected"
            :disabled="reprocessing"
            class="px-3 py-1 text-sm bg-orange-600 text-white rounded hover:bg-orange-700 disabled:opacity-50"
          >
            Reprocessa valda
          </button>
        </div>
      </template>

      <template #empty-actions>
        <div class="text-center">
          <CheckCircleIcon class="w-12 h-12 text-green-500 mx-auto mb-4" />
          <h3 class="text-lg font-medium text-gray-900 mb-2">Inga dead letters</h3>
          <p class="text-gray-600">Alla meddelanden har bearbetats framgångsrikt!</p>
        </div>
      </template>
    </DataTable>

    <!-- Dead Letter Detail Drawer -->
    <div
      v-if="selectedDeadLetter"
      class="fixed inset-0 z-50 overflow-hidden"
    >
      <div class="absolute inset-0 bg-gray-500 bg-opacity-75" @click="closeDeadLetterDetail"></div>
      <div class="absolute right-0 top-0 h-full w-96 bg-white shadow-xl">
        <DetailDrawer
          :title="`Dead Letter ${selectedDeadLetter.id}`"
          :tabs="deadLetterTabs"
          :actions="deadLetterActions"
          @close="closeDeadLetterDetail"
          @action="handleDeadLetterAction"
        >
          <template #tab-summary>
            <div class="space-y-4">
              <div>
                <h4 class="text-sm font-medium text-gray-900 mb-2">Felinformation</h4>
                <div class="space-y-2">
                  <div class="flex items-center justify-between">
                    <span class="text-sm text-gray-600">Feltyp:</span>
                    <span class="text-sm font-medium">{{ selectedDeadLetter.errorType }}</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-sm text-gray-600">Källa:</span>
                    <span class="text-sm">{{ selectedDeadLetter.source }}</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-sm text-gray-600">Försök:</span>
                    <span class="text-sm">{{ selectedDeadLetter.attempts }}</span>
                  </div>
                  <div class="flex items-center justify-between">
                    <span class="text-sm text-gray-600">Senaste fel:</span>
                    <span class="text-sm">{{ formatDateTime(selectedDeadLetter.lastErrorAt) }}</span>
                  </div>
                </div>
              </div>

              <div>
                <h4 class="text-sm font-medium text-gray-900 mb-2">Felmeddelande</h4>
                <div class="bg-red-50 border border-red-200 rounded-lg p-3">
                  <p class="text-sm text-red-800">{{ selectedDeadLetter.lastError }}</p>
                </div>
              </div>
            </div>
          </template>

          <template #tab-root-cause>
            <div class="space-y-4">
              <div>
                <h4 class="text-sm font-medium text-gray-900 mb-2">Root Cause Analysis</h4>
                <div class="space-y-3">
                  <div class="p-3 bg-yellow-50 border border-yellow-200 rounded-lg">
                    <h5 class="text-sm font-medium text-yellow-800 mb-1">Identifierat problem</h5>
                    <p class="text-sm text-yellow-700">{{ getRootCause(selectedDeadLetter.errorType) }}</p>
                  </div>
                  <div class="p-3 bg-blue-50 border border-blue-200 rounded-lg">
                    <h5 class="text-sm font-medium text-blue-800 mb-1">Föreslagen lösning</h5>
                    <p class="text-sm text-blue-700">{{ getSuggestedSolution(selectedDeadLetter.errorType) }}</p>
                  </div>
                </div>
              </div>
            </div>
          </template>

          <template #tab-attempts>
            <div class="space-y-4">
              <div>
                <h4 class="text-sm font-medium text-gray-900 mb-2">Försökshistorik</h4>
                <div class="space-y-3">
                  <div
                    v-for="(attempt, index) in selectedDeadLetter.attemptHistory"
                    :key="index"
                    class="flex items-start space-x-3 p-3 border rounded-lg"
                  >
                    <div
                      class="w-2 h-2 rounded-full mt-2"
                      :class="attempt.success ? 'bg-green-500' : 'bg-red-500'"
                    ></div>
                    <div class="flex-1">
                      <div class="flex items-center justify-between">
                        <span class="text-sm font-medium text-gray-900">Försök {{ index + 1 }}</span>
                        <span class="text-xs text-gray-500">{{ formatDateTime(attempt.timestamp) }}</span>
                      </div>
                      <p class="text-sm text-gray-600 mt-1">{{ attempt.error || 'Framgångsrikt' }}</p>
                    </div>
                  </div>
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
import { ref, computed } from 'vue'
import { 
  ExclamationTriangleIcon, 
  ClockIcon, 
  CheckCircleIcon, 
  XCircleIcon 
} from '@heroicons/vue/24/outline'
import DataTable from '@/components/ui/DataTable.vue'
import FilterBar from '@/components/ui/FilterBar.vue'
import DetailDrawer from '@/components/ui/DetailDrawer.vue'

interface DeadLetter {
  id: number
  errorType: string
  source: string
  attempts: number
  lastError: string
  lastErrorAt: string
  attemptHistory: Array<{
    timestamp: string
    success: boolean
    error?: string
  }>
}

const loading = ref(false)
const reprocessing = ref(false)
const error = ref<string | null>(null)
const selectedDeadLetter = ref<DeadLetter | null>(null)
const selectedDeadLetters = ref<DeadLetter[]>([])
const filters = ref<Record<string, any>>({})

const deadLetters = ref<DeadLetter[]>([
  {
    id: 1,
    errorType: 'Email Delivery Failed',
    source: 'Campaign System',
    attempts: 3,
    lastError: 'SMTP server timeout',
    lastErrorAt: '2025-01-08T14:30:00Z',
    attemptHistory: [
      { timestamp: '2025-01-08T14:30:00Z', success: false, error: 'SMTP server timeout' },
      { timestamp: '2025-01-08T14:25:00Z', success: false, error: 'Connection refused' },
      { timestamp: '2025-01-08T14:20:00Z', success: false, error: 'Invalid recipient' }
    ]
  },
  {
    id: 2,
    errorType: 'Order Export Failed',
    source: 'Order Management',
    attempts: 2,
    lastError: 'BL API authentication failed',
    lastErrorAt: '2025-01-08T13:45:00Z',
    attemptHistory: [
      { timestamp: '2025-01-08T13:45:00Z', success: false, error: 'BL API authentication failed' },
      { timestamp: '2025-01-08T13:40:00Z', success: false, error: 'Invalid order format' }
    ]
  },
  {
    id: 3,
    errorType: 'Customer Sync Failed',
    source: 'Sync Service',
    attempts: 5,
    lastError: 'Database connection timeout',
    lastErrorAt: '2025-01-08T12:15:00Z',
    attemptHistory: [
      { timestamp: '2025-01-08T12:15:00Z', success: false, error: 'Database connection timeout' },
      { timestamp: '2025-01-08T12:10:00Z', success: false, error: 'Database connection timeout' },
      { timestamp: '2025-01-08T12:05:00Z', success: false, error: 'Database connection timeout' },
      { timestamp: '2025-01-08T12:00:00Z', success: false, error: 'Database connection timeout' },
      { timestamp: '2025-01-08T11:55:00Z', success: false, error: 'Database connection timeout' }
    ]
  }
])

const selectFilters = [
  {
    key: 'errorType',
    label: 'Feltyp',
    options: [
      { value: 'Email Delivery Failed', label: 'Email Delivery Failed' },
      { value: 'Order Export Failed', label: 'Order Export Failed' },
      { value: 'Customer Sync Failed', label: 'Customer Sync Failed' },
      { value: 'API Timeout', label: 'API Timeout' }
    ]
  },
  {
    key: 'source',
    label: 'Källa',
    options: [
      { value: 'Campaign System', label: 'Campaign System' },
      { value: 'Order Management', label: 'Order Management' },
      { value: 'Sync Service', label: 'Sync Service' },
      { value: 'Email Service', label: 'Email Service' }
    ]
  }
]

const columns = [
  { key: 'error-type', label: 'Feltyp', sortable: true },
  { key: 'source', label: 'Källa', sortable: true },
  { key: 'attempts', label: 'Försök', sortable: true },
  { key: 'last-error', label: 'Senaste fel', sortable: true },
  { key: 'lastErrorAt', label: 'Senaste fel', sortable: true }
]

const deadLetterTabs = [
  { key: 'summary', label: 'Sammanfattning', icon: 'div' },
  { key: 'root-cause', label: 'Root Cause', icon: 'div' },
  { key: 'attempts', label: 'Försök', icon: 'div' }
]

const deadLetterActions = [
  { key: 'reprocess', label: 'Reprocessa', variant: 'primary' as const },
  { key: 'ignore', label: 'Ignorera', variant: 'secondary' as const }
]

const filteredDeadLetters = computed(() => {
  let filtered = deadLetters.value

  if (filters.value.search) {
    const search = filters.value.search.toLowerCase()
    filtered = filtered.filter(dl => 
      dl.errorType.toLowerCase().includes(search) ||
      dl.source.toLowerCase().includes(search) ||
      dl.lastError.toLowerCase().includes(search)
    )
  }

  if (filters.value.errorType) {
    filtered = filtered.filter(dl => dl.errorType === filters.value.errorType)
  }

  if (filters.value.source) {
    filtered = filtered.filter(dl => dl.source === filters.value.source)
  }

  if (filters.value.dateFrom) {
    filtered = filtered.filter(dl => new Date(dl.lastErrorAt) >= new Date(filters.value.dateFrom))
  }

  if (filters.value.dateTo) {
    filtered = filtered.filter(dl => new Date(dl.lastErrorAt) <= new Date(filters.value.dateTo))
  }

  return filtered.sort((a, b) => new Date(b.lastErrorAt).getTime() - new Date(a.lastErrorAt).getTime())
})

const pendingReprocess = computed(() => 
  deadLetters.value.filter(dl => dl.attempts < 5).length
)

const reprocessedToday = computed(() => 12) // Mock data

const permanentFailures = computed(() => 
  deadLetters.value.filter(dl => dl.attempts >= 5).length
)

function handleFilterChange(newFilters: Record<string, any>) {
  filters.value = newFilters
}

function clearAllFilters() {
  filters.value = {}
}

function handleSelectAll(checked: boolean) {
  if (checked) {
    selectedDeadLetters.value = [...filteredDeadLetters.value]
  } else {
    selectedDeadLetters.value = []
  }
}

function handleSelectItem(item: DeadLetter, checked: boolean) {
  if (checked) {
    selectedDeadLetters.value.push(item)
  } else {
    const index = selectedDeadLetters.value.findIndex(dl => dl.id === item.id)
    if (index > -1) {
      selectedDeadLetters.value.splice(index, 1)
    }
  }
}

function openDeadLetterDetail(deadLetter: DeadLetter) {
  selectedDeadLetter.value = deadLetter
}

function closeDeadLetterDetail() {
  selectedDeadLetter.value = null
}

function handleDeadLetterAction(action: string) {
  if (action === 'reprocess') {
    reprocessDeadLetter(selectedDeadLetter.value!)
  } else if (action === 'ignore') {
    ignoreDeadLetter(selectedDeadLetter.value!)
  }
}

function handleSort(key: string) {
  console.log('Sort by:', key)
}

function getAttemptStatusColor(attempts: number): string {
  if (attempts >= 5) return 'bg-red-100 text-red-800'
  if (attempts >= 3) return 'bg-yellow-100 text-yellow-800'
  return 'bg-green-100 text-green-800'
}

function getAttemptStatus(attempts: number): string {
  if (attempts >= 5) return 'Permanent fel'
  if (attempts >= 3) return 'Många försök'
  return 'Nya försök'
}

function getRootCause(errorType: string): string {
  const causes = {
    'Email Delivery Failed': 'SMTP-server är otillgänglig eller konfigurationen är felaktig',
    'Order Export Failed': 'BL Administration API-nycklar är ogiltiga eller har upphört',
    'Customer Sync Failed': 'Databasanslutning är instabil eller överbelastad',
    'API Timeout': 'Externa API:er svarar för långsamt eller är otillgängliga'
  }
  return causes[errorType as keyof typeof causes] || 'Okänd orsak'
}

function getSuggestedSolution(errorType: string): string {
  const solutions = {
    'Email Delivery Failed': 'Kontrollera SMTP-inställningar och testa anslutningen',
    'Order Export Failed': 'Uppdatera API-nycklar i inställningar och testa anslutningen',
    'Customer Sync Failed': 'Kontrollera databasanslutning och överväg att skala upp',
    'API Timeout': 'Implementera retry-logik med exponentiell backoff'
  }
  return solutions[errorType as keyof typeof solutions] || 'Kontakta systemadministratör'
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

function refreshDeadLetters() {
  loading.value = true
  // Simulate API call
  setTimeout(() => {
    loading.value = false
  }, 1000)
}

function reprocessSelected() {
  reprocessing.value = true
  // Simulate reprocessing
  setTimeout(() => {
    reprocessing.value = false
    selectedDeadLetters.value = []
    console.log('Reprocessed dead letters:', selectedDeadLetters.value)
  }, 2000)
}

function reprocessDeadLetter(deadLetter: DeadLetter) {
  console.log('Reprocessing dead letter:', deadLetter)
  closeDeadLetterDetail()
}

function ignoreDeadLetter(deadLetter: DeadLetter) {
  console.log('Ignoring dead letter:', deadLetter)
  closeDeadLetterDetail()
}
</script>
