<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <div>
        <h1 class="text-2xl font-bold text-gray-900">Maskinpark</h1>
        <p class="text-gray-600">Hantera maskiner och deras journaler</p>
      </div>
      <div class="flex items-center space-x-3">
        <button
          @click="console.log('Add machine')"
          class="px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700"
        >
          Lägg till maskin
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
        <div class="aspect-video bg-gray-300 rounded-lg mb-4"></div>
        <div class="space-y-3">
          <div class="h-4 bg-gray-300 rounded w-3/4"></div>
          <div class="h-3 bg-gray-300 rounded w-1/2"></div>
          <div class="flex items-center space-x-2">
            <div class="w-8 h-8 bg-gray-300 rounded-full"></div>
            <div class="h-3 bg-gray-300 rounded w-1/3"></div>
          </div>
        </div>
      </div>
    </div>

    <!-- Machine Cards Grid -->
    <div v-else class="space-y-8">

      <!-- All Machines Section -->
      <div>
        <h2 class="text-xl font-semibold text-gray-900 mb-4">Alla Maskiner</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
          <div
            v-for="machine in machines"
            :key="machine.id"
            class="bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden hover:shadow-lg hover:border-blue-300 transition-all duration-200"
          >
            <!-- Machine Image -->
            <div class="aspect-video bg-gray-100">
              <img
                :src="machine.image"
                :alt="machine.name"
                class="w-full h-full object-cover"
                @error="handleImageError"
              />
            </div>

            <!-- Machine Info -->
            <div class="p-6">
              <div class="flex items-start justify-between mb-3">
                <div class="flex-1 min-w-0">
                  <h3 class="text-lg font-semibold text-gray-900 truncate">{{ machine.name }}</h3>
                  <p class="text-sm text-gray-600">{{ machine.model }}</p>
                </div>
                <div class="flex items-center space-x-2">
                  <span 
                    class="px-2 py-1 text-xs font-medium rounded-full"
                    :class="getCategoryColor(machine.category)"
                  >
                    {{ getCategoryLabel(machine.category) }}
                  </span>
                  <div
                    class="w-3 h-3 rounded-full"
                    :class="getStatusColor(machine.status)"
                  ></div>
                </div>
              </div>

              <!-- Next Service Date -->
              <div class="mb-4">
                <div class="flex items-center space-x-2">
                  <CalendarIcon class="w-4 h-4 text-gray-400" />
                  <span class="text-sm text-gray-600">Nästa service:</span>
                  <span 
                    class="text-sm font-medium"
                    :class="getServiceDateColor(machine.nextServiceDate)"
                  >
                    {{ formatDate(machine.nextServiceDate) }}
                  </span>
                </div>
              </div>

              <!-- Responsible Employee -->
              <div class="mb-4">
                <div class="flex items-center space-x-3">
                  <img
                    :src="getResponsibleEmployee(machine.responsibleEmployeeId)?.image"
                    :alt="getResponsibleEmployee(machine.responsibleEmployeeId)?.name"
                    class="w-8 h-8 rounded-full object-cover border-2 border-gray-200"
                    @error="handleImageError"
                  />
                  <div class="flex-1 min-w-0">
                    <p class="text-sm text-gray-600">Ansvarig:</p>
                    <button
                      @click="selectEmployee(getResponsibleEmployee(machine.responsibleEmployeeId))"
                      class="text-sm font-medium text-blue-600 hover:text-blue-800 truncate"
                    >
                      {{ getResponsibleEmployee(machine.responsibleEmployeeId)?.name }}
                    </button>
                  </div>
                </div>
              </div>

              <!-- Journals Section -->
              <section class="mb-4">
                <h4 class="text-sm font-medium text-gray-900 mb-2">Journaler</h4>
                <template v-if="(mj = getMachineJournals(machine.id)) && mj.length">
                  <div class="grid grid-cols-3 gap-2">
                    <MachineJournalCard 
                      v-for="j in mj.slice(0, 3)" 
                      :key="j.id" 
                      :journal="j" 
                      @click="selectJournal" 
                    />
                  </div>
                  <p v-if="mj.length > 3" class="text-xs text-gray-500 mt-2 text-center">
                    +{{ mj.length - 3 }} fler journaler
                  </p>
                </template>
                <p v-else class="text-xs text-gray-500 py-2 text-center">Inga journaler ännu.</p>
              </section>

              <!-- Action Button -->
              <div class="pt-4 border-t border-gray-100">
                <button
                  @click="selectMachine(machine)"
                  class="w-full px-3 py-2 text-sm font-medium text-blue-600 bg-blue-50 border border-blue-200 rounded-md hover:bg-blue-100 transition-colors"
                >
                  Visa detaljer
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Employee's Own Machines Section (only for employees) -->
      <div v-if="isEmployee && myMachines.length > 0">
        <!-- Divider -->
        <div class="relative">
          <div class="absolute inset-0 flex items-center">
            <div class="w-full border-t border-gray-300"></div>
          </div>
          <div class="relative flex justify-center">
            <span class="bg-white px-4 text-lg font-semibold text-gray-900">Mina Maskiner</span>
          </div>
        </div>

        <!-- My Machines Grid -->
        <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6 mt-6">
          <div
            v-for="machine in myMachines"
            :key="`my-${machine.id}`"
            class="bg-gradient-to-br from-blue-50 to-indigo-50 rounded-xl shadow-sm border-2 border-blue-200 overflow-hidden hover:shadow-lg hover:border-blue-400 transition-all duration-200"
          >
            <!-- Machine Image -->
            <div class="aspect-video bg-gray-100 relative">
              <img
                :src="machine.image"
                :alt="machine.name"
                class="w-full h-full object-cover"
                @error="handleImageError"
              />
              <!-- My Machine Badge -->
              <div class="absolute top-2 right-2">
                <span class="px-2 py-1 text-xs font-bold text-white bg-blue-600 rounded-full shadow-lg">
                  MIN MASKIN
                </span>
              </div>
            </div>

            <!-- Machine Info -->
            <div class="p-6">
              <div class="flex items-start justify-between mb-3">
                <div class="flex-1 min-w-0">
                  <h3 class="text-lg font-semibold text-gray-900 truncate">{{ machine.name }}</h3>
                  <p class="text-sm text-gray-600">{{ machine.model }}</p>
                </div>
                <div class="flex items-center space-x-2">
                  <span 
                    class="px-2 py-1 text-xs font-medium rounded-full"
                    :class="getCategoryColor(machine.category)"
                  >
                    {{ getCategoryLabel(machine.category) }}
                  </span>
                  <div
                    class="w-3 h-3 rounded-full"
                    :class="getStatusColor(machine.status)"
                  ></div>
                </div>
              </div>

              <!-- Next Service Date -->
              <div class="mb-4">
                <div class="flex items-center space-x-2">
                  <CalendarIcon class="w-4 h-4 text-gray-400" />
                  <span class="text-sm text-gray-600">Nästa service:</span>
                  <span 
                    class="text-sm font-medium"
                    :class="getServiceDateColor(machine.nextServiceDate)"
                  >
                    {{ formatDate(machine.nextServiceDate) }}
                  </span>
                </div>
              </div>

              <!-- Responsible Employee (You) -->
              <div class="mb-4">
                <div class="flex items-center space-x-3">
                  <img
                    :src="getResponsibleEmployee(machine.responsibleEmployeeId)?.image"
                    :alt="getResponsibleEmployee(machine.responsibleEmployeeId)?.name"
                    class="w-8 h-8 rounded-full object-cover border-2 border-blue-300"
                    @error="handleImageError"
                  />
                  <div class="flex-1 min-w-0">
                    <p class="text-sm text-gray-600">Ansvarig:</p>
                    <span class="text-sm font-medium text-blue-700">
                      Du ({{ getResponsibleEmployee(machine.responsibleEmployeeId)?.name }})
                    </span>
                  </div>
                </div>
              </div>

              <!-- Journals Section -->
              <section class="mb-4">
                <h4 class="text-sm font-medium text-gray-900 mb-2">Journaler</h4>
                <template v-if="(mj = getMachineJournals(machine.id)) && mj.length">
                  <div class="grid grid-cols-3 gap-2">
                    <MachineJournalCard 
                      v-for="j in mj.slice(0, 3)" 
                      :key="j.id" 
                      :journal="j" 
                      @click="selectJournal" 
                    />
                  </div>
                  <p v-if="mj.length > 3" class="text-xs text-gray-500 mt-2 text-center">
                    +{{ mj.length - 3 }} fler journaler
                  </p>
                </template>
                <p v-else class="text-xs text-gray-500 py-2 text-center">Inga journaler ännu.</p>
              </section>

              <!-- Action Buttons -->
              <div class="pt-4 border-t border-gray-100 space-y-2">
                <button
                  @click="selectMachine(machine)"
                  class="w-full px-3 py-2 text-sm font-medium text-blue-600 bg-blue-50 border border-blue-200 rounded-md hover:bg-blue-100 transition-colors"
                >
                  Visa detaljer
                </button>
                <button
                  @click="reportFault(machine)"
                  class="w-full px-3 py-2 text-sm font-medium text-red-600 bg-red-50 border border-red-200 rounded-md hover:bg-red-100 transition-colors"
                >
                  Felanmäl
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Machine Detail Modal -->
    <div
      v-if="selectedMachine"
      class="fixed inset-0 z-50 overflow-hidden"
    >
      <div class="absolute inset-0 bg-gray-500 bg-opacity-75" @click="closeMachineDetail"></div>
      <div class="absolute inset-0 flex items-center justify-center p-4">
        <div class="bg-white rounded-2xl shadow-2xl w-full max-w-4xl max-h-[90vh] overflow-hidden">
          <!-- Header -->
          <div class="px-8 py-6 border-b border-gray-200 bg-gradient-to-r from-blue-50 to-indigo-50">
            <div class="flex items-center justify-between">
              <div class="flex items-center space-x-4">
                <div class="w-20 h-20 bg-white rounded-xl shadow-lg overflow-hidden">
                  <img
                    :src="selectedMachine.image"
                    :alt="selectedMachine.name"
                    class="w-full h-full object-cover"
                    @error="handleImageError"
                  />
                </div>
                <div>
                  <h3 class="text-2xl font-bold text-gray-900">{{ selectedMachine.name }}</h3>
                  <p class="text-lg text-gray-600">{{ selectedMachine.model }}</p>
                  <p class="text-sm text-gray-500">{{ selectedMachine.location }}</p>
                </div>
              </div>
              <button
                @click="closeMachineDetail"
                class="p-3 text-gray-400 hover:text-gray-600 hover:bg-white/80 rounded-xl transition-all duration-200"
              >
                <XMarkIcon class="w-6 h-6" />
              </button>
            </div>
          </div>

          <!-- Content -->
          <div class="flex-1 overflow-y-auto p-8">
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
              <!-- Left Column - Machine Info -->
              <div class="space-y-6">
                <div class="bg-gradient-to-br from-gray-50 to-blue-50 rounded-xl p-6 border border-gray-200">
                  <h4 class="text-lg font-semibold text-gray-900 mb-4 flex items-center">
                    <CogIcon class="w-5 h-5 mr-2 text-blue-600" />
                    Maskininformation
                  </h4>
                  <div class="space-y-3">
                    <div class="flex justify-between">
                      <span class="text-gray-600">Kategori:</span>
                      <span class="font-medium text-gray-900">{{ getCategoryLabel(selectedMachine.category) }}</span>
                    </div>
                    <div class="flex justify-between">
                      <span class="text-gray-600">Status:</span>
                      <span class="font-medium" :class="getStatusTextColor(selectedMachine.status)">
                        {{ getStatusLabel(selectedMachine.status) }}
                      </span>
                    </div>
                    <div class="flex justify-between">
                      <span class="text-gray-600">Köpt:</span>
                      <span class="font-medium text-gray-900">{{ formatDate(selectedMachine.purchaseDate) }}</span>
                    </div>
                    <div v-if="selectedMachine.hours" class="flex justify-between">
                      <span class="text-gray-600">Arbetstimmar:</span>
                      <span class="font-medium text-gray-900">{{ selectedMachine.hours.toLocaleString() }}h</span>
                    </div>
                    <div v-if="selectedMachine.kilometers" class="flex justify-between">
                      <span class="text-gray-600">Kilometer:</span>
                      <span class="font-medium text-gray-900">{{ selectedMachine.kilometers.toLocaleString() }} km</span>
                    </div>
                  </div>
                </div>

                <div class="bg-gradient-to-br from-green-50 to-emerald-50 rounded-xl p-6 border border-gray-200">
                  <h4 class="text-lg font-semibold text-gray-900 mb-4 flex items-center">
                    <CalendarIcon class="w-5 h-5 mr-2 text-green-600" />
                    Serviceinformation
                  </h4>
                  <div class="space-y-3">
                    <div class="flex justify-between">
                      <span class="text-gray-600">Nästa service:</span>
                      <span class="font-medium" :class="getServiceDateColor(selectedMachine.nextServiceDate)">
                        {{ formatDate(selectedMachine.nextServiceDate) }}
                      </span>
                    </div>
                    <div class="flex items-center space-x-3">
                      <img
                        :src="getResponsibleEmployee(selectedMachine.responsibleEmployeeId)?.image"
                        :alt="getResponsibleEmployee(selectedMachine.responsibleEmployeeId)?.name"
                        class="w-8 h-8 rounded-full object-cover border-2 border-gray-200"
                        @error="handleImageError"
                      />
                      <div>
                        <p class="text-sm text-gray-600">Ansvarig:</p>
                        <button
                          @click="selectEmployee(getResponsibleEmployee(selectedMachine.responsibleEmployeeId))"
                          class="text-sm font-medium text-blue-600 hover:text-blue-800"
                        >
                          {{ getResponsibleEmployee(selectedMachine.responsibleEmployeeId)?.name }}
                        </button>
                      </div>
                    </div>
                  </div>
                </div>
              </div>

              <!-- Right Column - Journals -->
              <div class="space-y-6">
                <div class="bg-gradient-to-br from-purple-50 to-pink-50 rounded-xl p-6 border border-gray-200">
                  <h4 class="text-lg font-semibold text-gray-900 mb-4 flex items-center">
                    <DocumentTextIcon class="w-5 h-5 mr-2 text-purple-600" />
                    Journaler ({{ getMachineJournals(selectedMachine.id).length }})
                  </h4>
                  <div v-if="getMachineJournals(selectedMachine.id).length > 0" class="space-y-3 max-h-64 overflow-y-auto">
                    <div
                      v-for="journal in getMachineJournals(selectedMachine.id)"
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
                    <DocumentTextIcon class="w-12 h-12 text-gray-300 mx-auto mb-2" />
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
                @click="closeMachineDetail"
                class="px-6 py-3 text-sm font-semibold text-gray-700 bg-white border border-gray-300 rounded-xl hover:bg-gray-50 hover:border-gray-400 transition-all duration-200 shadow-sm hover:shadow-md"
              >
                Stäng
              </button>
              <button
                @click="editMachine"
                class="px-6 py-3 text-sm font-semibold text-white bg-blue-600 border border-transparent rounded-xl hover:bg-blue-700 hover:shadow-lg transition-all duration-200 shadow-sm"
              >
                Redigera maskin
              </button>
              <button
                @click="addJournal"
                class="px-6 py-3 text-sm font-semibold text-white bg-green-600 border border-transparent rounded-xl hover:bg-green-700 hover:shadow-lg transition-all duration-200 shadow-sm"
              >
                Lägg till journal
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
                    <span>Kategori:</span>
                    <span>{{ getCategoryLabel(selectedJournal.category) }}</span>
                  </div>
                  <div class="flex justify-between">
                    <span>Prioritet:</span>
                    <span>{{ getPriorityLabel(selectedJournal.priority) }}</span>
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
import { ref, computed, onMounted, watch } from 'vue'
import { 
  CalendarIcon,
  CogIcon,
  DocumentTextIcon,
  XMarkIcon
} from '@heroicons/vue/24/outline'
import { http } from '@/api/mockClient'
import { fetchJournals } from '@/api/journals'
import type { Machine, MachineJournal } from '@/types/domain'
import MachineJournalCard from '@/components/ui/MachineJournalCard.vue'
import employees from '@/mock/employees'

const machines = ref<Machine[]>([])
const machineJournals = ref<MachineJournal[]>([])
const selectedMachine = ref<Machine | null>(null)
const selectedJournal = ref<MachineJournal | null>(null)
const loading = ref(true)

// Get current user from localStorage - make it reactive
const currentUser = ref(JSON.parse(localStorage.getItem('user') || '{}'))

// Debug logging
console.log('Current user:', currentUser.value)
console.log('User role:', currentUser.value.role)
console.log('User employeeId:', currentUser.value.employeeId)

// Check if current user is an employee (not administrator)
const isEmployee = computed(() => {
  const result = currentUser.value.role === 'employee'
  console.log('Is employee:', result)
  return result
})

// Get machines where current user is responsible
const myMachines = computed(() => {
  if (!isEmployee.value) {
    console.log('Not an employee, returning empty array')
    return []
  }
  const filtered = machines.value.filter(machine => machine.responsibleEmployeeId === currentUser.value.employeeId)
  console.log('My machines:', filtered)
  console.log('All machines:', machines.value)
  return filtered
})
let mj: MachineJournal[] // For template optimization

// Group journals by machine ID for O(1) lookup
const journalsByMachine = computed(() => {
  const map = new Map<string, MachineJournal[]>()
  for (const journal of machineJournals.value) {
    const arr = map.get(journal.machineId) ?? []
    arr.push(journal)
    map.set(journal.machineId, arr)
  }
  return map
})

async function loadData() {
  try {
    const [machinesRes, journalsData] = await Promise.all([
      http.get('/api/machines'),
      http.get('/api/journals/machine')
    ])
    
    machines.value = machinesRes.data.results
    machineJournals.value = journalsData.data.results
    
    console.log('Loaded machines:', machines.value)
    console.log('Current user employeeId:', currentUser.value.employeeId)
    console.log('Machines for current user:', machines.value.filter(m => m.responsibleEmployeeId === currentUser.value.employeeId))
  } catch (error) {
    console.error('Error loading data:', error)
  } finally {
    loading.value = false
  }
}

function getMachineJournals(machineId: number): MachineJournal[] {
  return journalsByMachine.value.get(machineId.toString()) ?? []
}

function getResponsibleEmployee(employeeId: number) {
  return employees.find(emp => emp.id === employeeId)
}

function selectMachine(machine: Machine) {
  selectedMachine.value = machine
}

function closeMachineDetail() {
  selectedMachine.value = null
}

function selectEmployee(employee: any) {
  // Navigate to employee detail or show employee info
  console.log('Selected employee:', employee)
  // Could navigate to employees view or show employee modal
}

function selectJournal(journal: MachineJournal) {
  selectedJournal.value = journal
}

function closeJournalDetail() {
  selectedJournal.value = null
}

function editJournal() {
  console.log('Edit journal:', selectedJournal.value)
}

function editMachine() {
  console.log('Edit machine:', selectedMachine.value)
  alert(`Redigera maskin: ${selectedMachine.value?.name}`)
}


function reportFault(machine: Machine) {
  console.log('Report fault for machine:', machine)
  alert(`Felanmälan för maskin: ${machine.name}`)
  // TODO: Implement fault reporting modal
}

function addJournal() {
  console.log('Add journal for machine:', selectedMachine.value)
  alert(`Lägg till journal för: ${selectedMachine.value?.name}`)
}

function getStatusColor(status: string): string {
  const colors = {
    operational: 'bg-green-500',
    maintenance: 'bg-yellow-500',
    out_of_service: 'bg-red-500'
  }
  return colors[status as keyof typeof colors] || 'bg-gray-500'
}

function getStatusTextColor(status: string): string {
  const colors = {
    operational: 'text-green-600',
    maintenance: 'text-yellow-600',
    out_of_service: 'text-red-600'
  }
  return colors[status as keyof typeof colors] || 'text-gray-600'
}

function getStatusLabel(status: string): string {
  const labels = {
    operational: 'I drift',
    maintenance: 'Underhåll',
    out_of_service: 'Ur drift'
  }
  return labels[status as keyof typeof labels] || status
}

function getCategoryColor(category: string): string {
  const colors = {
    transport: 'bg-blue-100 text-blue-800',
    cleaning: 'bg-green-100 text-green-800',
    winter: 'bg-purple-100 text-purple-800'
  }
  return colors[category as keyof typeof colors] || 'bg-gray-100 text-gray-800'
}

function getCategoryLabel(category: string): string {
  const labels = {
    transport: 'Transport',
    cleaning: 'Städning',
    winter: 'Vinter'
  }
  return labels[category as keyof typeof labels] || category
}

function getPriorityLabel(priority: string): string {
  const labels = {
    low: 'Låg',
    medium: 'Medium',
    high: 'Hög'
  }
  return labels[priority as keyof typeof labels] || priority
}

function getServiceDateColor(dateString: string): string {
  const date = new Date(dateString)
  const now = new Date()
  const diffDays = Math.ceil((date.getTime() - now.getTime()) / (1000 * 60 * 60 * 24))
  
  if (diffDays < 0) return 'text-red-600' // Overdue
  if (diffDays <= 7) return 'text-yellow-600' // Soon
  return 'text-green-600' // OK
}

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString('sv-SE')
}

function handleImageError(event: Event) {
  const img = event.target as HTMLImageElement
  img.src = '/maskiner/default.jpg'
}

// Watch for changes in localStorage
watch(() => localStorage.getItem('user'), (newUser) => {
  if (newUser) {
    currentUser.value = JSON.parse(newUser)
    console.log('User updated from localStorage:', currentUser.value)
  }
}, { immediate: true })

onMounted(() => {
  // Refresh user data from localStorage
  const storedUser = localStorage.getItem('user')
  if (storedUser) {
    currentUser.value = JSON.parse(storedUser)
    console.log('User loaded on mount:', currentUser.value)
  }
  loadData()
})
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
