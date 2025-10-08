<template>
  <div class="bg-white rounded-lg shadow-sm border">
    <div class="px-6 py-4 border-b border-gray-200">
      <div class="flex items-center justify-between">
        <h3 class="text-lg font-medium text-gray-900">{{ title }}</h3>
        <div class="flex items-center space-x-2">
          <slot name="actions" />
        </div>
      </div>
    </div>
    
    <div class="p-6">
      <div v-if="loading" class="space-y-3">
        <div v-for="i in 5" :key="i" class="animate-pulse">
          <div class="h-4 bg-gray-200 rounded w-3/4"></div>
        </div>
      </div>
      
      <div v-else-if="error" class="text-center py-8">
        <div class="text-red-500 mb-2">
          <ExclamationTriangleIcon class="w-8 h-8 mx-auto" />
        </div>
        <p class="text-gray-600">{{ error }}</p>
        <button 
          @click="$emit('retry')" 
          class="mt-2 text-blue-600 hover:text-blue-800 text-sm"
        >
          Försök igen
        </button>
      </div>
      
      <div v-else-if="!items || items.length === 0" class="text-center py-8">
        <div class="text-gray-400 mb-2">
          <component :is="emptyIcon" class="w-8 h-8 mx-auto" />
        </div>
        <p class="text-gray-600">{{ emptyMessage }}</p>
        <slot name="empty-actions" />
      </div>
      
      <div v-else class="space-y-3">
        <slot :items="items" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ExclamationTriangleIcon } from '@heroicons/vue/24/outline'

interface Props {
  title: string
  items?: any[]
  loading?: boolean
  error?: string | null
  emptyMessage?: string
  emptyIcon?: any
}

defineProps<Props>()
defineEmits<{
  retry: []
}>()
</script>
