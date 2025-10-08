import { fileURLToPath, URL } from "node:url";
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
      "vue": "vue/dist/vue.esm-bundler.js"
    },
    extensions: [".mjs", ".js", ".ts", ".jsx", ".tsx", ".json", ".vue"],
  },
  server: {
    host: true, // bra i container
    port: 5173,
    strictPort: true,
  },
  optimizeDeps: {
    include: ['vue', 'vue-router', '@heroicons/vue'],
    force: true
  },
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ['vue', 'vue-router'],
          icons: ['@heroicons/vue']
        }
      }
    }
  }
});
