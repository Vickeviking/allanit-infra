<template>
  <div class="bg-white rounded-lg shadow-sm border">
    <!-- Header -->
    <div class="px-6 py-4 border-b border-gray-200">
      <div class="flex items-center justify-between">
        <div class="flex items-center space-x-4">
          <h3 class="text-lg font-medium text-gray-900">{{ title }}</h3>
          <div v-if="selectedCount > 0" class="text-sm text-gray-600">
            {{ selectedCount }} valda
          </div>
        </div>
        <div class="flex items-center space-x-2">
          <slot name="header-actions" />
        </div>
      </div>
    </div>

    <!-- Filters -->
    <div v-if="showFilters" class="px-6 py-4 border-b border-gray-200">
      <slot name="filters" />
    </div>

    <!-- Table -->
    <div class="overflow-x-auto">
      <table class="min-w-full divide-y divide-gray-200">
        <thead class="bg-white border-b-2 border-gray-300">
          <tr>
            <th v-if="selectable" class="w-12 px-6 py-3 text-left">
              <input
                type="checkbox"
                :checked="allSelected"
                @change="
                  $emit(
                    'select-all',
                    ($event.target as HTMLInputElement).checked,
                  )
                "
                class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
              />
            </th>
            <th
              v-for="column in columns"
              :key="column.key"
              class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider cursor-pointer hover:bg-gray-100"
              @click="column.sortable && $emit('sort', column.key)"
            >
              <div class="flex items-center space-x-1">
                <span>{{ column.label }}</span>
                <component
                  v-if="column.sortable && sortBy === column.key"
                  :is="
                    sortDirection === 'asc'
                      ? 'ChevronUpIcon'
                      : 'ChevronDownIcon'
                  "
                  class="w-4 h-4"
                />
              </div>
            </th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          <tr
            v-for="(item, index) in items"
            :key="getItemKey(item, index)"
            class="hover:bg-gray-50 cursor-pointer"
            @click="$emit('row-click', item)"
          >
            <td v-if="selectable" class="w-12 px-6 py-4">
              <input
                type="checkbox"
                :checked="isSelected(item)"
                @change.stop="
                  $emit(
                    'select-item',
                    item,
                    ($event.target as HTMLInputElement).checked,
                  )
                "
                class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
              />
            </td>
            <td
              v-for="column in columns"
              :key="column.key"
              class="px-6 py-4 whitespace-nowrap text-sm text-gray-900"
            >
              <slot
                :name="`cell-${column.key}`"
                :item="item"
                :value="getNestedValue(item, column.key)"
              >
                {{ formatValue(getNestedValue(item, column.key), column) }}
              </slot>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Empty state -->
    <div v-if="!loading && items.length === 0" class="text-center py-12">
      <div class="text-gray-400 mb-4">
        <component :is="emptyIcon" class="w-12 h-12 mx-auto" />
      </div>
      <h3 class="text-lg font-medium text-gray-900 mb-2">{{ emptyTitle }}</h3>
      <p class="text-gray-600 mb-4">{{ emptyMessage }}</p>
      <slot name="empty-actions" />
    </div>

    <!-- Loading state -->
    <div v-if="loading" class="px-6 py-12">
      <div class="space-y-3">
        <div v-for="i in 5" :key="i" class="animate-pulse">
          <div class="h-4 bg-gray-200 rounded"></div>
        </div>
      </div>
    </div>

    <!-- Pagination -->
    <div
      v-if="showPagination && totalPages > 1"
      class="px-6 py-4 border-t border-gray-200"
    >
      <div class="flex items-center justify-between">
        <div class="text-sm text-gray-700">
          Visar {{ startItem }}-{{ endItem }} av {{ totalItems }} poster
        </div>
        <div class="flex items-center space-x-2">
          <button
            @click="$emit('page-change', currentPage - 1)"
            :disabled="currentPage === 1"
            class="px-3 py-1 text-sm border rounded disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Föregående
          </button>
          <span class="text-sm text-gray-700">
            Sida {{ currentPage }} av {{ totalPages }}
          </span>
          <button
            @click="$emit('page-change', currentPage + 1)"
            :disabled="currentPage === totalPages"
            class="px-3 py-1 text-sm border rounded disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Nästa
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { ChevronUpIcon, ChevronDownIcon } from "@heroicons/vue/24/outline";

interface Column {
  key: string;
  label: string;
  sortable?: boolean;
  formatter?: (value: any) => string;
}

interface Props {
  title: string;
  items: any[];
  columns: Column[];
  loading?: boolean;
  selectable?: boolean;
  selectedItems?: any[];
  showFilters?: boolean;
  showPagination?: boolean;
  currentPage?: number;
  totalPages?: number;
  totalItems?: number;
  sortBy?: string;
  sortDirection?: "asc" | "desc";
  emptyTitle?: string;
  emptyMessage?: string;
  emptyIcon?: any;
  getItemKey?: (item: any, index: number) => string | number;
}

const props = withDefaults(defineProps<Props>(), {
  loading: false,
  selectable: false,
  selectedItems: () => [],
  showFilters: false,
  showPagination: false,
  currentPage: 1,
  totalPages: 1,
  totalItems: 0,
  sortBy: "",
  sortDirection: "asc",
  emptyTitle: "Inga poster",
  emptyMessage: "Det finns inga poster att visa.",
  emptyIcon: "div",
  getItemKey: (item: any, index: number) => item.id || index,
});

defineEmits<{
  "select-all": [checked: boolean];
  "select-item": [item: any, checked: boolean];
  "row-click": [item: any];
  sort: [key: string];
  "page-change": [page: number];
}>();

const selectedCount = computed(() => props.selectedItems?.length || 0);
const allSelected = computed(
  () =>
    props.items.length > 0 &&
    props.selectedItems?.length === props.items.length,
);

const startItem = computed(() => (props.currentPage - 1) * 10 + 1);
const endItem = computed(() =>
  Math.min(props.currentPage * 10, props.totalItems),
);

function getNestedValue(obj: any, path: string) {
  return path.split(".").reduce((current, key) => current?.[key], obj);
}

function formatValue(value: any, column: Column) {
  if (column.formatter) {
    return column.formatter(value);
  }
  if (value === null || value === undefined) {
    return "-";
  }
  return String(value);
}

function isSelected(item: any) {
  return (
    props.selectedItems?.some(
      (selected) => props.getItemKey(selected, 0) === props.getItemKey(item, 0),
    ) || false
  );
}
</script>
