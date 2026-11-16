#!/usr/bin/env -S npx tsx
// Runs the SIS MCP server over stdio. Configure with environment variables:
//   SIS_API_URL    API origin (default http://localhost:8080)
//   SIS_API_TOKEN  bearer token (optional)
// Note: stdout carries the MCP protocol, so log with console.error only.
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import { createSisClient, DEFAULT_BASE_URL } from "@sis/api-client";
import { createServer } from "./server";

const sis = createSisClient({
  baseUrl: process.env.SIS_API_URL ?? DEFAULT_BASE_URL,
  token: process.env.SIS_API_TOKEN,
});

const server = createServer(sis);
await server.connect(new StdioServerTransport());
console.error("SIS MCP server running on stdio");
