<template>
  <div class="min-h-screen flex bg-gray-50">
    <!-- Sidebar -->
    <aside class="w-64 bg-white shadow-sm border-r border-gray-200">
      <div class="p-6">
        <img 
          src="/src/resources/allanit-logo.png" 
          alt="Allanit Logo" 
          class="h-8 w-auto mb-4"
        />
        <h1 class="text-xl font-bold text-gray-900">Moderbolags Portal</h1>
      </div>
      
      <nav class="px-4 pb-4">
        <div class="space-y-1">
          <RouterLink
            v-for="item in navigationItems"
            :key="item.name"
            :to="item.href"
            class="flex items-center px-3 py-2 text-sm font-medium rounded-md transition-colors"
            :class="isActive(item.href) 
              ? 'bg-blue-100 text-blue-700' 
              : 'text-gray-600 hover:bg-gray-100 hover:text-gray-900'"
          >
            <component :is="item.icon" class="w-5 h-5 mr-3" />
            {{ item.name }}
          </RouterLink>
        </div>
      </nav>
    </aside>

    <!-- Main content -->
    <div class="flex-1 flex flex-col">
      <!-- Top bar -->
      <header class="bg-white shadow-sm border-b border-gray-200">
        <div class="px-6 py-4">
          <div class="flex items-center justify-between">
            <!-- Breadcrumbs -->
            <nav class="flex items-center space-x-2 text-sm">
              <RouterLink to="/" class="text-gray-500 hover:text-gray-700">Home</RouterLink>
              <ChevronRightIcon v-if="breadcrumbs.length > 0" class="w-4 h-4 text-gray-400" />
              <template v-for="(crumb, index) in breadcrumbs" :key="crumb.name">
                <RouterLink 
                  :to="crumb.href" 
                  class="text-gray-500 hover:text-gray-700"
                >
                  {{ crumb.name }}
                </RouterLink>
                <ChevronRightIcon 
                  v-if="index < breadcrumbs.length - 1" 
                  class="w-4 h-4 text-gray-400" 
                />
              </template>
            </nav>

            <!-- Search -->
            <div class="flex items-center space-x-4">
              <div class="relative">
                <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                  <MagnifyingGlassIcon class="h-5 w-5 text-gray-400" />
                </div>
                <input
                  v-model="searchQuery"
                  type="text"
                  placeholder="Sök..."
                  class="block w-64 pl-10 pr-3 py-2 border border-gray-300 rounded-md leading-5 bg-white placeholder-gray-500 focus:outline-none focus:placeholder-gray-400 focus:ring-1 focus:ring-blue-500 focus:border-blue-500"
                />
              </div>
              
              <!-- User menu -->
              <div class="relative">
                <button
                  @click="showUserMenu = !showUserMenu"
                  class="flex items-center space-x-2 text-sm text-gray-700 hover:text-gray-900"
                >
                  <div class="w-8 h-8 bg-gray-300 rounded-full flex items-center justify-center">
                    <UserIcon class="w-5 h-5 text-gray-600" />
                  </div>
                  <span>{{ currentUser?.name || 'Användare' }}</span>
                  <ChevronDownIcon class="w-4 h-4" />
                </button>
                
                <!-- User dropdown -->
                <div
                  v-if="showUserMenu"
                  class="absolute right-0 mt-2 w-48 bg-white rounded-md shadow-lg border border-gray-200 z-50"
                >
                  <div class="py-1">
                    <RouterLink
                      to="/settings"
                      class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
                      @click="showUserMenu = false"
                    >
                      Inställningar
                    </RouterLink>
                    <button
                      @click="logout"
                      class="block w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
                    >
                      Logga ut
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </header>

      <!-- Page content -->
      <main class="flex-1 p-6">
        <RouterView />
      </main>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { 
  HomeIcon, 
  UserGroupIcon, 
  ShoppingBagIcon, 
  MegaphoneIcon,
  DocumentTextIcon,
  ExclamationTriangleIcon,
  ArrowPathIcon,
  CogIcon,
  ChevronRightIcon,
  ChevronDownIcon,
  MagnifyingGlassIcon,
  UserIcon,
  BuildingOfficeIcon
} from '@heroicons/vue/24/outline'

const route = useRoute()
const router = useRouter()

const searchQuery = ref('')
const showUserMenu = ref(false)

const currentUser = computed(() => {
  try {
    const userStr = localStorage.getItem('user')
    return userStr ? JSON.parse(userStr) : null
  } catch {
    return null
  }
})

const navigationItems = [
  { name: 'Översikt', href: '/', icon: HomeIcon },
  { name: 'Kunder', href: '/customers', icon: UserGroupIcon },
  { name: 'Uppdrag', href: '/orders', icon: ShoppingBagIcon },
  { name: 'E-post', href: '/email-management', icon: MegaphoneIcon },
  { name: 'Medarbetare', href: '/employees', icon: UserIcon },
  { name: 'Fakturor', href: '/invoices', icon: DocumentTextIcon },
  { name: 'Utskick', href: '/email-history', icon: ArrowPathIcon },
  { name: 'Dotterbolag', href: '/subsidiaries', icon: BuildingOfficeIcon },
  { name: 'Utvecklare', href: '/developer', icon: CogIcon },
  { name: 'Inställningar', href: '/settings', icon: CogIcon },
]

const breadcrumbs = computed(() => {
  const pathSegments = route.path.split('/').filter(Boolean)
  const crumbs = []
  
  for (let i = 0; i < pathSegments.length; i++) {
    const segment = pathSegments[i]
    const href = '/' + pathSegments.slice(0, i + 1).join('/')
    
    // Find matching navigation item
    const navItem = navigationItems.find(item => 
      item.href === href || item.href === '/' + segment
    )
    
    if (navItem) {
      crumbs.push({
        name: navItem.name,
        href: navItem.href
      })
    }
  }
  
  return crumbs
})

function isActive(href: string): boolean {
  if (href === '/') {
    return route.path === '/'
  }
  return route.path.startsWith(href)
}

function logout() {
  showUserMenu.value = false
  localStorage.removeItem('isAuthenticated')
  localStorage.removeItem('user')
  router.push('/login')
}

// Close user menu when clicking outside
onMounted(() => {
  document.addEventListener('click', (e) => {
    if (!(e.target as Element).closest('.relative')) {
      showUserMenu.value = false
    }
  })
})
</script>
