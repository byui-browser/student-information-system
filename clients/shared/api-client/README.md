# @sis/api-client

Typed TypeScript client for the SIS API. Every frontend (web, desktop,
mobile, MCP) uses this package so request code and types live in one place.

```ts
import { createSisClient } from "@sis/api-client";

const sis = createSisClient({ baseUrl: "http://localhost:8080" });
const health = await sis.health();
```

When you add or change an endpoint in a backend domain:

1. Add or update the matching type in `src/types.ts`.
2. Add a method in `src/client.ts`, grouped under the domain name.
3. Add a test in `src/client.test.ts`.
