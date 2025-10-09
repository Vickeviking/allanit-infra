<template>
  <div class="bg-white rounded-lg shadow-sm border p-6">
    <div class="flex items-center justify-between">
      <div>
        <p class="text-sm font-medium text-gray-600">{{ title }}</p>
        <p class="text-2xl font-bold text-gray-900">{{ value }}</p>
        <p v-if="subtitle" class="text-xs text-gray-500 mt-1">{{ subtitle }}</p>
      </div>
      <div class="flex-shrink-0">
        <div class="w-8 h-8 rounded-full flex items-center justify-center p-1" :class="iconBgClass">
          <component :is="icon" class="w-4 h-4" :class="iconClass" style="margin-top: 2px;" />
        </div>
      </div>
    </div>
    <div v-if="trend" class="mt-4 flex items-center">
      <component 
        :is="trend.direction === 'up' ? 'ArrowUpIcon' : 'ArrowDownIcon'" 
        class="w-4 h-4 mr-1"
        :class="trend.direction === 'up' ? 'text-green-500' : 'text-red-500'"
      />
      <span class="text-sm" :class="trend.direction === 'up' ? 'text-green-600' : 'text-red-600'">
        {{ trend.value }}
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ArrowUpIcon, ArrowDownIcon } from '@heroicons/vue/24/outline'

interface Trend {
  direction: 'up' | 'down'
  value: string
}

interface Props {
  title: string
  value: string | number
  subtitle?: string
  icon: any
  iconClass?: string
  iconBgClass?: string
  trend?: Trend
}

defineProps<Props>()
</script>
