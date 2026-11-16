import { Client } from "@modelcontextprotocol/sdk/client/index.js";
import { InMemoryTransport } from "@modelcontextprotocol/sdk/inMemory.js";
import { createSisClient } from "@sis/api-client";
import { describe, expect, it } from "vitest";
import { createServer } from "./server";
import { jsonResult } from "./tools/result";

async function connectedClient(fetchImpl: typeof fetch) {
  const server = createServer(createSisClient({ baseUrl: "http://api.test", fetch: fetchImpl }));
  const client = new Client({ name: "test", version: "0.0.0" });
  const [clientTransport, serverTransport] = InMemoryTransport.createLinkedPair();
  await Promise.all([server.connect(serverTransport), client.connect(clientTransport)]);
  return client;
}

describe("jsonResult", () => {
  it("wraps values as text content", () => {
    expect(jsonResult({ a: 1 }).content[0]?.text).toBe('{\n  "a": 1\n}');
  });
});

describe("SIS MCP server", () => {
  it("calls the API for api_health", async () => {
    const fakeFetch = async (url: string | URL | Request) => {
      expect(String(url)).toBe("http://api.test/health");
      return new Response(JSON.stringify({ service: "api", status: "ok", version: "0.1.0" }));
    };
    const client = await connectedClient(fakeFetch as typeof fetch);

    const result = await client.callTool({ name: "api_health", arguments: {} });

    expect(JSON.stringify(result.content)).toContain('\\"status\\": \\"ok\\"');
  });
});
