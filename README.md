# Student Information System

A modern student information system for universities and colleges.

- **Backend:** Rust ([axum](https://github.com/tokio-rs/axum)) modular monolith, one crate per domain, served as a single HTTP API.
- **Clients:** web (React + Vite), desktop ([Tauri](https://tauri.app)), iOS and Android ([Expo](https://expo.dev)), and an [MCP](https://modelcontextprotocol.io) server for AI assistants. All of them share one typed API client.

## Quick start

Prerequisites: [Rust](https://rustup.rs) (stable) and [Node.js](https://nodejs.org) 22 or newer.

```sh
# 1. Backend: run the API on http://localhost:8080
cargo run -p sis-api

# 2. Clients: install once from the repo root, then start any client
npm install
npm run dev:web        # http://localhost:5173
npm run dev:desktop    # native window (first build takes a few minutes)
npm run dev:mobile     # Expo: press i (iOS simulator), a (Android), or scan the QR code
npm run dev:mcp        # MCP server on stdio, see clients/mcp/README.md
```

Try the API:

```sh
curl localhost:8080/health
curl localhost:8080/api/v1/catalog/health
```

## Checks (CI runs these on every pull request)

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace

npm run typecheck
npm run lint
npm test
```

Run a single crate's tests with `cargo test -p sis-catalog`.

## Repository layout

```
backend/
  platform/
    core/            sis-core     shared types used by every domain
    api/             sis-api      the HTTP server; mounts every domain at /api/v1/<domain>
  domains/
    identity/        sis-identity     users, roles, authentication
    catalog/         sis-catalog      courses and sections
    enrollment/      sis-enrollment   registration and waitlists
    records/         sis-records      student profiles, grades, transcripts
    finance/         sis-finance      charges, payments, holds
    advising/        sis-advising     degree programs and degree audits
    admissions/      sis-admissions   applications and decisions
clients/
  shared/api-client/ @sis/api-client  typed API client used by every client
  web/               @sis/web         React + Vite
  desktop/           @sis/desktop     Tauri (React UI + Rust shell in src-tauri/)
  mobile/            @sis/mobile      Expo / React Native for iOS and Android
  mcp/               @sis/mcp         MCP server exposing API calls as tools
docs/domains/        what each domain is responsible for and the features it supports
docs/adr/            architecture decision records
```

### Inside a domain crate

Each domain starts as a single `src/lib.rs` with one example type, one
example function, a `router()` with a `/health` route, and tests. Grow it
into this shape as the domain is built:

```
backend/domains/catalog/
  src/
    lib.rs         module list and router()
    contract.rs    public types other domains and clients depend on (shared by both owning teams)
    courses.rs     one slice: logic + unit tests
    sections.rs    the other slice: logic + unit tests
    routes.rs      HTTP handlers, mounted at /api/v1/catalog
  tests/
    api.rs         HTTP-level tests against the domain router
```

Split each domain into two slices so two teams can work in parallel without
editing the same files. Domains should not import each other; share data
through contracts and HTTP.

### Inside a client

Each client keeps an app shell (`src/App.tsx`: layout, navigation) owned by
platform. Put each domain's screens in `clients/<client>/src/features/<domain>/`
and MCP tools in `clients/mcp/src/tools/<domain>.ts`.

## API

All domain routes live under `/api/v1/<domain>`. Today every domain exposes
only `GET /api/v1/<domain>/health`, plus `GET /health` for the server.

Not built yet: persistence, authentication, a shared error format, and every
domain endpoint.

## Configuration

See [`.env.example`](.env.example). The main settings:

| Variable | Used by | Default |
| --- | --- | --- |
| `SIS_ADDR` | API | `127.0.0.1:8080` |
| `RUST_LOG` | API | `info,tower_http=debug` |
| `SIS_API_URL` | desktop, MCP, web dev proxy | `http://localhost:8080` |
| `EXPO_PUBLIC_SIS_API_URL` | mobile | `http://localhost:8080` (Android emulator: `http://10.0.2.2:8080`) |

## Domains

See [docs/domains](docs/domains/README.md) for each domain's responsibilities,
features, business rules, and dependencies.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

[Apache 2.0](LICENSE)
