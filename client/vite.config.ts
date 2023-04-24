import { fileURLToPath, URL } from "node:url";

import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
  build: {
    outDir: "../server/dist",
  },
  server: {
    host: "0.0.0.0",
    proxy: {
      "/": {
        target: "http://127.0.0.1:80",
        // rewrite: (path) => path.replace(/^\/api/, ""),
      },
    },
  },
});
