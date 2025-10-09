<template>
  <div class="max-w-screen-xl mx-auto px-4">
    <!-- Header -->
    <div class="flex items-center justify-between mb-8">
      <div class="flex items-center space-x-16">
        <div>
          <h1 class="text-2xl font-bold text-gray-900">E-posthantering</h1>
          <p class="text-gray-600">Hantera e-postmallar och skicka utskick</p>
        </div>

        <div class="flex items-center space-x-8">
          <button
            @click="showCreateTemplateModal = true"
            class="px-5 py-2.5 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50"
          >
            Ny mall
          </button>
          <button
            @click="showSendEmailModal = true"
            class="px-5 py-2.5 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700"
          >
            Skicka e-post
          </button>
        </div>
      </div>
    </div>

    <!-- Two Column Layout -->
    <div class="grid grid-cols-12 gap-6">
      <!-- Main Content -->
      <section class="col-span-12 xl:col-span-7">
        <!-- Tabs -->
        <div class="bg-white rounded-lg shadow-sm border mb-4">
          <nav class="flex">
            <button
              v-for="tab in tabs"
              :key="tab.key"
              @click="activeTab = tab.key"
              class="px-6 py-3 text-sm font-medium border-b-2 transition-all duration-200 whitespace-nowrap"
              :class="
                activeTab === tab.key
                  ? 'border-blue-500 text-blue-600 bg-blue-50'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 hover:bg-gray-50'
              "
            >
              {{ tab.label }}
            </button>
          </nav>
        </div>

        <!-- Templates Tab -->
        <div v-if="activeTab === 'templates'" class="space-y-3">
          <!-- Loading State -->
          <div v-if="loading" class="flex items-center justify-center py-8">
            <div
              class="animate-spin rounded-full h-6 w-6 border-b-2 border-blue-600"
            ></div>
            <span class="ml-2 text-sm text-gray-600">Laddar mallar...</span>
          </div>

          <!-- Compact Template Table -->
          <div
            v-else
            class="bg-white rounded-lg shadow-sm border overflow-hidden"
          >
            <table class="w-full text-sm">
              <thead class="bg-gray-50 border-b border-gray-200">
                <tr>
                  <th class="px-4 py-3 text-left font-medium text-gray-700">
                    Namn
                  </th>
                  <th class="px-4 py-3 text-left font-medium text-gray-700">
                    Taggar
                  </th>
                  <th class="px-4 py-3 text-left font-medium text-gray-700">
                    Senast ändrad
                  </th>
                  <th class="px-4 py-3 text-left font-medium text-gray-700">
                    Åtgärder
                  </th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-200">
                <tr
                  v-for="template in templates"
                  :key="template.id"
                  class="hover:bg-gray-50 cursor-pointer"
                  @click="selectTemplate(template)"
                >
                  <td class="px-4 py-3">
                    <div>
                      <div class="font-medium text-gray-900">
                        {{ template.name }}
                      </div>
                      <div class="text-xs text-gray-500 truncate max-w-xs">
                        {{ template.subject }}
                      </div>
                    </div>
                  </td>
                  <td class="px-4 py-3">
                    <div class="flex flex-wrap gap-1">
                      <span
                        v-for="tag in template.tags.slice(0, 2)"
                        :key="tag"
                        class="inline-flex px-2 py-1 text-xs font-medium rounded-full bg-blue-100 text-blue-800"
                      >
                        {{ tag }}
                      </span>
                      <span
                        v-if="template.tags.length > 2"
                        class="inline-flex px-2 py-1 text-xs font-medium rounded-full bg-gray-100 text-gray-600"
                      >
                        +{{ template.tags.length - 2 }}
                      </span>
                    </div>
                  </td>
                  <td class="px-4 py-3 text-gray-500">
                    {{ formatDate(template.updated_at || template.created_at) }}
                  </td>
                  <td class="px-4 py-3">
                    <div class="flex space-x-1">
                      <button
                        @click.stop="editTemplate(template)"
                        class="p-1 text-gray-400 hover:text-blue-600 rounded"
                        title="Redigera"
                      >
                        <PencilIcon class="w-4 h-4" />
                      </button>
                      <button
                        @click.stop="deleteTemplate(template)"
                        class="p-1 text-gray-400 hover:text-red-600 rounded"
                        title="Ta bort"
                      >
                        <TrashIcon class="w-4 h-4" />
                      </button>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <!-- Send Email Tab -->
        <div v-if="activeTab === 'send'" class="space-y-6">
          <div class="bg-white rounded-lg shadow-sm border p-6">
            <h3 class="text-lg font-medium text-gray-900 mb-4">
              Skicka e-postutskick
            </h3>

            <div class="space-y-6">
              <!-- Template Selection -->
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-2"
                  >Välj mall</label
                >
                <select
                  v-model="emailForm.templateId"
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
                >
                  <option value="">Välj en mall...</option>
                  <option
                    v-for="template in templates"
                    :key="template.id"
                    :value="template.id"
                  >
                    {{ template.name }}
                  </option>
                </select>
              </div>

              <!-- Target Groups -->
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-2"
                  >Målgrupper</label
                >
                <div class="space-y-2">
                  <div
                    v-for="segment in segments"
                    :key="segment.id"
                    class="flex items-center space-x-3"
                  >
                    <input
                      :id="segment.id"
                      v-model="emailForm.segmentIds"
                      :value="segment.id"
                      type="checkbox"
                      class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                    />
                    <label :for="segment.id" class="text-sm text-gray-700">
                      {{ segment.name }} ({{ segment.count_hint }} mottagare)
                    </label>
                  </div>
                </div>
              </div>

              <!-- Custom Recipients -->
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-2"
                  >Anpassade mottagare</label
                >
                <textarea
                  v-model="emailForm.customRecipients"
                  placeholder="En e-postadress per rad..."
                  class="w-full px-3 py-2 border border-gray-300 rounded-md focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
                  rows="3"
                ></textarea>
              </div>

              <!-- Preview -->
              <div v-if="selectedTemplate">
                <label class="block text-sm font-medium text-gray-700 mb-2"
                  >Förhandsvisning</label
                >
                <div class="border rounded-md p-4 bg-gray-50">
                  <h4 class="font-medium text-gray-900 mb-2">
                    {{ selectedTemplate.subject }}
                  </h4>
                  <div
                    v-html="selectedTemplate.html"
                    class="text-sm text-gray-700"
                  ></div>
                </div>
              </div>

              <!-- Send Button -->
              <div class="flex justify-end">
                <button
                  @click="sendEmail"
                  :disabled="!canSendEmail"
                  class="px-6 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  Skicka e-post
                </button>
              </div>
            </div>
          </div>
        </div>

        <!-- Utskick Tab -->
        <div v-if="activeTab === 'utskick'" class="space-y-6">
          <div class="bg-white rounded-lg shadow-sm border">
            <div class="px-6 py-4 border-b border-gray-200">
              <h3 class="text-lg font-medium text-gray-900">
                Skickade e-postmeddelanden
              </h3>
              <p class="text-sm text-gray-600 mt-1">
                Visa alla skickade e-postutskick
              </p>
            </div>

            <div class="overflow-x-auto">
              <table class="min-w-full divide-y divide-gray-200">
                <thead class="bg-gray-50">
                  <tr>
                    <th
                      class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                    >
                      Kampanj
                    </th>
                    <th
                      class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                    >
                      Mall
                    </th>
                    <th
                      class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                    >
                      Mottagare
                    </th>
                    <th
                      class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                    >
                      Levererat
                    </th>
                    <th
                      class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                    >
                      Öppnat
                    </th>
                    <th
                      class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                    >
                      Skickat
                    </th>
                    <th
                      class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                    >
                      Åtgärder
                    </th>
                  </tr>
                </thead>
                <tbody class="bg-white divide-y divide-gray-200">
                  <tr
                    v-for="send in sentEmails"
                    :key="send.id"
                    class="hover:bg-gray-50"
                  >
                    <td
                      class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900"
                    >
                      {{ send.campaign_name }}
                    </td>
                    <td
                      class="px-6 py-4 whitespace-nowrap text-sm text-gray-900"
                    >
                      {{ send.template_name }}
                    </td>
                    <td
                      class="px-6 py-4 whitespace-nowrap text-sm text-gray-900"
                    >
                      {{ send.to_count }}
                    </td>
                    <td
                      class="px-6 py-4 whitespace-nowrap text-sm text-gray-900"
                    >
                      <span
                        class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-green-100 text-green-800"
                      >
                        {{ send.delivered }}
                      </span>
                    </td>
                    <td
                      class="px-6 py-4 whitespace-nowrap text-sm text-gray-900"
                    >
                      <span
                        class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800"
                      >
                        {{ send.opened }}
                      </span>
                    </td>
                    <td
                      class="px-6 py-4 whitespace-nowrap text-sm text-gray-900"
                    >
                      {{ formatDate(send.sent_at) }}
                    </td>
                    <td
                      class="px-6 py-4 whitespace-nowrap text-sm text-gray-500"
                    >
                      <button
                        @click="viewEmailDetails(send)"
                        class="text-blue-600 hover:text-blue-800 font-medium"
                      >
                        Visa detaljer
                      </button>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>

        <!-- History Tab -->
        <div v-if="activeTab === 'history'" class="space-y-6">
          <div class="bg-white rounded-lg shadow-sm border">
            <div class="px-6 py-4 border-b border-gray-200">
              <h3 class="text-lg font-medium text-gray-900">
                Utskickshistorik
              </h3>
            </div>

            <div class="overflow-x-auto">
              <table class="min-w-full divide-y divide-gray-200">
                <thead class="bg-gray-50">
                  <tr>
                    <th
                      class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                    >
                      Kampanj
                    </th>
                    <th
                      class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                    >
                      Mall
                    </th>
                    <th
                      class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                    >
                      Mottagare
                    </th>
                    <th
                      class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                    >
                      Levererat
                    </th>
                    <th
                      class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                    >
                      Öppnat
                    </th>
                    <th
                      class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
                    >
                      Skickat
                    </th>
                  </tr>
                </thead>
                <tbody class="bg-white divide-y divide-gray-200">
                  <tr
                    v-for="send in sentEmails"
                    :key="send.id"
                    class="hover:bg-gray-50"
                  >
                    <td
                      class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900"
                    >
                      {{ send.campaign_name }}
                    </td>
                    <td
                      class="px-6 py-4 whitespace-nowrap text-sm text-gray-900"
                    >
                      {{ send.template_name }}
                    </td>
                    <td
                      class="px-6 py-4 whitespace-nowrap text-sm text-gray-900"
                    >
                      {{ send.to_count }}
                    </td>
                    <td
                      class="px-6 py-4 whitespace-nowrap text-sm text-gray-900"
                    >
                      {{ send.delivered }}
                    </td>
                    <td
                      class="px-6 py-4 whitespace-nowrap text-sm text-gray-900"
                    >
                      {{ send.opened }}
                    </td>
                    <td
                      class="px-6 py-4 whitespace-nowrap text-sm text-gray-900"
                    >
                      {{ formatDate(send.sent_at) }}
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </section>

      <!-- Right Rail -->
      <aside class="col-span-12 xl:col-span-5 xl:sticky xl:top-4">
        <div class="space-y-4">
          <!-- Template Preview -->
          <div
            v-if="selectedTemplate"
            class="bg-white rounded-lg shadow-sm border p-4"
          >
            <h3 class="text-lg font-medium text-gray-900 mb-3">
              Förhandsvisning
            </h3>
            <div class="border border-gray-200 rounded-lg p-4 bg-gray-50">
              <h4 class="font-medium text-gray-900 mb-2">
                {{ selectedTemplate.subject }}
              </h4>
              <div
                v-html="selectedTemplate.html"
                class="text-sm text-gray-700"
              ></div>
            </div>
            <div class="mt-3 flex space-x-2">
              <button
                @click="editTemplate(selectedTemplate)"
                class="flex-1 px-3 py-2 text-sm bg-blue-600 text-white rounded hover:bg-blue-700"
              >
                Redigera
              </button>
              <button
                @click="sendTestEmail"
                class="flex-1 px-3 py-2 text-sm bg-gray-600 text-white rounded hover:bg-gray-700"
              >
                Skicka test
              </button>
            </div>
          </div>

          <!-- Recent Sends -->
          <div class="bg-white rounded-lg shadow-sm border p-4">
            <h3 class="text-lg font-medium text-gray-900 mb-3">
              Senaste utskick
            </h3>
            <div class="space-y-2">
              <div
                v-for="send in recentSends"
                :key="send.id"
                class="flex items-center justify-between p-2 bg-gray-50 rounded"
              >
                <div>
                  <div class="text-sm font-medium text-gray-900">
                    {{ send.campaign_name }}
                  </div>
                  <div class="text-xs text-gray-500">
                    {{ formatDate(send.sent_at) }}
                  </div>
                </div>
                <div class="text-right">
                  <div class="text-sm text-gray-900">
                    {{ send.delivered }}/{{ send.to_count }}
                  </div>
                  <div class="text-xs text-gray-500">
                    {{ Math.round((send.opened / send.delivered) * 100) || 0 }}%
                    öppnat
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- KPI Stats -->
          <div class="bg-white rounded-lg shadow-sm border p-4">
            <h3 class="text-lg font-medium text-gray-900 mb-3">Statistik</h3>
            <div class="space-y-3">
              <div class="flex justify-between">
                <span class="text-sm text-gray-600">Öppningsgrad</span>
                <span class="text-sm font-medium text-gray-900"
                  >{{ overallStats.openRate }}%</span
                >
              </div>
              <div class="flex justify-between">
                <span class="text-sm text-gray-600">Klickgrad</span>
                <span class="text-sm font-medium text-gray-900"
                  >{{ overallStats.clickRate }}%</span
                >
              </div>
              <div class="flex justify-between">
                <span class="text-sm text-gray-600">Studsgrad</span>
                <span class="text-sm font-medium text-gray-900"
                  >{{ overallStats.bounceRate }}%</span
                >
              </div>
              <div class="flex justify-between">
                <span class="text-sm text-gray-600">Totalt skickat</span>
                <span class="text-sm font-medium text-gray-900">{{
                  overallStats.totalSent
                }}</span>
              </div>
            </div>
          </div>
        </div>
      </aside>
    </div>

    <!-- Template Editor Fullscreen Modal -->
    <div v-if="showTemplateEditor" class="fixed inset-0 z-50 overflow-hidden">
      <div class="absolute inset-0 bg-white">
        <div class="h-full flex flex-col">
          <!-- Header -->
          <div class="px-6 py-4 border-b border-gray-200 bg-gray-50">
            <div class="flex items-center justify-between">
              <h3 class="text-xl font-semibold text-gray-900">
                {{ editingTemplate ? "Redigera mall" : "Skapa ny mall" }}
              </h3>
              <button
                @click="closeTemplateEditor"
                class="text-gray-400 hover:text-gray-600 transition-colors"
              >
                <XMarkIcon class="w-6 h-6" />
              </button>
            </div>
          </div>

          <!-- Content -->
          <div class="flex-1 overflow-hidden flex">
            <!-- Left Side - Form -->
            <div class="w-1/2 p-6 border-r border-gray-200 overflow-y-auto">
              <div class="space-y-6">
                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-2"
                    >Namn</label
                  >
                  <input
                    v-model="templateForm.name"
                    type="text"
                    placeholder="T.ex. Veckorapport BRF"
                    class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors"
                  />
                </div>

                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-2"
                    >Ämne</label
                  >
                  <input
                    v-model="templateForm.subject"
                    type="text"
                    placeholder="T.ex. Veckorapport från Allanit Service"
                    class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors"
                  />
                </div>

                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-2"
                    >Taggar</label
                  >
                  <input
                    v-model="templateForm.tagsString"
                    type="text"
                    placeholder="brf, vinter, info (komma-separerat)"
                    class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors"
                  />
                  <p class="text-xs text-gray-500 mt-1">
                    Använd taggar för att kategorisera mallen
                  </p>
                </div>

                <div>
                  <label class="block text-sm font-medium text-gray-700 mb-2"
                    >HTML-innehåll</label
                  >
                  <textarea
                    v-model="templateForm.html"
                    placeholder="<h1>Hej!</h1><p>Detta är din mall...</p>"
                    class="w-full px-4 py-3 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors font-mono text-sm"
                    rows="15"
                  ></textarea>
                  <p class="text-xs text-gray-500 mt-1">
                    Använd HTML för att formatera e-postmeddelandet
                  </p>
                </div>
              </div>
            </div>

            <!-- Right Side - Live Preview -->
            <div class="w-1/2 p-6 bg-gray-50">
              <div class="h-full flex flex-col">
                <h4 class="text-lg font-medium text-gray-900 mb-4">
                  Live-förhandsvisning
                </h4>
                <div
                  class="flex-1 border border-gray-300 rounded-lg bg-white overflow-y-auto"
                >
                  <div class="p-6">
                    <div v-if="templateForm.subject" class="mb-4">
                      <h2 class="text-lg font-semibold text-gray-900">
                        {{ templateForm.subject }}
                      </h2>
                    </div>
                    <div
                      v-if="templateForm.html"
                      v-html="templateForm.html"
                      class="text-gray-700"
                    ></div>
                    <div v-else class="text-gray-500 italic">
                      Skriv HTML-innehåll för att se förhandsvisning...
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Footer with forced action -->
          <div
            class="px-6 py-4 border-t border-gray-200 bg-gray-50 flex space-x-3"
          >
            <button
              @click="closeTemplateEditor"
              class="px-8 py-3 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors"
            >
              Avbryt
            </button>
            <button
              @click="saveTemplate"
              class="px-8 py-3 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-lg hover:bg-blue-700 transition-colors"
            >
              Spara mall
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from "vue";
import { PencilIcon, TrashIcon, XMarkIcon } from "@heroicons/vue/24/outline";
import { http } from "@/api/mockClient";

const activeTab = ref("templates");
const templates = ref<any[]>([]);
const segments = ref<any[]>([]);
const sentEmails = ref<any[]>([]);
const selectedTemplate = ref<any>(null);
const showTemplateEditor = ref(false);
const editingTemplate = ref<any>(null);
const loading = ref(false);
const showCreateTemplateModal = ref(false);
const showSendEmailModal = ref(false);

const emailForm = ref({
  templateId: "",
  segmentIds: [] as string[],
  customRecipients: "",
});

const templateForm = ref({
  name: "",
  subject: "",
  tagsString: "",
  html: "",
});

const tabs = [
  { key: "templates", label: "Mallar" },
  { key: "send", label: "Skicka" },
  { key: "utskick", label: "Utskick" },
  { key: "history", label: "Historik" },
];

const canSendEmail = computed(() => {
  return (
    emailForm.value.templateId &&
    (emailForm.value.segmentIds.length > 0 ||
      emailForm.value.customRecipients.trim())
  );
});

// Right rail computed properties
const recentSends = computed(() => {
  return sentEmails.value
    .sort(
      (a, b) => new Date(b.sent_at).getTime() - new Date(a.sent_at).getTime(),
    )
    .slice(0, 5);
});

const overallStats = computed(() => {
  const totalSent = sentEmails.value.reduce(
    (sum, send) => sum + send.to_count,
    0,
  );
  const totalDelivered = sentEmails.value.reduce(
    (sum, send) => sum + send.delivered,
    0,
  );
  const totalOpened = sentEmails.value.reduce(
    (sum, send) => sum + send.opened,
    0,
  );
  const totalFailed = sentEmails.value.reduce(
    (sum, send) => sum + (send.to_count - send.delivered),
    0,
  );

  return {
    totalSent,
    openRate:
      totalDelivered > 0 ? Math.round((totalOpened / totalDelivered) * 100) : 0,
    clickRate:
      totalDelivered > 0
        ? Math.round(((totalOpened * 0.3) / totalDelivered) * 100)
        : 0, // Mock click rate
    bounceRate: totalSent > 0 ? Math.round((totalFailed / totalSent) * 100) : 0,
  };
});

async function loadData() {
  if (loading.value) return;

  loading.value = true;
  try {
    // Load data based on active tab to improve performance
    const promises = [];

    if (activeTab.value === "templates" || activeTab.value === "send") {
      promises.push(http.get("/api/mail/templates"));
      promises.push(http.get("/api/segments"));
    }

    if (activeTab.value === "utskick" || activeTab.value === "history") {
      promises.push(http.get("/api/sent-emails"));
    }

    const results = await Promise.all(promises);

    let resultIndex = 0;
    if (activeTab.value === "templates" || activeTab.value === "send") {
      templates.value = results[resultIndex++]?.data?.results || [];
      segments.value = results[resultIndex++]?.data?.results || [];
    }

    if (activeTab.value === "utskick" || activeTab.value === "history") {
      sentEmails.value = results[resultIndex]?.data?.results || [];
    }
  } catch (error) {
    console.error("Error loading data:", error);
  } finally {
    loading.value = false;
  }
}

function selectTemplate(template: any) {
  selectedTemplate.value = template;
  emailForm.value.templateId = template.id;
}

function editTemplate(template: any) {
  editingTemplate.value = template;
  templateForm.value = {
    name: template.name,
    subject: template.subject,
    tagsString: template.tags.join(", "),
    html: template.html,
  };
  showTemplateEditor.value = true;
}

function deleteTemplate(template: any) {
  if (
    confirm(`Är du säker på att du vill ta bort mallen "${template.name}"?`)
  ) {
    templates.value = templates.value.filter((t) => t.id !== template.id);
  }
}

function closeTemplateEditor() {
  showTemplateEditor.value = false;
  editingTemplate.value = null;
  templateForm.value = {
    name: "",
    subject: "",
    tagsString: "",
    html: "",
  };
}

function saveTemplate() {
  const template = {
    id: editingTemplate.value?.id || `tmpl_${Date.now()}`,
    name: templateForm.value.name,
    subject: templateForm.value.subject,
    tags: templateForm.value.tagsString
      .split(",")
      .map((t) => t.trim())
      .filter((t) => t),
    html: templateForm.value.html,
  };

  if (editingTemplate.value) {
    const index = templates.value.findIndex(
      (t) => t.id === editingTemplate.value.id,
    );
    templates.value[index] = template;
  } else {
    templates.value.unshift(template);
  }

  closeTemplateEditor();
}

function sendEmail() {
  if (!canSendEmail.value) return;

  const template = templates.value.find(
    (t) => t.id === emailForm.value.templateId,
  );
  if (!template) return;

  const customEmails = emailForm.value.customRecipients
    .split("\n")
    .map((email) => email.trim())
    .filter((email) => email);

  const totalRecipients =
    emailForm.value.segmentIds.length + customEmails.length;

  const send = {
    id: `send_${Date.now()}`,
    campaign_name: `Utskick ${template.name}`,
    template_name: template.name,
    recipient_count: totalRecipients,
    delivered: totalRecipients,
    opened: Math.floor(totalRecipients * 0.7),
    failed: 0,
    sent_at: new Date().toISOString(),
  };

  sentEmails.value.unshift(send);

  // Reset form
  emailForm.value = {
    templateId: "",
    segmentIds: [],
    customRecipients: "",
  };

  alert(`E-postutskick skickat till ${totalRecipients} mottagare!`);
}

function viewEmailDetails(send: any) {
  console.log("View email details:", send);
  // Could open a modal with detailed statistics
}

// Right rail functions
function sendTestEmail() {
  if (!selectedTemplate.value) return;

  console.log("Sending test email for template:", selectedTemplate.value.name);
  // Mock test email send
  alert(`Test-e-post skickat för "${selectedTemplate.value.name}"`);
}

function importRecipients() {
  console.log("Import recipients");
  // Mock import functionality
  alert("Importera mottagare - funktion kommer snart!");
}

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString("sv-SE");
}

watch(activeTab, () => {
  loadData();
});

onMounted(() => {
  loadData();
});
</script>

