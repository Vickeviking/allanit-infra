import { createRouter, createWebHistory } from "vue-router";

// Import all components directly to avoid lazy loading issues
import LoginView from "@/views/LoginView.vue";
import HomeView from "@/views/HomeView.vue";
import CustomersView from "@/views/CustomersView.vue";
import OrdersView from "@/views/OrdersView.vue";
import SettingsView from "@/views/SettingsView.vue";
import EmailManagementView from "@/views/EmailManagementView.vue";
import EmployeesView from "@/views/EmployeesView.vue";
import InvoicesView from "@/views/InvoicesView.vue";

// Create simple placeholder components for the remaining ones
const createPlaceholderComponent = (name: string) => ({
  template: `
    <div class="p-6">
      <h1 class="text-2xl font-bold text-gray-900">${name}</h1>
      <p class="text-gray-600 mt-2">Denna vy är under utveckling.</p>
      <div class="mt-4 p-4 bg-blue-50 rounded-lg">
        <p class="text-sm text-blue-800">
          <strong>Status:</strong> Kommer snart med full funktionalitet!
        </p>
      </div>
    </div>
  `
});

const EmailHistoryView = createPlaceholderComponent('E-posthistorik');
const SubsidiariesView = createPlaceholderComponent('Dotterbolag');
const DeveloperView = createPlaceholderComponent('Utvecklare');

export default createRouter({
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
      meta: { requiresAuth: true }
    },
    {
      path: "/customers",
      name: "customers",
      component: CustomersView,
      meta: { requiresAuth: true }
    },
    {
      path: "/orders",
      name: "orders",
      component: OrdersView,
      meta: { requiresAuth: true }
    },
    {
      path: "/settings",
      name: "settings",
      component: SettingsView,
      meta: { requiresAuth: true }
    },
    {
      path: "/email-management",
      name: "email-management",
      component: EmailManagementView,
      meta: { requiresAuth: true }
    },
    {
      path: "/employees",
      name: "employees",
      component: EmployeesView,
      meta: { requiresAuth: true }
    },
    {
      path: "/invoices",
      name: "invoices",
      component: InvoicesView,
      meta: { requiresAuth: true }
    },
    {
      path: "/email-history",
      name: "email-history",
      component: EmailHistoryView,
      meta: { requiresAuth: true }
    },
    {
      path: "/subsidiaries",
      name: "subsidiaries",
      component: SubsidiariesView,
      meta: { requiresAuth: true }
    },
    {
      path: "/developer",
      name: "developer",
      component: DeveloperView,
      meta: { requiresAuth: true }
    },
  ],
});
