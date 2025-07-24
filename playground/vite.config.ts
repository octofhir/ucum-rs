import react from '@vitejs/plugin-react';
import { defineConfig } from 'vite';

// Plugin to serve WASM files with correct MIME type
const wasmPlugin = () => {
  return {
    name: 'wasm-mime',
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        if (req.url?.endsWith('.wasm')) {
          res.setHeader('Content-Type', 'application/wasm');
        }
        next();
      });
    },
  };
};

export default defineConfig({
  plugins: [react(), wasmPlugin()],
  base: process.env.NODE_ENV === 'production' ? '/ucum-rs/' : '/',
  server: {
    port: 5555,
    headers: {
      'Cross-Origin-Embedder-Policy': 'require-corp',
      'Cross-Origin-Opener-Policy': 'same-origin',
    },
    fs: {
      allow: ['..'],
    },
  },
  build: {
    outDir: 'dist',
  },
  css: {
    modules: {
      localsConvention: 'camelCase',
      generateScopedName: '[name]__[local]__[hash:base64:5]',
    },
    postcss: {
      plugins: [],
    },
  },
  assetsInclude: ['**/*.wasm'],
  optimizeDeps: {
    exclude: ['@octofhir/ucum-wasm'],
  },
});
