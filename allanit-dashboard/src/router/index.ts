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
import EmailHistoryView from "@/views/EmailHistoryView.vue";
import SubsidiariesView from "@/views/SubsidiariesView.vue";
import DeveloperView from "@/views/DeveloperView.vue";
import CampaignsView from "@/views/CampaignsView.vue";
import DeadLettersView from "@/views/DeadLettersView.vue";
import LogsView from "@/views/LogsView.vue";
import SyncView from "@/views/SyncView.vue";


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
    {
      path: "/campaigns",
      name: "campaigns",
      component: CampaignsView,
      meta: { requiresAuth: true }
    },
    {
      path: "/dead-letters",
      name: "dead-letters",
      component: DeadLettersView,
      meta: { requiresAuth: true }
    },
    {
      path: "/logs",
      name: "logs",
      component: LogsView,
      meta: { requiresAuth: true }
    },
    {
      path: "/sync",
      name: "sync",
      component: SyncView,
      meta: { requiresAuth: true }
    },
  ],
});
