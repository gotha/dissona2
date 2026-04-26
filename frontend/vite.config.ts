import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { VitePWA } from 'vite-plugin-pwa';
import path from 'path';

export default defineConfig({
  plugins: [
    react(),
    VitePWA({
      registerType: 'autoUpdate',
      includeAssets: ['favicon.ico', 'apple-touch-icon.png', 'robots.txt'],
      manifest: {
        name: 'Dissona',
        short_name: 'Dissona',
        description: 'Turn documents into intelligent audio',
        theme_color: '#4f46e5',
        background_color: '#ffffff',
        display: 'standalone',
        icons: [
          {
            src: '/icons/icon-192.png',
            sizes: '192x192',
            type: 'image/png',
          },
          {
            src: '/icons/icon-512.png',
            sizes: '512x512',
            type: 'image/png',
          },
        ],
      },
      workbox: {
        globPatterns: ['**/*.{js,css,html,ico,png,svg,woff2}'],
        runtimeCaching: [
          {
            urlPattern: /^https:\/\/cdn\.dissona\.app\/.*\.aac$/,
            handler: 'CacheOnly',
            options: {
              cacheName: 'audio-cache',
            },
          },
          {
            urlPattern: /\/api\/.*/,
            handler: 'NetworkFirst',
            options: {
              cacheName: 'api-cache',
              networkTimeoutSeconds: 5,
            },
          },
        ],
      },
    }),
  ],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  server: {
    port: 15003,
    proxy: {
      '/api': {
        target: 'http://localhost:15002',
        changeOrigin: true,
      },
      '/auth': {
        target: 'http://localhost:15001',
        changeOrigin: true,
        // Don't proxy /auth/callback — that's a frontend route
        bypass(req) {
          if (req.url?.startsWith('/auth/callback')) {
            return req.url;
          }
        },
      },
    },
  },
});
