import { describe, expect, it, vi } from "vitest";
import { createSisClient, SisApiError } from "./client";

function fakeFetch(status: number, body: unknown) {
  return vi.fn(
    async (_url: RequestInfo | URL, _init?: RequestInit) =>
      new Response(JSON.stringify(body), { status }),
  );
}

describe("createSisClient", () => {
  it("calls the health endpoint and returns JSON", async () => {
    const fetch = fakeFetch(200, { service: "api", status: "ok", version: "0.1.0" });
    const client = createSisClient({ baseUrl: "http://api.test/", fetch });

    const health = await client.health();

    expect(health.status).toBe("ok");
    expect(fetch).toHaveBeenCalledWith("http://api.test/health", expect.anything());
  });

  it("throws SisApiError with the server error code", async () => {
    const fetch = fakeFetch(500, { error: { code: "internal", message: "boom" } });
    const client = createSisClient({ fetch });

    const error = await client.health().catch((e: unknown) => e);

    expect(error).toBeInstanceOf(SisApiError);
    expect(error).toMatchObject({ status: 500, code: "internal", message: "boom" });
  });
});
