import { svelte } from "@sveltejs/vite-plugin-svelte";
import tailwindcss from "@tailwindcss/vite";
import { defineConfig } from "vite";
import { paraglideVitePlugin } from "@inlang/paraglide-js";
import { readdirSync, readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const devHost = process.env.TAURI_DEV_HOST;

// The service worker's "no connection" screen (ui/public/offline.html) is no Svelte page:
// its three texts in every language are written next to it at build time, from messages/.
const offlineTexts = () => ({
  name: "offline-texts",
  generateBundle() {
    const dir = fileURLToPath(new URL("./messages", import.meta.url));
    const texts = Object.fromEntries(
      readdirSync(dir)
        .filter((file) => file.endsWith(".json"))
        .map((file) => {
          const messages = JSON.parse(readFileSync(`${dir}/${file}`, "utf8"));

          return [
            file.slice(0, -".json".length),
            [messages.offline_title, messages.offline_text, messages.offline_retry],
          ];
        }),
    );

    this.emitFile({
      type: "asset",
      fileName: "offline-texts.js",
      source: `self.OFFLINE_TEXTS = ${JSON.stringify(texts)};\n`,
    });
  },
});

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
    offlineTexts(),
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
