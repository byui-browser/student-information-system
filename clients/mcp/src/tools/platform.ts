import type { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import type { SisClient } from "@sis/api-client";
import { jsonResult } from "./result";

export function registerPlatformTools(server: McpServer, sis: SisClient) {
  server.registerTool(
    "api_health",
    { description: "Check whether the SIS API is up." },
    async () => jsonResult(await sis.health()),
  );
}
