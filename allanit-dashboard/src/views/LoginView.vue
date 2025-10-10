<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-md w-full space-y-8">
      <div>
        <div class="flex justify-center">
          <img 
            src="/LogoDesign.png" 
            alt="Allanit Logo" 
            class="h-8 w-auto"
          />
        </div>
        <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">
          Logga in på Moderbolags Portal
        </h2>
        <p class="mt-2 text-center text-sm text-gray-600">
          Allanit Service & Industrimålning i Stockholm AB
        </p>
      </div>
      <form class="mt-8 space-y-6" @submit.prevent="handleLogin">
        <div class="rounded-md shadow-sm -space-y-px">
          <div>
            <label for="username" class="sr-only">Användarnamn</label>
            <input
              id="username"
              v-model="username"
              name="username"
              type="text"
              required
              class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-t-md focus:outline-none focus:ring-blue-500 focus:border-blue-500 focus:z-10 sm:text-sm"
              placeholder="Användarnamn"
            />
          </div>
          <div>
            <label for="password" class="sr-only">Lösenord</label>
            <input
              id="password"
              v-model="password"
              name="password"
              type="password"
              required
              class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-b-md focus:outline-none focus:ring-blue-500 focus:border-blue-500 focus:z-10 sm:text-sm"
              placeholder="Lösenord"
            />
          </div>
        </div>

        <div v-if="error" class="text-red-600 text-sm text-center">
          {{ error }}
        </div>

        <div>
          <button
            type="submit"
            :disabled="loading"
            class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50"
          >
            {{ loading ? 'Loggar in...' : 'Logga in' }}
          </button>
        </div>

        <!-- Quick Login Dropdown -->
        <div class="text-center">
          <div class="relative inline-block text-left">
            <button
              type="button"
              @click="showQuickLogin = !showQuickLogin"
              class="inline-flex justify-center w-full rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
            >
              Snabb-inloggning
              <ChevronDownIcon class="-mr-1 ml-2 h-5 w-5" />
            </button>

            <div
              v-if="showQuickLogin"
              class="origin-top-right absolute right-0 mt-2 w-80 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5 focus:outline-none z-10"
            >
              <div class="py-1">
                <div class="px-4 py-2 text-xs font-medium text-gray-500 uppercase tracking-wide">
                  Välj användare
                </div>
                <button
                  v-for="user in quickLoginUsers"
                  :key="user.id"
                  type="button"
                  @click="quickLoginAs(user)"
                  class="flex items-center w-full px-4 py-3 text-sm text-gray-700 hover:bg-gray-100"
                >
                  <img
                    :src="user.image"
                    :alt="user.name"
                    class="w-8 h-8 rounded-full object-cover mr-3"
                    @error="handleImageError"
                  />
              <div class="flex-1 text-left">
                <div class="font-medium">{{ user.name }}</div>
                <div class="text-xs text-gray-500">{{ user.role === 'administrator' ? 'Administratör' : 'Medarbetare' }} ({{ getCompanyName(user.employeeId || 0) }})</div>
              </div>
                  <div class="text-xs text-gray-400">
                    {{ user.credentials }}
                  </div>
                </button>
              </div>
            </div>
          </div>
        </div>

        <div class="text-center text-sm text-gray-600">
          <p>Demo-inloggning:</p>
          <p>Användarnamn: <strong>admin</strong></p>
          <p>Lösenord: <strong>admin</strong></p>
          <p class="mt-2 text-xs">Eller använd snabb-inloggning ovan</p>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { ChevronDownIcon } from '@heroicons/vue/24/outline'
import type { AuthenticatedUser } from '@/types/domain'

const router = useRouter()
const username = ref('')
const password = ref('')
const loading = ref(false)
const error = ref('')
const showQuickLogin = ref(false)

const quickLoginUsers: (AuthenticatedUser & { image: string; credentials: string })[] = [
  {
    id: 4,
    username: 'johan',
    name: 'Johan Liljenberg',
    role: 'administrator',
    employeeId: 4,
    image: '/medarbetare/johan.jpg',
    credentials: 'johan johan'
  },
  {
    id: 1,
    username: 'tobias',
    name: 'Tobias Högberg',
    role: 'administrator',
    employeeId: 1,
    image: '/medarbetare/tobias.jpg',
    credentials: 'tobias tobias'
  },
  {
    id: 2,
    username: 'alfons',
    name: 'Alfons Högberg',
    role: 'employee',
    employeeId: 2,
    image: '/medarbetare/alfons.jpg',
    credentials: 'alfons alfons'
  },
  {
    id: 3,
    username: 'janus',
    name: 'Janus',
    role: 'employee',
    employeeId: 3,
    image: '/medarbetare/janus.jpg',
    credentials: 'janus janus'
  },
  {
    id: 5,
    username: 'knut',
    name: 'Knut Rogerson',
    role: 'administrator',
    employeeId: 5,
    image: '/medarbetare/knut.png',
    credentials: 'knut knut'
  },
  {
    id: 6,
    username: 'niklas',
    name: 'Niklas Danielsson',
    role: 'administrator',
    employeeId: 6,
    image: '/medarbetare/niklas.png',
    credentials: 'niklas niklas'
  },
  {
    id: 7,
    username: 'zamdall',
    name: 'Zamdall Gröndal',
    role: 'employee',
    employeeId: 7,
    image: '/medarbetare/zamdall.png',
    credentials: 'zamdall zamdall'
  },
  {
    id: 8,
    username: 'ake',
    name: 'Åke Jäger',
    role: 'employee',
    employeeId: 8,
    image: '/medarbetare/ake.png',
    credentials: 'ake ake'
  }
]

function handleLogin() {
  loading.value = true
  error.value = ''
  
  // Check for quick login credentials
  const credentials = `${username.value} ${password.value}`.toLowerCase()
  const quickUser = quickLoginUsers.find(user => 
    user.credentials.toLowerCase() === credentials
  )
  
  setTimeout(() => {
    let user: AuthenticatedUser
    
    if (quickUser) {
      // Use quick login user
      user = {
        id: quickUser.id,
        username: quickUser.username,
        name: quickUser.name,
        role: quickUser.role,
        employeeId: quickUser.employeeId
      }
    } else {
      // Default admin login
      user = {
        id: 4,
        username: username.value || 'admin',
        name: 'Johan Liljenberg',
        role: 'administrator',
        employeeId: 4
      }
    }
    
    localStorage.setItem('isAuthenticated', 'true')
    localStorage.setItem('user', JSON.stringify(user))
    
    // Force navigation
    window.location.href = '/'
    loading.value = false
  }, 500)
}

function quickLoginAs(user: AuthenticatedUser & { image: string; credentials: string }) {
  showQuickLogin.value = false
  username.value = user.username
  password.value = user.username
  handleLogin()
}

function handleImageError(event: Event) {
  const img = event.target as HTMLImageElement
  img.src = '/medarbetare/default.jpg'
}

function getCompanyName(employeeId: number): string {
  const companyMap: { [key: number]: string } = {
    1: 'Allanit Service AB',
    2: 'Allanit Service AB', 
    3: 'Allanit Service AB',
    4: 'Allanit Service AB',
    5: 'Industrimålning Stockholm AB',
    6: 'Industrimålning Stockholm AB',
    7: 'Industrimålning Stockholm AB',
    8: 'Industrimålning Stockholm AB'
  }
  return companyMap[employeeId] || 'Okänt företag'
}
</script>