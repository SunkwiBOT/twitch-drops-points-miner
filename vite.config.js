import { svelte } from "@sveltejs/vite-plugin-svelte";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";
import { paraglideVitePlugin } from "@inlang/paraglide-js";
import { fileURLToPath } from "node:url";

const devHost = process.env.TAURI_DEV_HOST;

export default defineConfig({
  root: "ui",
  plugins: [
    paraglideVitePlugin({
      project: fileURLToPath(new URL("./project.inlang", import.meta.url)),
      outdir: fileURLToPath(new URL("./ui/src/lib/paraglide", import.meta.url)),
      strategy: ["globalVariable", "baseLocale"],
      outputStructure: "locale-modules",
      emitTsDeclarations: true,
    }),
    tailwindcss(),
    svelte(),
  ],
  resolve: {
    alias: {
      $lib: fileURLToPath(new URL("./ui/src/lib", import.meta.url)),
    },
  },
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
    host: devHost || false,
    hmr: devHost ? { protocol: "ws", host: devHost, port: 1421 } : undefined,
    watch: { ignored: ["**/src-tauri/**", "**/target/**"] },
    proxy: {
      "/api": { target: process.env.MINER_PROXY ?? "http://127.0.0.1:8080", ws: true },
    },
  },
  envPrefix: ["VITE_", "TAURI_ENV_*"],
  build: {
    outDir: "dist",
    chunkSizeWarningLimit: 1000,
    emptyOutDir: true,
    target: process.env.TAURI_ENV_PLATFORM === "windows" ? "chrome105" : "safari13",
    minify: !process.env.TAURI_ENV_DEBUG,
    sourcemap: Boolean(process.env.TAURI_ENV_DEBUG),
  },
});
