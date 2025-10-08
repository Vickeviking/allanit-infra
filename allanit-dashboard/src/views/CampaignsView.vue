<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-gray-900">Kampanjer</h1>
        <p class="text-gray-600">Skapa och hantera e-postkampanjer</p>
      </div>
      <div class="flex items-center space-x-3">
        <button
          @click="showCampaignHistory = true"
          class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
        >
          Visa historik
        </button>
        <button
          @click="startCampaignWizard"
          class="px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700"
        >
          Skapa kampanj
        </button>
      </div>
    </div>

    <!-- Campaign Wizard -->
    <div v-if="showWizard" class="bg-white rounded-lg shadow-sm border p-6">
      <div class="flex items-center justify-between mb-6">
        <h2 class="text-lg font-medium text-gray-900">Skapa kampanj</h2>
        <button
          @click="closeWizard"
          class="text-gray-400 hover:text-gray-600"
        >
          <XMarkIcon class="w-6 h-6" />
        </button>
      </div>

      <!-- Progress Steps -->
      <div class="mb-8">
        <div class="flex items-center">
          <div
            v-for="(step, index) in wizardSteps"
            :key="step.key"
            class="flex items-center"
          >
            <div
              class="flex items-center justify-center w-8 h-8 rounded-full text-sm font-medium"
              :class="getStepClass(index)"
            >
              {{ index + 1 }}
            </div>
            <span class="ml-2 text-sm font-medium" :class="getStepTextClass(index)">
              {{ step.label }}
            </span>
            <ChevronRightIcon
              v-if="index < wizardSteps.length - 1"
              class="w-5 h-5 mx-4 text-gray-400"
            />
          </div>
        </div>
      </div>

      <!-- Step Content -->
      <div class="min-h-96">
        <!-- Step 1: Target Audience -->
        <div v-if="currentStep === 0" class="space-y-6">
          <h3 class="text-lg font-medium text-gray-900">Välj målgrupp</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">Kundtyp</label>
              <select
                v-model="campaignData.customerType"
                class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
              >
                <option value="">Alla kunder</option>
                <option value="brf">BRF</option>
                <option value="kommersiell">Kommersiell</option>
                <option value="vip">VIP</option>
              </select>
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-2">Antal mottagare</label>
              <input
                v-model="campaignData.recipientCount"
                type="number"
                readonly
                class="w-full px-3 py-2 border border-gray-300 rounded-md bg-gray-50"
              />
            </div>
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Taggar</label>
            <TagSelector
              :tags="customerTags"
              v-model:selected-tags="campaignData.tags"
              allow-multiple
            />
          </div>
        </div>

        <!-- Step 2: Template -->
        <div v-if="currentStep === 1" class="space-y-6">
          <h3 class="text-lg font-medium text-gray-900">Välj mall</h3>
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            <div
              v-for="template in mailTemplates"
              :key="template.id"
              @click="selectTemplate(template)"
              class="p-4 border rounded-lg cursor-pointer hover:border-blue-500 hover:bg-blue-50"
              :class="campaignData.template?.id === template.id ? 'border-blue-500 bg-blue-50' : 'border-gray-300'"
            >
              <h4 class="font-medium text-gray-900">{{ template.name }}</h4>
              <p class="text-sm text-gray-600 mt-1">{{ template.subject }}</p>
              <div class="mt-2 flex flex-wrap gap-1">
                <span
                  v-for="tag in template.tags"
                  :key="tag"
                  class="px-2 py-1 text-xs bg-gray-100 text-gray-700 rounded"
                >
                  {{ tag }}
                </span>
              </div>
            </div>
          </div>
        </div>

        <!-- Step 3: Content -->
        <div v-if="currentStep === 2" class="space-y-6">
          <h3 class="text-lg font-medium text-gray-900">Anpassa innehåll</h3>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Ämne</label>
            <input
              v-model="campaignData.subject"
              type="text"
              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 mb-2">Innehåll</label>
            <textarea
              v-model="campaignData.content"
              rows="8"
              class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
            ></textarea>
          </div>
        </div>

        <!-- Step 4: Preview -->
        <div v-if="currentStep === 3" class="space-y-6">
          <h3 class="text-lg font-medium text-gray-900">Förhandsvisning</h3>
          <div class="border rounded-lg p-6 bg-gray-50">
            <div class="bg-white rounded-lg p-4 shadow-sm">
              <h4 class="font-medium text-gray-900 mb-2">{{ campaignData.subject }}</h4>
              <div class="text-sm text-gray-600" v-html="campaignData.content"></div>
            </div>
          </div>
          <div class="text-sm text-gray-600">
            <p>Mottagare: {{ campaignData.recipientCount }} kunder</p>
            <p>Mall: {{ campaignData.template?.name }}</p>
          </div>
        </div>

        <!-- Step 5: Send -->
        <div v-if="currentStep === 4" class="space-y-6">
          <h3 class="text-lg font-medium text-gray-900">Skicka kampanj</h3>
          <div class="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
            <div class="flex">
              <ExclamationTriangleIcon class="w-5 h-5 text-yellow-400" />
              <div class="ml-3">
                <h4 class="text-sm font-medium text-yellow-800">Viktigt</h4>
                <p class="text-sm text-yellow-700 mt-1">
                  Kontrollera att all information är korrekt innan du skickar kampanjen.
                </p>
              </div>
            </div>
          </div>
          <div class="space-y-4">
            <div class="flex items-center justify-between">
              <span class="text-sm text-gray-600">Mottagare:</span>
              <span class="text-sm font-medium">{{ campaignData.recipientCount }} kunder</span>
            </div>
            <div class="flex items-center justify-between">
              <span class="text-sm text-gray-600">Mall:</span>
              <span class="text-sm font-medium">{{ campaignData.template?.name }}</span>
            </div>
            <div class="flex items-center justify-between">
              <span class="text-sm text-gray-600">Ämne:</span>
              <span class="text-sm font-medium">{{ campaignData.subject }}</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Navigation -->
      <div class="flex items-center justify-between pt-6 border-t border-gray-200">
        <button
          v-if="currentStep > 0"
          @click="previousStep"
          class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
        >
          Föregående
        </button>
        <div v-else></div>
        
        <div class="flex items-center space-x-3">
          <button
            v-if="currentStep < wizardSteps.length - 1"
            @click="nextStep"
            :disabled="!canProceed"
            class="px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            Nästa
          </button>
          <button
            v-else
            @click="sendCampaign"
            :disabled="sending"
            class="px-4 py-2 text-sm font-medium text-white bg-green-600 border border-transparent rounded-md hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {{ sending ? 'Skickar...' : 'Skicka kampanj' }}
          </button>
        </div>
      </div>
    </div>

    <!-- Quick Actions -->
    <div v-if="!showWizard" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div class="bg-white rounded-lg shadow-sm border p-6">
        <h3 class="text-lg font-medium text-gray-900 mb-4">Snabbval</h3>
        <div class="space-y-3">
          <button
            @click="sendTestEmail"
            class="w-full px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
          >
            Skicka testmail
          </button>
          <button
            @click="createTemplate"
            class="w-full px-4 py-2 text-sm font-medium text-blue-600 bg-blue-50 border border-blue-200 rounded-md hover:bg-blue-100"
          >
            Skapa ny mall
          </button>
        </div>
      </div>

      <div class="bg-white rounded-lg shadow-sm border p-6">
        <h3 class="text-lg font-medium text-gray-900 mb-4">Senaste kampanjer</h3>
        <div class="space-y-3">
          <div
            v-for="campaign in recentCampaigns"
            :key="campaign.id"
            class="flex items-center justify-between p-3 bg-gray-50 rounded-md"
          >
            <div>
              <p class="text-sm font-medium text-gray-900">{{ campaign.name }}</p>
              <p class="text-xs text-gray-600">{{ campaign.recipients }} mottagare</p>
            </div>
            <span
              class="inline-flex px-2 py-1 text-xs font-medium rounded-full"
              :class="getCampaignStatusColor(campaign.status)"
            >
              {{ campaign.status }}
            </span>
          </div>
        </div>
      </div>

      <div class="bg-white rounded-lg shadow-sm border p-6">
        <h3 class="text-lg font-medium text-gray-900 mb-4">Statistik</h3>
        <div class="space-y-3">
          <div class="flex items-center justify-between">
            <span class="text-sm text-gray-600">Skickade denna månad:</span>
            <span class="text-sm font-medium">12</span>
          </div>
          <div class="flex items-center justify-between">
            <span class="text-sm text-gray-600">Öppningsgrad:</span>
            <span class="text-sm font-medium">68%</span>
          </div>
          <div class="flex items-center justify-between">
            <span class="text-sm text-gray-600">Klickgrad:</span>
            <span class="text-sm font-medium">12%</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Campaign History Modal -->
    <Modal
      v-if="showCampaignHistory"
      title="Kampanjhistorik"
      @close="showCampaignHistory = false"
      :show-actions="false"
    >
      <div class="space-y-4">
        <div
          v-for="campaign in campaignHistory"
          :key="campaign.id"
          class="flex items-center justify-between p-4 border rounded-lg"
        >
          <div>
            <h4 class="font-medium text-gray-900">{{ campaign.name }}</h4>
            <p class="text-sm text-gray-600">{{ campaign.recipients }} mottagare • {{ formatDate(campaign.sentAt) }}</p>
          </div>
          <div class="text-right">
            <span
              class="inline-flex px-2 py-1 text-xs font-medium rounded-full"
              :class="getCampaignStatusColor(campaign.status)"
            >
              {{ campaign.status }}
            </span>
            <p class="text-xs text-gray-500 mt-1">{{ campaign.openRate }}% öppningsgrad</p>
          </div>
        </div>
      </div>
    </Modal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { 
  XMarkIcon, 
  ChevronRightIcon, 
  ExclamationTriangleIcon 
} from '@heroicons/vue/24/outline'
import TagSelector from '@/components/ui/TagSelector.vue'
import Modal from '@/components/ui/Modal.vue'
import { useMail } from '@/stores/mail'

const mailStore = useMail()

const showWizard = ref(false)
const showCampaignHistory = ref(false)
const currentStep = ref(0)
const sending = ref(false)

const campaignData = ref({
  customerType: '',
  tags: [] as string[],
  template: null as any,
  subject: '',
  content: '',
  recipientCount: 0
})

const wizardSteps = [
  { key: 'audience', label: 'Målgrupp' },
  { key: 'template', label: 'Mall' },
  { key: 'content', label: 'Innehåll' },
  { key: 'preview', label: 'Förhandsvisa' },
  { key: 'send', label: 'Skicka' }
]

const customerTags = [
  { value: 'brf', label: 'BRF' },
  { value: 'kommersiell', label: 'Kommersiell' },
  { value: 'vip', label: 'VIP' },
  { value: 'problem', label: 'Problem' }
]

const recentCampaigns = ref([
  {
    id: 1,
    name: 'Vinterunderhåll 2024',
    recipients: 45,
    status: 'Skickad',
    sentAt: '2025-01-05T10:00:00Z'
  },
  {
    id: 2,
    name: 'Nyårsönskningar',
    recipients: 38,
    status: 'Skickad',
    sentAt: '2025-01-01T09:00:00Z'
  }
])

const campaignHistory = ref([
  {
    id: 1,
    name: 'Vinterunderhåll 2024',
    recipients: 45,
    status: 'Skickad',
    sentAt: '2025-01-05T10:00:00Z',
    openRate: 72
  },
  {
    id: 2,
    name: 'Nyårsönskningar',
    recipients: 38,
    status: 'Skickad',
    sentAt: '2025-01-01T09:00:00Z',
    openRate: 68
  },
  {
    id: 3,
    name: 'Miljörum tips',
    recipients: 42,
    status: 'Skickad',
    sentAt: '2024-12-28T14:00:00Z',
    openRate: 65
  }
])

const mailTemplates = computed(() => mailStore.templates)

const canProceed = computed(() => {
  switch (currentStep.value) {
    case 0: return campaignData.value.customerType || campaignData.value.tags.length > 0
    case 1: return campaignData.value.template
    case 2: return campaignData.value.subject && campaignData.value.content
    case 3: return true
    case 4: return true
    default: return false
  }
})

function getStepClass(index: number): string {
  if (index < currentStep.value) {
    return 'bg-green-500 text-white'
  } else if (index === currentStep.value) {
    return 'bg-blue-500 text-white'
  } else {
    return 'bg-gray-300 text-gray-600'
  }
}

function getStepTextClass(index: number): string {
  if (index <= currentStep.value) {
    return 'text-gray-900'
  } else {
    return 'text-gray-500'
  }
}

function getCampaignStatusColor(status: string): string {
  const colors = {
    'Skickad': 'bg-green-100 text-green-800',
    'Pågår': 'bg-yellow-100 text-yellow-800',
    'Avbruten': 'bg-red-100 text-red-800',
    'Schemalagd': 'bg-blue-100 text-blue-800'
  }
  return colors[status as keyof typeof colors] || 'bg-gray-100 text-gray-800'
}

function startCampaignWizard() {
  showWizard.value = true
  currentStep.value = 0
  campaignData.value = {
    customerType: '',
    tags: [],
    template: null,
    subject: '',
    content: '',
    recipientCount: 0
  }
}

function closeWizard() {
  showWizard.value = false
  currentStep.value = 0
}

function nextStep() {
  if (currentStep.value < wizardSteps.length - 1) {
    currentStep.value++
  }
}

function previousStep() {
  if (currentStep.value > 0) {
    currentStep.value--
  }
}

function selectTemplate(template: any) {
  campaignData.value.template = template
  campaignData.value.subject = template.subject
  campaignData.value.content = template.html
}

function sendCampaign() {
  sending.value = true
  // Simulate sending
  setTimeout(() => {
    sending.value = false
    showWizard.value = false
    console.log('Campaign sent:', campaignData.value)
  }, 2000)
}

function sendTestEmail() {
  console.log('Sending test email')
}

function createTemplate() {
  console.log('Create new template')
}

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString('sv-SE')
}

// Load templates on mount
onMounted(async () => {
  await mailStore.fetchTemplates()
})
</script>
