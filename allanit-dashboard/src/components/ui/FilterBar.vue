<template>
  <div class="bg-white rounded-lg border border-gray-200 shadow-sm">
    <!-- Header -->
    <div class="px-2 py-4 border-b border-gray-200">
      <h3 class="text-lg font-medium text-gray-900">Filter</h3>
    </div>

    <!-- Filter Content -->
    <div class="px-8 py-6 space-y-6">
      <!-- Search -->
      <div class="flex items-center space-x-4">
        <label class="text-sm font-medium text-gray-700 whitespace-nowrap"
          >Sök:</label
        >
        <div class="flex items-center gap-2 w-80">
          <MagnifyingGlassIcon class="h-5 w-5 text-gray-400 flex-shrink-0" />
          <input
            v-model="searchQuery"
            type="text"
            placeholder="Sök efter namn, email, eller annan information..."
            class="flex-1 px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors"
          />
        </div>
      </div>

      <!-- Date Range -->
      <div v-if="showDateRange" class="flex items-center space-x-4">
        <label class="text-sm font-medium text-gray-700 whitespace-nowrap"
          >Datumintervall:</label
        >
        <div class="flex items-center space-x-4 flex-1">
          <div class="flex items-center space-x-2">
            <label class="text-xs font-medium text-gray-600 whitespace-nowrap"
              >Från:</label
            >
            <input
              v-model="dateFrom"
              type="date"
              class="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors"
            />
          </div>
          <div class="flex items-center space-x-2">
            <label class="text-xs font-medium text-gray-600 whitespace-nowrap"
              >Till:</label
            >
            <input
              v-model="dateTo"
              type="date"
              class="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors"
            />
          </div>
        </div>
      </div>

      <!-- Select Filters -->
      <div
        v-if="selectFilters.length > 0"
        class="flex flex-wrap items-center gap-6"
      >
        <div
          v-for="filter in selectFilters"
          :key="filter.key"
          class="flex items-center space-x-2"
        >
          <label class="text-sm font-medium text-gray-700 whitespace-nowrap"
            >{{ filter.label }}:</label
          >
          <select
            v-model="filterValues[filter.key]"
            class="px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors min-w-[120px]"
          >
            <option value="">{{ filter.placeholder || "Alla" }}</option>
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
      <div v-if="tagFilters.length > 0" class="mt-6 flex">
        <div
          v-for="filter in tagFilters"
          :key="filter.key"
          class="mb-6 flex first:mt-4 mt-2"
        >
          <label class="block text-sm font-medium text-gray-700 mb-3">{{
            filter.label
          }}</label>
          <div class="flex gap-3 flex-nowrap overflow-x-auto">
            <button
              v-for="tag in filter.tags"
              :key="tag.value"
              @click="toggleTag(filter.key, tag.value)"
              class="px-3 py-1.5 text-sm font-medium rounded-full border transition-all duration-200"
              :class="
                isTagSelected(filter.key, tag.value)
                  ? 'bg-blue-600 text-white border-blue-600 shadow-sm'
                  : 'bg-white text-gray-700 border-gray-300 hover:bg-gray-50 hover:border-gray-400'
              "
            >
              {{ tag.label }}
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Footer Actions -->
    <div class="py-4 border-t border-gray-200 rounded-b-lg">
      <div class="flex items-center justify-between">
        <div class="flex items-center space-x-3">
          <button
            @click="clearFilters"
            class="px-3 py-1.5 text-sm font-medium text-gray-600 hover:text-gray-800 hover:bg-gray-100 rounded-md transition-colors"
          >
            Rensa alla filter
          </button>
          <button
            v-if="showSaveFilter"
            @click="$emit('save-filter')"
            class="px-3 py-1.5 text-sm font-medium text-blue-600 hover:text-blue-800 hover:bg-blue-50 rounded-md transition-colors"
          >
            Spara filter
          </button>
        </div>
        <div class="flex items-center space-x-2">
          <slot name="actions" />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, nextTick } from "vue";
import { MagnifyingGlassIcon } from "@heroicons/vue/24/outline";

interface SelectFilter {
  key: string;
  label: string;
  placeholder?: string;
  options: Array<{ value: string; label: string }>;
}

interface TagFilter {
  key: string;
  label: string;
  tags: Array<{ value: string; label: string }>;
}

interface Props {
  showDateRange?: boolean;
  showSaveFilter?: boolean;
  selectFilters?: SelectFilter[];
  tagFilters?: TagFilter[];
}

const props = withDefaults(defineProps<Props>(), {
  showDateRange: false,
  showSaveFilter: false,
  selectFilters: () => [],
  tagFilters: () => [],
});

const emit = defineEmits<{
  "filter-change": [filters: Record<string, any>];
  "save-filter": [];
}>();

const searchQuery = ref("");
const dateFrom = ref("");
const dateTo = ref("");
const filterValues = ref<Record<string, string>>({});
const selectedTags = ref<Record<string, string[]>>({});

// Initialize filter values
watch(
  () => props.selectFilters,
  (filters) => {
    filters.forEach((filter) => {
      if (!(filter.key in filterValues.value)) {
        filterValues.value[filter.key] = "";
      }
    });
  },
  { immediate: true },
);

// Initialize tag selections
watch(
  () => props.tagFilters,
  (filters) => {
    filters.forEach((filter) => {
      if (!(filter.key in selectedTags.value)) {
        selectedTags.value[filter.key] = [];
      }
    });
  },
  { immediate: true },
);

// Emit filter changes
watch(
  [searchQuery, dateFrom, dateTo, filterValues, selectedTags],
  () => {
    const filters = {
      search: searchQuery.value,
      dateFrom: dateFrom.value,
      dateTo: dateTo.value,
      ...filterValues.value,
      tags: selectedTags.value,
    };
    emit("filter-change", filters);
  },
  { deep: true },
);

function toggleTag(filterKey: string, tagValue: string) {
  if (!selectedTags.value[filterKey]) {
    selectedTags.value[filterKey] = [];
  }

  const index = selectedTags.value[filterKey].indexOf(tagValue);
  if (index > -1) {
    selectedTags.value[filterKey].splice(index, 1);
  } else {
    selectedTags.value[filterKey].push(tagValue);
  }
}

function isTagSelected(filterKey: string, tagValue: string): boolean {
  return selectedTags.value[filterKey]?.includes(tagValue) || false;
}

function clearFilters() {
  searchQuery.value = "";
  dateFrom.value = "";
  dateTo.value = "";

  Object.keys(filterValues.value).forEach((key) => {
    filterValues.value[key] = "";
  });

  Object.keys(selectedTags.value).forEach((key) => {
    selectedTags.value[key] = [];
  });
}
</script>
