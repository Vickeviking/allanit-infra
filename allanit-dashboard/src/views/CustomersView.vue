<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-gray-900">Kunder</h1>
        <p class="text-gray-600">Hantera kundinformation och relaterade ordrar</p>
      </div>
      <div class="flex items-center space-x-3">
        <button
          @click="exportCustomers"
          class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
        >
          Exportera CSV
        </button>
        <button
          @click="showCreateModal = true"
          class="px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700"
        >
          Skapa kund
        </button>
      </div>
    </div>

    <!-- Filters -->
    <div class="bg-white rounded-lg shadow-sm border p-6">
      <FilterBar
        :show-date-range="false"
        :select-filters="selectFilters"
        :tag-filters="tagFilters"
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

    <!-- Table -->
    <DataTable
      title="Kunder"
      :items="filteredCustomers"
      :columns="columns"
      :loading="customersStore.loading"
      :error="customersStore.error"
      selectable
      :selected-items="selectedCustomers"
      show-filters
      @select-all="handleSelectAll"
      @select-item="handleSelectItem"
      @row-click="openCustomerDetail"
      @sort="handleSort"
    >
      <template #filters>
        <div class="flex items-center space-x-4">
          <div class="text-sm text-gray-600">
            {{ filteredCustomers.length }} av {{ customersStore.items.length }} kunder
          </div>
        </div>
      </template>

      <template #cell-name="{ item }">
        <div class="flex items-center space-x-3">
          <div class="w-8 h-8 bg-gray-300 rounded-full flex items-center justify-center">
            <UserIcon class="w-4 h-4 text-gray-600" />
          </div>
          <div>
            <p class="text-sm font-medium text-gray-900">{{ item.name }}</p>
            <p class="text-xs text-gray-500">{{ item.external_id }}</p>
          </div>
        </div>
      </template>

      <template #cell-contact="{ item }">
        <div>
          <p v-if="item.email" class="text-sm text-gray-900">{{ item.email }}</p>
          <p v-if="item.phone" class="text-xs text-gray-500">{{ item.phone }}</p>
        </div>
      </template>

      <template #cell-last-order="{ item }">
        <span class="text-sm text-gray-600">
          {{ getLastOrderForCustomer(item.id) }}
        </span>
      </template>

      <template #header-actions>
        <div v-if="selectedCustomers.length > 0" class="flex items-center space-x-2">
          <span class="text-sm text-gray-600">{{ selectedCustomers.length }} valda</span>
          <button
            @click="bulkExport"
            class="px-3 py-1 text-sm bg-blue-600 text-white rounded hover:bg-blue-700"
          >
            Exportera valda
          </button>
        </div>
      </template>

      <template #empty-actions>
        <button
          @click="showCreateModal = true"
          class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
        >
          Skapa första kunden
        </button>
      </template>
    </DataTable>

    <!-- Customer Detail Drawer -->
    <div
      v-if="selectedCustomer"
      class="fixed inset-0 z-50 overflow-hidden"
    >
      <div class="absolute inset-0 bg-gray-500 bg-opacity-75" @click="closeCustomerDetail"></div>
      <div class="absolute right-0 top-0 h-full w-96 bg-white shadow-xl">
        <DetailDrawer
          :title="selectedCustomer.name"
          :tabs="customerTabs"
          :actions="customerActions"
          @close="closeCustomerDetail"
          @action="handleCustomerAction"
        >
          <template #tab-summary>
            <div class="space-y-4">
              <div>
                <h4 class="text-sm font-medium text-gray-900 mb-2">Kontaktinformation</h4>
                <div class="space-y-2">
                  <div class="flex items-center space-x-2">
                    <span class="text-sm text-gray-600">Email:</span>
                    <span class="text-sm">{{ selectedCustomer.email || 'Ej angiven' }}</span>
                  </div>
                  <div class="flex items-center space-x-2">
                    <span class="text-sm text-gray-600">Telefon:</span>
                    <span class="text-sm">{{ selectedCustomer.phone || 'Ej angiven' }}</span>
                  </div>
                  <div class="flex items-center space-x-2">
                    <span class="text-sm text-gray-600">Org.nr:</span>
                    <span class="text-sm">{{ selectedCustomer.org_no || 'Ej angivet' }}</span>
                  </div>
                </div>
              </div>

              <div>
                <h4 class="text-sm font-medium text-gray-900 mb-2">Statistik</h4>
                <div class="grid grid-cols-2 gap-4">
                  <div class="text-center p-3 bg-gray-50 rounded">
                    <p class="text-lg font-semibold text-gray-900">{{ getCustomerOrderCount(selectedCustomer.id) }}</p>
                    <p class="text-xs text-gray-600">Totalt ordrar</p>
                  </div>
                  <div class="text-center p-3 bg-gray-50 rounded">
                    <p class="text-lg font-semibold text-gray-900">{{ getCustomerTotalAmount(selectedCustomer.id) }} SEK</p>
                    <p class="text-xs text-gray-600">Totalt belopp</p>
                  </div>
                </div>
              </div>
            </div>
          </template>

          <template #tab-orders>
            <div class="space-y-3">
              <div
                v-for="order in getCustomerOrders(selectedCustomer.id)"
                :key="order.id"
                class="p-3 border rounded-md hover:bg-gray-50 cursor-pointer"
                @click="viewOrder(order)"
              >
                <div class="flex items-center justify-between">
                  <div>
                    <p class="text-sm font-medium text-gray-900">{{ order.external_id }}</p>
                    <p class="text-xs text-gray-600">{{ order.description }}</p>
                  </div>
                  <div class="text-right">
                    <p class="text-sm font-medium text-gray-900">{{ order.amount }} SEK</p>
                    <span
                      class="inline-flex px-2 py-1 text-xs font-medium rounded-full"
                      :class="getStatusColor(order.status)"
                    >
                      {{ order.status }}
                    </span>
                  </div>
                </div>
              </div>
            </div>
          </template>

          <template #tab-tags>
            <div class="space-y-4">
              <TagSelector
                :tags="availableTags"
                v-model:selected-tags="customerTags"
                allow-add
                @tag-add="addCustomerTag"
              />
            </div>
          </template>

          <template #tab-notes>
            <div class="space-y-4">
              <textarea
                v-model="customerNotes"
                placeholder="Lägg till anteckningar om kunden..."
                class="w-full p-3 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
                rows="6"
              ></textarea>
              <button
                @click="saveCustomerNotes"
                class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
              >
                Spara anteckningar
              </button>
            </div>
          </template>
        </DetailDrawer>
      </div>
    </div>

    <!-- Create Customer Modal -->
    <Modal
      v-if="showCreateModal"
      title="Skapa ny kund"
      @close="showCreateModal = false"
      @confirm="createCustomer"
    >
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Namn *</label>
          <input
            v-model="newCustomer.name"
            type="text"
            required
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
          />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Email</label>
          <input
            v-model="newCustomer.email"
            type="email"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
          />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Telefon</label>
          <input
            v-model="newCustomer.phone"
            type="tel"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
          />
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Organisationsnummer</label>
          <input
            v-model="newCustomer.org_no"
            type="text"
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
          />
        </div>
      </div>
    </Modal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { UserIcon } from '@heroicons/vue/24/outline'
import DataTable from '@/components/ui/DataTable.vue'
import FilterBar from '@/components/ui/FilterBar.vue'
import DetailDrawer from '@/components/ui/DetailDrawer.vue'
import Modal from '@/components/ui/Modal.vue'
import TagSelector from '@/components/ui/TagSelector.vue'
import { useCustomers } from '@/stores/customers'
import { usePurchaseOrders } from '@/stores/purchaseOrders'
import type { Customer } from '@/types/domain'

const customersStore = useCustomers()
const ordersStore = usePurchaseOrders()

const selectedCustomer = ref<Customer | null>(null)
const selectedCustomers = ref<Customer[]>([])
const showCreateModal = ref(false)
const filters = ref<Record<string, any>>({})
const customerTags = ref<string[]>([])
const customerNotes = ref('')

const newCustomer = ref({
  name: '',
  email: '',
  phone: '',
  org_no: ''
})

const selectFilters = [
  {
    key: 'activity',
    label: 'Aktivitet',
    options: [
      { value: 'active', label: 'Aktiva' },
      { value: 'inactive', label: 'Inaktiva' },
      { value: 'new', label: 'Nya' }
    ]
  }
]

const tagFilters = [
  {
    key: 'tags',
    label: 'Taggar',
    tags: [
      { value: 'brf', label: 'BRF' },
      { value: 'kommersiell', label: 'Kommersiell' },
      { value: 'vip', label: 'VIP' },
      { value: 'problem', label: 'Problem' }
    ]
  }
]

const columns = [
  { key: 'name', label: 'Namn', sortable: true },
  { key: 'org_no', label: 'Org.nr', sortable: true },
  { key: 'contact', label: 'Kontakt', sortable: false },
  { key: 'last_order', label: 'Senaste order', sortable: true },
  { key: 'created_at', label: 'Skapad', sortable: true }
]

const customerTabs = [
  { key: 'summary', label: 'Sammanfattning', icon: 'div' },
  { key: 'orders', label: 'Relaterade ordrar', icon: 'div' },
  { key: 'tags', label: 'Taggar', icon: 'div' },
  { key: 'notes', label: 'Anteckningar', icon: 'div' }
]

const customerActions = [
  { key: 'edit', label: 'Redigera', variant: 'secondary' as const },
  { key: 'export', label: 'Exportera', variant: 'primary' as const }
]

const availableTags = [
  { value: 'brf', label: 'BRF' },
  { value: 'kommersiell', label: 'Kommersiell' },
  { value: 'vip', label: 'VIP' },
  { value: 'problem', label: 'Problem' },
  { value: 'ny', label: 'Ny' }
]

const filteredCustomers = computed(() => {
  let customers = customersStore.items

  if (filters.value.search) {
    const search = filters.value.search.toLowerCase()
    customers = customers.filter(c => 
      c.name.toLowerCase().includes(search) ||
      c.email?.toLowerCase().includes(search) ||
      c.org_no?.includes(search)
    )
  }

  if (filters.value.activity) {
    // Mock activity filter
    customers = customers.filter(c => {
      const orderCount = getCustomerOrderCount(c.id)
      switch (filters.value.activity) {
        case 'active': return orderCount > 0
        case 'inactive': return orderCount === 0
        case 'new': return isNewCustomer(c.created_at)
        default: return true
      }
    })
  }

  return customers
})

function handleFilterChange(newFilters: Record<string, any>) {
  filters.value = newFilters
}

function clearAllFilters() {
  filters.value = {}
}

function handleSelectAll(checked: boolean) {
  if (checked) {
    selectedCustomers.value = [...filteredCustomers.value]
  } else {
    selectedCustomers.value = []
  }
}

function handleSelectItem(item: Customer, checked: boolean) {
  if (checked) {
    selectedCustomers.value.push(item)
  } else {
    const index = selectedCustomers.value.findIndex(c => c.id === item.id)
    if (index > -1) {
      selectedCustomers.value.splice(index, 1)
    }
  }
}

function openCustomerDetail(customer: Customer) {
  selectedCustomer.value = customer
  // Load customer-specific data
  customerTags.value = ['brf'] // Mock data
  customerNotes.value = 'Anteckningar om kunden...' // Mock data
}

function closeCustomerDetail() {
  selectedCustomer.value = null
}

function handleCustomerAction(action: string) {
  if (action === 'edit') {
    // Open edit modal
    console.log('Edit customer')
  } else if (action === 'export') {
    // Export customer data
    console.log('Export customer')
  }
}

function getCustomerOrderCount(customerId: number): number {
  return ordersStore.items.filter(o => o.customer_id === customerId).length
}

function getCustomerTotalAmount(customerId: number): number {
  return ordersStore.items
    .filter(o => o.customer_id === customerId)
    .reduce((sum, o) => sum + o.amount, 0)
}

function getCustomerOrders(customerId: number) {
  return ordersStore.items.filter(o => o.customer_id === customerId)
}

function getLastOrderForCustomer(customerId: number): string {
  const orders = getCustomerOrders(customerId)
  if (orders.length === 0) return 'Inga ordrar'
  
  const lastOrder = orders.sort((a, b) => 
    new Date(b.created_at).getTime() - new Date(a.created_at).getTime()
  )[0]
  
  return formatDate(lastOrder.created_at)
}

function getStatusColor(status: string): string {
  const colors = {
    open: 'bg-yellow-100 text-yellow-800',
    closed: 'bg-green-100 text-green-800',
    cancelled: 'bg-red-100 text-red-800'
  }
  return colors[status as keyof typeof colors] || 'bg-gray-100 text-gray-800'
}

function isNewCustomer(createdAt: string): boolean {
  const created = new Date(createdAt)
  const now = new Date()
  const diffDays = (now.getTime() - created.getTime()) / (1000 * 60 * 60 * 24)
  return diffDays <= 30
}

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString('sv-SE')
}

function viewOrder(order: any) {
  // Navigate to orders view with this order selected
  console.log('View order:', order)
}

function addCustomerTag(tag: string) {
  if (!customerTags.value.includes(tag)) {
    customerTags.value.push(tag)
  }
}

function saveCustomerNotes() {
  // Save notes to backend
  console.log('Saving notes:', customerNotes.value)
}

function exportCustomers() {
  // Export all customers
  console.log('Exporting customers')
}

function bulkExport() {
  // Export selected customers
  console.log('Bulk export:', selectedCustomers.value)
}

function createCustomer() {
  // Create new customer
  console.log('Creating customer:', newCustomer.value)
  showCreateModal.value = false
  // Reset form
  newCustomer.value = { name: '', email: '', phone: '', org_no: '' }
}

function handleSort(key: string) {
  console.log('Sort by:', key)
}

// Load data on mount
onMounted(async () => {
  await customersStore.fetchAll()
  await ordersStore.fetchAll()
})
</script>
