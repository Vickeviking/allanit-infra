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
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <div
        v-for="subsidiary in subsidiaries"
        :key="subsidiary.id"
        class="bg-white rounded-lg shadow-sm border p-6 hover:shadow-md transition-shadow"
      >
        <div class="flex items-center justify-between mb-4">
          <div class="flex items-center space-x-3">
            <div 
              class="w-4 h-4 rounded-full"
              :style="{ backgroundColor: subsidiary.color }"
            ></div>
            <h3 class="text-lg font-semibold text-gray-900">{{ subsidiary.name }}</h3>
          </div>
          <span class="text-sm text-gray-500">{{ subsidiary.employees }} medarbetare</span>
        </div>
        
        <p class="text-gray-600 mb-4">{{ subsidiary.description }}</p>
        
        <div class="grid grid-cols-3 gap-4">
          <div class="text-center">
            <div class="text-2xl font-bold text-gray-900">{{ subsidiary.customers }}</div>
            <div class="text-sm text-gray-500">Kunder</div>
          </div>
          <div class="text-center">
            <div class="text-2xl font-bold text-gray-900">{{ subsidiary.active_orders }}</div>
            <div class="text-sm text-gray-500">Aktiva uppdrag</div>
          </div>
          <div class="text-center">
            <div class="text-2xl font-bold text-gray-900">{{ formatCurrency(subsidiary.monthly_revenue) }}</div>
            <div class="text-sm text-gray-500">Månadsomsättning</div>
          </div>
        </div>
        
        <div class="mt-4 pt-4 border-t border-gray-200">
          <div class="flex justify-between text-sm text-gray-600">
            <span>Org.nr: {{ subsidiary.org_number }}</span>
            <span>VD: {{ subsidiary.ceo }}</span>
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

onMounted(() => {
  loadData()
})
</script>
