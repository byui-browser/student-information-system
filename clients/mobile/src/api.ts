import { createSisClient } from "@sis/api-client";

// Expo inlines EXPO_PUBLIC_* variables at build time.
// On the Android emulator, use http://10.0.2.2:8080 to reach your machine.
// On a physical device, use your computer's LAN address, e.g. http://192.168.1.20:8080.
export const sis = createSisClient({
  baseUrl: process.env.EXPO_PUBLIC_SIS_API_URL ?? "http://localhost:8080",
});
