# @sis/mcp

A [Model Context Protocol](https://modelcontextprotocol.io) server that lets AI
assistants use the SIS API through tools. It starts with a single
`api_health` tool; add tools as domain endpoints are built.

## Run

Start the API first (`cargo run -p sis-api` from the repo root), then:

```sh
npm run dev:mcp          # from the repo root, restarts on change
npm run inspect -w @sis/mcp   # open the MCP Inspector against this server
```

## Connect an MCP client

Most clients accept a JSON config like this (use your absolute path):

```json
{
  "mcpServers": {
    "sis": {
      "command": "npx",
      "args": ["tsx", "/absolute/path/to/clients/mcp/src/index.ts"],
      "env": { "SIS_API_URL": "http://localhost:8080" }
    }
  }
}
```

## Adding a tool

1. Make sure the endpoint exists in `@sis/api-client`.
2. Add the tool to `src/tools/<domain>.ts` (create the file and register it in `src/server.ts` if it is new) with a clear description and a zod input schema.
3. Add a test in `src/server.test.ts`.
