<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-gray-900">Dotterbolag</h1>
        <p class="text-gray-600">Översikt över Allanit Service och Industrimålning</p>
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

    <!-- Subsidiary Cards -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
      <div
        v-for="subsidiary in subsidiaries"
        :key="subsidiary.id"
        class="bg-white rounded-2xl shadow-sm border border-gray-200 p-8 hover:shadow-lg transition-all duration-200"
      >
        <!-- Header -->
        <div class="flex items-center justify-between mb-6">
          <div class="flex items-center space-x-4">
            <div 
              class="w-6 h-6 rounded-full shadow-sm"
              :style="{ backgroundColor: subsidiary.color }"
            ></div>
            <div>
              <h3 class="text-xl font-bold text-gray-900">{{ subsidiary.name }}</h3>
              <p class="text-sm text-gray-600">{{ subsidiary.location }}</p>
            </div>
          </div>
          <div class="text-right">
            <div class="text-sm text-gray-500">Etablerad</div>
            <div class="text-sm font-medium text-gray-900">{{ formatDate(subsidiary.established) }}</div>
          </div>
        </div>
        
        <p class="text-gray-600 mb-6">{{ subsidiary.description }}</p>
        
        <!-- Stats Grid -->
        <div class="grid grid-cols-2 gap-6 mb-6">
          <div class="text-center p-4 bg-gradient-to-br from-blue-50 to-indigo-50 rounded-xl">
            <div class="text-2xl font-bold text-blue-600">{{ subsidiary.customers }}</div>
            <div class="text-sm text-gray-600 font-medium">Kunder</div>
          </div>
          <div class="text-center p-4 bg-gradient-to-br from-green-50 to-emerald-50 rounded-xl">
            <div class="text-2xl font-bold text-green-600">{{ subsidiary.active_orders }}</div>
            <div class="text-sm text-gray-600 font-medium">Aktiva uppdrag</div>
          </div>
          <div class="text-center p-4 bg-gradient-to-br from-purple-50 to-pink-50 rounded-xl">
            <div class="text-2xl font-bold text-purple-600">{{ subsidiary.employees }}</div>
            <div class="text-sm text-gray-600 font-medium">Medarbetare</div>
          </div>
          <div class="text-center p-4 bg-gradient-to-br from-yellow-50 to-orange-50 rounded-xl">
            <div class="text-2xl font-bold text-yellow-600">{{ formatCurrency(subsidiary.monthly_revenue) }}</div>
            <div class="text-sm text-gray-600 font-medium">Månadsomsättning</div>
          </div>
        </div>

        <!-- Performance Metrics -->
        <div class="mb-6">
          <h4 class="text-sm font-semibold text-gray-900 mb-3">Prestanda</h4>
          <div class="space-y-3">
            <div class="flex items-center justify-between">
              <span class="text-sm text-gray-600">Kundnöjdhet</span>
              <div class="flex items-center space-x-2">
                <div class="w-16 bg-gray-200 rounded-full h-2">
                  <div class="bg-green-500 h-2 rounded-full" :style="{ width: `${subsidiary.performance.customer_satisfaction * 20}%` }"></div>
                </div>
                <span class="text-sm font-medium text-gray-900">{{ subsidiary.performance.customer_satisfaction }}/5</span>
              </div>
            </div>
            <div class="flex items-center justify-between">
              <span class="text-sm text-gray-600">Genomförandegrad</span>
              <div class="flex items-center space-x-2">
                <div class="w-16 bg-gray-200 rounded-full h-2">
                  <div class="bg-blue-500 h-2 rounded-full" :style="{ width: `${subsidiary.performance.completion_rate}%` }"></div>
                </div>
                <span class="text-sm font-medium text-gray-900">{{ subsidiary.performance.completion_rate }}%</span>
              </div>
            </div>
            <div class="flex items-center justify-between">
              <span class="text-sm text-gray-600">Tillväxt</span>
              <div class="flex items-center space-x-2">
                <div class="w-16 bg-gray-200 rounded-full h-2">
                  <div class="bg-purple-500 h-2 rounded-full" :style="{ width: `${subsidiary.performance.growth_rate}%` }"></div>
                </div>
                <span class="text-sm font-medium text-gray-900">{{ subsidiary.performance.growth_rate }}%</span>
              </div>
            </div>
          </div>
        </div>

        <!-- Services -->
        <div class="mb-6">
          <h4 class="text-sm font-semibold text-gray-900 mb-3">Tjänster</h4>
          <div class="flex flex-wrap gap-2">
            <span
              v-for="service in subsidiary.services.slice(0, 4)"
              :key="service"
              class="inline-flex px-3 py-1 text-xs font-medium rounded-full bg-gray-100 text-gray-800"
            >
              {{ service }}
            </span>
            <span
              v-if="subsidiary.services.length > 4"
              class="inline-flex px-3 py-1 text-xs font-medium rounded-full bg-blue-100 text-blue-800"
            >
              +{{ subsidiary.services.length - 4 }} fler
            </span>
          </div>
        </div>

        <!-- Contact Info -->
        <div class="border-t border-gray-200 pt-4">
          <h4 class="text-sm font-semibold text-gray-900 mb-3">Kontakt</h4>
          <div class="space-y-2 text-sm text-gray-600">
            <div class="flex items-center space-x-2">
              <svg class="w-4 h-4 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z"></path>
                <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z"></path>
              </svg>
              <span>{{ subsidiary.contact.email }}</span>
            </div>
            <div class="flex items-center space-x-2">
              <svg class="w-4 h-4 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                <path d="M2 3a1 1 0 011-1h2.153a1 1 0 01.986.836l.74 4.435a1 1 0 01-.01.42l-.66 3.438a1 1 0 01-.5.5l-3.438.66a1 1 0 01-.42-.01L2.836 6.153a1 1 0 01-.836-.986L2 3z"></path>
              </svg>
              <span>{{ subsidiary.contact.phone }}</span>
            </div>
            <div class="flex items-center space-x-2">
              <svg class="w-4 h-4 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M5.05 4.05a7 7 0 119.9 9.9L10 18.9l-4.95-4.95a7 7 0 010-9.9zM10 11a2 2 0 100-4 2 2 0 000 4z" clip-rule="evenodd"></path>
              </svg>
              <span>{{ subsidiary.contact.address }}</span>
            </div>
          </div>
        </div>

        <!-- Recent Activities -->
        <div class="mt-6 border-t border-gray-200 pt-4">
          <h4 class="text-sm font-semibold text-gray-900 mb-3">Senaste aktiviteter</h4>
          <div class="space-y-2">
            <div
              v-for="activity in subsidiary.recent_activities.slice(0, 3)"
              :key="activity.id"
              class="flex items-start space-x-3 text-sm"
            >
              <div class="w-2 h-2 rounded-full mt-2 flex-shrink-0" :class="getActivityColor(activity.type)"></div>
              <div class="flex-1 min-w-0">
                <p class="text-gray-900">{{ activity.description }}</p>
                <p class="text-xs text-gray-500">{{ formatTime(activity.date) }}</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Combined Stats -->
    <div class="bg-white rounded-lg shadow-sm border p-6">
      <h3 class="text-lg font-medium text-gray-900 mb-4">Kombinerad statistik</h3>
      <div class="grid grid-cols-1 md:grid-cols-4 gap-6">
        <div class="text-center">
          <div class="text-2xl font-bold text-gray-900">{{ totalCustomers }}</div>
          <div class="text-sm text-gray-500">Totalt kunder</div>
        </div>
        <div class="text-center">
          <div class="text-2xl font-bold text-gray-900">{{ totalEmployees }}</div>
          <div class="text-sm text-gray-500">Totalt medarbetare</div>
        </div>
        <div class="text-center">
          <div class="text-2xl font-bold text-gray-900">{{ totalActiveOrders }}</div>
          <div class="text-sm text-gray-500">Totalt aktiva uppdrag</div>
        </div>
        <div class="text-center">
          <div class="text-2xl font-bold text-gray-900">{{ formatCurrency(totalRevenue) }}</div>
          <div class="text-sm text-gray-500">Total månadsomsättning</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { http } from '@/api/mockClient'

const subsidiaries = ref<any[]>([])

const totalCustomers = computed(() => 
  subsidiaries.value.reduce((sum, s) => sum + s.customers, 0)
)

const totalEmployees = computed(() => 
  subsidiaries.value.reduce((sum, s) => sum + s.employees, 0)
)

const totalActiveOrders = computed(() => 
  subsidiaries.value.reduce((sum, s) => sum + s.active_orders, 0)
)

const totalRevenue = computed(() => 
  subsidiaries.value.reduce((sum, s) => sum + s.monthly_revenue, 0)
)

async function loadData() {
  try {
    const subsidiariesRes = await http.get('/api/subsidiaries')
    subsidiaries.value = subsidiariesRes.data.results
  } catch (error) {
    console.error('Error loading data:', error)
  }
}

function refreshData() {
  loadData()
}

function formatCurrency(amount: number): string {
  return new Intl.NumberFormat('sv-SE', {
    style: 'currency',
    currency: 'SEK',
    minimumFractionDigits: 0
  }).format(amount)
}

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString('sv-SE')
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

function getActivityColor(type: string): string {
  const colors = {
    order: 'bg-blue-500',
    customer: 'bg-green-500',
    employee: 'bg-purple-500',
    maintenance: 'bg-yellow-500'
  }
  return colors[type as keyof typeof colors] || 'bg-gray-500'
}

onMounted(() => {
  loadData()
})
</script>
