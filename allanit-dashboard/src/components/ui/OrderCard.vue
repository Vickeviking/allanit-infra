<template>
  <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6 hover:shadow-lg hover:border-blue-300 transition-all duration-200">
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
        <span class="text-sm text-gray-600">{{ customerName }}</span>
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
    <div v-if="order.supervisor_id && supervisor" class="mb-4">
      <div class="flex items-center space-x-3">
        <img
          :src="supervisor.image"
          :alt="supervisor.name"
          class="w-8 h-8 rounded-full object-cover border-2 border-gray-200"
          @error="handleImageError"
        />
        <div class="flex-1 min-w-0">
          <p class="text-sm text-gray-600">Handledare:</p>
          <p class="text-sm font-medium text-gray-900 truncate">
            {{ supervisor.name }}
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

    <!-- Actions -->
    <div class="pt-4 border-t border-gray-100">
      <div class="flex items-center justify-between">
        <div v-if="order.status === 'in_progress'" class="flex items-center space-x-2">
          <input
            type="checkbox"
            :id="`complete-${order.id}`"
            @change="$emit('markComplete', order)"
            class="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
          />
          <label :for="`complete-${order.id}`" class="text-sm text-gray-600">
            Markera som klar
          </label>
        </div>
        <button
          @click="$emit('viewDetails', order)"
          class="px-3 py-2 text-sm font-medium text-blue-600 bg-blue-50 border border-blue-200 rounded-md hover:bg-blue-100 transition-colors"
        >
          Visa detaljer
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { UserGroupIcon, CalendarIcon } from '@heroicons/vue/24/outline'
import type { PurchaseOrder } from '@/types/domain'
import employees from '@/mock/employees'

interface Props {
  order: PurchaseOrder
  customerName?: string
}

const props = defineProps<Props>()

defineEmits<{
  markComplete: [order: PurchaseOrder]
  viewDetails: [order: PurchaseOrder]
}>()

const supervisor = computed(() => {
  if (!props.order.supervisor_id) return null
  return employees.find(emp => emp.id === props.order.supervisor_id)
})

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
