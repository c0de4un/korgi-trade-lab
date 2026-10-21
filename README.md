# Korgi.Trade-Lab

[![License: MIT](https://img.shields.io/github/license/c0de4un/korgi-trade-lab)](LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/c0de4un/korgi-trade-lab)](https://github.com/c0de4un/korgi-trade-lab/stargazers)
[![GitHub issues](https://img.shields.io/github/issues/c0de4un/korgi-trade-lab)](https://github.com/c0de4un/korgi-trade-lab/issues)

[![Rust](https://img.shields.io/badge/Rust-2024-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Tokio](https://img.shields.io/badge/Tokio-async-black?logo=tokio&logoColor=white)](https://tokio.rs/)
[![Axum](https://img.shields.io/badge/Axum-0.8-blue)](https://github.com/tokio-rs/axum)
[![Vue.js](https://img.shields.io/badge/Vue.js-3-4FC08D?logo=vuedotjs&logoColor=white)](https://vuejs.org/)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-16-4169E1?logo=postgresql&logoColor=white)](https://www.postgresql.org/)
[![Redis](https://img.shields.io/badge/Redis-cache-DC382D?logo=redis&logoColor=white)](https://redis.io/)

[![LOB](https://img.shields.io/badge/LOB-engine-2E7D32)](https://github.com/c0de4un/korgi-trade-lab)
[![Backtesting](https://img.shields.io/badge/Backtesting-simulation-1565C0)](https://github.com/c0de4un/korgi-trade-lab)
[![Trading](https://img.shields.io/badge/Trading-SaaS-6A1B9A)](https://github.com/c0de4un/korgi-trade-lab)

**Korgi.Trade-Lab** is a SaaS platform for historical market simulation, Limit Order Book (LOB) engine, strategy backtesting, and external trading bot integration.

The project is a **Cargo workspace** (modular monolith): shared domain crates plus two backend binaries and two Vue frontends. Focus areas: deterministic execution, strict resource isolation, and clear hexagonal boundaries.

License: **MIT**. Architecture decisions: [`docs/adr/`](docs/adr/).

## Architecture & Modules

```
korgi-trade-lab/
├── crates/                    # Shared Rust libraries
│   ├── market-types/          # Canonical events, instrument specs
│   ├── lob-core/              # Order book + matching (no I/O)
│   ├── importers/             # Market data adapters
│   ├── sim-engine/            # Replay / SimGateway
│   ├── bot-protocol/          # Bot REST DTOs
│   ├── storage/               # PG metadata + FS chunk ports
│   └── fix-codec/             # FIX stub (post-MVP)
├── lob/backend/
│   ├── lob-server/            # LOB HTTP API (:8080)
│   └── lob-cli/               # import / sim / bench without SaaS
├── cabinet/backend/
│   ├── cabinet-server/        # Identity, quotas, orchestration (:8081)
│   └── cabinet-cli/           # Admin / dev utilities
├── lob/frontend/              # Vue SPA (client)
└── cabinet/frontend/          # Vue SPA (admin)
```

- **LOB Engine** (`lob-server`): L2 MBP engine, replay, execution simulator, bot REST; gRPC/WS/FIX are stubs.
- **Trader Cabinet** (`cabinet-server`): users, simulation orchestration, resource limits, admin API.
- **Storage**: PostgreSQL for metadata; filesystem chunks under `data/chunks/` (see [ADR 0002](docs/adr/0002-chunk-storage-fs.md)).

## Hardware Requirements

Minimum recommended server specifications for production:

* **CPU**: 2 vCPU
* **RAM**: 2 GB (strict memory budgets per simulation task)
* **Disk**: SSD recommended for historical market data chunk storage

## Software Requirements

* **Rust**: 2024 Edition (Stable), see `rust-toolchain.toml`
* **Node.js**: 22 LTS (for frontend tooling)
* **Database**: PostgreSQL 16+
* **Cache/Queue**: Redis
* **Tools**: Docker, Docker Compose, Make, Git

## Quick Start

### Infrastructure

```bash
make infra-up
# or: docker compose -f compose.local.yml up -d
```

### Build & run backends

```bash
make build
make run-lob-dev       # http://127.0.0.1:8080/healthz
make run-cabinet-dev   # http://127.0.0.1:8081/healthz
```

Override bind addresses with `LOB_SERVER_ADDR` / `CABINET_SERVER_ADDR`.

### CLI

```bash
cargo run -p lob-cli -- --help
cargo run -p cabinet-cli -- --help
```

### Testing & linting

```bash
make test-all
make lint
make fmt
```

### Docker image (optional)

```bash
docker build --build-arg BIN=lob-server -t korgi-lob .
docker build --build-arg BIN=cabinet-server -t korgi-cabinet .
```

Application services in `compose.local.yml` are commented until you enable them after the first image build.

Production target remains static musl binaries + systemd; Docker is primarily for local development and CI.
