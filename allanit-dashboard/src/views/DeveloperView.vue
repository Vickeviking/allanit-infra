<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-gray-900">Utvecklare</h1>
        <p class="text-gray-600">Utvecklarverktyg och systemövervakning</p>
      </div>
    </div>

    <!-- Developer Tools Grid -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <!-- Logs -->
      <div class="bg-white rounded-lg shadow-sm border p-6">
        <div class="flex items-center justify-between mb-4">
          <h3 class="text-lg font-medium text-gray-900">Systemloggar</h3>
          <span class="text-sm text-gray-500">{{ logs.length }} poster</span>
        </div>
        <p class="text-gray-600 mb-4">Visa systemloggar och felmeddelanden</p>
        <button
          @click="$router.push('/logs')"
          class="w-full px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700"
        >
          Visa loggar
        </button>
      </div>

      <!-- Dead Letters -->
      <div class="bg-white rounded-lg shadow-sm border p-6">
        <div class="flex items-center justify-between mb-4">
          <h3 class="text-lg font-medium text-gray-900">Dead Letters</h3>
          <span class="text-sm text-gray-500">{{ deadLetters.length }} fel</span>
        </div>
        <p class="text-gray-600 mb-4">Hantera misslyckade meddelanden</p>
        <button
          @click="$router.push('/dead-letters')"
          class="w-full px-4 py-2 text-sm font-medium text-white bg-red-600 border border-transparent rounded-md hover:bg-red-700"
        >
          Visa Dead Letters
        </button>
      </div>

      <!-- Sync -->
      <div class="bg-white rounded-lg shadow-sm border p-6">
        <div class="flex items-center justify-between mb-4">
          <h3 class="text-lg font-medium text-gray-900">Synkronisering</h3>
          <span class="text-sm text-gray-500">{{ syncHistory.length }} körningar</span>
        </div>
        <p class="text-gray-600 mb-4">Hantera datasynkronisering</p>
        <button
          @click="$router.push('/sync')"
          class="w-full px-4 py-2 text-sm font-medium text-white bg-green-600 border border-transparent rounded-md hover:bg-green-700"
        >
          Visa Sync
        </button>
      </div>

      <!-- Settings -->
      <div class="bg-white rounded-lg shadow-sm border p-6">
        <div class="flex items-center justify-between mb-4">
          <h3 class="text-lg font-medium text-gray-900">Systeminställningar</h3>
          <span class="text-sm text-gray-500">Konfiguration</span>
        </div>
        <p class="text-gray-600 mb-4">Konfigurera systeminställningar</p>
        <button
          @click="$router.push('/settings')"
          class="w-full px-4 py-2 text-sm font-medium text-white bg-gray-600 border border-transparent rounded-md hover:bg-gray-700"
        >
          Visa inställningar
        </button>
      </div>
    </div>

    <!-- Quick Stats -->
    <div class="bg-white rounded-lg shadow-sm border p-6">
      <h3 class="text-lg font-medium text-gray-900 mb-4">Snabbstatistik</h3>
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <div class="text-center">
          <div class="text-2xl font-bold text-gray-900">{{ errorLogs }}</div>
          <div class="text-sm text-gray-500">Fel i loggar</div>
        </div>
        <div class="text-center">
          <div class="text-2xl font-bold text-gray-900">{{ warningLogs }}</div>
          <div class="text-sm text-gray-500">Varningar</div>
        </div>
        <div class="text-center">
          <div class="text-2xl font-bold text-gray-900">{{ failedSyncs }}</div>
          <div class="text-sm text-gray-500">Misslyckade synkar</div>
        </div>
        <div class="text-center">
          <div class="text-2xl font-bold text-gray-900">{{ pendingDeadLetters }}</div>
          <div class="text-sm text-gray-500">Väntande Dead Letters</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { http } from '@/api/mockClient'

const logs = ref<any[]>([])
const deadLetters = ref<any[]>([])
const syncHistory = ref<any[]>([])

const errorLogs = computed(() => 
  logs.value.filter(log => log.level === 'error').length
)

const warningLogs = computed(() => 
  logs.value.filter(log => log.level === 'warning').length
)

const failedSyncs = computed(() => 
  syncHistory.value.filter(sync => sync.status === 'failed').length
)

const pendingDeadLetters = computed(() => 
  deadLetters.value.filter(letter => letter.status === 'pending').length
)

async function loadData() {
  try {
    const [logsRes, deadLettersRes, syncHistoryRes] = await Promise.all([
      http.get('/api/logs'),
      http.get('/api/dead-letters'),
      http.get('/api/sync/history')
    ])
    
    logs.value = logsRes.data.results
    deadLetters.value = deadLettersRes.data.results
    syncHistory.value = syncHistoryRes.data.results
  } catch (error) {
    console.error('Error loading data:', error)
  }
}

onMounted(() => {
  loadData()
})
</script>
