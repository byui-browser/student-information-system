import type { Health } from "@sis/api-client";

/** Short label describing the API's health, for example `ok (v0.1.0)`. */
export function statusLabel(health: Health): string {
  return `${health.status} (v${health.version})`;
}
