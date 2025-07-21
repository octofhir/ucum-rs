import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import type { Plugin } from 'vite';

// Plugin to serve WASM files with correct MIME type
const wasmMimeTypePlugin = (): Plugin => ({
  name: 'wasm-mime-type',
  configureServer(server) {
    server.middlewares.use((req, res, next) => {
      if (req.url?.endsWith('.wasm')) {
        res.setHeader('Content-Type', 'application/wasm');
      }
      next();
    });
  }
});

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte(), wasmMimeTypePlugin()],
  // Configure base path for GitHub Pages deployment
  base: './',
  // Configure server to use port 3005
  server: {
    port: 3005,
    fs: {
      allow: ['..']
    }
  },
  // Configure build output
  build: {
    outDir: 'dist',
    target: 'esnext',
    // Ensure proper handling of WASM files
    assetsInlineLimit: 0
  },
  // Configure optimizations
  optimizeDeps: {
    exclude: ['@octofhir/ucum-wasm']
  },
  // Configure WASM handling
  assetsInclude: ['**/*.wasm'],
  // Configure worker handling for WASM
  worker: {
    format: 'es'
  },
  // Configure resolve for better WASM handling
  resolve: {
    alias: {
      '@octofhir/ucum-wasm': new URL('../ucum-wasm/pkg', import.meta.url).pathname
    }
  }
});
