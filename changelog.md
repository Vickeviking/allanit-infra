# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned

- Add integration tests for full Fetcher → Ingestor → DBWriter pipeline.
- Introduce dashboard module for live monitoring of module status.
- Add structured JSON logs ingestion to external observability stack.

## [1.0.0] - 2025-10-07

### Added

#### Core Infrastructure

- Initial Docker Compose setup including `core`, `postgres`, `redis`, and `seventimes-mock`.
- Rocket-based backend service managing all modules under `allanit_infra_core`.
- Environment configuration via `.env` and Docker Compose variables (`SEVENTIME_BASE_URL`, `SEVENTIME_CLIENT_SECRET`, etc.).

#### Database & Persistence

- PostgreSQL schema and Diesel migrations for the following tables:
  - `customers`
  - `purchase_orders`
  - `emails_inbound`
  - `invoices_raw`
  - `system_logs`
  - `users`

- Enum SQL types: `email_status`, `invoice_status`, and `log_level`.
- Generated Diesel schema with async support (`diesel_async`).
- Log entries persisted both to disk and to the `system_logs` table.

#### Data Models

- Strongly typed Rust models (`Queryable`, `Insertable`, `Identifiable`) for all database entities.
- `New*` insert variants for batch inserts and upserts.

#### Repositories

- `CustomerRepository` and `PurchaseOrderRepository` with async upsert, list, and find methods.
- `EmailInboundRepository` and `InvoiceRawRepository` for ingesting incoming messages and invoice payloads.
- `LogRepository` for structured log persistence.
- `UserRepository` handling registration, lookup, and authentication.

#### Application Modules

- **Logger:** Central asynchronous logger storing logs in both file and database.
- **CommandBus:** Core event bus for inter-module communication using broadcast/mpsc.
- **SeventimeFetcher:** Fetches data from Seventime Mock API on each pulse, converts responses to domain types, and emits them downstream.
- **IngestorRouter:** Validates and deduplicates incoming payloads; routes them to DBWriter or DeadLetter.
- **DBWriter:** Executes transactional batch upserts into PostgreSQL.
- **DeadLetterWorker:** Handles failed or malformed messages for later review.

#### Domain Contracts

- Extended `EventPayload` with domain-level variants:
  - `UpsertCustomers(Vec<NewCustomer>)`
  - `UpsertPurchaseOrders(Vec<NewPurchaseOrder>)`
  - `DeadLettered { reason: String }`

- `CoreEvent` and `Pulse` enums drive lifecycle and heartbeat logic.

#### API and Routes

- Rocket-based REST API with:
  - `User` routes for authentication and session management.
  - `Logs` routes for querying stored system logs.

- Request guards using Redis cache for session validation.

#### Mock Integration

- Included `seventimes-mock` Python FastAPI app:
  - Endpoints: `GET /api/v1/customers` and `GET /api/v1/purchase-orders`.
  - Simulates Seventime’s external API for local development and tests.

### Changed

- Unified configuration loading via environment variables and centralized `Cfg` struct.
- Improved logging granularity for all lifecycle events (`Startup`, `Restart`, `Shutdown`).
- Standardized naming across modules and repository methods.
- Normalized time handling using `chrono::Utc` and `TIMESTAMPTZ` for all timestamps.

### Fixed

- Ensured all async database operations are awaited properly within transactions.
- Fixed race condition in logger shutdown by ensuring final flush before exit.
- Corrected deduplication logic in `IngestorRouter` for `external_id` handling.
- Resolved missing index definitions for `customers` and `purchase_orders` tables.

### Removed

- Placeholder data structures for legacy log handling.
- Outdated local-only file logger implementation replaced by unified DB logger.

[unreleased]: https://github.com/allanit/allanit-infra/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/allanit/allanit-infra/releases/tag/v1.0.0
