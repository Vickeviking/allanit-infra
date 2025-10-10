<template>
  <div class="space-y-6">
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <div>
        <h1 class="text-2xl font-bold text-gray-900">Uppdrag</h1>
        <p class="text-gray-600">Hantera alla uppdrag och tilldelningar</p>
      </div>
      <div class="flex items-center space-x-3">
        <button
          @click="createOrder"
          class="px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700"
        >
          Skapa nytt uppdrag
        </button>
      </div>
    </div>

    <!-- Status Summary Cards -->
    <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-3 sm:gap-4 mb-6">
      <div class="bg-white rounded-lg shadow-sm border p-4">
        <div class="flex items-center">
          <div class="w-3 h-3 bg-gray-500 rounded-full mr-3"></div>
          <div>
            <p class="text-sm font-medium text-gray-900">Ej planerade</p>
            <p class="text-2xl font-bold text-gray-600">{{ notPlannedCount }}</p>
          </div>
        </div>
      </div>
      <div class="bg-white rounded-lg shadow-sm border p-4">
        <div class="flex items-center">
          <div class="w-3 h-3 bg-blue-500 rounded-full mr-3"></div>
          <div>
            <p class="text-sm font-medium text-gray-900">Planerade</p>
            <p class="text-2xl font-bold text-blue-600">{{ plannedCount }}</p>
          </div>
        </div>
      </div>
      <div class="bg-white rounded-lg shadow-sm border p-4">
        <div class="flex items-center">
          <div class="w-3 h-3 bg-yellow-500 rounded-full mr-3"></div>
          <div>
            <p class="text-sm font-medium text-gray-900">Pågående</p>
            <p class="text-2xl font-bold text-yellow-600">{{ inProgressCount }}</p>
          </div>
        </div>
      </div>
      <div class="bg-white rounded-lg shadow-sm border p-4">
        <div class="flex items-center">
          <div class="w-3 h-3 bg-orange-500 rounded-full mr-3"></div>
          <div>
            <p class="text-sm font-medium text-gray-900">Pausade</p>
            <p class="text-2xl font-bold text-orange-600">{{ pausedCount }}</p>
          </div>
        </div>
      </div>
      <div class="bg-white rounded-lg shadow-sm border p-4">
        <div class="flex items-center">
          <div class="w-3 h-3 bg-purple-500 rounded-full mr-3"></div>
          <div>
            <p class="text-sm font-medium text-gray-900">Väntar material</p>
            <p class="text-2xl font-bold text-purple-600">{{ waitingCount }}</p>
          </div>
        </div>
      </div>
      <div class="bg-white rounded-lg shadow-sm border p-4">
        <div class="flex items-center">
          <div class="w-3 h-3 bg-green-500 rounded-full mr-3"></div>
          <div>
            <p class="text-sm font-medium text-gray-900">Slutförda</p>
            <p class="text-2xl font-bold text-green-600">{{ completedCount }}</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Recent Events -->
    <div class="bg-white rounded-lg shadow-sm border p-6">
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-lg font-semibold text-gray-900">Senaste händelser</h2>
        <button 
          @click="showRecentEvents = !showRecentEvents" 
          class="text-sm text-gray-600 hover:text-gray-900 flex items-center space-x-1"
        >
          <span>{{ showRecentEvents ? 'Minimera' : 'Expandera' }}</span>
          <svg 
            :class="showRecentEvents ? 'rotate-180' : ''" 
            class="w-4 h-4 transition-transform duration-200" 
            fill="none" 
            stroke="currentColor" 
            viewBox="0 0 24 24"
          >
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
          </svg>
        </button>
      </div>
      <div v-if="showRecentEvents" class="space-y-3">
        <!-- Show latest 5 comments/status changes -->
        <div v-for="event in recentEvents.slice(0, 5)" :key="event.id" 
             class="flex items-start space-x-3 p-3 bg-gray-50 rounded-lg">
          <div class="flex-1">
            <p class="text-sm font-medium text-gray-900">{{ event.orderExternalId }}</p>
            <p class="text-sm text-gray-600">{{ event.comment || event.statusChange }}</p>
            <p class="text-xs text-gray-500 mt-1">{{ formatDate(event.createdAt) }} - {{ event.employeeName }}</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Needs Attention -->
    <div class="bg-red-50 border border-red-200 rounded-lg p-6">
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-lg font-semibold text-red-900 flex items-center">
          <ExclamationTriangleIcon class="w-5 h-5 mr-2" />
          Behöver uppmärksamhet ({{ needsAttentionOrders.length }})
        </h2>
        <button 
          @click="showNeedsAttention = !showNeedsAttention" 
          class="text-sm text-red-700 hover:text-red-900 flex items-center space-x-1"
        >
          <span>{{ showNeedsAttention ? 'Minimera' : 'Expandera' }}</span>
          <svg 
            :class="showNeedsAttention ? 'rotate-180' : ''" 
            class="w-4 h-4 transition-transform duration-200" 
            fill="none" 
            stroke="currentColor" 
            viewBox="0 0 24 24"
          >
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
          </svg>
        </button>
      </div>
      <div v-if="showNeedsAttention" class="space-y-2">
        <div v-for="order in needsAttentionOrders" :key="order.id"
             class="flex items-center justify-between p-3 bg-white rounded-lg">
          <div class="flex-1">
            <p class="text-sm font-medium text-gray-900">{{ order.external_id }}</p>
            <div class="flex items-center space-x-2 mt-1">
              <span v-if="isOverdue(order)" class="text-xs bg-red-100 text-red-800 px-2 py-1 rounded">Försenad</span>
              <span v-if="order.status === 'blocked'" class="text-xs bg-orange-100 text-orange-800 px-2 py-1 rounded">Blockerad</span>
              <span v-if="!order.assigned_employee_id" class="text-xs bg-yellow-100 text-yellow-800 px-2 py-1 rounded">Ej tilldelad</span>
              <span v-if="isHighPriorityNearDeadline(order)" class="text-xs bg-purple-100 text-purple-800 px-2 py-1 rounded">Hög prio snart deadline</span>
            </div>
          </div>
          <button @click="viewOrderDetails(order)" class="text-sm text-blue-600 hover:text-blue-800">
            Hantera →
          </button>
        </div>
      </div>
    </div>

    <!-- Advanced Filters -->
    <div class="bg-white rounded-lg shadow-sm border p-6">
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-5 gap-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Status</label>
          <select 
            v-model="filters.status" 
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value="">Alla statusar</option>
            <option value="not_planned">Ej planerade</option>
            <option value="planned">Planerade</option>
            <option value="in_progress">Pågående</option>
            <option value="paused">Pausade</option>
            <option value="waiting_for_materials">Väntar på material</option>
            <option value="blocked">Blockerade</option>
            <option value="in_review">Under granskning</option>
            <option value="completed">Slutförda</option>
            <option value="cancelled">Avbrutna</option>
          </select>
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Medarbetare</label>
          <select 
            v-model="filters.employee" 
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value="">Alla medarbetare</option>
            <option value="unassigned">Ej tilldelade</option>
            <option 
              v-for="employee in employees" 
              :key="employee.id" 
              :value="employee.id.toString()"
            >
              {{ employee.name }}
            </option>
          </select>
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Prioritet</label>
          <select 
            v-model="filters.priority" 
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value="">Alla prioriteringar</option>
            <option value="low">Låg</option>
            <option value="medium">Medium</option>
            <option value="high">Hög</option>
          </select>
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Företag</label>
          <select 
            v-model="filters.company" 
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value="">Alla företag</option>
            <option value="allanit">Allanit Service AB</option>
            <option value="industrimålning">Industrimålning Stockholm AB</option>
          </select>
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-2">Sök</label>
          <input
            v-model="filters.search"
            type="text"
            placeholder="Sök i beskrivning..."
            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
        <div class="flex items-end">
          <button
            @click="clearFilters"
            class="w-full px-4 py-2 text-sm text-gray-600 hover:text-gray-800 border border-gray-300 rounded-md hover:bg-gray-50"
          >
            Rensa filter
          </button>
        </div>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="loading" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      <div
        v-for="i in 6"
        :key="i"
        class="bg-white rounded-xl shadow-sm border border-gray-200 p-6 animate-pulse"
      >
        <div class="space-y-3">
          <div class="h-4 bg-gray-300 rounded w-3/4"></div>
          <div class="h-3 bg-gray-300 rounded w-1/2"></div>
          <div class="h-3 bg-gray-300 rounded w-1/3"></div>
          <div class="flex items-center space-x-2">
            <div class="w-8 h-8 bg-gray-300 rounded-full"></div>
            <div class="h-3 bg-gray-300 rounded w-1/4"></div>
          </div>
        </div>
      </div>
    </div>

    <!-- Orders Grid -->
    <div v-else class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 gap-4 sm:gap-6">
      <div
        v-for="order in paginatedOrders"
        :key="order.id"
        class="bg-white rounded-xl shadow-sm border border-gray-200 p-6 hover:shadow-lg hover:border-blue-300 transition-all duration-200"
      >
        <!-- Order Header -->
        <div class="flex items-start justify-between mb-4">
          <div class="flex-1 min-w-0">
            <h3 class="text-lg font-semibold text-gray-900 truncate">
              {{ order.external_id }}
            </h3>
            <p class="text-sm text-gray-600 line-clamp-2 mt-1">
              {{ order.description }}
            </p>
          </div>
          <div class="flex items-center space-x-2 ml-4">
            <span 
              class="px-2 py-1 text-xs font-medium rounded-full"
              :class="getCompanyColor(order.company)"
            >
              {{ getCompanyLabel(order.company) }}
            </span>
            <div
              class="w-3 h-3 rounded-full"
              :class="getStatusColor(order.status)"
            ></div>
          </div>
        </div>

        <!-- Customer Info -->
        <div class="mb-4">
          <div class="flex items-center space-x-2">
            <UserGroupIcon class="w-4 h-4 text-gray-400" />
            <span class="text-sm text-gray-600">{{ getCustomerName(order.customer_id) }}</span>
          </div>
        </div>

        <!-- Assigned Employee -->
        <div v-if="order.assigned_employee_id" class="mb-4">
          <div class="flex items-center space-x-3">
            <img
              :src="getEmployee(order.assigned_employee_id)?.image"
              :alt="getEmployee(order.assigned_employee_id)?.name"
              class="w-8 h-8 rounded-full object-cover border-2 border-gray-200 cursor-pointer"
              @error="handleImageError"
              @click="viewEmployeeOrders(getEmployee(order.assigned_employee_id))"
            />
            <div class="flex-1 min-w-0">
              <p class="text-sm text-gray-600">Tilldelad:</p>
              <button
                @click="viewEmployeeOrders(getEmployee(order.assigned_employee_id))"
                class="text-sm font-medium text-blue-600 hover:text-blue-800 truncate"
              >
                {{ getEmployee(order.assigned_employee_id)?.name }}
              </button>
            </div>
          </div>
        </div>
        <div v-else class="mb-4">
          <p class="text-sm text-gray-500 italic">Ej tilldelad</p>
        </div>

        <!-- Priority -->
        <div class="mb-4">
          <div class="flex items-center space-x-2">
            <span class="text-sm text-gray-600">Prioritet:</span>
            <span 
              class="px-2 py-1 text-xs font-medium rounded-full"
              :class="getPriorityColor(order.priority)"
            >
              {{ getPriorityLabel(order.priority) }}
            </span>
          </div>
        </div>

        <!-- Due Date -->
        <div v-if="order.dueDate" class="mb-4">
          <div class="flex items-center space-x-2">
            <CalendarIcon class="w-4 h-4 text-gray-400" />
            <span class="text-sm text-gray-600">Förfallodatum:</span>
            <span 
              class="text-sm font-medium"
              :class="getDueDateColor(order.dueDate)"
            >
              {{ formatDate(order.dueDate) }}
            </span>
          </div>
        </div>

        <!-- Scheduled Date -->
        <div v-if="order.scheduled_date" class="mb-4">
          <div class="flex items-center space-x-2">
            <CalendarIcon class="w-4 h-4 text-gray-400" />
            <span class="text-sm text-gray-600">Planerat:</span>
            <span class="text-sm font-medium text-gray-900">
              {{ formatDate(order.scheduled_date) }}
            </span>
          </div>
        </div>

        <!-- Amount -->
        <div class="mb-4">
          <div class="flex items-center justify-between">
            <span class="text-sm text-gray-600">Belopp:</span>
            <span class="text-sm font-medium text-gray-900">
              {{ formatCurrency(order.amount) }}
            </span>
          </div>
        </div>

        <!-- Actions -->
        <div class="pt-4 border-t border-gray-100">
          <div class="flex items-center justify-between">
            <button
              @click="viewOrderDetails(order)"
              class="px-3 py-2 text-sm font-medium text-blue-600 bg-blue-50 border border-blue-200 rounded-md hover:bg-blue-100 transition-colors"
            >
              Visa detaljer
            </button>
            <div class="flex space-x-2">
              <button
                v-if="!order.assigned_employee_id"
                @click="assignOrder(order)"
                class="px-3 py-2 text-sm font-medium text-green-600 bg-green-50 border border-green-200 rounded-md hover:bg-green-100 transition-colors"
              >
                Tilldela
              </button>
              <button
                v-if="order.status === 'in_progress'"
                @click="markAsComplete(order)"
                class="px-3 py-2 text-sm font-medium text-green-600 bg-green-50 border border-green-200 rounded-md hover:bg-green-100 transition-colors"
              >
                Slutför
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-if="!loading && filteredOrders.length === 0" class="text-center py-12">
      <ShoppingBagIcon class="w-12 h-12 text-gray-300 mx-auto mb-4" />
      <h3 class="text-lg font-medium text-gray-900 mb-2">Inga uppdrag</h3>
      <p class="text-gray-600">
        {{ hasActiveFilters ? 'Inga uppdrag matchar dina filter.' : 'Inga uppdrag har skapats än.' }}
      </p>
    </div>

    <!-- Pagination Controls -->
    <div v-if="filteredOrders.length > itemsPerPage" class="flex flex-col sm:flex-row items-center justify-between mt-6 bg-white rounded-lg shadow-sm border p-4 gap-4">
      <div class="text-sm text-gray-600 text-center sm:text-left">
        Visar {{ (currentPage - 1) * itemsPerPage + 1 }}-{{ Math.min(currentPage * itemsPerPage, filteredOrders.length) }} 
        av {{ filteredOrders.length }} uppdrag
      </div>
      
      <div class="flex items-center space-x-2">
        <button 
          @click="currentPage--" 
          :disabled="currentPage === 1"
          class="px-3 py-1 rounded-md border disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-50"
        >
          Föregående
        </button>
        
        <div class="flex space-x-1">
          <button 
            v-for="page in totalPages" 
            :key="page"
            @click="currentPage = page"
            :class="currentPage === page ? 'bg-blue-600 text-white' : 'bg-white text-gray-700 hover:bg-gray-50'"
            class="px-3 py-1 rounded-md border"
          >
            {{ page }}
          </button>
        </div>
        
        <button 
          @click="currentPage++" 
          :disabled="currentPage === totalPages"
          class="px-3 py-1 rounded-md border disabled:opacity-50 disabled:cursor-not-allowed hover:bg-gray-50"
        >
          Nästa
        </button>
      </div>
    </div>

    <!-- Order Detail Modal -->
    <div
      v-if="selectedOrder"
      class="fixed inset-0 z-50 overflow-hidden"
    >
      <div class="absolute inset-0 bg-gray-500 bg-opacity-75" @click="closeOrderDetail"></div>
      <div class="absolute inset-0 flex items-center justify-center p-4">
        <div class="bg-white rounded-2xl shadow-2xl w-full max-w-4xl max-h-[90vh] overflow-hidden">
          <!-- Header -->
          <div class="px-8 py-6 border-b border-gray-200 bg-gradient-to-r from-blue-50 to-indigo-50">
            <div class="flex items-center justify-between">
              <div>
                <h3 class="text-2xl font-bold text-gray-900">{{ selectedOrder.external_id }}</h3>
                <p class="text-lg text-gray-600">{{ selectedOrder.description }}</p>
              </div>
              <button
                @click="closeOrderDetail"
                class="p-3 text-gray-400 hover:text-gray-600 hover:bg-white/80 rounded-xl transition-all duration-200"
              >
                <XMarkIcon class="w-6 h-6" />
              </button>
            </div>
          </div>

          <!-- Content -->
          <div class="flex-1 overflow-y-auto p-8">
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
              <!-- Left Column - Order Info -->
              <div class="space-y-6">
                <div class="bg-gradient-to-br from-gray-50 to-blue-50 rounded-xl p-6 border border-gray-200">
                  <h4 class="text-lg font-semibold text-gray-900 mb-4 flex items-center">
                    <ShoppingBagIcon class="w-5 h-5 mr-2 text-blue-600" />
                    Uppdragsinformation
                  </h4>
                  <div class="space-y-3">
                    <div class="flex justify-between">
                      <span class="text-gray-600">Status:</span>
                      <span class="inline-flex px-2 py-1 text-xs font-medium rounded-full" :class="getStatusBadgeColor(selectedOrder.status)">
                        {{ getStatusLabel(selectedOrder.status) }}
                      </span>
                    </div>
                    <div class="flex justify-between">
                      <span class="text-gray-600">Belopp:</span>
                      <span class="font-medium">{{ formatCurrency(selectedOrder.amount) }}</span>
                    </div>
                    <div class="flex justify-between">
                      <span class="text-gray-600">Företag:</span>
                      <span class="inline-flex px-2 py-1 text-xs font-medium rounded-full" :class="getCompanyColor(selectedOrder.company)">
                        {{ getCompanyLabel(selectedOrder.company) }}
                      </span>
                    </div>
                    <div v-if="selectedOrder.scheduled_date" class="flex justify-between">
                      <span class="text-gray-600">Planerat:</span>
                      <span class="font-medium">{{ formatDate(selectedOrder.scheduled_date) }}</span>
                    </div>
                    <div v-if="selectedOrder.completed_date" class="flex justify-between">
                      <span class="text-gray-600">Slutfört:</span>
                      <span class="font-medium">{{ formatDate(selectedOrder.completed_date) }}</span>
                    </div>
                  </div>
                </div>

                <div class="bg-gradient-to-br from-green-50 to-emerald-50 rounded-xl p-6 border border-gray-200">
                  <h4 class="text-lg font-semibold text-gray-900 mb-4 flex items-center">
                    <UserGroupIcon class="w-5 h-5 mr-2 text-green-600" />
                    Kundinformation
                  </h4>
                  <div class="space-y-2">
                    <div class="flex items-center space-x-2">
                      <span class="text-sm text-gray-600">Kund:</span>
                      <span class="text-sm font-medium">{{ getCustomerName(selectedOrder.customer_id) }}</span>
                    </div>
                  </div>
                </div>

                <div v-if="selectedOrder.notes" class="bg-gradient-to-br from-purple-50 to-pink-50 rounded-xl p-6 border border-gray-200">
                  <h4 class="text-lg font-semibold text-gray-900 mb-4 flex items-center">
                    <DocumentTextIcon class="w-5 h-5 mr-2 text-purple-600" />
                    Anteckningar
                  </h4>
                  <p class="text-sm text-gray-700">{{ selectedOrder.notes }}</p>
                </div>
              </div>

              <!-- Right Column - Assignment Info -->
              <div class="space-y-6">
                <div v-if="selectedOrder.assigned_employee_id" class="bg-gradient-to-br from-blue-50 to-indigo-50 rounded-xl p-6 border border-gray-200">
                  <h4 class="text-lg font-semibold text-gray-900 mb-4 flex items-center">
                    <UserIcon class="w-5 h-5 mr-2 text-blue-600" />
                    Tilldelad medarbetare
                  </h4>
                  <div class="flex items-center space-x-4">
                    <img
                      :src="getEmployee(selectedOrder.assigned_employee_id)?.image"
                      :alt="getEmployee(selectedOrder.assigned_employee_id)?.name"
                      class="w-12 h-12 rounded-full object-cover border-4 border-white shadow-lg cursor-pointer"
                      @error="handleImageError"
                      @click="viewEmployeeOrders(getEmployee(selectedOrder.assigned_employee_id))"
                    />
                    <div>
                      <button
                        @click="viewEmployeeOrders(getEmployee(selectedOrder.assigned_employee_id))"
                        class="text-lg font-medium text-blue-600 hover:text-blue-800"
                      >
                        {{ getEmployee(selectedOrder.assigned_employee_id)?.name }}
                      </button>
                      <p class="text-sm text-gray-600">{{ getEmployee(selectedOrder.assigned_employee_id)?.role }}</p>
                      <p class="text-sm text-gray-600">{{ getEmployee(selectedOrder.assigned_employee_id)?.subsidiary }}</p>
                    </div>
                  </div>
                </div>

                <div v-if="selectedOrder.supervisor_id" class="bg-gradient-to-br from-yellow-50 to-orange-50 rounded-xl p-6 border border-gray-200">
                  <h4 class="text-lg font-semibold text-gray-900 mb-4 flex items-center">
                    <UserIcon class="w-5 h-5 mr-2 text-yellow-600" />
                    Handledare
                  </h4>
                  <div class="flex items-center space-x-4">
                    <img
                      :src="getEmployee(selectedOrder.supervisor_id)?.image"
                      :alt="getEmployee(selectedOrder.supervisor_id)?.name"
                      class="w-12 h-12 rounded-full object-cover border-4 border-white shadow-lg"
                      @error="handleImageError"
                    />
                    <div>
                      <p class="text-lg font-medium text-gray-900">{{ getEmployee(selectedOrder.supervisor_id)?.name }}</p>
                      <p class="text-sm text-gray-600">{{ getEmployee(selectedOrder.supervisor_id)?.role }}</p>
                      <p class="text-sm text-gray-600">{{ getEmployee(selectedOrder.supervisor_id)?.email }}</p>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Actions -->
          <div class="px-8 py-6 border-t border-gray-200 bg-gradient-to-r from-gray-50 to-white">
            <div class="flex space-x-4">
              <button
                @click="closeOrderDetail"
                class="px-6 py-3 text-sm font-semibold text-gray-700 bg-white border border-gray-300 rounded-xl hover:bg-gray-50 hover:border-gray-400 transition-all duration-200 shadow-sm hover:shadow-md"
              >
                Stäng
              </button>
              <button
                v-if="!selectedOrder.assigned_employee_id"
                @click="assignOrder(selectedOrder)"
                class="px-6 py-3 text-sm font-semibold text-white bg-green-600 border border-transparent rounded-xl hover:bg-green-700 hover:shadow-lg transition-all duration-200 shadow-sm"
              >
                Tilldela uppdrag
              </button>
              <button
                v-if="selectedOrder.status === 'in_progress'"
                @click="markAsComplete(selectedOrder)"
                class="px-6 py-3 text-sm font-semibold text-white bg-green-600 border border-transparent rounded-xl hover:bg-green-700 hover:shadow-lg transition-all duration-200 shadow-sm"
              >
                Markera som klar
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Employee Orders Modal -->
    <EmployeeOrdersModal
      v-if="selectedEmployee"
      :employee="selectedEmployee"
      :orders="orders"
      @close="selectedEmployee = null"
      @viewOrder="viewOrderDetails"
      @assignOrder="assignOrder"
    />

    <!-- Order Assignment Modal -->
    <OrderAssignmentModal
      v-if="orderToAssign"
      :order="orderToAssign"
      @close="orderToAssign = null"
      @assign="handleOrderAssignment"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { 
  ShoppingBagIcon,
  UserGroupIcon,
  CalendarIcon,
  UserIcon,
  XMarkIcon,
  DocumentTextIcon,
  ExclamationTriangleIcon
} from '@heroicons/vue/24/outline'
import { http } from '@/api/mockClient'
import type { PurchaseOrder } from '@/types/domain'
import employees from '@/mock/employees'
import orderComments from '@/mock/orderComments'
import EmployeeOrdersModal from '@/components/ui/EmployeeOrdersModal.vue'
import OrderAssignmentModal from '@/components/ui/OrderAssignmentModal.vue'

const orders = ref<PurchaseOrder[]>([])
const selectedOrder = ref<PurchaseOrder | null>(null)
const selectedEmployee = ref<any>(null)
const orderToAssign = ref<PurchaseOrder | null>(null)
const loading = ref(true)
const showRecentEvents = ref(true)
const showNeedsAttention = ref(true)
const currentPage = ref(1)
const itemsPerPage = 6

const filters = ref({
  status: '',
  employee: '',
  company: '',
  priority: '',
  search: ''
})

const filteredOrders = computed(() => {
  let filtered = orders.value

  if (filters.value.status) {
    filtered = filtered.filter(order => order.status === filters.value.status)
  }

  if (filters.value.employee) {
    if (filters.value.employee === 'unassigned') {
      filtered = filtered.filter(order => !order.assigned_employee_id)
    } else {
      filtered = filtered.filter(order => order.assigned_employee_id === parseInt(filters.value.employee))
    }
  }

  if (filters.value.company) {
    filtered = filtered.filter(order => order.company === filters.value.company)
  }

  if (filters.value.priority) {
    filtered = filtered.filter(order => order.priority === filters.value.priority)
  }

  if (filters.value.search) {
    const search = filters.value.search.toLowerCase()
    filtered = filtered.filter(order => 
      order.description?.toLowerCase().includes(search) ||
      order.external_id.toLowerCase().includes(search)
    )
  }

  return filtered
})

const totalPages = computed(() => Math.ceil(filteredOrders.value.length / itemsPerPage))

const paginatedOrders = computed(() => {
  const start = (currentPage.value - 1) * itemsPerPage
  const end = start + itemsPerPage
  return filteredOrders.value.slice(start, end)
})

const hasActiveFilters = computed(() => {
  return Object.values(filters.value).some(value => value !== '')
})

const notPlannedCount = computed(() => 
  orders.value.filter(order => order.status === 'not_planned').length
)

const plannedCount = computed(() => 
  orders.value.filter(order => order.status === 'planned').length
)

const inProgressCount = computed(() => 
  orders.value.filter(order => order.status === 'in_progress').length
)

const completedCount = computed(() => 
  orders.value.filter(order => order.status === 'completed').length
)

const pausedCount = computed(() => 
  orders.value.filter(order => order.status === 'paused').length
)

const waitingCount = computed(() => 
  orders.value.filter(order => order.status === 'waiting_for_materials').length
)

const cancelledCount = computed(() => 
  orders.value.filter(order => order.status === 'cancelled').length
)

const recentEvents = computed(() => {
  // Fetch and combine comments + status changes, sort by date
  return orderComments
    .map(c => ({
      ...c, 
      orderExternalId: orders.value.find(o => o.id === c.orderId)?.external_id,
      statusChange: c.status ? `Status ändrad till: ${c.status}` : '',
      employeeName: employees.find(e => e.id === c.employeeId)?.name || 'Okänd medarbetare'
    }))
    .sort((a, b) => new Date(b.createdAt).getTime() - new Date(a.createdAt).getTime())
})

const needsAttentionOrders = computed(() => {
  return orders.value.filter(order => {
    const overdue = order.dueDate && new Date(order.dueDate) < new Date()
    const blocked = order.status === 'blocked' || order.status === 'paused'
    const unassigned = !order.assigned_employee_id
    const highPrioSoon = order.priority === 'high' && order.dueDate && 
      Math.ceil((new Date(order.dueDate).getTime() - new Date().getTime()) / (1000*60*60*24)) <= 3
    
    return overdue || blocked || unassigned || highPrioSoon
  })
})

function isOverdue(order: PurchaseOrder) {
  return order.dueDate && new Date(order.dueDate) < new Date()
}

function isHighPriorityNearDeadline(order: PurchaseOrder) {
  if (order.priority !== 'high' || !order.dueDate) return false
  const daysUntil = Math.ceil((new Date(order.dueDate).getTime() - new Date().getTime()) / (1000*60*60*24))
  return daysUntil <= 3 && daysUntil >= 0
}

function quickAssign(order: PurchaseOrder) {
  // Open assignment modal or navigate to order details
  console.log('Quick assign:', order)
}

async function loadData() {
  try {
    const response = await http.get('/api/purchase-orders')
    orders.value = response.data.results
  } catch (error) {
    console.error('Error loading orders:', error)
  } finally {
    loading.value = false
  }
}

function getCustomerName(customerId: number | null): string {
  if (!customerId) return "Okänd kund"
  return `Kund ${customerId}`
}

function getEmployee(employeeId: number) {
  return employees.find(emp => emp.id === employeeId)
}

function getStatusColor(status: string): string {
  const colors = {
    not_planned: 'bg-gray-500',
    planned: 'bg-blue-500',
    in_progress: 'bg-yellow-500',
    completed: 'bg-green-500',
    cancelled: 'bg-red-500'
  }
  return colors[status as keyof typeof colors] || 'bg-gray-500'
}

function getStatusBadgeColor(status: string): string {
  const colors = {
    not_planned: 'bg-gray-100 text-gray-800',
    planned: 'bg-blue-100 text-blue-800',
    in_progress: 'bg-yellow-100 text-yellow-800',
    paused: 'bg-orange-100 text-orange-800',
    waiting_for_materials: 'bg-purple-100 text-purple-800',
    blocked: 'bg-red-100 text-red-800',
    in_review: 'bg-indigo-100 text-indigo-800',
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
    in_review: 'Under granskning',
    completed: 'Slutförd',
    cancelled: 'Avbruten'
  }
  return labels[status as keyof typeof labels] || status
}

function getCompanyColor(company: string): string {
  const colors = {
    allanit: 'bg-green-100 text-green-800',
    industrimålning: 'bg-purple-100 text-purple-800'
  }
  return colors[company as keyof typeof colors] || 'bg-gray-100 text-gray-800'
}

function getCompanyLabel(company: string): string {
  const labels = {
    allanit: 'Allanit',
    industrimålning: 'Industrimålning'
  }
  return labels[company as keyof typeof labels] || company
}

function getPriorityColor(priority: string): string {
  const colors = {
    low: 'bg-blue-100 text-blue-800',
    medium: 'bg-yellow-100 text-yellow-800',
    high: 'bg-red-100 text-red-800'
  }
  return colors[priority as keyof typeof colors] || 'bg-gray-100 text-gray-800'
}

function getPriorityLabel(priority: string): string {
  const labels = {
    low: 'Låg',
    medium: 'Medium',
    high: 'Hög'
  }
  return labels[priority as keyof typeof labels] || priority
}

function getDueDateColor(dueDate: string): string {
  const now = new Date()
  const due = new Date(dueDate)
  const diffDays = Math.ceil((due.getTime() - now.getTime()) / (1000 * 60 * 60 * 24))
  
  if (diffDays < 0) return 'text-red-600 font-semibold' // Overdue
  if (diffDays <= 3) return 'text-orange-600 font-semibold' // Due soon
  return 'text-gray-900' // Normal
}

function formatCurrency(amount: number): string {
  return new Intl.NumberFormat('sv-SE', {
    style: 'currency',
    currency: 'SEK'
  }).format(amount)
}

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString('sv-SE')
}

function handleImageError(event: Event) {
  const img = event.target as HTMLImageElement
  img.src = '/medarbetare/default.jpg'
}

function clearFilters() {
  filters.value = {
    status: '',
    employee: '',
    company: '',
    priority: '',
    search: ''
  }
  currentPage.value = 1
}

function resetPagination() {
  currentPage.value = 1
}

function createOrder() {
  console.log('Create new order')
  alert('Skapa nytt uppdrag - funktionalitet kommer snart!')
}

function viewOrderDetails(order: PurchaseOrder) {
  selectedOrder.value = order
}

function closeOrderDetail() {
  selectedOrder.value = null
}

function assignOrder(order: PurchaseOrder) {
  orderToAssign.value = order
}

function handleOrderAssignment(assignment: any) {
  console.log('Order assignment:', assignment)
  orderToAssign.value = null
  // In a real app, this would update the order and refresh the data
}

function markAsComplete(order: PurchaseOrder) {
  console.log('Marking order as complete:', order.external_id)
  alert(`Uppdrag ${order.external_id} markerat som klart!`)
}

function viewEmployeeOrders(employee: any) {
  if (employee) {
    selectedEmployee.value = employee
  }
}

// Reset pagination when filters change
watch(filters, () => {
  resetPagination()
}, { deep: true })

onMounted(() => {
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