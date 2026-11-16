/** Wraps any value as MCP tool output (pretty JSON text). */
export function jsonResult(value: unknown) {
  return { content: [{ type: "text" as const, text: JSON.stringify(value, null, 2) }] };
}
