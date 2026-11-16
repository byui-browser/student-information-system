import { createSisClient } from '@sis/api-client'

// Empty base URL means same origin; the Vite dev server proxies /api and /health to the backend.
export const sis = createSisClient({
  baseUrl: import.meta.env.VITE_SIS_API_URL ?? '',
})
