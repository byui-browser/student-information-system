import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import type { SisClient } from "@sis/api-client";
import { registerPlatformTools } from "./tools/platform";

/**
 * Builds the MCP server. Group tools by domain in src/tools/<domain>.ts and
 * register each group here. Each tool should be a thin wrapper over one API
 * call; business rules stay in the backend.
 */
export function createServer(sis: SisClient): McpServer {
  const server = new McpServer({ name: "student-information-system", version: "0.1.0" });

  registerPlatformTools(server, sis);

  return server;
}
