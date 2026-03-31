# Backend Documentation

See also:

- [Main Docs Index](../README.md)
- [System Architecture](../ARCHITECTURE.md)
- [API and Realtime](../API.md)

This section documents the backend by layer and by runtime process.

## Scope

The backend project is located in `backend/` and includes:

- `src/`: application code
- `migrations/`: SQLite migrations
- `tests/`: functional/integration tests
- `backend-derive/`: derive macro crate
- `var/`: local runtime files

## Stack

- Rust (edition 2024)
- Axum (HTTP + WebSocket)
- SQLx + SQLite
- JWT-based GM and User authentication

## Documentation Map

- [Layer Overview](layers/README.md)
- [Process Overview](processes/README.md)
- [Plugin Overview](plugins/README.md)
