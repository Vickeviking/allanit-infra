<template>
  <div class="space-y-4">
    <!-- Search -->
    <div class="relative">
      <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
        <MagnifyingGlassIcon class="h-5 w-5 text-gray-400" />
      </div>
      <input
        v-model="searchQuery"
        type="text"
        placeholder="Sök..."
        class="block w-full pl-10 pr-3 py-2 border border-gray-300 rounded-md leading-5 bg-white placeholder-gray-500 focus:outline-none focus:placeholder-gray-400 focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
      />
    </div>

    <!-- Date Range -->
    <div v-if="showDateRange" class="grid grid-cols-2 gap-4">
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Från</label>
        <input
          v-model="dateFrom"
          type="date"
          class="block w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
        />
      </div>
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-1">Till</label>
        <input
          v-model="dateTo"
          type="date"
          class="block w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
        />
      </div>
    </div>

    <!-- Select Filters -->
    <div v-if="selectFilters.length > 0" class="space-y-3">
      <div
        v-for="filter in selectFilters"
        :key="filter.key"
        class="space-y-1"
      >
        <label class="block text-sm font-medium text-gray-700">{{ filter.label }}</label>
        <select
          v-model="filterValues[filter.key]"
          class="block w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
        >
          <option value="">{{ filter.placeholder || 'Alla' }}</option>
          <option
            v-for="option in filter.options"
            :key="option.value"
            :value="option.value"
          >
            {{ option.label }}
          </option>
        </select>
      </div>
    </div>

    <!-- Tag Filters -->
    <div v-if="tagFilters.length > 0" class="space-y-3">
      <div
        v-for="filter in tagFilters"
        :key="filter.key"
        class="space-y-2"
      >
        <label class="block text-sm font-medium text-gray-700">{{ filter.label }}</label>
        <div class="flex flex-wrap gap-2">
          <button
            v-for="tag in filter.tags"
            :key="tag.value"
            @click="toggleTag(filter.key, tag.value)"
            class="px-3 py-1 text-sm rounded-full border transition-colors"
            :class="isTagSelected(filter.key, tag.value)
              ? 'bg-blue-100 text-blue-800 border-blue-200'
              : 'bg-gray-100 text-gray-700 border-gray-200 hover:bg-gray-200'"
          >
            {{ tag.label }}
          </button>
        </div>
      </div>
    </div>

    <!-- Actions -->
    <div class="flex items-center justify-between pt-4 border-t border-gray-200">
      <div class="flex items-center space-x-2">
        <button
          @click="clearFilters"
          class="text-sm text-gray-600 hover:text-gray-800"
        >
          Rensa filter
        </button>
        <button
          v-if="showSaveFilter"
          @click="$emit('save-filter')"
          class="text-sm text-blue-600 hover:text-blue-800"
        >
          Spara filter
        </button>
      </div>
      <div class="flex items-center space-x-2">
        <slot name="actions" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, nextTick } from 'vue'
import { MagnifyingGlassIcon } from '@heroicons/vue/24/outline'

interface SelectFilter {
  key: string
  label: string
  placeholder?: string
  options: Array<{ value: string; label: string }>
}

interface TagFilter {
  key: string
  label: string
  tags: Array<{ value: string; label: string }>
}

interface Props {
  showDateRange?: boolean
  showSaveFilter?: boolean
  selectFilters?: SelectFilter[]
  tagFilters?: TagFilter[]
}

const props = withDefaults(defineProps<Props>(), {
  showDateRange: false,
  showSaveFilter: false,
  selectFilters: () => [],
  tagFilters: () => []
})

const emit = defineEmits<{
  'filter-change': [filters: Record<string, any>]
  'save-filter': []
}>()

const searchQuery = ref('')
const dateFrom = ref('')
const dateTo = ref('')
const filterValues = ref<Record<string, string>>({})
const selectedTags = ref<Record<string, string[]>>({})

// Initialize filter values
watch(() => props.selectFilters, (filters) => {
  filters.forEach(filter => {
    if (!(filter.key in filterValues.value)) {
      filterValues.value[filter.key] = ''
    }
  })
}, { immediate: true })

// Initialize tag selections
watch(() => props.tagFilters, (filters) => {
  filters.forEach(filter => {
    if (!(filter.key in selectedTags.value)) {
      selectedTags.value[filter.key] = []
    }
  })
}, { immediate: true })

// Emit filter changes
watch([searchQuery, dateFrom, dateTo, filterValues, selectedTags], () => {
  const filters = {
    search: searchQuery.value,
    dateFrom: dateFrom.value,
    dateTo: dateTo.value,
    ...filterValues.value,
    tags: selectedTags.value
  }
  emit('filter-change', filters)
}, { deep: true })

function toggleTag(filterKey: string, tagValue: string) {
  if (!selectedTags.value[filterKey]) {
    selectedTags.value[filterKey] = []
  }
  
  const index = selectedTags.value[filterKey].indexOf(tagValue)
  if (index > -1) {
    selectedTags.value[filterKey].splice(index, 1)
  } else {
    selectedTags.value[filterKey].push(tagValue)
  }
}

function isTagSelected(filterKey: string, tagValue: string): boolean {
  return selectedTags.value[filterKey]?.includes(tagValue) || false
}

function clearFilters() {
  searchQuery.value = ''
  dateFrom.value = ''
  dateTo.value = ''
  
  Object.keys(filterValues.value).forEach(key => {
    filterValues.value[key] = ''
  })
  
  Object.keys(selectedTags.value).forEach(key => {
    selectedTags.value[key] = []
  })
}
</script>
