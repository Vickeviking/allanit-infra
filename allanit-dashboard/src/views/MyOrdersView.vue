<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <div>
        <h1 class="text-2xl font-bold text-gray-900">Mina Uppdrag</h1>
        <p class="text-gray-600">Hantera dina tilldelade uppdrag</p>
      </div>
      <div class="flex items-center space-x-3">
        <div class="text-sm text-gray-600">
          {{ myOrders.length }} uppdrag totalt
        </div>
      </div>
    </div>

    <!-- Status Summary Cards -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
      <div class="bg-white rounded-lg shadow-sm border p-4">
        <div class="flex items-center">
          <div class="w-3 h-3 bg-blue-500 rounded-full mr-3"></div>
          <div>
            <p class="text-sm font-medium text-gray-900">Planerade</p>
            <p class="text-2xl font-bold text-blue-600">{{ plannedCount }}</p>
          </div>
        </div>
      </div>
      <div class="bg-white rounded-lg shadow-sm border p-4">
        <div class="flex items-center">
          <div class="w-3 h-3 bg-yellow-500 rounded-full mr-3"></div>
          <div>
            <p class="text-sm font-medium text-gray-900">Pågående</p>
            <p class="text-2xl font-bold text-yellow-600">{{ inProgressCount }}</p>
          </div>
        </div>
      </div>
      <div class="bg-white rounded-lg shadow-sm border p-4">
        <div class="flex items-center">
          <div class="w-3 h-3 bg-green-500 rounded-full mr-3"></div>
          <div>
            <p class="text-sm font-medium text-gray-900">Slutförda</p>
            <p class="text-2xl font-bold text-green-600">{{ completedCount }}</p>
          </div>
        </div>
      </div>
      <div class="bg-white rounded-lg shadow-sm border p-4">
        <div class="flex items-center">
          <div class="w-3 h-3 bg-gray-500 rounded-full mr-3"></div>
          <div>
            <p class="text-sm font-medium text-gray-900">Totalt</p>
            <p class="text-2xl font-bold text-gray-600">{{ myOrders.length }}</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Filters -->
    <div class="bg-white rounded-lg shadow-sm border p-4">
      <div class="flex items-center space-x-4">
        <div class="flex items-center space-x-2">
          <label class="text-sm font-medium text-gray-700">Status:</label>
          <select 
            v-model="statusFilter" 
            class="px-3 py-1 border border-gray-300 rounded-md text-sm"
          >
            <option value="">Alla</option>
            <option value="planned">Planerade</option>
            <option value="in_progress">Pågående</option>
            <option value="completed">Slutförda</option>
          </select>
        </div>
        <div class="flex items-center space-x-2">
          <label class="text-sm font-medium text-gray-700">Företag:</label>
          <select 
            v-model="companyFilter" 
            class="px-3 py-1 border border-gray-300 rounded-md text-sm"
          >
            <option value="">Alla</option>
            <option value="allanit">Allanit Service AB</option>
            <option value="industrimålning">Industrimålning Stockholm AB</option>
          </select>
        </div>
        <button
          @click="clearFilters"
          class="px-3 py-1 text-sm text-gray-600 hover:text-gray-800"
        >
          Rensa filter
        </button>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="loading" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div
        v-for="i in 6"
        :key="i"
        class="bg-white rounded-xl shadow-sm border border-gray-200 p-6 animate-pulse"
      >
        <div class="space-y-3">
          <div class="h-4 bg-gray-300 rounded w-3/4"></div>
          <div class="h-3 bg-gray-300 rounded w-1/2"></div>
          <div class="h-3 bg-gray-300 rounded w-1/3"></div>
          <div class="flex items-center space-x-2">
            <div class="w-8 h-8 bg-gray-300 rounded-full"></div>
            <div class="h-3 bg-gray-300 rounded w-1/4"></div>
          </div>
        </div>
      </div>
    </div>

    <!-- Orders Grid -->
    <div v-else class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-4 sm:gap-6">
      <div
        v-for="order in paginatedOrders"
        :key="order.id"
        class="bg-white rounded-xl shadow-sm border border-gray-200 p-6 hover:shadow-lg hover:border-blue-300 transition-all duration-200"
      >
        <!-- Order Header -->
        <div class="flex items-start justify-between mb-4">
          <div class="flex-1 min-w-0">
            <h3 class="text-lg font-semibold text-gray-900 truncate">
              {{ order.external_id }}
            </h3>
            <p class="text-sm text-gray-600 line-clamp-2 mt-1">
              {{ order.description }}
            </p>
          </div>
          <div class="flex items-center space-x-2 ml-4">
            <span 
              class="px-2 py-1 text-xs font-medium rounded-full"
              :class="getCompanyColor(order.company)"
            >
              {{ getCompanyLabel(order.company) }}
            </span>
            <div
              class="w-3 h-3 rounded-full"
              :class="getStatusColor(order.status)"
            ></div>
          </div>
        </div>

        <!-- Customer Info -->
        <div class="mb-4">
          <div class="flex items-center space-x-2">
            <UserGroupIcon class="w-4 h-4 text-gray-400" />
            <span class="text-sm text-gray-600">{{ getCustomerName(order.customer_id) }}</span>
          </div>
        </div>

        <!-- Scheduled Date -->
        <div v-if="order.scheduled_date" class="mb-4">
          <div class="flex items-center space-x-2">
            <CalendarIcon class="w-4 h-4 text-gray-400" />
            <span class="text-sm text-gray-600">Planerat:</span>
            <span class="text-sm font-medium text-gray-900">
              {{ formatDate(order.scheduled_date) }}
            </span>
          </div>
        </div>

        <!-- Supervisor Info -->
        <div v-if="order.supervisor_id" class="mb-4">
          <div class="flex items-center space-x-3">
            <img
              :src="getSupervisor(order.supervisor_id)?.image"
              :alt="getSupervisor(order.supervisor_id)?.name"
              class="w-8 h-8 rounded-full object-cover border-2 border-gray-200"
              @error="handleImageError"
            />
            <div class="flex-1 min-w-0">
              <p class="text-sm text-gray-600">Handledare:</p>
              <p class="text-sm font-medium text-gray-900 truncate">
                {{ getSupervisor(order.supervisor_id)?.name }}
              </p>
            </div>
          </div>
        </div>

        <!-- Amount -->
        <div class="mb-4">
          <div class="flex items-center justify-between">
            <span class="text-sm text-gray-600">Belopp:</span>
            <span class="text-sm font-medium text-gray-900">
              {{ formatCurrency(order.amount) }}
            </span>
          </div>
        </div>

        <!-- Status Change & Comments Section -->
        <div class="pt-4 border-t border-gray-100 space-y-3">
          <!-- Status Dropdown -->
          <div>
            <label class="text-xs font-medium text-gray-700 mb-1 block">Ändra status:</label>
            <select 
              :value="order.status"
              @change="(e) => handleStatusChange(order, e.target.value)"
              class="w-full px-3 py-2 text-sm border border-gray-300 rounded-md"
            >
              <option value="planned">Planerad</option>
              <option value="in_progress">Pågående</option>
              <option value="paused">Pausad</option>
              <option value="waiting_for_materials">Väntar på material</option>
              <option value="blocked">Blockerad</option>
              <option value="completed">Slutförd</option>
              <option value="cancelled">Avbruten</option>
            </select>
          </div>
          
          <!-- Comment Input -->
          <div>
            <label class="text-xs font-medium text-gray-700 mb-1 block">Lägg till kommentar:</label>
            <textarea
              v-model="commentTexts[order.id]"
              placeholder="Skriv en kommentar..."
              class="w-full px-3 py-2 text-sm border border-gray-300 rounded-md"
              rows="2"
            ></textarea>
          </div>
          
          <!-- Submit Button -->
          <button
            @click="submitComment(order)"
            :disabled="!commentTexts[order.id]?.trim()"
            class="w-full px-3 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:bg-gray-300"
          >
            Spara kommentar
          </button>
          
          <!-- View Details Button -->
          <button
            @click="viewOrderDetails(order)"
            class="w-full px-3 py-2 text-sm font-medium text-blue-600 bg-blue-50 border border-blue-200 rounded-md hover:bg-blue-100"
          >
            Visa detaljer & historik
          </button>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-if="!loading && filteredOrders.length === 0" class="text-center py-12">
      <ShoppingBagIcon class="w-12 h-12 text-gray-300 mx-auto mb-4" />
      <h3 class="text-lg font-medium text-gray-900 mb-2">Inga uppdrag</h3>
      <p class="text-gray-600">
        {{ statusFilter || companyFilter ? 'Inga uppdrag matchar dina filter.' : 'Du har inga tilldelade uppdrag än.' }}
      </p>
    </div>

    <!-- Order Detail Modal -->
    <div
      v-if="selectedOrder"
      class="fixed inset-0 z-50 overflow-hidden"
    >
      <div class="absolute inset-0 bg-gray-500 bg-opacity-75" @click="closeOrderDetail"></div>
      <div class="absolute inset-0 flex items-center justify-center p-4">
        <div class="bg-white rounded-2xl shadow-2xl w-full max-w-2xl max-h-[90vh] overflow-hidden">
          <!-- Header -->
          <div class="px-8 py-6 border-b border-gray-200 bg-gradient-to-r from-blue-50 to-indigo-50">
            <div class="flex items-center justify-between">
              <div>
                <h3 class="text-2xl font-bold text-gray-900">{{ selectedOrder.external_id }}</h3>
                <p class="text-lg text-gray-600">{{ selectedOrder.description }}</p>
              </div>
              <button
                @click="closeOrderDetail"
                class="p-3 text-gray-400 hover:text-gray-600 hover:bg-white/80 rounded-xl transition-all duration-200"
              >
                <XMarkIcon class="w-6 h-6" />
              </button>
            </div>
          </div>

          <!-- Content -->
          <div class="flex-1 overflow-y-auto p-8">
            <div class="space-y-6">
              <!-- Order Info -->
              <div class="bg-gradient-to-br from-gray-50 to-blue-50 rounded-xl p-6 border border-gray-200">
                <h4 class="text-lg font-semibold text-gray-900 mb-4 flex items-center">
                  <ShoppingBagIcon class="w-5 h-5 mr-2 text-blue-600" />
                  Uppdragsinformation
                </h4>
                <div class="grid grid-cols-2 gap-4">
                  <div>
                    <span class="text-sm text-gray-600">Status:</span>
                    <span class="ml-2 inline-flex px-2 py-1 text-xs font-medium rounded-full" :class="getStatusBadgeColor(selectedOrder.status)">
                      {{ getStatusLabel(selectedOrder.status) }}
                    </span>
                  </div>
                  <div>
                    <span class="text-sm text-gray-600">Belopp:</span>
                    <span class="ml-2 text-sm font-medium">{{ formatCurrency(selectedOrder.amount) }}</span>
                  </div>
                  <div>
                    <span class="text-sm text-gray-600">Företag:</span>
                    <span class="ml-2 inline-flex px-2 py-1 text-xs font-medium rounded-full" :class="getCompanyColor(selectedOrder.company)">
                      {{ getCompanyLabel(selectedOrder.company) }}
                    </span>
                  </div>
                  <div v-if="selectedOrder.scheduled_date">
                    <span class="text-sm text-gray-600">Planerat:</span>
                    <span class="ml-2 text-sm font-medium">{{ formatDate(selectedOrder.scheduled_date) }}</span>
                  </div>
                </div>
              </div>

              <!-- Customer Info -->
              <div class="bg-gradient-to-br from-green-50 to-emerald-50 rounded-xl p-6 border border-gray-200">
                <h4 class="text-lg font-semibold text-gray-900 mb-4 flex items-center">
                  <UserGroupIcon class="w-5 h-5 mr-2 text-green-600" />
                  Kundinformation
                </h4>
                <div class="space-y-2">
                  <div class="flex items-center space-x-2">
                    <span class="text-sm text-gray-600">Kund:</span>
                    <span class="text-sm font-medium">{{ getCustomerName(selectedOrder.customer_id) }}</span>
                  </div>
                </div>
              </div>

              <!-- Supervisor Info -->
              <div v-if="selectedOrder.supervisor_id" class="bg-gradient-to-br from-purple-50 to-pink-50 rounded-xl p-6 border border-gray-200">
                <h4 class="text-lg font-semibold text-gray-900 mb-4 flex items-center">
                  <UserIcon class="w-5 h-5 mr-2 text-purple-600" />
                  Handledare
                </h4>
                <div class="flex items-center space-x-4">
                  <img
                    :src="getSupervisor(selectedOrder.supervisor_id)?.image"
                    :alt="getSupervisor(selectedOrder.supervisor_id)?.name"
                    class="w-12 h-12 rounded-full object-cover border-4 border-white shadow-lg"
                    @error="handleImageError"
                  />
                  <div>
                    <p class="text-lg font-medium text-gray-900">{{ getSupervisor(selectedOrder.supervisor_id)?.name }}</p>
                    <p class="text-sm text-gray-600">{{ getSupervisor(selectedOrder.supervisor_id)?.role }}</p>
                    <p class="text-sm text-gray-600">{{ getSupervisor(selectedOrder.supervisor_id)?.email }}</p>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Actions -->
          <div class="px-8 py-6 border-t border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <div class="flex space-x-4">
              <button
                @click="closeOrderDetail"
                class="px-6 py-3 text-sm font-semibold text-gray-700 bg-white border border-gray-300 rounded-xl hover:bg-gray-50 hover:border-gray-400 transition-all duration-200 shadow-sm hover:shadow-md"
              >
                Stäng
              </button>
              <button
                v-if="selectedOrder.status === 'in_progress'"
                @click="markAsComplete(selectedOrder)"
                class="px-6 py-3 text-sm font-semibold text-white bg-green-600 border border-transparent rounded-xl hover:bg-green-700 hover:shadow-lg transition-all duration-200 shadow-sm"
              >
                Markera som klar
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Pagination Controls -->
    <div v-if="filteredOrders.length > itemsPerPage" class="flex flex-col sm:flex-row items-center justify-between mt-6 bg-white rounded-lg shadow-sm border p-4 gap-4">
      <div class="text-sm text-gray-600 text-center sm:text-left">
        Visar {{ (currentPage - 1) * itemsPerPage + 1 }}-{{ Math.min(currentPage * itemsPerPage, filteredOrders.length) }} 
        av {{ filteredOrders.length }} uppdrag
      </div>
      
      <div class="flex items-center space-x-2">
        <button 
          @click="currentPage--" 
          :disabled="currentPage === 1"
          class="px-3 py-1 rounded-md border disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-50"
        >
          Föregående
        </button>
        
        <div class="flex space-x-1">
          <button
            v-for="page in totalPages"
            :key="page"
            @click="currentPage = page"
            :class="[
              'px-3 py-1 rounded-md border text-sm',
              currentPage === page 
                ? 'bg-blue-600 text-white border-blue-600' 
                : 'hover:bg-gray-50'
            ]"
          >
            {{ page }}
          </button>
        </div>
        
        <button 
          @click="currentPage++" 
          :disabled="currentPage === totalPages"
          class="px-3 py-1 rounded-md border disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-50"
        >
          Nästa
        </button>
      </div>
    </div>

    <!-- Order Details & Comments Modal -->
    <div v-if="selectedOrderForDetails" class="fixed inset-0 z-50 overflow-hidden">
      <div class="absolute inset-0 bg-gray-500 bg-opacity-75" @click="selectedOrderForDetails = null"></div>
      <div class="absolute inset-0 flex items-center justify-center p-4">
        <div class="bg-white rounded-2xl shadow-2xl w-full max-w-3xl max-h-[90vh] overflow-hidden">
          <!-- Modal Header -->
          <div class="px-6 py-4 border-b border-gray-200">
            <div class="flex items-center justify-between">
              <h3 class="text-lg font-semibold text-gray-900">{{ selectedOrderForDetails.external_id }}</h3>
              <button
                @click="selectedOrderForDetails = null"
                class="text-gray-400 hover:text-gray-600"
              >
                <XMarkIcon class="w-6 h-6" />
              </button>
            </div>
          </div>
          
          <!-- Modal Content -->
          <div class="flex-1 overflow-y-auto p-6">
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
              <!-- Order Details -->
              <div class="space-y-4">
                <h4 class="text-md font-semibold text-gray-900">Uppdragsdetaljer</h4>
                <div class="space-y-2 text-sm">
                  <div class="flex justify-between">
                    <span class="text-gray-600">Beskrivning:</span>
                    <span class="font-medium text-gray-900">{{ selectedOrderForDetails.description }}</span>
                  </div>
                  <div class="flex justify-between">
                    <span class="text-gray-600">Status:</span>
                    <span class="font-medium" :class="getStatusColor(selectedOrderForDetails.status)">
                      {{ getStatusLabel(selectedOrderForDetails.status) }}
                    </span>
                  </div>
                  <div class="flex justify-between">
                    <span class="text-gray-600">Belopp:</span>
                    <span class="font-medium text-gray-900">{{ formatCurrency(selectedOrderForDetails.amount) }}</span>
                  </div>
                  <div v-if="selectedOrderForDetails.scheduled_date" class="flex justify-between">
                    <span class="text-gray-600">Planerat datum:</span>
                    <span class="font-medium text-gray-900">{{ formatDate(selectedOrderForDetails.scheduled_date) }}</span>
                  </div>
                </div>
              </div>
              
              <!-- Comments History -->
              <div class="space-y-4">
                <h4 class="text-md font-semibold text-gray-900">Kommentarer</h4>
                <div class="space-y-3 max-h-64 overflow-y-auto">
                  <div v-for="comment in getOrderComments(selectedOrderForDetails.id)" :key="comment.id"
                       class="p-3 bg-gray-50 rounded-lg">
                    <p class="text-sm text-gray-900">{{ comment.comment }}</p>
                    <p class="text-xs text-gray-500 mt-1">{{ formatDate(comment.createdAt) }}</p>
                  </div>
                  <div v-if="getOrderComments(selectedOrderForDetails.id).length === 0" class="text-center py-4">
                    <p class="text-sm text-gray-500">Inga kommentarer ännu</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { 
  ShoppingBagIcon,
  UserGroupIcon,
  CalendarIcon,
  UserIcon,
  XMarkIcon
} from '@heroicons/vue/24/outline'
import { http } from '@/api/mockClient'
import type { PurchaseOrder } from '@/types/domain'
import employees from '@/mock/employees'
import orderComments from '@/mock/orderComments'

const orders = ref<PurchaseOrder[]>([])
const selectedOrder = ref<PurchaseOrder | null>(null)
const loading = ref(true)
const statusFilter = ref('')
const companyFilter = ref('')
const commentTexts = ref<Record<number, string>>({})
const selectedOrderForDetails = ref<PurchaseOrder | null>(null)
const currentPage = ref(1)
const itemsPerPage = 6

// Get current user's employee ID
const currentUser = computed(() => {
  try {
    const userStr = localStorage.getItem('user')
    return userStr ? JSON.parse(userStr) : null
  } catch {
    return null
  }
})

const myOrders = computed(() => {
  const user = currentUser.value
  if (!user?.employeeId) return []
  
  return orders.value.filter(order => order.assigned_employee_id === user.employeeId)
})

const filteredOrders = computed(() => {
  let filtered = myOrders.value

  if (statusFilter.value) {
    filtered = filtered.filter(order => order.status === statusFilter.value)
  }

  if (companyFilter.value) {
    filtered = filtered.filter(order => order.company === companyFilter.value)
  }

  return filtered
})

const plannedCount = computed(() => 
  myOrders.value.filter(order => order.status === 'planned').length
)

const inProgressCount = computed(() => 
  myOrders.value.filter(order => order.status === 'in_progress').length
)

const completedCount = computed(() => 
  myOrders.value.filter(order => order.status === 'completed').length
)

const totalPages = computed(() => Math.ceil(filteredOrders.value.length / itemsPerPage))

const paginatedOrders = computed(() => {
  const start = (currentPage.value - 1) * itemsPerPage
  return filteredOrders.value.slice(start, start + itemsPerPage)
})

// Reset pagination when filters change
watch([statusFilter, companyFilter], () => {
  currentPage.value = 1
})

async function loadData() {
  try {
    const response = await http.get('/api/purchase-orders')
    orders.value = response.data.results
  } catch (error) {
    console.error('Error loading orders:', error)
  } finally {
    loading.value = false
  }
}

function getCustomerName(customerId: number | null): string {
  if (!customerId) return "Okänd kund"
  // In a real app, you'd fetch customer data
  return `Kund ${customerId}`
}

function getSupervisor(supervisorId: number) {
  return employees.find(emp => emp.id === supervisorId)
}

function getStatusColor(status: string): string {
  const colors = {
    not_planned: 'bg-gray-500',
    planned: 'bg-blue-500',
    in_progress: 'bg-yellow-500',
    completed: 'bg-green-500',
    cancelled: 'bg-red-500'
  }
  return colors[status as keyof typeof colors] || 'bg-gray-500'
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

function clearFilters() {
  statusFilter.value = ''
  companyFilter.value = ''
}

function viewOrderDetails(order: PurchaseOrder) {
  selectedOrder.value = order
}

function closeOrderDetail() {
  selectedOrder.value = null
}

function markAsComplete(order: PurchaseOrder) {
  console.log('Marking order as complete:', order.external_id)
  // In a real app, this would make an API call
  alert(`Uppdrag ${order.external_id} markerat som klart!`)
}

async function handleStatusChange(order: PurchaseOrder, newStatus: string) {
  try {
    await http.patch(`/api/purchase-orders/${order.id}`, { status: newStatus })
    order.status = newStatus
    // Show success message
    alert(`Status uppdaterad till: ${getStatusLabel(newStatus)}`)
  } catch (error) {
    console.error('Error updating status:', error)
    alert('Kunde inte uppdatera status')
  }
}

async function submitComment(order: PurchaseOrder) {
  const comment = commentTexts.value[order.id]?.trim()
  if (!comment) return
  
  const user = currentUser.value
  const newComment = {
    orderId: order.id,
    employeeId: user.employeeId,
    comment: comment,
    createdAt: new Date().toISOString(),
    status: order.status
  }
  
  try {
    await http.post('/api/order-comments', newComment)
    commentTexts.value[order.id] = ''
    alert('Kommentar sparad!')
  } catch (error) {
    console.error('Error saving comment:', error)
    alert('Kunde inte spara kommentar')
  }
}

function getStatusLabel(status: string): string {
  const labels = {
    planned: 'Planerad',
    in_progress: 'Pågående',
    paused: 'Pausad',
    waiting_for_materials: 'Väntar på material',
    blocked: 'Blockerad',
    completed: 'Slutförd',
    cancelled: 'Avbruten'
  }
  return labels[status as keyof typeof labels] || status
}

function getOrderComments(orderId: number) {
  return orderComments.filter(comment => comment.orderId === orderId)
}

onMounted(() => {
  loadData()
})
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
