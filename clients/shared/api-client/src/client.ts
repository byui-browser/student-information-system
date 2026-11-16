import type { ApiErrorBody, Health } from "./types";

export const DEFAULT_BASE_URL = "http://localhost:8080";

export interface SisClientOptions {
  /** Server origin, for example `http://localhost:8080`. Use "" for same-origin. */
  baseUrl?: string;
  /** Bearer token sent on every request. In development use `dev-<username>`. */
  token?: string;
  /** Custom fetch, mainly for tests. Defaults to the global fetch. */
  fetch?: typeof fetch;
}

/** Thrown when the API returns a non-2xx response. */
export class SisApiError extends Error {
  readonly status: number;
  readonly code: string;

  constructor(status: number, code: string, message: string) {
    super(message);
    this.name = "SisApiError";
    this.status = status;
    this.code = code;
  }
}

/** Creates a client with one method per API endpoint. */
export function createSisClient(options: SisClientOptions = {}) {
  const baseUrl = (options.baseUrl ?? DEFAULT_BASE_URL).replace(/\/$/, "");
  const doFetch = options.fetch ?? globalThis.fetch.bind(globalThis);

  async function get<T>(path: string): Promise<T> {
    const headers: Record<string, string> = { Accept: "application/json" };
    if (options.token) headers.Authorization = `Bearer ${options.token}`;

    const response = await doFetch(`${baseUrl}${path}`, { headers });
    const body: unknown = await response.json().catch(() => null);
    if (!response.ok) {
      const error = (body as ApiErrorBody | null)?.error;
      throw new SisApiError(
        response.status,
        error?.code ?? "unknown",
        error?.message ?? response.statusText,
      );
    }
    return body as T;
  }

  return {
    health: () => get<Health>("/health"),
    // Add one group per domain as endpoints are built, for example:
    // catalog: { listCourses: () => get<Course[]>("/api/v1/catalog/courses") },
  };
}

export type SisClient = ReturnType<typeof createSisClient>;
