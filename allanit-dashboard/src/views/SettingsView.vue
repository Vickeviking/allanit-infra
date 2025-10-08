<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-gray-900">Inställningar</h1>
        <p class="text-gray-600">Konfigurera systeminställningar och API-anslutningar</p>
      </div>
      <div class="flex items-center space-x-3">
        <button
          @click="testConnections"
          :disabled="testing"
          class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50"
        >
          {{ testing ? 'Testar...' : 'Testa anslutningar' }}
        </button>
        <button
          @click="saveAllSettings"
          :disabled="saving"
          class="px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700 disabled:opacity-50"
        >
          {{ saving ? 'Sparar...' : 'Spara alla' }}
        </button>
      </div>
    </div>

    <!-- Settings Tabs -->
    <div class="bg-white rounded-lg shadow-sm border">
      <div class="border-b border-gray-200">
        <nav class="flex space-x-8 px-6">
          <button
            v-for="tab in settingsTabs"
            :key="tab.key"
            @click="activeTab = tab.key"
            class="py-4 px-1 border-b-2 font-medium text-sm"
            :class="activeTab === tab.key 
              ? 'border-blue-500 text-blue-600' 
              : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'"
          >
            {{ tab.label }}
          </button>
        </nav>
      </div>

      <div class="p-6">
        <!-- Environment Settings -->
        <div v-if="activeTab === 'environment'" class="space-y-6">
          <div>
            <h3 class="text-lg font-medium text-gray-900 mb-4">Miljöinställningar</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-2">Miljö</label>
                <select
                  v-model="settings.environment"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
                >
                  <option value="development">Development</option>
                  <option value="staging">Staging</option>
                  <option value="production">Production</option>
                </select>
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-2">Debug-läge</label>
                <div class="flex items-center space-x-3">
                  <input
                    v-model="settings.debugMode"
                    type="checkbox"
                    class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                  />
                  <span class="text-sm text-gray-600">Aktivera debug-läge</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- API Settings -->
        <div v-if="activeTab === 'api'" class="space-y-6">
          <div>
            <h3 class="text-lg font-medium text-gray-900 mb-4">API-inställningar</h3>
            <div class="space-y-6">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-2">API Base URL</label>
                <input
                  v-model="settings.apiBaseUrl"
                  type="url"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
                />
              </div>
              
              <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-2">API-nyckel</label>
                  <div class="relative">
                    <input
                      v-model="settings.apiKey"
                      :type="showApiKey ? 'text' : 'password'"
                      class="w-full px-3 py-2 pr-10 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
                    />
                    <button
                      @click="showApiKey = !showApiKey"
                      class="absolute inset-y-0 right-0 pr-3 flex items-center"
                    >
                      <EyeIcon v-if="!showApiKey" class="w-5 h-5 text-gray-400" />
                      <EyeSlashIcon v-else class="w-5 h-5 text-gray-400" />
                    </button>
                  </div>
                </div>
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-2">API Secret</label>
                  <div class="relative">
                    <input
                      v-model="settings.apiSecret"
                      :type="showApiSecret ? 'text' : 'password'"
                      class="w-full px-3 py-2 pr-10 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
                    />
                    <button
                      @click="showApiSecret = !showApiSecret"
                      class="absolute inset-y-0 right-0 pr-3 flex items-center"
                    >
                      <EyeIcon v-if="!showApiSecret" class="w-5 h-5 text-gray-400" />
                      <EyeSlashIcon v-else class="w-5 h-5 text-gray-400" />
                    </button>
                  </div>
                </div>
              </div>
              
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-2">Timeout (sekunder)</label>
                <input
                  v-model="settings.apiTimeout"
                  type="number"
                  min="5"
                  max="300"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
                />
              </div>
            </div>
          </div>
        </div>

        <!-- Email Settings -->
        <div v-if="activeTab === 'email'" class="space-y-6">
          <div>
            <h3 class="text-lg font-medium text-gray-900 mb-4">E-postinställningar</h3>
            <div class="space-y-6">
              <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-2">SMTP Server</label>
                  <input
                    v-model="settings.smtpServer"
                    type="text"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
                  />
                </div>
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-2">SMTP Port</label>
                  <input
                    v-model="settings.smtpPort"
                    type="number"
                    min="1"
                    max="65535"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
                  />
                </div>
              </div>
              
              <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-2">SMTP Användarnamn</label>
                  <input
                    v-model="settings.smtpUsername"
                    type="text"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
                  />
                </div>
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-2">SMTP Lösenord</label>
                  <div class="relative">
                    <input
                      v-model="settings.smtpPassword"
                      :type="showSmtpPassword ? 'text' : 'password'"
                      class="w-full px-3 py-2 pr-10 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
                    />
                    <button
                      @click="showSmtpPassword = !showSmtpPassword"
                      class="absolute inset-y-0 right-0 pr-3 flex items-center"
                    >
                      <EyeIcon v-if="!showSmtpPassword" class="w-5 h-5 text-gray-400" />
                      <EyeSlashIcon v-else class="w-5 h-5 text-gray-400" />
                    </button>
                  </div>
                </div>
              </div>
              
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-2">Från-adress</label>
                <input
                  v-model="settings.fromEmail"
                  type="email"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
                />
              </div>
            </div>
          </div>
        </div>

        <!-- Sync Settings -->
        <div v-if="activeTab === 'sync'" class="space-y-6">
          <div>
            <h3 class="text-lg font-medium text-gray-900 mb-4">Synkroniseringsinställningar</h3>
            <div class="space-y-6">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-2">Synkintervall (minuter)</label>
                <input
                  v-model="settings.syncInterval"
                  type="number"
                  min="5"
                  max="1440"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
                />
              </div>
              
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-2">Batch-storlek</label>
                <input
                  v-model="settings.batchSize"
                  type="number"
                  min="1"
                  max="1000"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
                />
              </div>
              
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-2">Max försök</label>
                <input
                  v-model="settings.maxRetries"
                  type="number"
                  min="1"
                  max="10"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
                />
              </div>
            </div>
          </div>
        </div>

        <!-- User Roles -->
        <div v-if="activeTab === 'roles'" class="space-y-6">
          <div>
            <h3 class="text-lg font-medium text-gray-900 mb-4">Användarroller</h3>
            <div class="space-y-4">
              <div
                v-for="role in userRoles"
                :key="role.id"
                class="flex items-center justify-between p-4 border rounded-lg"
              >
                <div>
                  <h4 class="font-medium text-gray-900">{{ role.name }}</h4>
                  <p class="text-sm text-gray-600">{{ role.description }}</p>
                </div>
                <div class="flex items-center space-x-2">
                  <span class="text-sm text-gray-600">{{ role.userCount }} användare</span>
                  <button
                    @click="editRole(role)"
                    class="px-3 py-1 text-sm text-blue-600 hover:text-blue-800"
                  >
                    Redigera
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Connection Status -->
    <div class="bg-white rounded-lg shadow-sm border p-6">
      <h3 class="text-lg font-medium text-gray-900 mb-4">Anslutningsstatus</h3>
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div class="flex items-center space-x-3">
          <div
            class="w-3 h-3 rounded-full"
            :class="connectionStatus.api ? 'bg-green-500' : 'bg-red-500'"
          ></div>
          <span class="text-sm text-gray-600">API</span>
        </div>
        <div class="flex items-center space-x-3">
          <div
            class="w-3 h-3 rounded-full"
            :class="connectionStatus.email ? 'bg-green-500' : 'bg-red-500'"
          ></div>
          <span class="text-sm text-gray-600">E-post</span>
        </div>
        <div class="flex items-center space-x-3">
          <div
            class="w-3 h-3 rounded-full"
            :class="connectionStatus.database ? 'bg-green-500' : 'bg-red-500'"
          ></div>
          <span class="text-sm text-gray-600">Databas</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { EyeIcon, EyeSlashIcon } from '@heroicons/vue/24/outline'

const activeTab = ref('environment')
const testing = ref(false)
const saving = ref(false)
const showApiKey = ref(false)
const showApiSecret = ref(false)
const showSmtpPassword = ref(false)

const settings = ref({
  environment: 'development',
  debugMode: false,
  apiBaseUrl: 'http://localhost:8000',
  apiKey: '',
  apiSecret: '',
  apiTimeout: 30,
  smtpServer: 'smtp.example.com',
  smtpPort: 587,
  smtpUsername: '',
  smtpPassword: '',
  fromEmail: 'noreply@allanit.se',
  syncInterval: 60,
  batchSize: 100,
  maxRetries: 3
})

const connectionStatus = ref({
  api: true,
  email: false,
  database: true
})

const settingsTabs = [
  { key: 'environment', label: 'Miljö' },
  { key: 'api', label: 'API' },
  { key: 'email', label: 'E-post' },
  { key: 'sync', label: 'Synk' },
  { key: 'roles', label: 'Roller' }
]

const userRoles = ref([
  {
    id: 1,
    name: 'Administratör',
    description: 'Fullständig åtkomst till alla funktioner',
    userCount: 2
  },
  {
    id: 2,
    name: 'Operatör',
    description: 'Kan hantera kunder och ordrar',
    userCount: 5
  },
  {
    id: 3,
    name: 'Läsare',
    description: 'Kan endast visa data',
    userCount: 3
  }
])

function testConnections() {
  testing.value = true
  // Simulate connection tests
  setTimeout(() => {
    connectionStatus.value = {
      api: Math.random() > 0.2,
      email: Math.random() > 0.3,
      database: Math.random() > 0.1
    }
    testing.value = false
  }, 2000)
}

function saveAllSettings() {
  saving.value = true
  // Simulate save
  setTimeout(() => {
    saving.value = false
    console.log('Settings saved:', settings.value)
  }, 1000)
}

function editRole(role: any) {
  console.log('Edit role:', role)
}
</script>
