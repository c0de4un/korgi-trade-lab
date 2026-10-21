# AGENTS.md - AI Assistant Guidelines for Korgi.Trade-Lab

This file provides context and strict rules for AI assistants (Cursor, Copilot, etc.) working on the Korgi.Trade-Lab codebase.

## Project Context

Korgi.Trade-Lab is a high-performance SaaS for historical market simulation and LOB (Limit Order Book) engine. The backend is written in Rust, focusing on deterministic execution, low latency, and strict memory management.

## Repository Layout

One **Cargo workspace** at the repository root:

- `crates/*` — shared libraries (`market-types`, `lob-core`, `importers`, `sim-engine`, `bot-protocol`, `storage`, `fix-codec`)
- `lob/backend/{lob-server,lob-cli}` — LOB HTTP server and CLI
- `cabinet/backend/{cabinet-server,cabinet-cli}` — trader cabinet HTTP server and CLI
- `lob/frontend`, `cabinet/frontend` — Vue 3 + Vite SPAs (outside Cargo)

Architecture decisions: see `docs/adr/`.

## Architectural Principles

1. **Hexagonal Architecture (Ports and Adapters)**: Core business logic must not depend on frameworks, databases, or HTTP.
2. **Modular Monolith (workspace)**: One Cargo workspace, two backend processes (`lob-server`, `cabinet-server`). Boundaries between modules are enforced via Rust traits and clear module visibility. Shared code lives in `crates/`.
3. **SOLID Principles**: Strictly adhere to SOLID, especially Single Responsibility and Dependency Inversion.

## Strict Code Rules (Rust)

* **One Entity Per File**:
  * Every `enum` must be in its own separate file.
  * Every `trait` (interface) must be in its own separate file.
  * Every `struct` must be in its own separate file.
  * Use `mod.rs` or folder-level `mod` declarations to expose them.
* **Edition**: Use Rust 2024 Edition.
* **Identifiers**: Always use **UUID v7** (time-ordered) instead of UUID v4. Use the `uuid` crate with the `v7` feature.
* **Financial Math**: **NEVER** use `f32` or `f64` for prices, quantities, or financial calculations. Use exact integer types (e.g., `i64`, `u64`) scaled by tick/lot sizes.
* **Time**: All timestamps must be in UTC, represented as `i64` (nanoseconds).
* **Error Handling**: Use `thiserror` for library/domain errors and `anyhow` for application/top-level errors. Avoid `.unwrap()` in production code; use proper error propagation (`?`).
* **Async**: Use `tokio`. CPU-bound tasks (like LOB matching) must be offloaded to a dedicated thread pool (e.g., `tokio::task::spawn_blocking` or a custom OS thread pool) to avoid blocking the async executor.
* **Domain purity**: `lob-core` and `market-types` must not depend on `tokio`, `axum`, `sqlx`, or filesystem I/O.

## Database & Storage

* **PostgreSQL**: Used for metadata, users, configurations, and aggregated results. Use `sqlx` with compile-time checked queries.
* **File Storage**: Historical market data (ticks, L2 snapshots) is stored in compressed chunks on the local filesystem (`CHUNK_ROOT`, default `data/chunks/`), NOT in PostgreSQL.
* **Redis**: Used for caching, rate limiting, and ephemeral state.

## Testing

* Write unit tests for every domain entity and use case.
* Use `proptest` for invariant testing (e.g., LOB never crosses, FIFO is maintained).
* Ensure deterministic behavior: tests must not rely on system time or random number generators without fixed seeds.
