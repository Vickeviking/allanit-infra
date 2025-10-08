<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-gray-900">Sync</h1>
        <p class="text-gray-600">Hantera datasynkronisering och import/export</p>
      </div>
      <div class="flex items-center space-x-3">
        <button
          @click="pauseSync"
          :disabled="!isRunning"
          class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50"
        >
          Pausa
        </button>
        <button
          @click="startSync"
          :disabled="isRunning"
          class="px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700 disabled:opacity-50"
        >
          {{ isRunning ? 'Körs...' : 'Starta Sync' }}
        </button>
      </div>
    </div>

    <!-- Active Sync Status -->
    <div v-if="activeSync" class="bg-white rounded-lg shadow-sm border p-6">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-lg font-medium text-gray-900">Aktiv synk</h3>
        <span
          class="inline-flex px-2 py-1 text-xs font-medium rounded-full"
          :class="getSyncStatusColor(activeSync.status)"
        >
          {{ getSyncStatusLabel(activeSync.status) }}
        </span>
      </div>
      
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <span class="text-sm text-gray-600">Progress</span>
          <span class="text-sm font-medium">{{ activeSync.progress }}%</span>
        </div>
        <div class="w-full bg-gray-200 rounded-full h-2">
          <div
            class="bg-blue-600 h-2 rounded-full transition-all duration-300"
            :style="{ width: `${activeSync.progress}%` }"
          ></div>
        </div>
        
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div class="text-center">
            <p class="text-lg font-semibold text-gray-900">{{ activeSync.processed }}</p>
            <p class="text-xs text-gray-600">Bearbetade</p>
          </div>
          <div class="text-center">
            <p class="text-lg font-semibold text-gray-900">{{ activeSync.total }}</p>
            <p class="text-xs text-gray-600">Totalt</p>
          </div>
          <div class="text-center">
            <p class="text-lg font-semibold text-gray-900">{{ activeSync.errors }}</p>
            <p class="text-xs text-gray-600">Fel</p>
          </div>
          <div class="text-center">
            <p class="text-lg font-semibold text-gray-900">{{ formatDuration(activeSync.duration) }}</p>
            <p class="text-xs text-gray-600">Varaktighet</p>
          </div>
        </div>
        
        <div class="flex items-center justify-between text-sm text-gray-600">
          <span>Startad: {{ formatDateTime(activeSync.startedAt) }}</span>
          <span v-if="activeSync.estimatedCompletion">
            Beräknad sluttid: {{ formatDateTime(activeSync.estimatedCompletion) }}
          </span>
        </div>
      </div>
    </div>

    <!-- Sync History -->
    <div class="bg-white rounded-lg shadow-sm border">
      <div class="px-6 py-4 border-b border-gray-200">
        <h3 class="text-lg font-medium text-gray-900">Synkhistorik</h3>
      </div>
      
      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Status
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Typ
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Poster
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Varaktighet
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Startad
              </th>
              <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Åtgärder
              </th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            <tr
              v-for="sync in syncHistory"
              :key="sync.id"
              class="hover:bg-gray-50"
            >
              <td class="px-6 py-4 whitespace-nowrap">
                <span
                  class="inline-flex px-2 py-1 text-xs font-medium rounded-full"
                  :class="getSyncStatusColor(sync.status)"
                >
                  {{ getSyncStatusLabel(sync.status) }}
                </span>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                {{ sync.type }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                {{ sync.processed }}/{{ sync.total }}
                <span v-if="sync.errors > 0" class="text-red-600 ml-1">
                  ({{ sync.errors }} fel)
                </span>
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                {{ formatDuration(sync.duration) }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                {{ formatDateTime(sync.startedAt) }}
              </td>
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                <button
                  @click="viewSyncDetails(sync)"
                  class="text-blue-600 hover:text-blue-800"
                >
                  Visa detaljer
                </button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- Sync Configuration -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- Schedule -->
      <div class="bg-white rounded-lg shadow-sm border p-6">
        <h3 class="text-lg font-medium text-gray-900 mb-4">Schemaläggning</h3>
        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Automatisk synk</label>
            <div class="flex items-center space-x-3">
              <input
                v-model="syncConfig.autoSync"
                type="checkbox"
                class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
              />
              <span class="text-sm text-gray-600">Aktivera automatisk synk</span>
            </div>
          </div>
          
          <div v-if="syncConfig.autoSync">
            <label class="block text-sm font-medium text-gray-700 mb-2">Intervall</label>
            <select
              v-model="syncConfig.interval"
              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
            >
              <option value="15">Var 15:e minut</option>
              <option value="30">Var 30:e minut</option>
              <option value="60">Varje timme</option>
              <option value="240">Var 4:e timme</option>
              <option value="1440">Dagligen</option>
            </select>
          </div>
          
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Nästa synk</label>
            <p class="text-sm text-gray-600">{{ nextSyncTime }}</p>
          </div>
        </div>
      </div>

      <!-- Settings -->
      <div class="bg-white rounded-lg shadow-sm border p-6">
        <h3 class="text-lg font-medium text-gray-900 mb-4">Inställningar</h3>
        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Batch-storlek</label>
            <input
              v-model="syncConfig.batchSize"
              type="number"
              min="1"
              max="1000"
              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
            />
          </div>
          
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Timeout (sekunder)</label>
            <input
              v-model="syncConfig.timeout"
              type="number"
              min="30"
              max="3600"
              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
            />
          </div>
          
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Max försök</label>
            <input
              v-model="syncConfig.maxRetries"
              type="number"
              min="1"
              max="10"
              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
            />
          </div>
          
          <button
            @click="saveSyncConfig"
            class="w-full px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700"
          >
            Spara inställningar
          </button>
        </div>
      </div>
    </div>

    <!-- Sync Details Modal -->
    <Modal
      v-if="selectedSync"
      title="Synkdetaljer"
      @close="closeSyncDetails"
      :show-actions="false"
    >
      <div class="space-y-4">
        <div class="grid grid-cols-2 gap-4">
          <div>
            <h4 class="text-sm font-medium text-gray-900 mb-2">Status</h4>
            <span
              class="inline-flex px-2 py-1 text-xs font-medium rounded-full"
              :class="getSyncStatusColor(selectedSync.status)"
            >
              {{ getSyncStatusLabel(selectedSync.status) }}
            </span>
          </div>
          <div>
            <h4 class="text-sm font-medium text-gray-900 mb-2">Typ</h4>
            <p class="text-sm text-gray-600">{{ selectedSync.type }}</p>
          </div>
        </div>
        
        <div>
          <h4 class="text-sm font-medium text-gray-900 mb-2">Statistik</h4>
          <div class="grid grid-cols-3 gap-4">
            <div class="text-center p-3 bg-gray-50 rounded">
              <p class="text-lg font-semibold text-gray-900">{{ selectedSync.processed }}</p>
              <p class="text-xs text-gray-600">Bearbetade</p>
            </div>
            <div class="text-center p-3 bg-gray-50 rounded">
              <p class="text-lg font-semibold text-gray-900">{{ selectedSync.total }}</p>
              <p class="text-xs text-gray-600">Totalt</p>
            </div>
            <div class="text-center p-3 bg-gray-50 rounded">
              <p class="text-lg font-semibold text-gray-900">{{ selectedSync.errors }}</p>
              <p class="text-xs text-gray-600">Fel</p>
            </div>
          </div>
        </div>
        
        <div>
          <h4 class="text-sm font-medium text-gray-900 mb-2">Tidsinformation</h4>
          <div class="space-y-2 text-sm text-gray-600">
            <div class="flex justify-between">
              <span>Startad:</span>
              <span>{{ formatDateTime(selectedSync.startedAt) }}</span>
            </div>
            <div class="flex justify-between">
              <span>Slutad:</span>
              <span>{{ selectedSync.completedAt ? formatDateTime(selectedSync.completedAt) : 'Pågår' }}</span>
            </div>
            <div class="flex justify-between">
              <span>Varaktighet:</span>
              <span>{{ formatDuration(selectedSync.duration) }}</span>
            </div>
          </div>
        </div>
        
        <div v-if="selectedSync.errors > 0">
          <h4 class="text-sm font-medium text-gray-900 mb-2">Fel</h4>
          <div class="bg-red-50 border border-red-200 rounded-lg p-3">
            <p class="text-sm text-red-800">{{ selectedSync.errorMessage || 'Okänt fel' }}</p>
          </div>
        </div>
      </div>
    </Modal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import Modal from '@/components/ui/Modal.vue'

interface SyncJob {
  id: number
  status: 'running' | 'completed' | 'failed' | 'paused'
  type: string
  processed: number
  total: number
  errors: number
  duration: number
  startedAt: string
  completedAt?: string
  errorMessage?: string
}

const isRunning = ref(false)
const activeSync = ref<SyncJob | null>(null)
const selectedSync = ref<SyncJob | null>(null)

const syncConfig = ref({
  autoSync: true,
  interval: 60,
  batchSize: 100,
  timeout: 300,
  maxRetries: 3
})

const syncHistory = ref<SyncJob[]>([
  {
    id: 1,
    status: 'completed',
    type: 'Customer Import',
    processed: 45,
    total: 45,
    errors: 0,
    duration: 120,
    startedAt: '2025-01-08T10:00:00Z',
    completedAt: '2025-01-08T10:02:00Z'
  },
  {
    id: 2,
    status: 'completed',
    type: 'Order Export',
    processed: 23,
    total: 23,
    errors: 1,
    duration: 45,
    startedAt: '2025-01-08T09:30:00Z',
    completedAt: '2025-01-08T09:30:45Z',
    errorMessage: 'Order PO-203 export failed'
  },
  {
    id: 3,
    status: 'failed',
    type: 'Email Sync',
    processed: 12,
    total: 50,
    errors: 3,
    duration: 180,
    startedAt: '2025-01-08T08:00:00Z',
    completedAt: '2025-01-08T08:03:00Z',
    errorMessage: 'SMTP server timeout'
  }
])

const nextSyncTime = computed(() => {
  if (!syncConfig.value.autoSync) return 'Schemalagd synk inaktiverad'
  
  const now = new Date()
  const nextSync = new Date(now.getTime() + syncConfig.value.interval * 60 * 1000)
  return nextSync.toLocaleString('sv-SE')
})

function getSyncStatusColor(status: string): string {
  const colors = {
    running: 'bg-blue-100 text-blue-800',
    completed: 'bg-green-100 text-green-800',
    failed: 'bg-red-100 text-red-800',
    paused: 'bg-yellow-100 text-yellow-800'
  }
  return colors[status as keyof typeof colors] || 'bg-gray-100 text-gray-800'
}

function getSyncStatusLabel(status: string): string {
  const labels = {
    running: 'Körs',
    completed: 'Slutförd',
    failed: 'Misslyckades',
    paused: 'Pausad'
  }
  return labels[status as keyof typeof labels] || status
}

function formatDuration(seconds: number): string {
  const minutes = Math.floor(seconds / 60)
  const remainingSeconds = seconds % 60
  
  if (minutes > 0) {
    return `${minutes}m ${remainingSeconds}s`
  }
  return `${remainingSeconds}s`
}

function formatDateTime(dateString: string): string {
  return new Date(dateString).toLocaleString('sv-SE')
}

function startSync() {
  isRunning.value = true
  activeSync.value = {
    id: Date.now(),
    status: 'running',
    type: 'Full Sync',
    processed: 0,
    total: 100,
    errors: 0,
    duration: 0,
    startedAt: new Date().toISOString(),
    estimatedCompletion: new Date(Date.now() + 5 * 60 * 1000).toISOString()
  }
  
  // Simulate sync progress
  const interval = setInterval(() => {
    if (!activeSync.value) {
      clearInterval(interval)
      return
    }
    
    activeSync.value.processed += Math.floor(Math.random() * 5) + 1
    activeSync.value.duration += 1
    
    if (activeSync.value.processed >= activeSync.value.total) {
      activeSync.value.status = 'completed'
      activeSync.value.completedAt = new Date().toISOString()
      isRunning.value = false
      clearInterval(interval)
      
      // Add to history
      syncHistory.value.unshift(activeSync.value)
      activeSync.value = null
    }
  }, 1000)
}

function pauseSync() {
  if (activeSync.value) {
    activeSync.value.status = 'paused'
    isRunning.value = false
  }
}

function viewSyncDetails(sync: SyncJob) {
  selectedSync.value = sync
}

function closeSyncDetails() {
  selectedSync.value = null
}

function saveSyncConfig() {
  console.log('Saving sync config:', syncConfig.value)
  // Simulate save
  setTimeout(() => {
    console.log('Sync config saved')
  }, 1000)
}
</script>
