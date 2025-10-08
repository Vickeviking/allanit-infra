<template>
  <div class="bg-white rounded-lg shadow-sm border">
    <div class="px-6 py-4 border-b border-gray-200">
      <div class="flex items-center justify-between">
        <h3 class="text-lg font-medium text-gray-900">{{ title }}</h3>
        <button
          @click="$emit('close')"
          class="text-gray-400 hover:text-gray-600"
        >
          <XMarkIcon class="w-6 h-6" />
        </button>
      </div>
    </div>

    <div class="flex">
      <!-- Tabs -->
      <div class="w-48 border-r border-gray-200">
        <nav class="flex flex-col">
          <button
            v-for="tab in tabs"
            :key="tab.key"
            @click="activeTab = tab.key"
            class="px-6 py-3 text-left text-sm font-medium border-b border-gray-200 hover:bg-gray-50"
            :class="activeTab === tab.key ? 'bg-blue-50 text-blue-700 border-blue-200' : 'text-gray-600'"
          >
            <div class="flex items-center space-x-2">
              <component :is="tab.icon" class="w-4 h-4" />
              <span>{{ tab.label }}</span>
            </div>
          </button>
        </nav>
      </div>

      <!-- Content -->
      <div class="flex-1">
        <div class="p-6">
          <div
            v-for="tab in tabs"
            :key="tab.key"
            v-show="activeTab === tab.key"
          >
            <slot :name="`tab-${tab.key}`" />
          </div>
        </div>

        <!-- Actions -->
        <div v-if="actions.length > 0" class="px-6 py-4 border-t border-gray-200 bg-gray-50">
          <div class="flex items-center justify-end space-x-3">
            <button
              v-for="action in actions"
              :key="action.key"
              @click="$emit('action', action.key)"
              class="px-4 py-2 text-sm font-medium rounded-md"
              :class="action.variant === 'primary' 
                ? 'bg-blue-600 text-white hover:bg-blue-700' 
                : 'bg-white text-gray-700 border border-gray-300 hover:bg-gray-50'"
            >
              {{ action.label }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { XMarkIcon } from '@heroicons/vue/24/outline'

interface Tab {
  key: string
  label: string
  icon: any
}

interface Action {
  key: string
  label: string
  variant?: 'primary' | 'secondary'
}

interface Props {
  title: string
  tabs: Tab[]
  actions?: Action[]
  defaultTab?: string
}

const props = withDefaults(defineProps<Props>(), {
  actions: () => [],
  defaultTab: ''
})

const activeTab = ref(props.defaultTab || props.tabs[0]?.key)

defineEmits<{
  close: []
  action: [key: string]
}>()
</script>
