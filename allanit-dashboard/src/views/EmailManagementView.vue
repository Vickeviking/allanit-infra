<template>
  <div class="max-w-screen-xl mx-auto px-4">
    <!-- Header -->
    <div class="flex items-center justify-between mb-8">
      <div class="flex items-center space-x-16">
        <div class="flex items-center space-x-4">
          <div class="p-3 bg-blue-100 rounded-xl">
            <svg class="w-8 h-8 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
              <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z"></path>
              <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z"></path>
            </svg>
          </div>
          <div>
            <h1 class="text-3xl font-bold text-gray-900">E-posthantering</h1>
            <p class="text-gray-600 mt-1">Hantera e-postmallar och skicka utskick</p>
          </div>
        </div>

        <div class="flex items-center space-x-4">
          <button
            @click="showCreateTemplateModal = true"
            class="px-6 py-3 text-sm font-semibold text-gray-700 bg-white border border-gray-300 rounded-xl hover:bg-gray-50 hover:border-gray-400 transition-all duration-200 shadow-sm hover:shadow-md flex items-center space-x-2"
          >
            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M10 3a1 1 0 011 1v5h5a1 1 0 110 2h-5v5a1 1 0 11-2 0v-5H4a1 1 0 110-2h5V4a1 1 0 011-1z" clip-rule="evenodd"></path>
            </svg>
            <span>Ny mall</span>
          </button>
          <button
            @click="showSendEmailModal = true"
            class="px-6 py-3 text-sm font-semibold text-white bg-blue-600 border border-transparent rounded-xl hover:bg-blue-700 hover:shadow-lg transition-all duration-200 shadow-sm flex items-center space-x-2"
          >
            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
              <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z"></path>
              <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z"></path>
            </svg>
            <span>Skicka e-post</span>
          </button>
        </div>
      </div>
    </div>

    <!-- Two Column Layout -->
    <div class="grid grid-cols-12 gap-6">
      <!-- Main Content -->
      <section class="col-span-12 xl:col-span-7">
        <!-- Tabs -->
        <div class="bg-white rounded-2xl shadow-sm border border-gray-200 mb-6 overflow-hidden">
          <nav class="flex">
            <button
              v-for="tab in tabs"
              :key="tab.key"
              @click="activeTab = tab.key"
              class="px-8 py-4 text-sm font-semibold border-b-3 transition-all duration-200 whitespace-nowrap flex items-center space-x-2 relative"
              :class="
                activeTab === tab.key
                  ? 'border-blue-500 text-blue-600 bg-gradient-to-r from-blue-50 to-indigo-50'
                  : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300 hover:bg-gray-50'
              "
            >
              <svg 
                v-if="tab.key === 'templates'"
                class="w-4 h-4" 
                fill="currentColor" 
                viewBox="0 0 20 20"
              >
                <path fill-rule="evenodd" d="M4 4a2 2 0 00-2 2v8a2 2 0 002 2h12a2 2 0 002-2V6a2 2 0 00-2-2H4zm2 6a2 2 0 114 0 2 2 0 01-4 0zm8 0a2 2 0 114 0 2 2 0 01-4 0z" clip-rule="evenodd"></path>
              </svg>
              <svg 
                v-else-if="tab.key === 'send'"
                class="w-4 h-4" 
                fill="currentColor" 
                viewBox="0 0 20 20"
              >
                <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z"></path>
                <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z"></path>
              </svg>
              <svg 
                v-else-if="tab.key === 'utskick'"
                class="w-4 h-4" 
                fill="currentColor" 
                viewBox="0 0 20 20"
              >
                <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path>
              </svg>
              <svg 
                v-else-if="tab.key === 'history'"
                class="w-4 h-4" 
                fill="currentColor" 
                viewBox="0 0 20 20"
              >
                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd"></path>
              </svg>
              <span>{{ tab.label }}</span>
            </button>
          </nav>
        </div>

        <!-- Templates Tab -->
        <div v-if="activeTab === 'templates'" class="space-y-6">
          <!-- Loading State -->
          <div v-if="loading" class="flex items-center justify-center py-12">
            <div class="text-center">
              <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto"></div>
              <span class="ml-2 text-sm text-gray-600 mt-2 block">Laddar mallar...</span>
            </div>
          </div>

          <!-- Template Cards Grid -->
          <div v-else-if="templates.length === 0" class="text-center py-12">
            <div class="p-4 bg-gray-100 rounded-2xl mb-4 inline-block">
              <svg class="w-12 h-12 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
              </svg>
            </div>
            <h3 class="text-lg font-medium text-gray-900 mb-2">Inga mallar än</h3>
            <p class="text-gray-500 mb-4">Skapa din första e-postmall för att komma igång</p>
            <button
              @click="showCreateTemplateModal = true"
              class="px-6 py-3 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-xl hover:bg-blue-700 transition-colors"
            >
              Skapa första mallen
            </button>
          </div>

          <!-- Template Gallery with Carousel -->
          <div v-else class="space-y-6">
            <!-- Carousel Controls -->
            <div class="flex items-center justify-between">
              <div class="flex items-center space-x-4">
                <h3 class="text-lg font-semibold text-gray-900">Mallar</h3>
                <span class="text-sm text-gray-500">{{ templates.length }} mallar</span>
              </div>
              <div class="flex items-center space-x-2">
                <button
                  @click="previousTemplates"
                  :disabled="currentTemplateIndex === 0"
                  class="p-2 text-gray-400 hover:text-gray-600 hover:bg-gray-100 rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
                  </svg>
                </button>
                <span class="text-sm text-gray-500 px-2">
                  {{ currentTemplateIndex + 1 }} / {{ templates.length }}
                </span>
                <button
                  @click="nextTemplates"
                  :disabled="currentTemplateIndex >= templates.length - templatesPerPage"
                  class="p-2 text-gray-400 hover:text-gray-600 hover:bg-gray-100 rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                  </svg>
                </button>
              </div>
            </div>

            <!-- Template Cards Grid -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div
                v-for="template in visibleTemplates"
                :key="template.id"
                class="bg-white rounded-2xl shadow-sm border border-gray-200 hover:shadow-lg hover:border-blue-300 transition-all duration-200 cursor-pointer group"
                @click="selectTemplate(template)"
              >
                <!-- Card Header -->
                <div class="p-4 border-b border-gray-100">
                  <div class="flex items-start justify-between">
                    <div class="flex-1 min-w-0">
                      <h3 class="text-base font-bold text-gray-900 group-hover:text-blue-600 transition-colors mb-1">
                        {{ template.name }}
                      </h3>
                      <p class="text-xs text-gray-600 line-clamp-1">
                        {{ template.subject }}
                      </p>
                    </div>
                    <div class="flex space-x-1 ml-2">
                      <button
                        @click.stop="editTemplate(template)"
                        class="p-1.5 text-gray-400 hover:text-blue-600 hover:bg-blue-50 rounded-lg transition-all duration-200"
                        title="Redigera"
                      >
                        <PencilIcon class="w-3.5 h-3.5" />
                      </button>
                      <button
                        @click.stop="deleteTemplate(template)"
                        class="p-1.5 text-gray-400 hover:text-red-600 hover:bg-red-50 rounded-lg transition-all duration-200"
                        title="Ta bort"
                      >
                        <TrashIcon class="w-3.5 h-3.5" />
                      </button>
                    </div>
                  </div>
                </div>

                <!-- Card Content -->
                <div class="p-4">
                  <!-- Tags -->
                  <div v-if="template.tags?.length" class="mb-3">
                    <div class="flex flex-wrap gap-1">
                      <span
                        v-for="tag in template.tags.slice(0, 2)"
                        :key="tag"
                        class="inline-flex px-1.5 py-0.5 text-xs font-medium rounded-full bg-blue-100 text-blue-800"
                      >
                        {{ tag }}
                      </span>
                      <span
                        v-if="template.tags.length > 2"
                        class="inline-flex px-1.5 py-0.5 text-xs font-medium rounded-full bg-gray-100 text-gray-600"
                      >
                        +{{ template.tags.length - 2 }}
                      </span>
                    </div>
                  </div>

                  <!-- Preview -->
                  <div class="mb-3">
                    <div class="text-xs text-gray-500 mb-1 font-medium flex items-center">
                      <svg class="w-3 h-3 mr-1 text-blue-500" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
                      </svg>
                      Förhandsvisning
                    </div>
                    <div class="bg-gradient-to-br from-gray-50 to-blue-50 rounded-lg p-2 border border-gray-200 shadow-inner max-h-16 overflow-hidden">
                      <div class="text-xs text-gray-700 line-clamp-2">
                        {{ getPreviewText(template.html) }}
                      </div>
                    </div>
                  </div>

                  <!-- Footer -->
                  <div class="flex items-center justify-between text-xs text-gray-500">
                    <div class="flex items-center space-x-1">
                      <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd"></path>
                      </svg>
                      <span>{{ formatDate(template.updated_at || template.created_at) }}</span>
                    </div>
                    <div class="flex items-center space-x-1 text-blue-600 group-hover:text-blue-700">
                      <span class="text-xs">Välj</span>
                      <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                      </svg>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Send Email Tab -->
        <div v-if="activeTab === 'send'" class="space-y-6">
          <div class="bg-white rounded-2xl shadow-sm border border-gray-200 overflow-hidden">
            <div class="px-8 py-6 border-b border-gray-200 bg-gradient-to-r from-green-50 to-emerald-50">
              <div class="flex items-center space-x-3">
                <div class="p-2 bg-green-100 rounded-lg">
                  <svg class="w-6 h-6 text-green-600" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z"></path>
                    <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z"></path>
                  </svg>
                </div>
                <div>
                  <h3 class="text-xl font-bold text-gray-900">
                    Skicka e-postutskick
                  </h3>
                  <p class="text-sm text-gray-600 mt-1">
                    Välj mall och mottagare för att skicka e-postutskick
                  </p>
                </div>
              </div>
            </div>

            <div class="p-8">
              <div class="space-y-8">
                <!-- Template Selection -->
                <div class="space-y-3">
                  <label class="block text-sm font-semibold text-gray-800 flex items-center">
                    <svg class="w-4 h-4 mr-2 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M4 4a2 2 0 00-2 2v8a2 2 0 002 2h12a2 2 0 002-2V6a2 2 0 00-2-2H4zm2 6a2 2 0 114 0 2 2 0 01-4 0zm8 0a2 2 0 114 0 2 2 0 01-4 0z" clip-rule="evenodd"></path>
                    </svg>
                    Välj mall
                  </label>
                  <select
                    v-model="emailForm.templateId"
                    class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-all duration-200 bg-white shadow-sm hover:shadow-md"
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
                <div class="space-y-3">
                  <label class="block text-sm font-semibold text-gray-800 flex items-center">
                    <svg class="w-4 h-4 mr-2 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-6-3a2 2 0 11-4 0 2 2 0 014 0zM8 7a3 3 0 000 6h4a3 3 0 000-6H8z" clip-rule="evenodd"></path>
                    </svg>
                    Målgrupper
                  </label>
                  <div class="space-y-3">
                    <div
                      v-for="segment in segments"
                      :key="segment.id"
                      class="flex items-center space-x-3 p-3 bg-gray-50 rounded-xl border border-gray-200 hover:bg-blue-50 hover:border-blue-300 transition-all duration-200"
                    >
                      <input
                        :id="segment.id"
                        v-model="emailForm.segmentIds"
                        :value="segment.id"
                        type="checkbox"
                        class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
                      />
                      <label :for="segment.id" class="text-sm text-gray-700 flex-1 cursor-pointer">
                        <div class="font-medium">{{ segment.name }}</div>
                        <div class="text-xs text-gray-500">{{ segment.count_hint }} mottagare</div>
                      </label>
                    </div>
                  </div>
                </div>

                <!-- Custom Recipients -->
                <div class="space-y-3">
                  <label class="block text-sm font-semibold text-gray-800 flex items-center">
                    <svg class="w-4 h-4 mr-2 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                      <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z"></path>
                      <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z"></path>
                    </svg>
                    Anpassade mottagare
                  </label>
                  <textarea
                    v-model="emailForm.customRecipients"
                    placeholder="En e-postadress per rad..."
                    class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-all duration-200 bg-white shadow-sm hover:shadow-md resize-none"
                    rows="4"
                  ></textarea>
                  <div class="text-xs text-gray-500 flex items-center">
                    <svg class="w-4 h-4 mr-1 text-blue-500" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"></path>
                    </svg>
                    Lägg till specifika e-postadresser som ska få meddelandet
                  </div>
                </div>

              <!-- Email Preview -->
              <div v-if="emailForm.templateId">
                <div class="flex items-center justify-between mb-4">
                  <label class="block text-sm font-semibold text-gray-800 flex items-center">
                    <svg class="w-5 h-5 mr-2 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
                    </svg>
                    Förhandsvisning av e-post
                  </label>
                  <div class="flex items-center space-x-2 text-xs text-gray-500 bg-green-50 px-3 py-1 rounded-full">
                    <div class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                    <span class="font-medium">Live förhandsvisning</span>
                  </div>
                </div>
                
                <div class="border-2 border-gray-200 rounded-2xl bg-white shadow-lg overflow-hidden hover:shadow-xl transition-shadow duration-200">
                  <!-- Preview Header -->
                  <div class="px-6 py-4 bg-gradient-to-r from-blue-50 via-indigo-50 to-purple-50 border-b border-gray-200">
                    <div class="flex items-center justify-between">
                      <div class="flex items-center space-x-3">
                        <div class="p-2 bg-blue-100 rounded-lg">
                          <svg class="w-5 h-5 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                            <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z"></path>
                            <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z"></path>
                          </svg>
                        </div>
                        <div>
                          <h4 class="text-lg font-bold text-gray-900">
                            {{ selectedTemplateForSend?.subject || 'Välj en mall för att se förhandsvisning' }}
                          </h4>
                          <p class="text-sm text-gray-600 mt-1">
                            {{ selectedTemplateForSend?.name || 'Ingen mall vald' }}
                          </p>
                        </div>
                      </div>
                      <div class="text-right">
                        <div class="text-sm font-medium text-gray-900">{{ getRecipientCount() }} mottagare</div>
                        <div class="text-xs text-gray-500">kommer att få detta meddelande</div>
                      </div>
                    </div>
                  </div>
                  
                  <!-- Preview Content -->
                  <div class="p-6">
                    <div v-if="selectedTemplateForSend" class="space-y-6">
                      <!-- Email Header Info -->
                      <div class="bg-gradient-to-r from-gray-50 to-blue-50 p-4 rounded-xl border border-gray-200">
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-4 text-sm">
                          <div class="flex items-center space-x-3">
                            <div class="p-2 bg-white rounded-lg shadow-sm">
                              <svg class="w-4 h-4 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                                <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z"></path>
                                <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z"></path>
                              </svg>
                            </div>
                            <div>
                              <div class="font-medium text-gray-900">Från</div>
                              <div class="text-gray-600">Allanit Service</div>
                            </div>
                          </div>
                          <div class="flex items-center space-x-3">
                            <div class="p-2 bg-white rounded-lg shadow-sm">
                              <svg class="w-4 h-4 text-green-600" fill="currentColor" viewBox="0 0 20 20">
                                <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-6-3a2 2 0 11-4 0 2 2 0 014 0zM8 7a3 3 0 000 6h4a3 3 0 000-6H8z" clip-rule="evenodd"></path>
                              </svg>
                            </div>
                            <div>
                              <div class="font-medium text-gray-900">Till</div>
                              <div class="text-gray-600">{{ getRecipientCount() }} mottagare</div>
                            </div>
                          </div>
                        </div>
                      </div>
                      
                      <!-- Email Body -->
                      <div class="border-2 border-gray-200 rounded-xl bg-white shadow-inner overflow-hidden">
                        <div class="bg-gray-50 px-4 py-2 border-b border-gray-200">
                          <div class="flex items-center space-x-2 text-xs text-gray-500">
                            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                              <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path>
                            </svg>
                            <span class="font-medium">E-POSTINNEHÅLL</span>
                          </div>
                        </div>
                        <div class="p-6 min-h-[200px]">
                          <div
                            v-html="selectedTemplateForSend.html"
                            class="text-gray-700 prose prose-sm max-w-none prose-headings:text-gray-900 prose-p:text-gray-700 prose-a:text-blue-600 prose-strong:text-gray-900"
                          ></div>
                        </div>
                      </div>
                      
                      <!-- Preview Footer -->
                      <div class="bg-gradient-to-r from-gray-50 to-blue-50 p-4 rounded-xl border border-gray-200">
                        <div class="flex items-center justify-between">
                          <div class="flex items-center space-x-2 text-sm text-gray-600">
                            <svg class="w-4 h-4 text-blue-500" fill="currentColor" viewBox="0 0 20 20">
                              <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"></path>
                            </svg>
                            <span>Detta är en förhandsvisning av e-postmeddelandet</span>
                          </div>
                          <div v-if="selectedTemplateForSend.tags?.length" class="flex items-center space-x-2">
                            <span class="text-xs text-gray-500">Taggar:</span>
                            <div class="flex space-x-1">
                              <span
                                v-for="tag in selectedTemplateForSend.tags.slice(0, 3)"
                                :key="tag"
                                class="inline-flex px-2 py-1 text-xs font-medium rounded-full bg-blue-100 text-blue-800"
                              >
                                {{ tag }}
                              </span>
                              <span
                                v-if="selectedTemplateForSend.tags.length > 3"
                                class="inline-flex px-2 py-1 text-xs font-medium rounded-full bg-gray-100 text-gray-600"
                              >
                                +{{ selectedTemplateForSend.tags.length - 3 }}
                              </span>
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>
                    
                    <div v-else class="flex items-center justify-center h-48 text-gray-400">
                      <div class="text-center">
                        <div class="p-4 bg-gray-100 rounded-2xl mb-4">
                          <svg class="w-12 h-12 mx-auto text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
                          </svg>
                        </div>
                        <p class="text-sm font-medium">Välj en mall ovan för att se förhandsvisning</p>
                        <p class="text-xs text-gray-400 mt-1">Förhandsvisningen uppdateras automatiskt</p>
                      </div>
                    </div>
                  </div>
                </div>
              </div>

                <!-- Send Button -->
                <div class="flex justify-end pt-6 border-t border-gray-200">
                  <button
                    @click="sendEmail"
                    :disabled="!canSendEmail"
                    class="px-8 py-4 text-sm font-semibold text-white bg-blue-600 border border-transparent rounded-xl hover:bg-blue-700 hover:shadow-lg transition-all duration-200 shadow-sm disabled:opacity-50 disabled:cursor-not-allowed flex items-center space-x-2"
                  >
                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                      <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z"></path>
                      <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z"></path>
                    </svg>
                    <span>Skicka e-postutskick</span>
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Utskick Tab -->
        <div v-if="activeTab === 'utskick'" class="space-y-6">
          <div class="bg-white rounded-2xl shadow-sm border border-gray-200 overflow-hidden">
            <div class="px-8 py-6 border-b border-gray-200 bg-gradient-to-r from-blue-50 to-indigo-50">
              <div class="flex items-center space-x-3">
                <div class="p-2 bg-blue-100 rounded-lg">
                  <svg class="w-6 h-6 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z"></path>
                    <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z"></path>
                  </svg>
                </div>
                <div>
                  <h3 class="text-xl font-bold text-gray-900">
                    Skickade e-postmeddelanden
                  </h3>
                  <p class="text-sm text-gray-600 mt-1">
                    Visa alla skickade e-postutskick och deras statistik
                  </p>
                </div>
              </div>
            </div>

            <div v-if="sentEmails.length === 0" class="text-center py-12">
              <div class="p-4 bg-gray-100 rounded-2xl mb-4 inline-block">
                <svg class="w-12 h-12 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4"></path>
                </svg>
              </div>
              <h3 class="text-lg font-medium text-gray-900 mb-2">Inga utskick än</h3>
              <p class="text-gray-500 mb-4">Skicka ditt första e-postutskick för att se statistik här</p>
            </div>

            <div v-else class="overflow-x-auto">
              <table class="min-w-full divide-y divide-gray-200">
                <thead class="bg-gray-50">
                  <tr>
                    <th class="px-6 py-4 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                      Kampanj
                    </th>
                    <th class="px-6 py-4 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                      Mall
                    </th>
                    <th class="px-6 py-4 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                      Mottagare
                    </th>
                    <th class="px-6 py-4 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                      Levererat
                    </th>
                    <th class="px-6 py-4 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                      Öppnat
                    </th>
                    <th class="px-6 py-4 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                      Skickat
                    </th>
                    <th class="px-6 py-4 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                      Åtgärder
                    </th>
                  </tr>
                </thead>
                <tbody class="bg-white divide-y divide-gray-200">
                  <tr
                    v-for="send in sentEmails"
                    :key="send.id"
                    class="hover:bg-blue-50 transition-colors duration-200"
                  >
                    <td class="px-6 py-4 whitespace-nowrap">
                      <div class="flex items-center space-x-3">
                        <div class="p-2 bg-blue-100 rounded-lg">
                          <svg class="w-4 h-4 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path>
                          </svg>
                        </div>
                        <div>
                          <div class="text-sm font-semibold text-gray-900">{{ send.campaign_name }}</div>
                        </div>
                      </div>
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap">
                      <div class="text-sm text-gray-900">{{ send.template_name }}</div>
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap">
                      <div class="flex items-center space-x-2">
                        <svg class="w-4 h-4 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                          <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-6-3a2 2 0 11-4 0 2 2 0 014 0zM8 7a3 3 0 000 6h4a3 3 0 000-6H8z" clip-rule="evenodd"></path>
                        </svg>
                        <span class="text-sm font-medium text-gray-900">{{ send.to_count }}</span>
                      </div>
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap">
                      <div class="flex items-center space-x-2">
                        <span class="inline-flex items-center px-3 py-1 rounded-full text-xs font-semibold bg-green-100 text-green-800">
                          {{ send.delivered }}
                        </span>
                        <span class="text-xs text-gray-500">
                          {{ Math.round((send.delivered / send.to_count) * 100) }}%
                        </span>
                      </div>
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap">
                      <div class="flex items-center space-x-2">
                        <span class="inline-flex items-center px-3 py-1 rounded-full text-xs font-semibold bg-blue-100 text-blue-800">
                          {{ send.opened }}
                        </span>
                        <span class="text-xs text-gray-500">
                          {{ Math.round((send.opened / send.delivered) * 100) }}%
                        </span>
                      </div>
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap">
                      <div class="flex items-center space-x-2">
                        <svg class="w-4 h-4 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                          <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd"></path>
                        </svg>
                        <span class="text-sm text-gray-900">{{ formatDate(send.sent_at) }}</span>
                      </div>
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap">
                      <button
                        @click="viewEmailDetails(send)"
                        class="inline-flex items-center space-x-2 px-3 py-2 text-sm font-medium text-blue-600 bg-blue-50 rounded-lg hover:bg-blue-100 transition-colors duration-200"
                      >
                        <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                          <path d="M10 12a2 2 0 100-4 2 2 0 000 4z"></path>
                          <path fill-rule="evenodd" d="M.458 10C1.732 5.943 5.522 3 10 3s8.268 2.943 9.542 7c-1.274 4.057-5.064 7-9.542 7S1.732 14.057.458 10zM14 10a4 4 0 11-8 0 4 4 0 018 0z" clip-rule="evenodd"></path>
                        </svg>
                        <span>Visa detaljer</span>
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
          <div class="bg-white rounded-2xl shadow-sm border border-gray-200 overflow-hidden">
            <div class="px-8 py-6 border-b border-gray-200 bg-gradient-to-r from-indigo-50 to-purple-50">
              <div class="flex items-center space-x-3">
                <div class="p-2 bg-indigo-100 rounded-lg">
                  <svg class="w-6 h-6 text-indigo-600" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd"></path>
                  </svg>
                </div>
                <div>
                  <h3 class="text-xl font-bold text-gray-900">
                    Utskickshistorik
                  </h3>
                  <p class="text-sm text-gray-600 mt-1">
                    Komplett historik över alla e-postutskick och deras prestanda
                  </p>
                </div>
              </div>
            </div>

            <div v-if="sentEmails.length === 0" class="text-center py-12">
              <div class="p-4 bg-gray-100 rounded-2xl mb-4 inline-block">
                <svg class="w-12 h-12 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                </svg>
              </div>
              <h3 class="text-lg font-medium text-gray-900 mb-2">Ingen historik än</h3>
              <p class="text-gray-500 mb-4">Skicka e-postutskick för att se historik och statistik här</p>
            </div>

            <div v-else class="overflow-x-auto">
              <table class="min-w-full divide-y divide-gray-200">
                <thead class="bg-gray-50">
                  <tr>
                    <th class="px-6 py-4 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                      Kampanj
                    </th>
                    <th class="px-6 py-4 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                      Mall
                    </th>
                    <th class="px-6 py-4 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                      Mottagare
                    </th>
                    <th class="px-6 py-4 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                      Levererat
                    </th>
                    <th class="px-6 py-4 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                      Öppnat
                    </th>
                    <th class="px-6 py-4 text-left text-xs font-semibold text-gray-600 uppercase tracking-wider">
                      Skickat
                    </th>
                  </tr>
                </thead>
                <tbody class="bg-white divide-y divide-gray-200">
                  <tr
                    v-for="send in sentEmails"
                    :key="send.id"
                    class="hover:bg-indigo-50 transition-colors duration-200"
                  >
                    <td class="px-6 py-4 whitespace-nowrap">
                      <div class="flex items-center space-x-3">
                        <div class="p-2 bg-indigo-100 rounded-lg">
                          <svg class="w-4 h-4 text-indigo-600" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path>
                          </svg>
                        </div>
                        <div>
                          <div class="text-sm font-semibold text-gray-900">{{ send.campaign_name }}</div>
                        </div>
                      </div>
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap">
                      <div class="text-sm text-gray-900">{{ send.template_name }}</div>
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap">
                      <div class="flex items-center space-x-2">
                        <svg class="w-4 h-4 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                          <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-6-3a2 2 0 11-4 0 2 2 0 014 0zM8 7a3 3 0 000 6h4a3 3 0 000-6H8z" clip-rule="evenodd"></path>
                        </svg>
                        <span class="text-sm font-medium text-gray-900">{{ send.to_count }}</span>
                      </div>
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap">
                      <div class="flex items-center space-x-2">
                        <span class="inline-flex items-center px-3 py-1 rounded-full text-xs font-semibold bg-green-100 text-green-800">
                          {{ send.delivered }}
                        </span>
                        <span class="text-xs text-gray-500">
                          {{ Math.round((send.delivered / send.to_count) * 100) }}%
                        </span>
                      </div>
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap">
                      <div class="flex items-center space-x-2">
                        <span class="inline-flex items-center px-3 py-1 rounded-full text-xs font-semibold bg-blue-100 text-blue-800">
                          {{ send.opened }}
                        </span>
                        <span class="text-xs text-gray-500">
                          {{ Math.round((send.opened / send.delivered) * 100) }}%
                        </span>
                      </div>
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap">
                      <div class="flex items-center space-x-2">
                        <svg class="w-4 h-4 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                          <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd"></path>
                        </svg>
                        <span class="text-sm text-gray-900">{{ formatDate(send.sent_at) }}</span>
                      </div>
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
        <div class="space-y-6">
          <!-- Template Preview - Only show when template is selected -->
          <div
            v-if="selectedTemplate && activeTab === 'templates'"
            class="bg-white rounded-2xl shadow-sm border border-gray-200 overflow-hidden"
          >
            <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-blue-50 to-indigo-50">
              <div class="flex items-center space-x-3">
                <div class="p-2 bg-blue-100 rounded-lg">
                  <svg class="w-5 h-5 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
                  </svg>
                </div>
                <h3 class="text-lg font-bold text-gray-900">
                  Förhandsvisning
                </h3>
              </div>
            </div>
            
            <div class="p-6">
              <div class="mb-4">
                <h4 class="font-semibold text-gray-900 mb-2">{{ selectedTemplate.subject }}</h4>
                <div class="text-sm text-gray-600 mb-4">{{ selectedTemplate.name }}</div>
              </div>
              
              <div class="border border-gray-200 rounded-xl bg-gray-50 overflow-hidden mb-4">
                <div class="bg-gray-100 px-3 py-2 border-b border-gray-200">
                  <div class="flex items-center space-x-2 text-xs text-gray-500">
                    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path>
                    </svg>
                    <span class="font-medium">FÖRHANDSVISNING</span>
                  </div>
                </div>
                <div class="p-4 bg-white">
                  <div
                    v-html="selectedTemplate.html"
                    class="text-sm text-gray-700 prose prose-sm max-w-none prose-headings:text-gray-900 prose-p:text-gray-700 prose-a:text-blue-600 prose-strong:text-gray-900"
                  ></div>
                </div>
              </div>
              
              <div class="flex space-x-3">
                <button
                  @click="editTemplate(selectedTemplate)"
                  class="flex-1 px-4 py-3 text-sm font-semibold text-white bg-blue-600 rounded-xl hover:bg-blue-700 transition-colors duration-200 flex items-center justify-center space-x-2"
                >
                  <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M13.586 3.586a2 2 0 112.828 2.828l-.793.793-2.828-2.828.793-.793zM11.379 5.793L3 14.172V17h2.828l8.38-8.379-2.83-2.828z"></path>
                  </svg>
                  <span>Redigera</span>
                </button>
                <button
                  @click="sendTestEmail"
                  class="flex-1 px-4 py-3 text-sm font-semibold text-gray-700 bg-gray-100 rounded-xl hover:bg-gray-200 transition-colors duration-200 flex items-center justify-center space-x-2"
                >
                  <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z"></path>
                    <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z"></path>
                  </svg>
                  <span>Skicka test</span>
                </button>
              </div>
            </div>
          </div>

          <!-- No Template Selected Placeholder -->
          <div
            v-else-if="activeTab === 'templates'"
            class="bg-white rounded-2xl shadow-sm border border-gray-200 overflow-hidden"
          >
            <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-blue-50">
              <div class="flex items-center space-x-3">
                <div class="p-2 bg-gray-100 rounded-lg">
                  <svg class="w-5 h-5 text-gray-600" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
                  </svg>
                </div>
                <h3 class="text-lg font-bold text-gray-900">
                  Välj en mall
                </h3>
              </div>
            </div>
            
            <div class="p-6 text-center">
              <div class="p-4 bg-gray-100 rounded-2xl mb-4 inline-block">
                <svg class="w-12 h-12 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
                </svg>
              </div>
              <p class="text-sm text-gray-500">Klicka på en mall för att se förhandsvisning</p>
            </div>
          </div>

          <!-- Recent Sends -->
          <div class="bg-white rounded-2xl shadow-sm border border-gray-200 overflow-hidden">
            <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-green-50 to-emerald-50">
              <div class="flex items-center space-x-3">
                <div class="p-2 bg-green-100 rounded-lg">
                  <svg class="w-5 h-5 text-green-600" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z"></path>
                    <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z"></path>
                  </svg>
                </div>
                <h3 class="text-lg font-bold text-gray-900">
                  Senaste utskick
                </h3>
              </div>
            </div>
            
            <div class="p-6">
              <div v-if="recentSends.length === 0" class="text-center py-8">
                <div class="p-3 bg-gray-100 rounded-xl mb-3 inline-block">
                  <svg class="w-8 h-8 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4"></path>
                  </svg>
                </div>
                <p class="text-sm text-gray-500">Inga utskick än</p>
              </div>
              
              <div v-else class="space-y-3">
                <div
                  v-for="send in recentSends"
                  :key="send.id"
                  class="flex items-center justify-between p-4 bg-gradient-to-r from-gray-50 to-blue-50 rounded-xl border border-gray-200 hover:shadow-sm transition-shadow duration-200"
                >
                  <div class="flex-1 min-w-0">
                    <div class="text-sm font-semibold text-gray-900 truncate">
                      {{ send.campaign_name }}
                    </div>
                    <div class="text-xs text-gray-500 mt-1">
                      {{ formatDate(send.sent_at) }}
                    </div>
                  </div>
                  <div class="text-right ml-3">
                    <div class="text-sm font-semibold text-gray-900">
                      {{ send.delivered }}/{{ send.to_count }}
                    </div>
                    <div class="text-xs text-gray-500">
                      {{ Math.round((send.opened / send.delivered) * 100) || 0 }}% öppnat
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- KPI Stats -->
          <div class="bg-white rounded-2xl shadow-sm border border-gray-200 overflow-hidden">
            <div class="px-6 py-4 border-b border-gray-200 bg-gradient-to-r from-purple-50 to-pink-50">
              <div class="flex items-center space-x-3">
                <div class="p-2 bg-purple-100 rounded-lg">
                  <svg class="w-5 h-5 text-purple-600" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M2 11a1 1 0 011-1h2a1 1 0 011 1v5a1 1 0 01-1 1H3a1 1 0 01-1-1v-5zM8 7a1 1 0 011-1h2a1 1 0 011 1v9a1 1 0 01-1 1H9a1 1 0 01-1-1V7zM14 4a1 1 0 011-1h2a1 1 0 011 1v12a1 1 0 01-1 1h-2a1 1 0 01-1-1V4z"></path>
                  </svg>
                </div>
                <h3 class="text-lg font-bold text-gray-900">Statistik</h3>
              </div>
            </div>
            
            <div class="p-6">
              <div class="space-y-4">
                <div class="flex items-center justify-between p-3 bg-gradient-to-r from-blue-50 to-indigo-50 rounded-xl">
                  <div class="flex items-center space-x-3">
                    <div class="p-2 bg-blue-100 rounded-lg">
                      <svg class="w-4 h-4 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M3.172 5.172a4 4 0 015.656 0L10 6.343l1.172-1.171a4 4 0 115.656 5.656L10 17.657l-6.828-6.829a4 4 0 010-5.656z" clip-rule="evenodd"></path>
                      </svg>
                    </div>
                    <span class="text-sm font-medium text-gray-700">Öppningsgrad</span>
                  </div>
                  <span class="text-lg font-bold text-blue-600">{{ overallStats.openRate }}%</span>
                </div>
                
                <div class="flex items-center justify-between p-3 bg-gradient-to-r from-green-50 to-emerald-50 rounded-xl">
                  <div class="flex items-center space-x-3">
                    <div class="p-2 bg-green-100 rounded-lg">
                      <svg class="w-4 h-4 text-green-600" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
                      </svg>
                    </div>
                    <span class="text-sm font-medium text-gray-700">Klickgrad</span>
                  </div>
                  <span class="text-lg font-bold text-green-600">{{ overallStats.clickRate }}%</span>
                </div>
                
                <div class="flex items-center justify-between p-3 bg-gradient-to-r from-red-50 to-pink-50 rounded-xl">
                  <div class="flex items-center space-x-3">
                    <div class="p-2 bg-red-100 rounded-lg">
                      <svg class="w-4 h-4 text-red-600" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"></path>
                      </svg>
                    </div>
                    <span class="text-sm font-medium text-gray-700">Studsgrad</span>
                  </div>
                  <span class="text-lg font-bold text-red-600">{{ overallStats.bounceRate }}%</span>
                </div>
                
                <div class="flex items-center justify-between p-3 bg-gradient-to-r from-gray-50 to-slate-50 rounded-xl">
                  <div class="flex items-center space-x-3">
                    <div class="p-2 bg-gray-100 rounded-lg">
                      <svg class="w-4 h-4 text-gray-600" fill="currentColor" viewBox="0 0 20 20">
                        <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z"></path>
                        <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z"></path>
                      </svg>
                    </div>
                    <span class="text-sm font-medium text-gray-700">Totalt skickat</span>
                  </div>
                  <span class="text-lg font-bold text-gray-600">{{ overallStats.totalSent }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </aside>
    </div>

    <!-- Template Editor Modal -->
    <div v-if="showTemplateEditor" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <!-- Backdrop -->
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="closeTemplateEditor"></div>
      
      <!-- Modal Content -->
      <div class="relative bg-white rounded-2xl shadow-2xl w-full max-w-8xl h-[95vh] flex flex-col overflow-hidden border border-gray-200">
        <!-- Header -->
        <div class="px-8 py-6 border-b border-gray-200 bg-gradient-to-r from-blue-50 via-indigo-50 to-purple-50">
          <div class="flex items-center justify-between">
            <div class="flex items-center space-x-4">
              <div class="p-3 bg-blue-100 rounded-xl">
                <svg class="w-6 h-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
                </svg>
              </div>
              <div>
                <h3 class="text-2xl font-bold text-gray-900">
                  {{ editingTemplate ? "Redigera mall" : "Skapa ny mall" }}
                </h3>
                <p class="text-sm text-gray-600 mt-1">
                  {{ editingTemplate ? "Uppdatera din e-postmall" : "Skapa en ny e-postmall från grunden" }}
                </p>
              </div>
            </div>
            <button
              @click="closeTemplateEditor"
              class="p-3 text-gray-400 hover:text-gray-600 hover:bg-white/80 rounded-xl transition-all duration-200 group"
            >
              <XMarkIcon class="w-6 h-6 group-hover:scale-110 transition-transform" />
            </button>
          </div>
        </div>

        <!-- Content -->
        <div class="flex-1 overflow-hidden flex">
          <!-- Left Side - Form -->
          <div class="w-1/2 p-8 border-r border-gray-200 overflow-y-auto bg-gradient-to-b from-gray-50 to-white">
            <div class="space-y-8">
              <div class="space-y-2">
                <label class="block text-sm font-semibold text-gray-800 flex items-center">
                  <svg class="w-4 h-4 mr-2 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M4 4a2 2 0 00-2 2v8a2 2 0 002 2h12a2 2 0 002-2V6a2 2 0 00-2-2H4zm2 6a2 2 0 114 0 2 2 0 01-4 0zm8 0a2 2 0 114 0 2 2 0 01-4 0z" clip-rule="evenodd"></path>
                  </svg>
                  Mallnamn
                </label>
                <input
                  v-model="templateForm.name"
                  type="text"
                  placeholder="T.ex. Veckorapport BRF"
                  class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-all duration-200 bg-white shadow-sm hover:shadow-md"
                />
              </div>

              <div class="space-y-2">
                <label class="block text-sm font-semibold text-gray-800 flex items-center">
                  <svg class="w-4 h-4 mr-2 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z"></path>
                    <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z"></path>
                  </svg>
                  E-postämne
                </label>
                <input
                  v-model="templateForm.subject"
                  type="text"
                  placeholder="T.ex. Veckorapport från Allanit Service"
                  class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-all duration-200 bg-white shadow-sm hover:shadow-md"
                />
              </div>

              <div class="space-y-2">
                <label class="block text-sm font-semibold text-gray-800 flex items-center">
                  <svg class="w-4 h-4 mr-2 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M17.707 9.293a1 1 0 010 1.414l-7 7a1 1 0 01-1.414 0l-7-7A.997.997 0 012 10V5a3 3 0 013-3h5c.256 0 .512.098.707.293l7 7zM5 6a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd"></path>
                  </svg>
                  Taggar
                </label>
                <input
                  v-model="templateForm.tagsString"
                  type="text"
                  placeholder="brf, vinter, info (komma-separerat)"
                  class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-all duration-200 bg-white shadow-sm hover:shadow-md"
                />
                <div class="flex items-start space-x-2 text-xs text-gray-500">
                  <svg class="w-4 h-4 mt-0.5 text-blue-500" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"></path>
                  </svg>
                  <span>Använd taggar för att kategorisera och organisera dina mallar</span>
                </div>
              </div>

              <div class="space-y-2">
                <label class="block text-sm font-semibold text-gray-800 flex items-center">
                  <svg class="w-4 h-4 mr-2 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path>
                  </svg>
                  HTML-innehåll
                </label>
                <div class="relative">
                  <textarea
                    v-model="templateForm.html"
                    placeholder="<h1>Hej!</h1><p>Detta är din mall...</p>"
                    class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-all duration-200 font-mono text-sm bg-white shadow-sm resize-none hover:shadow-md"
                    rows="20"
                  ></textarea>
                  <div class="absolute top-3 right-3 text-xs text-gray-400 bg-white px-2 py-1 rounded-md border">
                    HTML
                  </div>
                </div>
                <div class="flex items-start space-x-2 text-xs text-gray-500">
                  <svg class="w-4 h-4 mt-0.5 text-blue-500" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"></path>
                  </svg>
                  <span>Använd HTML för att formatera e-postmeddelandet professionellt</span>
                </div>
              </div>
            </div>
          </div>

          <!-- Right Side - Live Preview -->
          <div class="w-1/2 p-8 bg-white">
            <div class="h-full flex flex-col">
              <div class="flex items-center justify-between mb-6">
                <div class="flex items-center space-x-3">
                  <div class="p-2 bg-green-100 rounded-lg">
                    <svg class="w-5 h-5 text-green-600" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"></path>
                    </svg>
                  </div>
                  <h4 class="text-xl font-bold text-gray-900">
                    Live-förhandsvisning
                  </h4>
                </div>
                <div class="flex items-center space-x-2 text-sm text-gray-500 bg-green-50 px-3 py-1 rounded-full">
                  <div class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                  <span class="font-medium">Live</span>
                </div>
              </div>
              
              <div class="flex-1 border-2 border-gray-200 rounded-2xl bg-gray-50 overflow-hidden shadow-inner">
                <div class="h-full overflow-y-auto bg-white">
                  <div class="p-8">
                    <div v-if="templateForm.subject" class="mb-6 pb-4 border-b border-gray-200">
                      <div class="flex items-center space-x-2 mb-2">
                        <svg class="w-4 h-4 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                          <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z"></path>
                          <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z"></path>
                        </svg>
                        <span class="text-xs text-gray-500 font-medium">ÄMNE</span>
                      </div>
                      <h2 class="text-xl font-bold text-gray-900">
                        {{ templateForm.subject }}
                      </h2>
                    </div>
                    <div v-if="templateForm.html" class="space-y-4">
                      <div class="flex items-center space-x-2 mb-4">
                        <svg class="w-4 h-4 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                          <path fill-rule="evenodd" d="M3 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm0 4a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z" clip-rule="evenodd"></path>
                        </svg>
                        <span class="text-xs text-gray-500 font-medium">INNEHÅLL</span>
                      </div>
                      <div
                        v-html="templateForm.html"
                        class="text-gray-700 prose prose-sm max-w-none prose-headings:text-gray-900 prose-p:text-gray-700 prose-a:text-blue-600 prose-strong:text-gray-900"
                      ></div>
                    </div>
                    <div v-else class="flex items-center justify-center h-64 text-gray-400">
                      <div class="text-center">
                        <div class="p-4 bg-gray-100 rounded-2xl mb-4">
                          <svg class="w-12 h-12 mx-auto text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
                          </svg>
                        </div>
                        <p class="text-sm font-medium">Skriv HTML-innehåll för att se förhandsvisning</p>
                        <p class="text-xs text-gray-400 mt-1">Förhandsvisningen uppdateras automatiskt</p>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Footer -->
        <div class="px-8 py-6 border-t border-gray-200 bg-gradient-to-r from-gray-50 to-white flex justify-between items-center">
          <div class="text-sm text-gray-500">
            <span class="font-medium">{{ editingTemplate ? "Redigerar" : "Skapar" }}:</span>
            <span class="ml-1">{{ templateForm.name || "Namnlös mall" }}</span>
          </div>
          <div class="flex space-x-4">
            <button
              @click="closeTemplateEditor"
              class="px-6 py-3 text-sm font-semibold text-gray-700 bg-white border border-gray-300 rounded-xl hover:bg-gray-50 hover:border-gray-400 transition-all duration-200 shadow-sm hover:shadow-md"
            >
              Avbryt
            </button>
            <button
              @click="saveTemplate"
              class="px-8 py-3 text-sm font-semibold text-white bg-blue-600 border border-transparent rounded-xl hover:bg-blue-700 hover:shadow-lg transition-all duration-200 shadow-sm disabled:opacity-50 disabled:cursor-not-allowed"
              :disabled="!templateForm.name || !templateForm.subject"
            >
              {{ editingTemplate ? "Uppdatera mall" : "Skapa mall" }}
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

// Carousel state
const currentTemplateIndex = ref(0);
const templatesPerPage = ref(2);

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

// Computed property for selected template in send tab
const selectedTemplateForSend = computed(() => {
  return templates.value.find(t => t.id === emailForm.value.templateId);
});

// Carousel computed properties
const visibleTemplates = computed(() => {
  return templates.value.slice(
    currentTemplateIndex.value,
    currentTemplateIndex.value + templatesPerPage.value
  );
});

// Function to get recipient count
function getRecipientCount() {
  const segmentCount = emailForm.value.segmentIds.length;
  const customCount = emailForm.value.customRecipients
    .split('\n')
    .filter(email => email.trim()).length;
  return segmentCount + customCount;
}

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

// Function to create a short preview text from HTML
function getPreviewText(html: string): string {
  // Remove HTML tags and get plain text
  const tempDiv = document.createElement('div');
  tempDiv.innerHTML = html;
  const text = tempDiv.textContent || tempDiv.innerText || '';
  
  // Limit to first 100 characters
  const preview = text.trim().substring(0, 100);
  
  // Add ellipsis if text was truncated
  return preview.length < text.trim().length ? preview + '...' : preview;
}

// Carousel functions
function previousTemplates() {
  if (currentTemplateIndex.value > 0) {
    currentTemplateIndex.value -= templatesPerPage.value;
  }
}

function nextTemplates() {
  if (currentTemplateIndex.value < templates.value.length - templatesPerPage.value) {
    currentTemplateIndex.value += templatesPerPage.value;
  }
}

watch(activeTab, () => {
  loadData();
});

onMounted(() => {
  loadData();
});
</script>
