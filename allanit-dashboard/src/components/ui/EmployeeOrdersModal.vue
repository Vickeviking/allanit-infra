<template>
  <div class="fixed inset-0 z-50 overflow-hidden">
    <div class="absolute inset-0 bg-gray-500 bg-opacity-75" @click="$emit('close')"></div>
    <div class="absolute inset-0 flex items-center justify-center p-4">
      <div class="bg-white rounded-2xl shadow-2xl w-full max-w-4xl max-h-[90vh] overflow-hidden">
        <!-- Header -->
        <div class="px-8 py-6 border-b border-gray-200 bg-gradient-to-r from-blue-50 to-indigo-50">
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-4">
              <img
                :src="employee.image"
                :alt="employee.name"
                class="w-16 h-16 rounded-full object-cover border-4 border-white shadow-lg"
                @error="handleImageError"
              />
              <div>
                <h3 class="text-2xl font-bold text-gray-900">{{ employee.name }}</h3>
                <p class="text-lg text-gray-600">{{ employee.role }}</p>
                <p class="text-sm text-gray-500">{{ employee.subsidiary }}</p>
              </div>
            </div>
            <button
              @click="$emit('close')"
              class="p-3 text-gray-400 hover:text-gray-600 hover:bg-white/80 rounded-xl transition-all duration-200"
            >
              <XMarkIcon class="w-6 h-6" />
            </button>
          </div>
        </div>

        <!-- Content -->
        <div class="flex-1 overflow-y-auto p-8">
          <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
            <!-- Left Column - Employee Stats -->
            <div class="space-y-6">
              <div class="bg-gradient-to-br from-gray-50 to-blue-50 rounded-xl p-6 border border-gray-200">
                <h4 class="text-lg font-semibold text-gray-900 mb-4 flex items-center">
                  <ChartBarIcon class="w-5 h-5 mr-2 text-blue-600" />
                  Statistik
                </h4>
                <div class="space-y-3">
                  <div class="flex justify-between">
                    <span class="text-gray-600">Totalt uppdrag:</span>
                    <span class="font-medium text-gray-900">{{ employeeOrders.length }}</span>
                  </div>
                  <div class="flex justify-between">
                    <span class="text-gray-600">Slutförda:</span>
                    <span class="font-medium text-green-600">{{ completedCount }}</span>
                  </div>
                  <div class="flex justify-between">
                    <span class="text-gray-600">Pågående:</span>
                    <span class="font-medium text-yellow-600">{{ inProgressCount }}</span>
                  </div>
                  <div class="flex justify-between">
                    <span class="text-gray-600">Planerade:</span>
                    <span class="font-medium text-blue-600">{{ plannedCount }}</span>
                  </div>
                </div>
              </div>

              <div class="bg-gradient-to-br from-green-50 to-emerald-50 rounded-xl p-6 border border-gray-200">
                <h4 class="text-lg font-semibold text-gray-900 mb-4 flex items-center">
                  <CurrencyDollarIcon class="w-5 h-5 mr-2 text-green-600" />
                  Omsättning
                </h4>
                <div class="space-y-3">
                  <div class="flex justify-between">
                    <span class="text-gray-600">Denna månad:</span>
                    <span class="font-medium text-gray-900">{{ formatCurrency(monthlyRevenue) }}</span>
                  </div>
                  <div class="flex justify-between">
                    <span class="text-gray-600">Totalt:</span>
                    <span class="font-medium text-gray-900">{{ formatCurrency(totalRevenue) }}</span>
                  </div>
                </div>
              </div>
            </div>

            <!-- Right Column - Orders List -->
            <div class="lg:col-span-2">
              <div class="bg-gradient-to-br from-purple-50 to-pink-50 rounded-xl p-6 border border-gray-200">
                <h4 class="text-lg font-semibold text-gray-900 mb-4 flex items-center">
                  <ShoppingBagIcon class="w-5 h-5 mr-2 text-purple-600" />
                  Uppdrag ({{ employeeOrders.length }})
                </h4>
                <div v-if="employeeOrders.length > 0" class="space-y-3 max-h-96 overflow-y-auto">
                  <div
                    v-for="order in employeeOrders"
                    :key="order.id"
                    @click="$emit('viewOrder', order)"
                    class="p-4 bg-white rounded-lg border border-gray-200 hover:border-blue-300 hover:shadow-sm transition-all duration-200 cursor-pointer"
                  >
                    <div class="flex items-start justify-between">
                      <div class="flex-1 min-w-0">
                        <h5 class="font-medium text-gray-900 text-sm">{{ order.external_id }}</h5>
                        <p class="text-xs text-gray-600 mt-1 line-clamp-2">{{ order.description }}</p>
                        <div class="flex items-center space-x-4 mt-2">
                          <span class="text-xs text-gray-500">{{ getCustomerName(order.customer_id) }}</span>
                          <span class="text-xs text-gray-500">{{ formatCurrency(order.amount) }}</span>
                          <span v-if="order.scheduled_date" class="text-xs text-gray-500">
                            {{ formatDate(order.scheduled_date) }}
                          </span>
                        </div>
                      </div>
                      <div class="flex items-center space-x-2 ml-4">
                        <span 
                          class="px-2 py-1 text-xs font-medium rounded-full"
                          :class="getCompanyColor(order.company)"
                        >
                          {{ getCompanyLabel(order.company) }}
                        </span>
                        <span 
                          class="px-2 py-1 text-xs font-medium rounded-full"
                          :class="getStatusBadgeColor(order.status)"
                        >
                          {{ getStatusLabel(order.status) }}
                        </span>
                      </div>
                    </div>
                  </div>
                </div>
                <div v-else class="text-center py-8">
                  <ShoppingBagIcon class="w-12 h-12 text-gray-300 mx-auto mb-2" />
                  <p class="text-sm text-gray-500">Inga uppdrag ännu</p>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Actions -->
        <div class="px-8 py-6 border-t border-gray-200 bg-gradient-to-r from-gray-50 to-white">
          <div class="flex space-x-4">
            <button
              @click="$emit('close')"
              class="px-6 py-3 text-sm font-semibold text-gray-700 bg-white border border-gray-300 rounded-xl hover:bg-gray-50 hover:border-gray-400 transition-all duration-200 shadow-sm hover:shadow-md"
            >
              Stäng
            </button>
            <button
              @click="$emit('assignOrder', employee)"
              class="px-6 py-3 text-sm font-semibold text-white bg-blue-600 border border-transparent rounded-xl hover:bg-blue-700 hover:shadow-lg transition-all duration-200 shadow-sm"
            >
              Tilldela nytt uppdrag
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { 
  XMarkIcon,
  ChartBarIcon,
  CurrencyDollarIcon,
  ShoppingBagIcon
} from '@heroicons/vue/24/outline'
import type { PurchaseOrder } from '@/types/domain'

interface Props {
  employee: any
  orders: PurchaseOrder[]
}

const props = defineProps<Props>()

defineEmits<{
  close: []
  viewOrder: [order: PurchaseOrder]
  assignOrder: [employee: any]
}>()

const employeeOrders = computed(() => {
  return props.orders.filter(order => order.assigned_employee_id === props.employee.id)
})

const completedCount = computed(() => 
  employeeOrders.value.filter(order => order.status === 'completed').length
)

const inProgressCount = computed(() => 
  employeeOrders.value.filter(order => order.status === 'in_progress').length
)

const plannedCount = computed(() => 
  employeeOrders.value.filter(order => order.status === 'planned').length
)

const monthlyRevenue = computed(() => {
  const currentMonth = new Date().getMonth()
  const currentYear = new Date().getFullYear()
  
  return employeeOrders.value
    .filter(order => {
      const orderDate = new Date(order.completed_date || order.created_at)
      return orderDate.getMonth() === currentMonth && 
             orderDate.getFullYear() === currentYear &&
             order.status === 'completed'
    })
    .reduce((sum, order) => sum + order.amount, 0)
})

const totalRevenue = computed(() => {
  return employeeOrders.value
    .filter(order => order.status === 'completed')
    .reduce((sum, order) => sum + order.amount, 0)
})

function getCustomerName(customerId: number | null): string {
  if (!customerId) return "Okänd kund"
  return `Kund ${customerId}`
}

function getStatusBadgeColor(status: string): string {
  const colors = {
    not_planned: 'bg-gray-100 text-gray-800',
    planned: 'bg-blue-100 text-blue-800',
    in_progress: 'bg-yellow-100 text-yellow-800',
    completed: 'bg-green-100 text-green-800',
    cancelled: 'bg-red-100 text-red-800'
  }
  return colors[status as keyof typeof colors] || 'bg-gray-100 text-gray-800'
}

function getStatusLabel(status: string): string {
  const labels = {
    not_planned: 'Ej planerad',
    planned: 'Planerad',
    in_progress: 'Pågående',
    completed: 'Slutförd',
    cancelled: 'Avbruten'
  }
  return labels[status as keyof typeof labels] || status
}

function getCompanyColor(company: string): string {
  const colors = {
    allanit: 'bg-green-100 text-green-800',
    industrimålning: 'bg-purple-100 text-purple-800'
  }
  return colors[company as keyof typeof colors] || 'bg-gray-100 text-gray-800'
}

function getCompanyLabel(company: string): string {
  const labels = {
    allanit: 'Allanit',
    industrimålning: 'Industrimålning'
  }
  return labels[company as keyof typeof labels] || company
}

function formatCurrency(amount: number): string {
  return new Intl.NumberFormat('sv-SE', {
    style: 'currency',
    currency: 'SEK'
  }).format(amount)
}

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString('sv-SE')
}

function handleImageError(event: Event) {
  const img = event.target as HTMLImageElement
  img.src = '/medarbetare/default.jpg'
}
</script>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  line-clamp: 2;
}
</style>
