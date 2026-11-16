import { invoke } from "@tauri-apps/api/core";
import { createSisClient, DEFAULT_BASE_URL, type SisClient } from "@sis/api-client";

/**
 * Builds an API client using the base URL provided by the Rust shell.
 * Falls back to the default when the UI runs in a plain browser (`npm run dev:ui`).
 */
export async function connect(): Promise<SisClient> {
  const baseUrl = await invoke<string>("api_base_url").catch(() => DEFAULT_BASE_URL);
  return createSisClient({ baseUrl });
}
