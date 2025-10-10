<template>
  <div class="min-h-screen flex bg-gray-50">
    <!-- Sidebar -->
    <aside 
      :class="[
        'w-64 bg-white shadow-sm border-r border-gray-200 min-h-screen',
        'fixed md:static inset-y-0 left-0 z-40 transform transition-transform duration-300 ease-in-out',
        mobileMenuOpen ? 'translate-x-0' : '-translate-x-full md:translate-x-0'
      ]"
    >
      <div class="p-6">
        <img
          src="/allanit-logo.png"
          alt="Allanit Logo"
          class="h-8 w-auto mb-4"
        />
        <h1 class="text-xl font-bold text-gray-900">Moderbolags Portal</h1>
      </div>

      <nav class="px-4 pb-4">
        <div class="space-y-6">
          <!-- Översikt Section -->
          <div>
            <h3 class="px-3 text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">
              Översikt
            </h3>
            <div class="space-y-1">
              <RouterLink
                v-for="item in overviewItems"
                :key="item.name"
                :to="item.href"
                class="flex items-center px-3 py-2 text-sm font-medium rounded-md transition-colors"
                :class="
                  isActive(item.href)
                    ? 'bg-blue-100 text-blue-700'
                    : 'text-gray-600 hover:bg-gray-100 hover:text-gray-900'
                "
              >
                <component :is="item.icon" class="w-5 h-5 mr-3" />
                {{ item.name }}
              </RouterLink>
            </div>
          </div>

          <!-- Kommunikation Section -->
          <div v-if="communicationItems.length > 0">
            <h3 class="px-3 text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">
              Kommunikation
            </h3>
            <div class="space-y-1">
              <RouterLink
                v-for="item in communicationItems"
                :key="item.name"
                :to="item.href"
                class="flex items-center px-3 py-2 text-sm font-medium rounded-md transition-colors"
                :class="
                  isActive(item.href)
                    ? 'bg-blue-100 text-blue-700'
                    : 'text-gray-600 hover:bg-gray-100 hover:text-gray-900'
                "
              >
                <component :is="item.icon" class="w-5 h-5 mr-3" />
                {{ item.name }}
              </RouterLink>
            </div>
          </div>

          <!-- Administration Section -->
          <div v-if="administrationItems.length > 0">
            <h3 class="px-3 text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">
              Administration
            </h3>
            <div class="space-y-1">
              <RouterLink
                v-for="item in administrationItems"
                :key="item.name"
                :to="item.href"
                class="flex items-center px-3 py-2 text-sm font-medium rounded-md transition-colors"
                :class="
                  isActive(item.href)
                    ? 'bg-blue-100 text-blue-700'
                    : 'text-gray-600 hover:bg-gray-100 hover:text-gray-900'
                "
              >
                <component :is="item.icon" class="w-5 h-5 mr-3" />
                {{ item.name }}
              </RouterLink>
            </div>
          </div>

          <!-- System Section -->
          <div v-if="systemItems.length > 0">
            <h3 class="px-3 text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">
              System
            </h3>
            <div class="space-y-1">
              <RouterLink
                v-for="item in systemItems"
                :key="item.name"
                :to="item.href"
                class="flex items-center px-3 py-2 text-sm font-medium rounded-md transition-colors"
                :class="
                  isActive(item.href)
                    ? 'bg-blue-100 text-blue-700'
                    : 'text-gray-600 hover:bg-gray-100 hover:text-gray-900'
                "
              >
                <component :is="item.icon" class="w-5 h-5 mr-3" />
                {{ item.name }}
              </RouterLink>
            </div>
          </div>
        </div>
      </nav>
    </aside>

    <!-- Mobile overlay -->
    <div 
      v-if="mobileMenuOpen"
      @click="closeMobileMenu"
      class="fixed inset-0 bg-black bg-opacity-50 z-30 md:hidden"
    ></div>

    <!-- Main content -->
    <div class="flex-1 flex flex-col min-h-screen bg-gray-50 md:ml-0">
      <!-- Top bar -->
      <header class="bg-white shadow-sm border-b border-gray-200">
        <div class="px-6 py-4">
          <div class="flex items-center justify-between">
            <!-- Mobile menu button -->
            <button 
              @click="toggleMobileMenu"
              class="md:hidden p-2 rounded-md text-gray-600 hover:text-gray-900 hover:bg-gray-100"
              aria-label="Toggle menu"
            >
              <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
              </svg>
            </button>
            
            <!-- Breadcrumbs -->
            <nav class="flex items-center space-x-2 text-sm">
              <RouterLink to="/" class="text-gray-500 hover:text-gray-700"
                >Home</RouterLink
              >
              <ChevronRightIcon
                v-if="breadcrumbs.length > 0"
                class="w-4 h-4 text-gray-400"
              />
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
              <!-- User menu -->
              <div class="relative">
                <button
                  @click="showUserMenu = !showUserMenu"
                  class="flex items-center space-x-2 text-sm text-gray-700 hover:text-gray-900"
                >
                  <div
                    class="w-8 h-8 bg-gray-300 rounded-full flex items-center justify-center"
                  >
                    <img
                      :src="employee.image"
                      :alt="employee.name"
                      class="w-8 h-8 rounded-full object-cover border-2 border-gray-200"
                      @error="handleImageError"
                    />
                  </div>

                  <span>{{ currentUser?.name || "Användare" }}</span>
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
      <main class="flex-1 p-6 bg-gray-50">
        <div class="h-full">
          <RouterView />
        </div>
      </main>

      <!-- Footer -->
      <footer class="mt-auto py-6 px-8 border-t border-gray-200 bg-white">
        <div class="flex flex-col sm:flex-row items-center justify-between gap-2 text-sm text-gray-600">
          <div>
            <span class="font-medium">Powered by Viktor Liljenberg</span>
          </div>
          <div>
            © 2025 Allanit AB. All rights reserved.
          </div>
        </div>
      </footer>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from "vue";
import { useRoute, useRouter } from "vue-router";
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
  BuildingOfficeIcon,
  TruckIcon,
} from "@heroicons/vue/24/outline";
import type { AuthenticatedUser } from "@/types/domain";

const route = useRoute();
const router = useRouter();

const searchQuery = ref("");
const showUserMenu = ref(false);
const mobileMenuOpen = ref(false);

const currentUser = computed(() => {
  try {
    const userStr = localStorage.getItem("user");
    return userStr ? JSON.parse(userStr) : null;
  } catch {
    return null;
  }
});

const allNavigationItems = [
  { name: "Översikt", href: "/", icon: HomeIcon, roles: ['administrator', 'employee'] },
  { name: "Kunder", href: "/customers", icon: UserGroupIcon, roles: ['administrator'] },
  { name: "Uppdrag", href: "/orders", icon: ShoppingBagIcon, roles: ['administrator'] },
  { name: "Mina Uppdrag", href: "/my-orders", icon: ShoppingBagIcon, roles: ['employee'] },
  { name: "Maskinpark", href: "/machines", icon: TruckIcon, roles: ['administrator', 'employee'] },
  { name: "Utskick", href: "/email-history", icon: ArrowPathIcon, roles: ['administrator', 'employee'] },
  { name: "E-post", href: "/email-management", icon: MegaphoneIcon, roles: ['administrator'] },
  { name: "Medarbetare", href: "/employees", icon: UserIcon, roles: ['administrator'] },
  { name: "Fakturor", href: "/invoices", icon: DocumentTextIcon, roles: ['administrator'] },
  { name: "Dotterbolag", href: "/subsidiaries", icon: BuildingOfficeIcon, roles: ['administrator'] },
  { name: "Utvecklare", href: "/developer", icon: CogIcon, roles: ['administrator'] },
  { name: "Inställningar", href: "/settings", icon: CogIcon, roles: ['administrator'] },
];

const navigationItems = computed(() => {
  const user = currentUser.value as AuthenticatedUser;
  if (!user) return [];

  return allNavigationItems.filter(item =>
    item.roles.includes(user.role)
  );
});

const overviewItems = computed(() => {
  const user = currentUser.value as AuthenticatedUser;
  if (!user) return [];
  
  return allNavigationItems.filter(item =>
    item.roles.includes(user.role) && 
    ['Översikt', 'Kunder', 'Uppdrag', 'Mina Uppdrag', 'Maskinpark'].includes(item.name)
  );
});

const communicationItems = computed(() => {
  const user = currentUser.value as AuthenticatedUser;
  if (!user) return [];
  
  return allNavigationItems.filter(item =>
    item.roles.includes(user.role) && 
    ['E-post', 'Utskick'].includes(item.name)
  );
});

const administrationItems = computed(() => {
  const user = currentUser.value as AuthenticatedUser;
  if (!user) return [];
  
  return allNavigationItems.filter(item =>
    item.roles.includes(user.role) && 
    ['Medarbetare', 'Fakturor', 'Dotterbolag'].includes(item.name)
  );
});

const systemItems = computed(() => {
  const user = currentUser.value as AuthenticatedUser;
  if (!user) return [];
  
  return allNavigationItems.filter(item =>
    item.roles.includes(user.role) && 
    ['Utvecklare', 'Inställningar'].includes(item.name)
  );
});

const breadcrumbs = computed(() => {
  const pathSegments = route.path.split("/").filter(Boolean);
  const crumbs = [];

  for (let i = 0; i < pathSegments.length; i++) {
    const segment = pathSegments[i];
    const href = "/" + pathSegments.slice(0, i + 1).join("/");

    // Find matching navigation item
    const navItem = navigationItems.value.find(
      (item) => item.href === href || item.href === "/" + segment,
    );

    if (navItem) {
      crumbs.push({
        name: navItem.name,
        href: navItem.href,
      });
    }
  }

  return crumbs;
});

function isActive(href: string): boolean {
  if (href === "/") {
    return route.path === "/";
  }
  return route.path.startsWith(href);
}

// Update employee profile based on current user
const employee = computed(() => {
  const user = currentUser.value as AuthenticatedUser;
  if (!user) return { name: "Användare", image: "/medarbetare/default.jpg" };
  
  // Find employee data based on user's employeeId
  const employeeData = {
    1: { name: "Tobias Högberg", image: "/medarbetare/tobias.jpg" },
    2: { name: "Alfons Högberg", image: "/medarbetare/alfons.jpg" },
    3: { name: "Janus", image: "/medarbetare/janus.jpg" },
    4: { name: "Johan Liljenberg", image: "/medarbetare/johan.jpg" }
  };
  
  return employeeData[user.employeeId as keyof typeof employeeData] || 
         { name: user.name, image: "/medarbetare/default.jpg" };
});

function toggleMobileMenu() {
  mobileMenuOpen.value = !mobileMenuOpen.value;
}

function closeMobileMenu() {
  mobileMenuOpen.value = false;
}

function logout() {
  showUserMenu.value = false;
  localStorage.removeItem("isAuthenticated");
  localStorage.removeItem("user");
  router.push("/login");
}

function handleImageError(event: Event) {
  const img = event.target as HTMLImageElement;
  img.src = "/medarbetare/default.jpg";
}

// Close mobile menu on route change
watch(() => route.path, () => {
  closeMobileMenu();
});

// Close user menu when clicking outside
onMounted(() => {
  document.addEventListener("click", (e) => {
    if (!(e.target as Element).closest(".relative")) {
      showUserMenu.value = false;
    }
  });
});
</script>
