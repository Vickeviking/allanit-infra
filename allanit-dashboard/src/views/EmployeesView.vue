<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <div>
        <h1 class="text-2xl font-bold text-gray-900">Medarbetare</h1>
        <p class="text-gray-600">Hantera medarbetare och deras journaler</p>
      </div>
      <div class="flex items-center space-x-3">
        <button
          @click="console.log('Add employee')"
          class="px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700"
        >
          Lägg till medarbetare
        </button>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="loading" class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
      <div
        v-for="i in 3"
        :key="i"
        class="bg-white rounded-xl shadow-sm border border-gray-200 p-6 animate-pulse"
      >
        <div class="flex items-center space-x-4 mb-4">
          <div class="w-16 h-16 bg-gray-300 rounded-full"></div>
          <div class="flex-1">
            <div class="h-4 bg-gray-300 rounded w-1/3 mb-2"></div>
            <div class="h-3 bg-gray-300 rounded w-1/4 mb-1"></div>
            <div class="h-3 bg-gray-300 rounded w-1/5"></div>
          </div>
        </div>
        <div class="space-y-2">
          <div class="h-3 bg-gray-300 rounded w-1/2"></div>
          <div class="h-3 bg-gray-300 rounded w-1/3"></div>
        </div>
      </div>
    </div>

    <!-- Employee Cards Grid -->
    <div v-else class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
      <div
        v-for="employee in employees"
        :key="employee.id"
        class="bg-white rounded-xl shadow-sm border border-gray-200 p-6 hover:shadow-lg hover:border-blue-300 transition-all duration-200"
      >
        <!-- Employee Info -->
        <div class="flex items-center space-x-4 mb-4">
          <div class="relative">
            <img
              :src="employee.image"
              :alt="employee.name"
              class="w-16 h-16 rounded-full object-cover border-2 border-gray-200"
              @error="handleImageError"
            />
            <div
              class="absolute -bottom-1 -right-1 w-5 h-5 rounded-full border-2 border-white shadow-sm"
              :class="getStatusColor(employee.status)"
            ></div>
          </div>
          <div class="flex-1 min-w-0">
            <h3 class="text-lg font-semibold text-gray-900 truncate">{{ employee.name }}</h3>
            <p class="text-sm text-gray-600">{{ employee.role }}</p>
            <p class="text-xs text-gray-500 truncate">{{ employee.subsidiary }}</p>
          </div>
        </div>
        
        <!-- Contact Info -->
        <div class="space-y-2 mb-4">
          <div class="flex items-center space-x-2">
            <EnvelopeIcon class="w-4 h-4 text-gray-400 flex-shrink-0" />
            <span class="text-sm text-gray-600 truncate">{{ employee.email }}</span>
          </div>
          <div class="flex items-center space-x-2">
            <PhoneIcon class="w-4 h-4 text-gray-400 flex-shrink-0" />
            <span class="text-sm text-gray-600">{{ employee.phone }}</span>
          </div>
        </div>

        <!-- Journals Section -->
        <section class="mt-4">
          <h3 class="text-sm font-medium text-gray-900 mb-3">Journaler</h3>
          <template v-if="(ej = getEmployeeJournals(employee.id)) && ej.length">
            <div class="grid grid-cols-3 gap-2">
              <JournalCard 
                v-for="j in ej" 
                :key="j.id" 
                :journal="j" 
                @click="selectJournal" 
              />
            </div>
          </template>
          <p v-else class="text-xs text-gray-500 py-2 text-center">Inga journaler ännu.</p>
        </section>

        <!-- Action Button -->
        <div class="pt-4 border-t border-gray-100 mt-4">
          <button
            @click="selectEmployee(employee)"
            class="w-full px-3 py-2 text-sm font-medium text-blue-600 bg-blue-50 border border-blue-200 rounded-md hover:bg-blue-100 transition-colors"
          >
            Visa detaljer
          </button>
        </div>
      </div>
    </div>

    <!-- Employee Detail Modal -->
    <div
      v-if="selectedEmployee"
      class="fixed inset-0 z-50 overflow-hidden"
    >
      <div class="absolute inset-0 bg-gray-500 bg-opacity-75" @click="closeEmployeeDetail"></div>
      <div class="absolute inset-0 flex items-center justify-center p-4">
        <div class="bg-white rounded-2xl shadow-2xl w-full max-w-4xl max-h-[90vh] overflow-hidden">
          <!-- Header -->
          <div class="px-8 py-6 border-b border-gray-200 bg-gradient-to-r from-blue-50 to-indigo-50">
            <div class="flex items-center justify-between">
              <div class="flex items-center space-x-4">
                <div class="relative">
                  <img
                    :src="selectedEmployee.image"
                    :alt="selectedEmployee.name"
                    class="w-16 h-16 rounded-full object-cover border-4 border-white shadow-lg"
                    @error="handleImageError"
                  />
                  <div
                    class="absolute -bottom-1 -right-1 w-6 h-6 rounded-full border-4 border-white shadow-sm"
                    :class="getStatusColor(selectedEmployee.status)"
                  ></div>
                </div>
                <div>
                  <h3 class="text-2xl font-bold text-gray-900">{{ selectedEmployee.name }}</h3>
                  <p class="text-lg text-gray-600">{{ selectedEmployee.role }}</p>
                  <p class="text-sm text-gray-500">{{ selectedEmployee.subsidiary }}</p>
                </div>
              </div>
              <button
                @click="closeEmployeeDetail"
                class="p-3 text-gray-400 hover:text-gray-600 hover:bg-white/80 rounded-xl transition-all duration-200"
              >
                <XMarkIcon class="w-6 h-6" />
              </button>
            </div>
          </div>

          <!-- Content -->
          <div class="flex-1 overflow-y-auto p-8">
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
              <!-- Left Column - Personal Info -->
              <div class="space-y-6">
                <div class="bg-gradient-to-br from-gray-50 to-blue-50 rounded-xl p-6 border border-gray-200">
                  <h4 class="text-lg font-semibold text-gray-900 mb-4 flex items-center">
                    <svg class="w-5 h-5 mr-2 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd"></path>
                    </svg>
                    Kontaktinformation
                  </h4>
                  <div class="space-y-3">
                    <div class="flex items-center space-x-3">
                      <EnvelopeIcon class="w-5 h-5 text-gray-400" />
                      <span class="text-gray-700">{{ selectedEmployee.email }}</span>
                    </div>
                    <div class="flex items-center space-x-3">
                      <PhoneIcon class="w-5 h-5 text-gray-400" />
                      <span class="text-gray-700">{{ selectedEmployee.phone }}</span>
                    </div>
                    <div class="flex items-center space-x-3">
                      <svg class="w-5 h-5 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M5.05 4.05a7 7 0 119.9 9.9L10 18.9l-4.95-4.95a7 7 0 010-9.9zM10 11a2 2 0 100-4 2 2 0 000 4z" clip-rule="evenodd"></path>
                      </svg>
                      <span class="text-gray-700">{{ selectedEmployee.address || 'Adress inte angiven' }}</span>
                    </div>
                  </div>
                </div>

                <div class="bg-gradient-to-br from-green-50 to-emerald-50 rounded-xl p-6 border border-gray-200">
                  <h4 class="text-lg font-semibold text-gray-900 mb-4 flex items-center">
                    <svg class="w-5 h-5 mr-2 text-green-600" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M6 6V5a3 3 0 013-3h2a3 3 0 013 3v1h2a2 2 0 012 2v3.57A22.952 22.952 0 0110 13a22.95 22.95 0 01-8-1.43V8a2 2 0 012-2h2zm2-1a1 1 0 011-1h2a1 1 0 011 1v1H8V5zm1 5a1 1 0 011-1h.01a1 1 0 110 2H10a1 1 0 01-1-1z" clip-rule="evenodd"></path>
                    </svg>
                    Anställningsinformation
                  </h4>
                  <div class="space-y-3">
                    <div class="flex justify-between">
                      <span class="text-gray-600">Status:</span>
                      <span class="font-medium" :class="getStatusTextColor(selectedEmployee.status)">
                        {{ getStatusLabel(selectedEmployee.status) }}
                      </span>
                    </div>
                    <div class="flex justify-between">
                      <span class="text-gray-600">Anställningsdatum:</span>
                      <span class="font-medium text-gray-900">{{ selectedEmployee.hireDate ? formatDate(selectedEmployee.hireDate) : 'Ej angivet' }}</span>
                    </div>
                    <div class="flex justify-between">
                      <span class="text-gray-600">Avdelning:</span>
                      <span class="font-medium text-gray-900">{{ selectedEmployee.department || 'Ej angivet' }}</span>
                    </div>
                    <div class="flex justify-between">
                      <span class="text-gray-600">Adress:</span>
                      <span class="font-medium text-gray-900">{{ selectedEmployee.address || 'Ej angiven' }}</span>
                    </div>
                  </div>
                </div>
              </div>

              <!-- Right Column - Journals -->
              <div class="space-y-6">
                <div class="bg-gradient-to-br from-purple-50 to-pink-50 rounded-xl p-6 border border-gray-200">
                  <h4 class="text-lg font-semibold text-gray-900 mb-4 flex items-center">
                    <svg class="w-5 h-5 mr-2 text-purple-600" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M4 4a2 2 0 012-2h4.586A2 2 0 0112 2.586L15.414 6A2 2 0 0116 7.414V16a2 2 0 01-2 2H6a2 2 0 01-2-2V4zm2 6a1 1 0 011-1h6a1 1 0 110 2H7a1 1 0 01-1-1zm1 3a1 1 0 100 2h6a1 1 0 100-2H7z" clip-rule="evenodd"></path>
                    </svg>
                    Journaler ({{ getEmployeeJournals(selectedEmployee.id).length }})
                  </h4>
                  <div v-if="getEmployeeJournals(selectedEmployee.id).length > 0" class="space-y-3 max-h-64 overflow-y-auto">
                    <div
                      v-for="journal in getEmployeeJournals(selectedEmployee.id)"
                      :key="journal.id"
                      @click="selectJournal(journal)"
                      class="p-3 bg-white rounded-lg border border-gray-200 hover:border-blue-300 hover:shadow-sm transition-all duration-200 cursor-pointer"
                    >
                      <h5 class="font-medium text-gray-900 text-sm">{{ journal.title }}</h5>
                      <p class="text-xs text-gray-600 mt-1 line-clamp-2">{{ journal.body }}</p>
                      <div class="flex items-center justify-between mt-2">
                        <span class="text-xs text-gray-500">{{ journal.author }}</span>
                        <span class="text-xs text-gray-500">{{ formatDate(journal.createdAt) }}</span>
                      </div>
                    </div>
                  </div>
                  <div v-else class="text-center py-8">
                    <svg class="w-12 h-12 text-gray-300 mx-auto mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
                    </svg>
                    <p class="text-sm text-gray-500">Inga journaler ännu</p>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Actions -->
          <div class="px-8 py-6 border-t border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <div class="flex space-x-4">
              <button
                @click="closeEmployeeDetail"
                class="px-6 py-3 text-sm font-semibold text-gray-700 bg-white border border-gray-300 rounded-xl hover:bg-gray-50 hover:border-gray-400 transition-all duration-200 shadow-sm hover:shadow-md"
              >
                Stäng
              </button>
              <button
                @click="editEmployee"
                class="px-6 py-3 text-sm font-semibold text-white bg-blue-600 border border-transparent rounded-xl hover:bg-blue-700 hover:shadow-lg transition-all duration-200 shadow-sm"
              >
                Redigera medarbetare
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Journal Detail Drawer -->
    <div
      v-if="selectedJournal"
      class="fixed inset-0 z-50 overflow-hidden"
    >
      <div class="absolute inset-0 bg-gray-500 bg-opacity-75" @click="closeJournalDetail"></div>
      <div class="absolute right-0 top-0 h-full w-96 bg-white shadow-xl">
        <div class="h-full flex flex-col">
          <!-- Header -->
          <div class="px-6 py-4 border-b border-gray-200">
            <div class="flex items-center justify-between">
              <h3 class="text-lg font-semibold text-gray-900">{{ selectedJournal.title }}</h3>
              <button
                @click="closeJournalDetail"
                class="text-gray-400 hover:text-gray-600"
              >
                <XMarkIcon class="w-6 h-6" />
              </button>
            </div>
          </div>

          <!-- Content -->
          <div class="flex-1 overflow-y-auto p-6">
            <div class="space-y-4">
              <div>
                <h4 class="text-sm font-medium text-gray-900 mb-2">Innehåll</h4>
                <p class="text-sm text-gray-700 leading-relaxed">{{ selectedJournal.body }}</p>
              </div>

              <div class="pt-4 border-t border-gray-200">
                <h4 class="text-sm font-medium text-gray-900 mb-2">Metadata</h4>
                <div class="space-y-2 text-sm text-gray-600">
                  <div class="flex justify-between">
                    <span>Författare:</span>
                    <span>{{ selectedJournal.author }}</span>
                  </div>
                  <div class="flex justify-between">
                    <span>Skapad:</span>
                    <span>{{ formatDate(selectedJournal.createdAt) }}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Actions -->
          <div class="px-6 py-4 border-t border-gray-200">
            <div class="flex space-x-3">
              <button
                @click="editJournal"
                class="flex-1 px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
              >
                Redigera
              </button>
              <button
                @click="closeJournalDetail"
                class="flex-1 px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700"
              >
                Stäng
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { 
  EnvelopeIcon, 
  PhoneIcon, 
  XMarkIcon 
} from '@heroicons/vue/24/outline'
import { http } from '@/api/mockClient'
import { fetchJournals } from '@/api/journals'
import type { Journal } from '@/mock/journals'
import JournalCard from '@/components/ui/JournalCard.vue'

const employees = ref<any[]>([])
const journals = ref<Journal[]>([])
const selectedEmployee = ref<any>(null)
const selectedJournal = ref<Journal | null>(null)
const loading = ref(true)
let ej: Journal[] // For template optimization

// Group journals by employee ID for O(1) lookup
const journalsByEmployee = computed(() => {
  const map = new Map<string, Journal[]>()
  for (const journal of journals.value) {
    const arr = map.get(journal.employeeId) ?? []
    arr.push(journal)
    map.set(journal.employeeId, arr)
  }
  return map
})

async function loadData() {
  try {
    const [employeesRes, journalsData] = await Promise.all([
      http.get('/api/employees'),
      fetchJournals()
    ])
    
    employees.value = employeesRes.data.results
    journals.value = journalsData
  } catch (error) {
    console.error('Error loading data:', error)
  } finally {
    loading.value = false
  }
}

function getEmployeeJournals(employeeId: number): Journal[] {
  return journalsByEmployee.value.get(employeeId.toString()) ?? []
}

function selectEmployee(employee: any) {
  selectedEmployee.value = employee
}

function closeEmployeeDetail() {
  selectedEmployee.value = null
}

function selectJournal(journal: Journal) {
  selectedJournal.value = journal
}

function closeJournalDetail() {
  selectedJournal.value = null
}

function editJournal() {
  console.log('Edit journal:', selectedJournal.value)
}

function getStatusColor(status: string): string {
  const colors = {
    active: 'bg-green-500',
    inactive: 'bg-gray-500',
    on_leave: 'bg-yellow-500'
  }
  return colors[status as keyof typeof colors] || 'bg-gray-500'
}

function getStatusTextColor(status: string): string {
  const colors = {
    active: 'text-green-600',
    inactive: 'text-gray-600',
    on_leave: 'text-yellow-600'
  }
  return colors[status as keyof typeof colors] || 'text-gray-600'
}

function getStatusLabel(status: string): string {
  const labels = {
    active: 'Aktiv',
    inactive: 'Inaktiv',
    on_leave: 'Ledig'
  }
  return labels[status as keyof typeof labels] || status
}

function editEmployee() {
  console.log('Edit employee:', selectedEmployee.value)
  alert(`Redigera medarbetare: ${selectedEmployee.value.name}`)
}

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString('sv-SE')
}

function handleImageError(event: Event) {
  const img = event.target as HTMLImageElement
  img.src = '/medarbetare/default.jpg'
}

onMounted(() => {
  loadData()
})
</script>

<style scoped>
.line-clamp-3 {
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
  line-clamp: 3;
}

/* Ensure all employee images have consistent size */
img[alt*="Tobias"], img[alt*="Alfons"], img[alt*="Janus"], img[alt*="Johan"] {
  width: 4rem !important;
  height: 4rem !important;
  object-fit: cover !important;
  border-radius: 50% !important;
}
</style>