<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-gray-900">Moderbolags Översikt</h1>
        <p class="text-gray-600">Översikt över Allanit Service och Industrimålning i Stockholm AB</p>
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

    <!-- Subsidiary Overview Cards -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <div 
        v-for="subsidiary in subsidiaries" 
        :key="subsidiary.id"
        class="bg-white rounded-lg shadow-sm border p-6"
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
      </div>
    </div>

    <!-- Quick Stats -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-6">
      <KpiCard
        title="Totalt antal kunder"
        :value="totalCustomers"
        subtitle="Aktiva kunder"
        icon="UserGroupIcon"
        :trend="{ direction: 'up', value: '+2 denna månad' }"
      />
      <KpiCard
        title="Aktiva uppdrag"
        :value="totalActiveOrders"
        subtitle="Pågående projekt"
        icon="ShoppingBagIcon"
        :trend="{ direction: 'up', value: '+1 denna vecka' }"
      />
      <KpiCard
        title="Månadsomsättning"
        :value="formatCurrency(totalRevenue)"
        subtitle="Totalt för båda bolag"
        icon="CurrencyDollarIcon"
        :trend="{ direction: 'up', value: '+12% från förra månaden' }"
      />
      <KpiCard
        title="Medarbetare"
        :value="totalEmployees"
        subtitle="Totalt antal"
        icon="UserIcon"
        :trend="{ direction: 'up', value: 'Stabil' }"
      />
    </div>

    <!-- Recent Activity -->
    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <!-- Recent Orders -->
      <DataCard title="Senaste uppdrag" :loading="loading">
        <div class="space-y-3">
          <div 
            v-for="order in recentOrders" 
            :key="order.id"
            class="flex items-center justify-between p-3 bg-gray-50 rounded-md"
          >
            <div class="flex items-center space-x-3">
              <img 
                :src="getOrderImage(order.description)"
                :alt="order.description"
                class="w-10 h-10 rounded-md object-cover"
              />
              <div>
                <div class="font-medium text-gray-900">{{ order.description }}</div>
                <div class="text-sm text-gray-500">{{ order.customer_name }}</div>
              </div>
            </div>
            <div class="text-right">
              <div class="font-medium text-gray-900">{{ formatCurrency(order.amount) }}</div>
              <div class="text-sm text-gray-500">{{ formatDate(order.created_at) }}</div>
            </div>
          </div>
        </div>
      </DataCard>

      <!-- Recent Emails -->
      <DataCard title="Senaste e-postutskick" :loading="loading">
        <div class="space-y-3">
          <div 
            v-for="email in recentEmails" 
            :key="email.id"
            class="flex items-center justify-between p-3 bg-gray-50 rounded-md"
          >
            <div>
              <div class="font-medium text-gray-900">{{ email.campaign_name }}</div>
              <div class="text-sm text-gray-500">{{ email.template_name }}</div>
            </div>
            <div class="text-right">
              <div class="text-sm text-gray-900">{{ email.recipient_count }} mottagare</div>
              <div class="text-sm text-gray-500">{{ formatDate(email.sent_at) }}</div>
            </div>
          </div>
        </div>
      </DataCard>
    </div>

    <!-- Quick Actions -->
    <div class="bg-white rounded-lg shadow-sm border p-6">
      <h3 class="text-lg font-semibold text-gray-900 mb-4">Snabbåtgärder</h3>
      <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
        <button
          @click="$router.push('/email-management')"
          class="flex flex-col items-center p-4 text-gray-600 hover:text-blue-600 hover:bg-blue-50 rounded-md transition-colors"
        >
          <MegaphoneIcon class="w-8 h-8 mb-2" />
          <span class="text-sm font-medium">Skicka e-post</span>
        </button>
        <button
          @click="$router.push('/invoices')"
          class="flex flex-col items-center p-4 text-gray-600 hover:text-green-600 hover:bg-green-50 rounded-md transition-colors"
        >
          <DocumentTextIcon class="w-8 h-8 mb-2" />
          <span class="text-sm font-medium">Skapa faktura</span>
        </button>
        <button
          @click="$router.push('/employees')"
          class="flex flex-col items-center p-4 text-gray-600 hover:text-purple-600 hover:bg-purple-50 rounded-md transition-colors"
        >
          <UserIcon class="w-8 h-8 mb-2" />
          <span class="text-sm font-medium">Medarbetare</span>
        </button>
        <button
          @click="$router.push('/customers')"
          class="flex flex-col items-center p-4 text-gray-600 hover:text-orange-600 hover:bg-orange-50 rounded-md transition-colors"
        >
          <UserGroupIcon class="w-8 h-8 mb-2" />
          <span class="text-sm font-medium">Kunder</span>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { 
  UserGroupIcon, 
  ShoppingBagIcon, 
  CurrencyDollarIcon,
  UserIcon,
  MegaphoneIcon,
  DocumentTextIcon
} from '@heroicons/vue/24/outline'
import KpiCard from '@/components/ui/KpiCard.vue'
import DataCard from '@/components/ui/DataCard.vue'
import { http } from '@/api/mockClient'

const loading = ref(false)
const subsidiaries = ref<any[]>([])
const customers = ref<any[]>([])
const orders = ref<any[]>([])
const employees = ref<any[]>([])
const sentEmails = ref<any[]>([])

const totalCustomers = computed(() => customers.value.length)
const totalActiveOrders = computed(() => orders.value.filter(o => o.status === 'open' || o.status === 'in_progress').length)
const totalRevenue = computed(() => subsidiaries.value.reduce((sum, s) => sum + s.monthly_revenue, 0))
const totalEmployees = computed(() => subsidiaries.value.reduce((sum, s) => sum + s.employees, 0))

const recentOrders = computed(() => 
  orders.value
    .sort((a, b) => new Date(b.created_at).getTime() - new Date(a.created_at).getTime())
    .slice(0, 5)
)

const recentEmails = computed(() => 
  sentEmails.value
    .sort((a, b) => new Date(b.sent_at).getTime() - new Date(a.sent_at).getTime())
    .slice(0, 5)
)

async function loadData() {
  loading.value = true
  try {
    const [subsidiariesRes, customersRes, ordersRes, employeesRes, emailsRes] = await Promise.all([
      http.get('/api/subsidiaries'),
      http.get('/api/customers'),
      http.get('/api/purchase-orders'),
      http.get('/api/employees'),
      http.get('/api/sent-emails')
    ])
    
    subsidiaries.value = subsidiariesRes.data.results
    customers.value = customersRes.data.results
    orders.value = ordersRes.data.results
    employees.value = employeesRes.data.results
    sentEmails.value = emailsRes.data.results
  } catch (error) {
    console.error('Error loading data:', error)
  } finally {
    loading.value = false
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

function getOrderImage(description: string): string {
  if (description.toLowerCase().includes('snöröjning') || description.toLowerCase().includes('vinter')) {
    return '/src/resources/updrag/Snöröjning.png'
  } else if (description.toLowerCase().includes('miljörum')) {
    return '/src/resources/updrag/Miljörum.png'
  }
  return '/src/resources/Generiskbostadsrätt.png'
}

onMounted(() => {
  loadData()
})
</script>