# Database

## Overview ✅

The diagram is available at `docs/Database.puml`.

## Design Decisions

- **UUIDs (`VARCHAR(36)`)**: Sqlite doesn't support UUIDs, so we use VARCHAR(36). The performance gain of BLOB is unnecessary.

## Migrations / Schema

- Migration scripts live in `migrations/`.
- When making schema changes, add a new migration and update `docs/Database.puml` and this `DATABASE.md` if the change affects constraints or important defaults.
