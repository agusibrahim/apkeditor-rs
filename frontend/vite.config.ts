import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "path";

export default defineConfig({
  plugins: [react()],
  base: "/apkeditor-rs/",
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  build: {
    outDir: "dist",
    assetsDir: "assets",
  },
  worker: {
    format: "es",
  },
  server: {
    port: 5174,
    open: true,
    headers: {
      // Allow WASM to be loaded
      "Cross-Origin-Opener-Policy": "same-origin",
      "Cross-Origin-Embedder-Policy": "require-corp",
    },
  },
  optimizeDeps: {
    exclude: ["@anthropic-ai/sdk"],
  },
  assetsInclude: ["**/*.wasm"],
});
