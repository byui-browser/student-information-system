# Contributing

## Workflow

1. **Start from an issue.** Every pull request links to one (`Closes #123`).
2. **Branch from `main`** with a short-lived branch named
   `<team>/<type>/<issue>-<slug>`, for example `a3-cat/feat/123-course-prereqs`.
   Types: `feat`, `fix`, `test`, `docs`, `refactor`, `chore`.
3. **Keep it small.** Aim for under 400 changed lines. Split larger work into
   a sequence of PRs, and hide unfinished features behind a flag.
4. **Run the checks locally** (see the README) before opening the PR.
5. **Get review.** One approval from a teammate. Contract and migration
   changes also need approval from the other team that owns the domain, or
   from the relevant guild steward.
6. **Merge through the merge queue.** No direct pushes to `main`.

## Contract first

Each domain keeps the types other domains and clients depend on in
`src/contract.rs` (create it the first time you need one). Change the contract
in its own PR before the code that uses it:

1. PR 1: update `contract.rs` and the mirrored types in
   `clients/shared/api-client/src/types.ts`. Both owning teams approve.
2. PR 2+: implement the logic, routes, and client screens.

Adding a field is usually safe. Renaming or removing one is a breaking change:
coordinate with every team that consumes it.

## Adding an endpoint

Using catalog as the example:

1. Add or extend types in `backend/domains/catalog/src/contract.rs`.
2. Put the logic in a slice module (for example `courses.rs`) with unit tests.
3. Add the handler and route to the domain's `router()`.
4. Add an HTTP test in `tests/api.rs`.
5. Add the method to `clients/shared/api-client/src/client.ts` and a test.
6. Use it from a client under `clients/<client>/src/features/catalog/`.

## Adding a new crate

If a domain grows enough to warrant another crate (for example a separate
persistence layer):

1. Create it under the domain folder, e.g. `backend/domains/finance/aid/`,
   named `sis-finance-aid`.
2. Add it to `members` and `[workspace.dependencies]` in the root `Cargo.toml`.
3. Inherit workspace settings (`version.workspace = true`, `[lints] workspace = true`).
4. Include a `lib.rs`, at least one public type and function, and tests.

## Code style

- Rust: `cargo fmt` and `cargo clippy` with no warnings. Prefer small, pure
  functions in slice modules and thin HTTP handlers.
- TypeScript: strict mode, no `any`. Keep API calls in `@sis/api-client`, not in components.
- Tests live next to the code (`#[cfg(test)]` / `*.test.ts`) plus HTTP tests in `tests/`.

## Decisions

Record significant technical decisions as ADRs in `docs/adr/`, copying
`0000-template.md` and using the next number.
