<template>
  <div class="flex items-center space-x-2">
    <button
      v-for="tag in tags"
      :key="tag.value"
      @click="toggleTag(tag.value)"
      class="px-3 py-1 text-sm rounded-full border transition-colors"
      :class="isSelected(tag.value)
        ? 'bg-blue-100 text-blue-800 border-blue-200'
        : 'bg-gray-100 text-gray-700 border-gray-200 hover:bg-gray-200'"
    >
      {{ tag.label }}
    </button>
    
    <!-- Add new tag -->
    <div v-if="allowAdd" class="relative">
      <input
        v-if="showAddInput"
        v-model="newTag"
        @keyup.enter="addTag"
        @blur="cancelAdd"
        placeholder="Ny tagg..."
        class="px-3 py-1 text-sm border border-gray-300 rounded-full focus:outline-none focus:ring-1 focus:ring-blue-500"
        ref="addInputRef"
      />
      <button
        v-else
        @click="startAdd"
        class="px-3 py-1 text-sm text-gray-500 border border-dashed border-gray-300 rounded-full hover:border-gray-400 hover:text-gray-600"
      >
        + Lägg till
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, nextTick } from 'vue'
interface Tag {
  value: string
  label: string
}

interface Props {
  tags: Tag[]
  selectedTags?: string[]
  allowAdd?: boolean
  allowMultiple?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  selectedTags: () => [],
  allowAdd: false,
  allowMultiple: true
})

const emit = defineEmits<{
  'update:selectedTags': [tags: string[]]
  'tag-add': [tag: string]
}>()

const showAddInput = ref(false)
const newTag = ref('')
const addInputRef = ref<HTMLInputElement>()

const selectedTags = computed({
  get: () => props.selectedTags,
  set: (value) => emit('update:selectedTags', value)
})

function toggleTag(tagValue: string) {
  if (!props.allowMultiple) {
    selectedTags.value = selectedTags.value.includes(tagValue) ? [] : [tagValue]
    return
  }
  
  const index = selectedTags.value.indexOf(tagValue)
  if (index > -1) {
    selectedTags.value = selectedTags.value.filter(t => t !== tagValue)
  } else {
    selectedTags.value = [...selectedTags.value, tagValue]
  }
}

function isSelected(tagValue: string): boolean {
  return selectedTags.value.includes(tagValue)
}

function startAdd() {
  showAddInput.value = true
  nextTick(() => {
    addInputRef.value?.focus()
  })
}

function addTag() {
  if (newTag.value.trim()) {
    emit('tag-add', newTag.value.trim())
    newTag.value = ''
  }
  showAddInput.value = false
}

function cancelAdd() {
  showAddInput.value = false
  newTag.value = ''
}
</script>
