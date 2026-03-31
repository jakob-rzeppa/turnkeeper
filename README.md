# Turnkeeper

Turnkeeper is a turn-based game management platform.

It consists of:

- A Rust backend (HTTP + WebSocket + SQLite)
    - A custom plugin language/runtime
- A GM web client (Vue 3 + TypeScript + Vite)
- A User web client (Vue 3 + TypeScript + Vite)

## Documentation

Start here:

- [Documentation Index](docs/README.md)
- [How To Use](docs/HOW_TO_USE.md)
- [Architecture](docs/ARCHITECTURE.md)

Deep dives:

- [Backend Documentation](docs/backend/README.md)
- [API and Realtime](docs/API.md)
- [Plugin Documentation](docs/backend/plugins/README.md)

## Repository Layout

- `backend/`: Rust backend service, migrations, integration tests
- `gm/`: GM frontend application
- `user/`: User frontend application
- `docs/`: project documentation
- `docker-compose.yml`: local containerized backend run
