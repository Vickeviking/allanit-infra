<template>
  <div class="fixed inset-0 z-50 overflow-hidden">
    <div class="absolute inset-0 bg-gray-500 bg-opacity-75" @click="$emit('close')"></div>
    <div class="absolute inset-0 flex items-center justify-center p-4">
      <div class="bg-white rounded-2xl shadow-2xl w-full max-w-2xl max-h-[90vh] overflow-hidden">
        <!-- Header -->
        <div class="px-8 py-6 border-b border-gray-200 bg-gradient-to-r from-blue-50 to-indigo-50">
          <div class="flex items-center justify-between">
            <div>
              <h3 class="text-2xl font-bold text-gray-900">Tilldela Uppdrag</h3>
              <p class="text-lg text-gray-600">{{ order.external_id }}</p>
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
          <div class="space-y-6">
            <!-- Order Info -->
            <div class="bg-gradient-to-br from-gray-50 to-blue-50 rounded-xl p-6 border border-gray-200">
              <h4 class="text-lg font-semibold text-gray-900 mb-4 flex items-center">
                <ShoppingBagIcon class="w-5 h-5 mr-2 text-blue-600" />
                Uppdragsinformation
              </h4>
              <div class="space-y-2">
                <p class="text-sm text-gray-600"><strong>Beskrivning:</strong> {{ order.description }}</p>
                <p class="text-sm text-gray-600"><strong>Belopp:</strong> {{ formatCurrency(order.amount) }}</p>
                <p class="text-sm text-gray-600"><strong>Företag:</strong> {{ getCompanyLabel(order.company) }}</p>
                <p class="text-sm text-gray-600"><strong>Kund:</strong> {{ getCustomerName(order.customer_id) }}</p>
              </div>
            </div>

            <!-- Assignment Form -->
            <div class="bg-gradient-to-br from-green-50 to-emerald-50 rounded-xl p-6 border border-gray-200">
              <h4 class="text-lg font-semibold text-gray-900 mb-4 flex items-center">
                <UserIcon class="w-5 h-5 mr-2 text-green-600" />
                Tilldelning
              </h4>
              
              <form @submit.prevent="handleSubmit" class="space-y-4">
                <!-- Employee Selection -->
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-2">
                    Tilldela medarbetare
                  </label>
                  <select 
                    v-model="assignment.employeeId"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                    required
                  >
                    <option value="">Välj medarbetare</option>
                    <option 
                      v-for="employee in employees" 
                      :key="employee.id" 
                      :value="employee.id"
                    >
                      {{ employee.name }} ({{ employee.subsidiary }})
                    </option>
                  </select>
                </div>

                <!-- Supervisor Selection -->
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-2">
                    Handledare/Ansvarig
                  </label>
                  <select 
                    v-model="assignment.supervisorId"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                    required
                  >
                    <option value="">Välj handledare</option>
                    <option 
                      v-for="employee in employees" 
                      :key="employee.id" 
                      :value="employee.id"
                    >
                      {{ employee.name }} ({{ employee.subsidiary }})
                    </option>
                  </select>
                </div>

                <!-- Scheduled Date -->
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-2">
                    Planerat datum
                  </label>
                  <input
                    v-model="assignment.scheduledDate"
                    type="datetime-local"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                    required
                  />
                </div>

                <!-- Notes -->
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-2">
                    Anteckningar
                  </label>
                  <textarea
                    v-model="assignment.notes"
                    rows="3"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                    placeholder="Lägg till anteckningar om uppdraget..."
                  ></textarea>
                </div>

                <!-- Status -->
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-2">
                    Status
                  </label>
                  <select 
                    v-model="assignment.status"
                    class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                    required
                  >
                    <option value="planned">Planerad</option>
                    <option value="in_progress">Pågående</option>
                  </select>
                </div>
              </form>
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
              Avbryt
            </button>
            <button
              @click="handleSubmit"
              class="px-6 py-3 text-sm font-semibold text-white bg-blue-600 border border-transparent rounded-xl hover:bg-blue-700 hover:shadow-lg transition-all duration-200 shadow-sm"
            >
              Tilldela uppdrag
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { 
  XMarkIcon,
  ShoppingBagIcon,
  UserIcon
} from '@heroicons/vue/24/outline'
import type { PurchaseOrder } from '@/types/domain'
import employees from '@/mock/employees'

interface Props {
  order: PurchaseOrder
}

const props = defineProps<Props>()

defineEmits<{
  close: []
  assign: [assignment: any]
}>()

const assignment = reactive({
  employeeId: props.order.assigned_employee_id || '',
  supervisorId: props.order.supervisor_id || '',
  scheduledDate: props.order.scheduled_date ? new Date(props.order.scheduled_date).toISOString().slice(0, 16) : '',
  notes: props.order.notes || '',
  status: props.order.status === 'not_planned' ? 'planned' : props.order.status
})

function getCustomerName(customerId: number | null): string {
  if (!customerId) return "Okänd kund"
  return `Kund ${customerId}`
}

function getCompanyLabel(company: string): string {
  const labels = {
    allanit: 'Allanit Service AB',
    industrimålning: 'Industrimålning Stockholm AB'
  }
  return labels[company as keyof typeof labels] || company
}

function formatCurrency(amount: number): string {
  return new Intl.NumberFormat('sv-SE', {
    style: 'currency',
    currency: 'SEK'
  }).format(amount)
}

function handleSubmit() {
  const assignmentData = {
    orderId: props.order.id,
    employeeId: parseInt(assignment.employeeId),
    supervisorId: parseInt(assignment.supervisorId),
    scheduledDate: new Date(assignment.scheduledDate).toISOString(),
    notes: assignment.notes,
    status: assignment.status
  }
  
  console.log('Assigning order:', assignmentData)
  // In a real app, this would make an API call
  alert(`Uppdrag ${props.order.external_id} tilldelat!`)
}
</script>
