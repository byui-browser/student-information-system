# 0001. Modular monolith with one crate per domain

- Status: accepted
- Date: 2026-11-16
- Domain: platform

## Context

Many people work in this repository at once. We need clear ownership
boundaries so changes rarely collide, while keeping a single deployable
service that is easy to run locally.

## Decision

- The backend is one Rust workspace. Each business domain is its own library
  crate under `backend/domains/<domain>`, exposing an `axum::Router`.
- `sis-api` (in `backend/platform/api`) is the only binary. It mounts each
  domain router at `/api/v1/<domain>`.
- Shared building blocks live in `sis-core`. Domains do not
  depend on each other directly; they talk through public contract types and
  HTTP APIs.
- Each domain has a `contract.rs` with the types other domains and clients rely
  on. Contract changes are reviewed by every team that owns the domain.
- Frontends (web, desktop, mobile, MCP) live under `clients/` and talk to the
  backend only through HTTP, using the shared `@sis/api-client` package.

## Consequences

- Ownership maps cleanly onto folders and CODEOWNERS.
- Domains can be split into separate services later if needed, because they
  already communicate through explicit contracts.
- TypeScript types in `@sis/api-client` mirror Rust contracts by hand for now.
  Generating them from an OpenAPI document is a likely follow-up.
