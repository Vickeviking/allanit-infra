<template>
  <article 
    class="rounded-md border p-2 bg-white shadow-sm hover:shadow-md transition cursor-pointer"
    @click="$emit('click', journal)"
  >
    <h4 class="text-xs font-semibold text-gray-900 mb-1 line-clamp-1">{{ journal.title }}</h4>
    <p class="text-[10px] text-gray-500 mb-1">{{ formatDate(journal.createdAt) }}</p>
    <p class="text-[10px] text-gray-600 line-clamp-2">{{ journal.body }}</p>
  </article>
</template>

<script setup lang="ts">
import type { MachineJournal } from '@/types/domain'

interface Props {
  journal: MachineJournal
}

defineProps<Props>()

defineEmits<{
  click: [journal: MachineJournal]
}>()

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString('sv-SE')
}
</script>

<style scoped>
.line-clamp-1 {
  display: -webkit-box;
  -webkit-line-clamp: 1;
  -webkit-box-orient: vertical;
  overflow: hidden;
  line-clamp: 1;
}
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  line-clamp: 2;
}
</style>
