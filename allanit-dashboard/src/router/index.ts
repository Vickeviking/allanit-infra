import { createRouter, createWebHistory } from "vue-router";
import type { AuthenticatedUser } from "@/types/domain";

// Import all components directly to avoid lazy loading issues
import LoginView from "@/views/LoginView.vue";
import HomeView from "@/views/HomeView.vue";
import CustomersView from "@/views/CustomersView.vue";
import OrdersView from "@/views/OrdersView.vue";
import SettingsView from "@/views/SettingsView.vue";
import EmailManagementView from "@/views/EmailManagementView.vue";
import EmployeesView from "@/views/EmployeesView.vue";
import InvoicesView from "@/views/InvoicesView.vue";
import EmailHistoryView from "@/views/EmailHistoryView.vue";
import SubsidiariesView from "@/views/SubsidiariesView.vue";
import DeveloperView from "@/views/DeveloperView.vue";
import CampaignsView from "@/views/CampaignsView.vue";
import DeadLettersView from "@/views/DeadLettersView.vue";
import LogsView from "@/views/LogsView.vue";
import SyncView from "@/views/SyncView.vue";
import MyOrdersView from "@/views/MyOrdersView.vue";
import MaskinparkView from "@/views/MaskinparkView.vue";


const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/login",
      name: "login",
      component: LoginView,
    },
    {
      path: "/",
      name: "home",
      component: HomeView,
      meta: { requiresAuth: true, roles: ['administrator', 'employee'] }
    },
    {
      path: "/customers",
      name: "customers",
      component: CustomersView,
      meta: { requiresAuth: true, roles: ['administrator'] }
    },
    {
      path: "/orders",
      name: "orders",
      component: OrdersView,
      meta: { requiresAuth: true, roles: ['administrator'] }
    },
    {
      path: "/my-orders",
      name: "my-orders",
      component: MyOrdersView,
      meta: { requiresAuth: true, roles: ['employee'] }
    },
    {
      path: "/machines",
      name: "machines",
      component: MaskinparkView,
      meta: { requiresAuth: true, roles: ['administrator', 'employee'] }
    },
    {
      path: "/email-history",
      name: "email-history",
      component: EmailHistoryView,
      meta: { requiresAuth: true, roles: ['administrator', 'employee'] }
    },
    {
      path: "/settings",
      name: "settings",
      component: SettingsView,
      meta: { requiresAuth: true, roles: ['administrator'] }
    },
    {
      path: "/email-management",
      name: "email-management",
      component: EmailManagementView,
      meta: { requiresAuth: true, roles: ['administrator'] }
    },
    {
      path: "/employees",
      name: "employees",
      component: EmployeesView,
      meta: { requiresAuth: true, roles: ['administrator'] }
    },
    {
      path: "/invoices",
      name: "invoices",
      component: InvoicesView,
      meta: { requiresAuth: true, roles: ['administrator'] }
    },
    {
      path: "/subsidiaries",
      name: "subsidiaries",
      component: SubsidiariesView,
      meta: { requiresAuth: true, roles: ['administrator'] }
    },
    {
      path: "/developer",
      name: "developer",
      component: DeveloperView,
      meta: { requiresAuth: true, roles: ['administrator'] }
    },
    {
      path: "/campaigns",
      name: "campaigns",
      component: CampaignsView,
      meta: { requiresAuth: true, roles: ['administrator'] }
    },
    {
      path: "/dead-letters",
      name: "dead-letters",
      component: DeadLettersView,
      meta: { requiresAuth: true, roles: ['administrator'] }
    },
    {
      path: "/logs",
      name: "logs",
      component: LogsView,
      meta: { requiresAuth: true, roles: ['administrator'] }
    },
    {
      path: "/sync",
      name: "sync",
      component: SyncView,
      meta: { requiresAuth: true, roles: ['administrator'] }
    },
  ],
});

// Route guard for authentication and role-based access
router.beforeEach((to, from, next) => {
  const isAuthenticated = localStorage.getItem('isAuthenticated') === 'true'
  
  if (to.meta.requiresAuth && !isAuthenticated) {
    next('/login')
    return
  }
  
  if (to.meta.requiresAuth && isAuthenticated) {
    try {
      const userStr = localStorage.getItem('user')
      const user: AuthenticatedUser = userStr ? JSON.parse(userStr) : null
      
      if (user && to.meta.roles) {
        const allowedRoles = to.meta.roles as string[]
        if (!allowedRoles.includes(user.role)) {
          // Redirect to home if user doesn't have required role
          next('/')
          return
        }
      }
    } catch (error) {
      console.error('Error parsing user data:', error)
      next('/login')
      return
    }
  }
  
  next()
})

export default router
