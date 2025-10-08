<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-md w-full space-y-8">
      <div>
        <div class="flex justify-center">
          <img 
            src="/src/resources/LogoDesign.png" 
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

        <div class="text-center text-sm text-gray-600">
          <p>Demo-inloggning:</p>
          <p>Användarnamn: <strong>admin</strong></p>
          <p>Lösenord: <strong>admin</strong></p>
          <p class="mt-2 text-xs">Eller skriv vad som helst - allt godkänns!</p>
        </div>
        
        <div class="text-center">
          <button
            type="button"
            @click="quickLogin"
            class="text-sm text-blue-600 hover:text-blue-800 underline"
          >
            Snabb-inloggning (klicka här)
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const username = ref('')
const password = ref('')
const loading = ref(false)
const error = ref('')

function handleLogin() {
  loading.value = true
  error.value = ''
  
  // Always accept login for demo purposes
  setTimeout(() => {
    localStorage.setItem('isAuthenticated', 'true')
    localStorage.setItem('user', JSON.stringify({
      username: username.value || 'admin',
      name: 'Administratör'
    }))
    
    // Force navigation
    window.location.href = '/'
    loading.value = false
  }, 500)
}

function quickLogin() {
  username.value = 'admin'
  password.value = 'admin'
  handleLogin()
}
</script>