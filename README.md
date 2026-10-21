# Korgi.Trade-Lab

**Korgi.Trade-Lab** is a comprehensive SaaS platform for historical market simulation, Limit Order Book (LOB) engine, strategy backtesting, and external trading bot integration. 

The project is designed as a modular monolith, focusing on high performance, deterministic execution, and strict resource isolation. It serves as a production-oriented tool for quantitative development, system engineering, and strategy research.

## 🏗 Architecture & Modules

The repository is structured as a collection of independent backend services and frontend applications:

*   **LOB Engine (`lob/backend`)**: Core L2 Market-by-Price engine, Replay Engine, and Execution Simulator. Built with Rust (Axum, Tokio, SQLx).
*   **Trader Cabinet (`cabinet/backend`)**: User management, simulation orchestration, resource limits, and SaaS administration. Built with Rust (Axum, Tokio, SQLx).
*   **Frontends (`lob/frontend`, `cabinet/frontend`)**: Vue 3 + TypeScript + Vite SPAs (Client and Admin dashboards).

## 💻 Hardware Requirements

The system is optimized for cost-effective deployment. Minimum recommended server specifications for production:
*   **CPU**: 2 vCPU
*   **RAM**: 2 GB (Strict memory budgets are applied per simulation task)
*   **Disk**: SSD recommended for historical market data chunk storage

## 🛠 Software Requirements

*   **Rust**: 2024 Edition (Stable)
*   **Node.js**: 22 LTS (for frontend tooling)
*   **Database**: PostgreSQL 16+
*   **Cache/Queue**: Redis
*   **Tools**: Docker, Docker Compose, Make, Git

## 🚀 Quick Start & Commands

We use a `Makefile` to abstract build, test, and run commands. 

### Build Modules
To build the backend binaries separately:
```bash
make build-lob
make build-cabinet
```

### Run in Development
To start the backend services in development mode:
```bash
make run-lob-dev
make run-cabinet-dev
```

### Testing & Linting
Tests and builds are strictly separated:
```bash
make test-lob
make test-cabinet
make test-all
make lint
```

### Infrastructure (Local Development)
To spin up the required infrastructure services (PostgreSQL, Redis) using Docker Compose:
```bash
docker compose up -d
```

(Note: Production deployment relies on systemd and static binaries, Docker is primarily for local dev and CI).