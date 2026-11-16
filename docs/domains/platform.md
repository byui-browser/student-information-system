# Platform

## Purpose

Platform provides the shared foundation every other domain builds on: the API
server, shared backend types, persistence setup, cross-cutting concerns, CI,
and the shells of each client app. Platform does **not** own business rules
for any domain.

## Slices

| Slice | Owns |
| --- | --- |
| Core and API server | `sis-core`, `sis-api`, database setup, migrations tooling, error format, logging, configuration, CI |
| Client shells and shared UI | `@sis/api-client`, app shells (navigation, layout, theming) for web, desktop, and mobile, shared UI components, the MCP server shell |

## Core concepts

- **Health:** status of the server and each domain.
- **API error:** one error shape returned by every endpoint.
- **Pagination:** a shared request and response shape for list endpoints.
- **Feature flag:** a named switch that hides unfinished work.
- **Audit entry:** who did what, to which record, and when.

## Features

### Foundation

- Consistent JSON error format and HTTP status mapping, used by every domain.
- Database connection, migrations runner, and a per-domain migrations folder.
- Seed data command that loads a realistic sample institution (terms,
  courses, students, users) for local development.
- Configuration from environment variables, with documented defaults.
- Request logging and tracing with a request id.
- CORS and security headers suitable for the web and desktop clients.
- Client app shells: navigation, routing, layout, and a place for each domain's screens.
- A typed API client that every frontend uses.

### Core

- Authentication middleware that turns a token (from Identity) into the
  current user for every handler.
- Role-based authorization helpers that domains can use in one line.
- Shared pagination, sorting, and filtering conventions for list endpoints.
- Feature flags readable from both backend and clients.
- Audit log of changes to sensitive records (grades, holds, decisions).
- OpenAPI document generated from or validated against the backend, published at a known URL.
- Shared UI kit: buttons, forms, tables, empty states, error states, loading states.

### Stretch

- Generate the TypeScript client types from the OpenAPI document.
- Background job runner for scheduled work (e.g. term-start billing, waitlist expiry).
- Event bus so domains can publish and subscribe to events instead of calling each other directly.
- Rate limiting.
- Metrics endpoint and a simple operations dashboard.
- Container image and one-command local environment.

## Business rules

- Every endpoint returns errors in the shared format.
- No domain may read another domain's tables directly; use its API or contract.
- Shared files (routes, navigation, UI kit, workspace manifests) change only
  through platform review.
- Breaking API changes require a new version or a deprecation period.

## Interactions

| Needs from | What |
| --- | --- |
| Identity | Token validation and role lookup for auth middleware |

| Provides to | What |
| --- | --- |
| Every domain | Error type, pagination, auth extractor, database access, migrations, flags, audit |
| Every client | API client, app shell, UI kit |

## Open questions

- Which database (PostgreSQL or SQLite), and which Rust data-access library?
- Is the OpenAPI document written by hand (contract first) or generated from code?
- Do domains talk through HTTP calls, in-process function calls, or events?
- How are feature flags stored and toggled?
