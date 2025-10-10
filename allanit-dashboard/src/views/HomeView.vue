<template>
  <div class="space-y-8">
    <!-- Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-3xl font-bold text-gray-900">Översikt</h1>
        <p class="text-gray-600 mt-2">
          Välkommen till Allanit Dashboard - Snabb överblick över verksamheten
        </p>
      </div>
      <div class="flex items-center space-x-3">
        <button
          @click="refreshData"
          class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-xl hover:bg-gray-50 hover:border-gray-400 transition-all duration-200 shadow-sm hover:shadow-md flex items-center space-x-2"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"></path>
          </svg>
          <span>Uppdatera</span>
        </button>
      </div>
    </div>

    <!-- Key Metrics Dashboard -->
    <div v-if="isAdmin" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 sm:gap-6">
      <div class="bg-gradient-to-br from-blue-50 to-indigo-100 rounded-2xl p-6 border border-blue-200 shadow-sm hover:shadow-md transition-shadow duration-200">
        <div class="flex items-center justify-between mb-4">
          <div class="p-3 bg-blue-100 rounded-xl">
            <UserGroupIcon class="w-6 h-6 text-blue-600" />
          </div>
          <div class="text-right">
            <div class="text-2xl font-bold text-blue-900">{{ totalCustomers }}</div>
            <div class="text-sm text-blue-600">Aktiva kunder</div>
          </div>
        </div>
        <div class="flex items-center text-sm text-blue-700">
          <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M5.293 7.707a1 1 0 010-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 01-1.414 1.414L11 5.414V17a1 1 0 11-2 0V5.414L6.707 7.707a1 1 0 01-1.414 0z" clip-rule="evenodd"></path>
          </svg>
          <span class="font-medium">+2 denna månad</span>
        </div>
      </div>

      <div class="bg-gradient-to-br from-green-50 to-emerald-100 rounded-2xl p-6 border border-green-200 shadow-sm hover:shadow-md transition-shadow duration-200">
        <div class="flex items-center justify-between mb-4">
          <div class="p-3 bg-green-100 rounded-xl">
            <ShoppingBagIcon class="w-6 h-6 text-green-600" />
          </div>
          <div class="text-right">
            <div class="text-2xl font-bold text-green-900">{{ totalActiveOrders }}</div>
            <div class="text-sm text-green-600">Aktiva uppdrag</div>
          </div>
        </div>
        <div class="flex items-center text-sm text-green-700">
          <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M5.293 7.707a1 1 0 010-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 01-1.414 1.414L11 5.414V17a1 1 0 11-2 0V5.414L6.707 7.707a1 1 0 01-1.414 0z" clip-rule="evenodd"></path>
          </svg>
          <span class="font-medium">+1 denna vecka</span>
        </div>
      </div>

      <div class="bg-gradient-to-br from-purple-50 to-pink-100 rounded-2xl p-6 border border-purple-200 shadow-sm hover:shadow-md transition-shadow duration-200">
        <div class="flex items-center justify-between mb-4">
          <div class="p-3 bg-purple-100 rounded-xl">
            <CurrencyDollarIcon class="w-6 h-6 text-purple-600" />
          </div>
          <div class="text-right">
            <div class="text-2xl font-bold text-purple-900">{{ formatCurrency(totalRevenue) }}</div>
            <div class="text-sm text-purple-600">Månadsomsättning</div>
          </div>
        </div>
        <div class="flex items-center text-sm text-purple-700">
          <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M5.293 7.707a1 1 0 010-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 01-1.414 1.414L11 5.414V17a1 1 0 11-2 0V5.414L6.707 7.707a1 1 0 01-1.414 0z" clip-rule="evenodd"></path>
          </svg>
          <span class="font-medium">+12% från förra månaden</span>
        </div>
      </div>

      <div class="bg-gradient-to-br from-orange-50 to-red-100 rounded-2xl p-6 border border-orange-200 shadow-sm hover:shadow-md transition-shadow duration-200">
        <div class="flex items-center justify-between mb-4">
          <div class="p-3 bg-orange-100 rounded-xl">
            <UserIcon class="w-6 h-6 text-orange-600" />
          </div>
          <div class="text-right">
            <div class="text-2xl font-bold text-orange-900">{{ totalEmployees }}</div>
            <div class="text-sm text-orange-600">Medarbetare</div>
          </div>
        </div>
        <div class="flex items-center text-sm text-orange-700">
          <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-8.293l-3-3a1 1 0 00-1.414 0l-3 3a1 1 0 001.414 1.414L9 9.414V13a1 1 0 102 0V9.414l1.293 1.293a1 1 0 001.414-1.414z" clip-rule="evenodd"></path>
          </svg>
          <span class="font-medium">Stabil</span>
        </div>
      </div>
    </div>

    <!-- Employee Dashboard -->
    <div v-if="!isAdmin" class="space-y-6">
      <!-- Employee Quick Stats -->
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 sm:gap-6">
        <div class="bg-gradient-to-br from-blue-50 to-indigo-100 rounded-2xl p-6 border border-blue-200 shadow-sm hover:shadow-md transition-shadow duration-200">
          <div class="flex items-center justify-between mb-4">
            <div class="p-3 bg-blue-100 rounded-xl">
              <ShoppingBagIcon class="w-6 h-6 text-blue-600" />
            </div>
            <div class="text-right">
              <div class="text-2xl font-bold text-blue-900">{{ myOrderStats.total }}</div>
              <div class="text-sm text-blue-600">Mina uppdrag</div>
            </div>
          </div>
          <div class="flex items-center text-sm text-blue-700">
            <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M5.293 7.707a1 1 0 010-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 01-1.414 1.414L11 5.414V17a1 1 0 11-2 0V5.414L6.707 7.707a1 1 0 01-1.414 0z" clip-rule="evenodd"></path>
            </svg>
            <span class="font-medium">{{ myOrderStats.inProgress }} pågående</span>
          </div>
        </div>

        <div class="bg-gradient-to-br from-green-50 to-emerald-100 rounded-2xl p-6 border border-green-200 shadow-sm hover:shadow-md transition-shadow duration-200">
          <div class="flex items-center justify-between mb-4">
            <div class="p-3 bg-green-100 rounded-xl">
              <svg class="w-6 h-6 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
              </svg>
            </div>
            <div class="text-right">
              <div class="text-2xl font-bold text-green-900">{{ myOrderStats.completedThisWeek }}</div>
              <div class="text-sm text-green-600">Slutförda denna vecka</div>
            </div>
          </div>
          <div class="flex items-center text-sm text-green-700">
            <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-8.293l-3-3a1 1 0 00-1.414 0l-3 3a1 1 0 001.414 1.414L9 9.414V13a1 1 0 102 0V9.414l1.293 1.293a1 1 0 001.414-1.414z" clip-rule="evenodd"></path>
            </svg>
            <span class="font-medium">Bra jobbat!</span>
          </div>
        </div>

        <div class="bg-gradient-to-br from-purple-50 to-pink-100 rounded-2xl p-6 border border-purple-200 shadow-sm hover:shadow-md transition-shadow duration-200">
          <div class="flex items-center justify-between mb-4">
            <div class="p-3 bg-purple-100 rounded-xl">
              <svg class="w-6 h-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"></path>
              </svg>
            </div>
            <div class="text-right">
              <div class="text-2xl font-bold text-purple-900">{{ myMachines.length }}</div>
              <div class="text-sm text-purple-600">Mina maskiner</div>
            </div>
          </div>
          <div class="flex items-center text-sm text-purple-700">
            <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-8.293l-3-3a1 1 0 00-1.414 0l-3 3a1 1 0 001.414 1.414L9 9.414V13a1 1 0 102 0V9.414l1.293 1.293a1 1 0 001.414-1.414z" clip-rule="evenodd"></path>
            </svg>
            <span class="font-medium">Under ditt ansvar</span>
          </div>
        </div>
      </div>

      <!-- Employee Content Grid -->
      <div class="grid grid-cols-1 xl:grid-cols-2 gap-6 lg:gap-8">
        <!-- My Orders -->
        <div class="bg-white rounded-2xl shadow-sm border border-gray-200 overflow-hidden">
          <div class="px-8 py-6 border-b border-gray-200 bg-gradient-to-r from-blue-50 to-indigo-50">
            <h3 class="text-xl font-bold text-gray-900 flex items-center">
              <ShoppingBagIcon class="w-6 h-6 mr-3 text-blue-600" />
              Mina uppdrag
            </h3>
            <p class="text-sm text-gray-600 mt-1">Dina tilldelade uppdrag</p>
          </div>
          <div class="p-8">
            <div v-if="loading" class="space-y-4">
              <div v-for="i in 3" :key="i" class="animate-pulse">
                <div class="h-16 bg-gray-200 rounded-xl"></div>
              </div>
            </div>
            <div v-else-if="myOrders.length === 0" class="text-center py-8">
              <ShoppingBagIcon class="w-12 h-12 text-gray-300 mx-auto mb-4" />
              <h3 class="text-lg font-medium text-gray-900 mb-2">Inga uppdrag</h3>
              <p class="text-gray-600">Du har inga tilldelade uppdrag för tillfället.</p>
            </div>
            <div v-else class="space-y-4">
              <div
                v-for="order in myOrders.slice(0, 5)"
                :key="order.id"
                class="flex items-center justify-between p-4 bg-gradient-to-r from-gray-50 to-blue-50 rounded-xl border border-gray-200 hover:shadow-sm transition-shadow duration-200"
              >
                <div class="flex-1">
                  <div class="font-semibold text-gray-900">{{ order.external_id }}</div>
                  <div class="text-sm text-gray-600">{{ order.description }}</div>
                  <div class="flex items-center space-x-2 mt-2">
                    <span 
                      class="px-2 py-1 text-xs font-medium rounded-full"
                      :class="getStatusColor(order.status)"
                    >
                      {{ getStatusLabel(order.status) }}
                    </span>
                    <span v-if="order.dueDate" class="text-xs text-gray-500">
                      Förfaller: {{ formatDate(order.dueDate) }}
                    </span>
                  </div>
                </div>
                <button 
                  @click="$router.push('/my-orders')"
                  class="text-sm text-blue-600 hover:text-blue-800"
                >
                  Hantera →
                </button>
              </div>
            </div>
          </div>
        </div>

        <!-- My Machines -->
        <div class="bg-white rounded-2xl shadow-sm border border-gray-200 overflow-hidden">
          <div class="px-8 py-6 border-b border-gray-200 bg-gradient-to-r from-purple-50 to-pink-50">
            <h3 class="text-xl font-bold text-gray-900 flex items-center">
              <svg class="w-6 h-6 mr-3 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"></path>
              </svg>
              Mina maskiner
            </h3>
            <p class="text-sm text-gray-600 mt-1">Maskiner under ditt ansvar</p>
          </div>
          <div class="p-8">
            <div v-if="loading" class="space-y-4">
              <div v-for="i in 3" :key="i" class="animate-pulse">
                <div class="h-16 bg-gray-200 rounded-xl"></div>
              </div>
            </div>
            <div v-else-if="myMachines.length === 0" class="text-center py-8">
              <svg class="w-12 h-12 text-gray-300 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"></path>
              </svg>
              <h3 class="text-lg font-medium text-gray-900 mb-2">Inga maskiner</h3>
              <p class="text-gray-600">Du har inga maskiner under ditt ansvar.</p>
            </div>
            <div v-else class="space-y-4">
              <div
                v-for="machine in myMachines.slice(0, 5)"
                :key="machine.id"
                class="flex items-center justify-between p-4 bg-gradient-to-r from-gray-50 to-purple-50 rounded-xl border border-gray-200 hover:shadow-sm transition-shadow duration-200"
              >
                <div class="flex items-center space-x-4">
                  <img
                    :src="machine.image"
                    :alt="machine.name"
                    class="w-12 h-12 rounded-xl object-cover shadow-sm"
                    @error="handleImageError"
                  />
                  <div>
                    <div class="font-semibold text-gray-900">{{ machine.name }}</div>
                    <div class="text-sm text-gray-600">{{ machine.model }}</div>
                    <div class="text-xs text-gray-500 mt-1">
                      Nästa service: {{ formatDate(machine.nextServiceDate) }}
                    </div>
                  </div>
                </div>
                <button 
                  @click="$router.push('/machines')"
                  class="text-sm text-purple-600 hover:text-purple-800"
                >
                  Hantera →
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Recent Activity -->
      <div class="bg-white rounded-2xl shadow-sm border border-gray-200 overflow-hidden">
        <div class="px-8 py-6 border-b border-gray-200 bg-gradient-to-r from-green-50 to-emerald-50">
          <h3 class="text-xl font-bold text-gray-900 flex items-center">
            <svg class="w-6 h-6 mr-3 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
            Senaste aktivitet
          </h3>
          <p class="text-sm text-gray-600 mt-1">Dina senaste journaler och kommentarer</p>
        </div>
        <div class="p-8">
          <div v-if="loading" class="space-y-4">
            <div v-for="i in 3" :key="i" class="animate-pulse">
              <div class="h-16 bg-gray-200 rounded-xl"></div>
            </div>
          </div>
          <div v-else-if="myRecentJournals.length === 0" class="text-center py-8">
            <svg class="w-12 h-12 text-gray-300 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
            <h3 class="text-lg font-medium text-gray-900 mb-2">Ingen aktivitet</h3>
            <p class="text-gray-600">Du har inga senaste journaler eller kommentarer.</p>
          </div>
          <div v-else class="space-y-4">
            <div
              v-for="journal in myRecentJournals"
              :key="journal.id"
              class="flex items-start space-x-4 p-4 bg-gradient-to-r from-gray-50 to-green-50 rounded-xl border border-gray-200"
            >
              <div class="p-2 bg-green-100 rounded-lg">
                <svg class="w-5 h-5 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
                </svg>
              </div>
              <div class="flex-1">
                <div class="font-semibold text-gray-900">{{ journal.title }}</div>
                <div class="text-sm text-gray-600 mt-1">{{ journal.body }}</div>
                <div class="text-xs text-gray-500 mt-2">{{ formatDate(journal.createdAt) }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Quick Actions Grid -->
    <div class="bg-white rounded-2xl shadow-sm border border-gray-200 overflow-hidden">
      <div class="px-8 py-6 border-b border-gray-200 bg-gradient-to-r from-gray-50 to-blue-50">
        <h3 class="text-xl font-bold text-gray-900 flex items-center">
          <svg class="w-6 h-6 mr-3 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
            <path fill-rule="evenodd" d="M11.3 1.046A1 1 0 0112 2v5h4a1 1 0 01.82 1.573l-7 10A1 1 0 018 18v-5H4a1 1 0 01-.82-1.573l7-10a1 1 0 011.12-.38z" clip-rule="evenodd"></path>
          </svg>
          {{ isAdmin ? 'Snabbåtgärder' : 'Snabbvägar' }}
        </h3>
        <p class="text-sm text-gray-600 mt-1">{{ isAdmin ? 'Här kan du snabbt komma åt de vanligaste funktionerna' : 'Snabb åtkomst till dina viktigaste funktioner' }}</p>
      </div>
      
      <div class="p-8">
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4 sm:gap-6">
          <button
            v-if="isAdmin"
            @click="$router.push('/email-management')"
            class="group flex flex-col items-center p-6 text-gray-600 hover:text-blue-600 hover:bg-blue-50 rounded-2xl transition-all duration-200 border border-transparent hover:border-blue-200 hover:shadow-md"
          >
            <div class="p-4 bg-blue-100 rounded-2xl mb-3 group-hover:bg-blue-200 transition-colors duration-200">
              <MegaphoneIcon class="w-8 h-8 text-blue-600" />
            </div>
            <span class="text-sm font-semibold text-center">E-posthantering</span>
            <span class="text-xs text-gray-500 text-center mt-1">Skicka utskick</span>
          </button>

          <button
            v-if="isAdmin"
            @click="$router.push('/invoices')"
            class="group flex flex-col items-center p-6 text-gray-600 hover:text-green-600 hover:bg-green-50 rounded-2xl transition-all duration-200 border border-transparent hover:border-green-200 hover:shadow-md"
          >
            <div class="p-4 bg-green-100 rounded-2xl mb-3 group-hover:bg-green-200 transition-colors duration-200">
              <DocumentTextIcon class="w-8 h-8 text-green-600" />
            </div>
            <span class="text-sm font-semibold text-center">Fakturor</span>
            <span class="text-xs text-gray-500 text-center mt-1">Skapa & hantera</span>
          </button>

          <button
            v-if="isAdmin"
            @click="$router.push('/employees')"
            class="group flex flex-col items-center p-6 text-gray-600 hover:text-purple-600 hover:bg-purple-50 rounded-2xl transition-all duration-200 border border-transparent hover:border-purple-200 hover:shadow-md"
          >
            <div class="p-4 bg-purple-100 rounded-2xl mb-3 group-hover:bg-purple-200 transition-colors duration-200">
              <UserIcon class="w-8 h-8 text-purple-600" />
            </div>
            <span class="text-sm font-semibold text-center">Medarbetare</span>
            <span class="text-xs text-gray-500 text-center mt-1">Hantera team</span>
          </button>

          <button
            v-if="isAdmin"
            @click="$router.push('/customers')"
            class="group flex flex-col items-center p-6 text-gray-600 hover:text-orange-600 hover:bg-orange-50 rounded-2xl transition-all duration-200 border border-transparent hover:border-orange-200 hover:shadow-md"
          >
            <div class="p-4 bg-orange-100 rounded-2xl mb-3 group-hover:bg-orange-200 transition-colors duration-200">
              <UserGroupIcon class="w-8 h-8 text-orange-600" />
            </div>
            <span class="text-sm font-semibold text-center">Kunder</span>
            <span class="text-xs text-gray-500 text-center mt-1">Kundregister</span>
          </button>

          <button
            v-if="isAdmin"
            @click="$router.push('/orders')"
            class="group flex flex-col items-center p-6 text-gray-600 hover:text-indigo-600 hover:bg-indigo-50 rounded-2xl transition-all duration-200 border border-transparent hover:border-indigo-200 hover:shadow-md"
          >
            <div class="p-4 bg-indigo-100 rounded-2xl mb-3 group-hover:bg-indigo-200 transition-colors duration-200">
              <ShoppingBagIcon class="w-8 h-8 text-indigo-600" />
            </div>
            <span class="text-sm font-semibold text-center">Uppdrag</span>
            <span class="text-xs text-gray-500 text-center mt-1">Beställningar</span>
          </button>

          <button
            v-if="isAdmin"
            @click="$router.push('/subsidiaries')"
            class="group flex flex-col items-center p-6 text-gray-600 hover:text-pink-600 hover:bg-pink-50 rounded-2xl transition-all duration-200 border border-transparent hover:border-pink-200 hover:shadow-md"
          >
            <div class="p-4 bg-pink-100 rounded-2xl mb-3 group-hover:bg-pink-200 transition-colors duration-200">
              <BuildingOfficeIcon class="w-8 h-8 text-pink-600" />
            </div>
            <span class="text-sm font-semibold text-center">Dotterbolag</span>
            <span class="text-xs text-gray-500 text-center mt-1">Bolagsinfo</span>
          </button>

          <!-- Employee-specific quick actions -->
          <button
            v-if="!isAdmin"
            @click="$router.push('/my-orders')"
            class="group flex flex-col items-center p-6 text-gray-600 hover:text-blue-600 hover:bg-blue-50 rounded-2xl transition-all duration-200 border border-transparent hover:border-blue-200 hover:shadow-md"
          >
            <div class="p-4 bg-blue-100 rounded-2xl mb-3 group-hover:bg-blue-200 transition-colors duration-200">
              <ShoppingBagIcon class="w-8 h-8 text-blue-600" />
            </div>
            <span class="text-sm font-semibold text-center">Mina uppdrag</span>
            <span class="text-xs text-gray-500 text-center mt-1">Hantera uppdrag</span>
          </button>

          <button
            v-if="!isAdmin"
            @click="$router.push('/machines')"
            class="group flex flex-col items-center p-6 text-gray-600 hover:text-purple-600 hover:bg-purple-50 rounded-2xl transition-all duration-200 border border-transparent hover:border-purple-200 hover:shadow-md"
          >
            <div class="p-4 bg-purple-100 rounded-2xl mb-3 group-hover:bg-purple-200 transition-colors duration-200">
              <svg class="w-8 h-8 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"></path>
              </svg>
            </div>
            <span class="text-sm font-semibold text-center">Maskinpark</span>
            <span class="text-xs text-gray-500 text-center mt-1">Mina maskiner</span>
          </button>

          <button
            v-if="!isAdmin"
            @click="$router.push('/logs')"
            class="group flex flex-col items-center p-6 text-gray-600 hover:text-green-600 hover:bg-green-50 rounded-2xl transition-all duration-200 border border-transparent hover:border-green-200 hover:shadow-md"
          >
            <div class="p-4 bg-green-100 rounded-2xl mb-3 group-hover:bg-green-200 transition-colors duration-200">
              <svg class="w-8 h-8 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"></path>
              </svg>
            </div>
            <span class="text-sm font-semibold text-center">Journaler</span>
            <span class="text-xs text-gray-500 text-center mt-1">Skriv journal</span>
          </button>
        </div>
      </div>
    </div>

    <!-- Recent Activity (Admin only) -->
    <div v-if="isAdmin" class="grid grid-cols-1 xl:grid-cols-2 gap-6 lg:gap-8">
      <!-- Recent Orders -->
      <div class="bg-white rounded-2xl shadow-sm border border-gray-200 overflow-hidden">
        <div class="px-8 py-6 border-b border-gray-200 bg-gradient-to-r from-blue-50 to-indigo-50">
          <h3 class="text-xl font-bold text-gray-900 flex items-center">
            <svg class="w-6 h-6 mr-3 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M10 2L3 7v11a1 1 0 001 1h12a1 1 0 001-1V7l-7-5zM8 15a1 1 0 11-2 0 1 1 0 012 0zm4 0a1 1 0 11-2 0 1 1 0 012 0z" clip-rule="evenodd"></path>
            </svg>
            Senaste uppdrag
          </h3>
        </div>
        <div class="p-8">
          <div v-if="loading" class="space-y-4">
            <div v-for="i in 3" :key="i" class="animate-pulse">
              <div class="h-16 bg-gray-200 rounded-xl"></div>
            </div>
          </div>
          <div v-else class="space-y-4">
            <div
              v-for="order in recentOrders"
              :key="order.id"
              class="flex items-center justify-between p-4 bg-gradient-to-r from-gray-50 to-blue-50 rounded-xl border border-gray-200 hover:shadow-sm transition-shadow duration-200"
            >
              <div class="flex items-center space-x-4">
                <img
                  :src="getOrderImage(order.description)"
                  :alt="order.description"
                  class="w-12 h-12 rounded-xl object-cover shadow-sm"
                />
                <div>
                  <div class="font-semibold text-gray-900">{{ order.description }}</div>
                  <div class="text-sm text-gray-600">{{ order.customer_name }}</div>
                </div>
              </div>
              <div class="text-right">
                <div class="font-bold text-gray-900">{{ formatCurrency(order.amount) }}</div>
                <div class="text-sm text-gray-500">{{ formatDate(order.created_at) }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Recent Emails -->
      <div class="bg-white rounded-2xl shadow-sm border border-gray-200 overflow-hidden">
        <div class="px-8 py-6 border-b border-gray-200 bg-gradient-to-r from-green-50 to-emerald-50">
          <h3 class="text-xl font-bold text-gray-900 flex items-center">
            <svg class="w-6 h-6 mr-3 text-green-600" fill="currentColor" viewBox="0 0 20 20">
              <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z"></path>
              <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z"></path>
            </svg>
            Senaste e-postutskick
          </h3>
        </div>
        <div class="p-8">
          <div v-if="loading" class="space-y-4">
            <div v-for="i in 3" :key="i" class="animate-pulse">
              <div class="h-16 bg-gray-200 rounded-xl"></div>
            </div>
          </div>
          <div v-else class="space-y-4">
            <div
              v-for="email in recentEmails"
              :key="email.id"
              class="flex items-center justify-between p-4 bg-gradient-to-r from-gray-50 to-green-50 rounded-xl border border-gray-200 hover:shadow-sm transition-shadow duration-200"
            >
              <div>
                <div class="font-semibold text-gray-900">{{ email.campaign_name }}</div>
                <div class="text-sm text-gray-600">{{ email.template_name }}</div>
              </div>
              <div class="text-right">
                <div class="font-bold text-gray-900">{{ email.recipient_count }} mottagare</div>
                <div class="text-sm text-gray-500">{{ formatDate(email.sent_at) }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import {
  UserGroupIcon,
  ShoppingBagIcon,
  CurrencyDollarIcon,
  UserIcon,
  MegaphoneIcon,
  DocumentTextIcon,
} from "@heroicons/vue/24/outline";
import KpiCard from "@/components/ui/KpiCard.vue";
import DataCard from "@/components/ui/DataCard.vue";
import { http } from "@/api/mockClient";

const loading = ref(false);
const subsidiaries = ref<any[]>([]);
const customers = ref<any[]>([]);
const orders = ref<any[]>([]);
const employees = ref<any[]>([]);
const sentEmails = ref<any[]>([]);
const machines = ref<any[]>([]);
const journals = ref<any[]>([]);

// Role detection
const currentUser = ref(JSON.parse(localStorage.getItem('user') || '{}'));
const isAdmin = computed(() => currentUser.value.role?.toLowerCase() === 'administrator');

const totalCustomers = computed(() => customers.value.length);
const totalActiveOrders = computed(
  () =>
    orders.value.filter(
      (o) => o.status === "open" || o.status === "in_progress",
    ).length,
);
const totalRevenue = computed(() =>
  subsidiaries.value.reduce((sum, s) => sum + s.monthly_revenue, 0),
);
const totalEmployees = computed(() =>
  subsidiaries.value.reduce((sum, s) => sum + s.employees, 0),
);

const recentOrders = computed(() =>
  orders.value
    .sort(
      (a, b) =>
        new Date(b.created_at).getTime() - new Date(a.created_at).getTime(),
    )
    .slice(0, 5),
);

const recentEmails = computed(() =>
  sentEmails.value
    .sort(
      (a, b) => new Date(b.sent_at).getTime() - new Date(a.sent_at).getTime(),
    )
    .slice(0, 5),
);

// Employee-specific computed properties
const myOrders = computed(() => 
  orders.value.filter(order => order.assigned_employee_id === currentUser.value.employeeId)
);

const myMachines = computed(() => 
  machines.value.filter(machine => machine.responsibleEmployeeId === currentUser.value.employeeId)
);

const myRecentJournals = computed(() => 
  journals.value
    .filter(journal => journal.employeeId === currentUser.value.employeeId?.toString())
    .sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime())
    .slice(0, 5)
);

const myOrderStats = computed(() => ({
  total: myOrders.value.length,
  inProgress: myOrders.value.filter(o => o.status === 'in_progress').length,
  completedThisWeek: myOrders.value.filter(o => {
    if (o.status !== 'completed') return false;
    const completedDate = new Date(o.updated_at || o.created_at);
    const weekAgo = new Date();
    weekAgo.setDate(weekAgo.getDate() - 7);
    return completedDate >= weekAgo;
  }).length
}));

async function loadData() {
  loading.value = true;
  try {
    const [subsidiariesRes, customersRes, ordersRes, employeesRes, emailsRes, machinesRes, journalsRes] =
      await Promise.all([
        http.get("/api/subsidiaries"),
        http.get("/api/customers"),
        http.get("/api/purchase-orders"),
        http.get("/api/employees"),
        http.get("/api/sent-emails"),
        http.get("/api/machines"),
        http.get("/api/journals"),
      ]);

    subsidiaries.value = subsidiariesRes.data.results;
    customers.value = customersRes.data.results;
    orders.value = ordersRes.data.results;
    employees.value = employeesRes.data.results;
    sentEmails.value = emailsRes.data.results;
    machines.value = machinesRes.data.results;
    journals.value = journalsRes.data.results;
  } catch (error) {
    console.error("Error loading data:", error);
  } finally {
    loading.value = false;
  }
}

function refreshData() {
  loadData();
}

function formatCurrency(amount: number): string {
  return new Intl.NumberFormat("sv-SE", {
    style: "currency",
    currency: "SEK",
    minimumFractionDigits: 0,
  }).format(amount);
}

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString("sv-SE");
}

function getOrderImage(description: string): string {
  if (
    description.toLowerCase().includes("snöröjning") ||
    description.toLowerCase().includes("vinter")
  ) {
    return "/updrag/Snöröjning.png";
  } else if (description.toLowerCase().includes("miljörum")) {
    return "/updrag/Miljörum.png";
  }
  return "/Generiskbostadsrätt.png";
}

function getStatusColor(status: string): string {
  const colors = {
    not_planned: 'bg-gray-100 text-gray-800',
    planned: 'bg-blue-100 text-blue-800',
    in_progress: 'bg-yellow-100 text-yellow-800',
    paused: 'bg-orange-100 text-orange-800',
    waiting_for_materials: 'bg-purple-100 text-purple-800',
    blocked: 'bg-red-100 text-red-800',
    completed: 'bg-green-100 text-green-800',
    cancelled: 'bg-red-100 text-red-800'
  }
  return colors[status as keyof typeof colors] || 'bg-gray-100 text-gray-800'
}

function getStatusLabel(status: string): string {
  const labels = {
    not_planned: 'Ej planerad',
    planned: 'Planerad',
    in_progress: 'Pågående',
    paused: 'Pausad',
    waiting_for_materials: 'Väntar på material',
    blocked: 'Blockerad',
    completed: 'Slutförd',
    cancelled: 'Avbruten'
  }
  return labels[status as keyof typeof labels] || status
}

function handleImageError(event: Event) {
  const img = event.target as HTMLImageElement;
  img.src = "/maskiner/default.jpg";
}

onMounted(() => {
  loadData();
});
</script>

