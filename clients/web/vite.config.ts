import react from '@vitejs/plugin-react'
import { defineConfig } from 'vite'

// In development, requests to /api and /health are proxied to the Rust API
// so the browser sees one origin. Override the target with SIS_API_URL.
const apiTarget = process.env.SIS_API_URL ?? 'http://localhost:8080'

export default defineConfig({
  plugins: [react()],
  server: {
    port: 5173,
    proxy: {
      '/api': apiTarget,
      '/health': apiTarget,
    },
  },
})
