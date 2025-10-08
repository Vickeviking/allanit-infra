import { createApp } from "vue";
import { createPinia } from "pinia";
import router from "@/router";
import App from "./App.vue";
import "./style.css";

const app = createApp(App);

// Authentication guard
router.beforeEach((to, from, next) => {
  const isAuthenticated = localStorage.getItem('isAuthenticated')
  console.log('🔒 Router Guard:', {
    to: to.path,
    from: from.path,
    isAuthenticated: !!isAuthenticated,
    requiresAuth: to.meta.requiresAuth
  })
  
  if (to.meta.requiresAuth && !isAuthenticated) {
    console.log('❌ Redirecting to login - not authenticated')
    next('/login')
  } else if (to.path === '/login' && isAuthenticated) {
    console.log('✅ Redirecting to home - already authenticated')
    next('/')
  } else {
    console.log('✅ Allowing navigation to:', to.path)
    next()
  }
})

// Add error handling for router
router.onError((error) => {
  console.error('🚨 Router Error:', error)
})

app.use(createPinia()).use(router).mount("#app");
