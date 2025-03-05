import { TinyVueSingleResolver } from '@opentiny/unplugin-tiny-vue';
import vue from "@vitejs/plugin-vue";
import AutoImport from 'unplugin-auto-import/vite';
import Components from 'unplugin-vue-components/vite';
import { defineConfig } from "vite";

const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
    plugins: [vue(),
        Components({
            resolvers: [TinyVueSingleResolver],
        }),
        AutoImport({
            resolvers: [TinyVueSingleResolver],
        }),],
    define: {
        'process.env': { ...process.env, TINY_MODE: 'pc' },
    },

    // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
    //
    // prevent vite from obscuring rust errors
    clearScreen: false,
    server: {
        port: 1420,
        // Tauri expects a fixed port, fail if that port is not available
        strictPort: true,
        // if the host Tauri is expecting is set, use it
        host: host || false,
        hmr: host
            ? {
                protocol: "ws",
                host,
                port: 1421,
            }
            : undefined,
        watch: {
            // 3. tell vite to ignore watching `src-tauri`
            ignored: ["**/src-tauri/**"],
        },
    },
    // Env variables starting with the item of `envPrefix` will be exposed in tauri's source code through `import.meta.env`.
    envPrefix: ['VITE_', 'TAURI_ENV_*'],
    build: {
        // Tauri uses Chromium on Windows and WebKit on macOS and Linux
        target:
            process.env.TAURI_ENV_PLATFORM === 'windows'
                ? 'chrome105'
                : 'safari13',
        // don't minify for debug builds
        minify: !process.env.TAURI_ENV_DEBUG ? 'esbuild' : false,
        // produce sourcemaps for debug builds
        sourcemap: !!process.env.TAURI_ENV_DEBUG,
    },
}));
