# Persistence and Migrations

Persistence is implemented with SQLx + SQLite and command logging for game state reconstruction.

## Schema Overview

```mermaid
erDiagram
    users {
        string id PK
        string name UK
        string password
    }

    games {
        string id PK
        string name UK
    }

    games_log {
        string game_id
        datetime timestamp
        string command
    }

    games ||--o{ games_log : has
```

## Repository Layout

- `SqliteUserRepository`: CRUD-like user access
- `SqliteGameRepository`: game metadata and command log
- `RepositoryManager`: shared access point injected into app state

## Event-Log Style Game Persistence

```mermaid
flowchart LR
    create[Create game metadata row] --> runtime[In-memory GameRuntime]
    runtime --> cmd[Apply command]
    cmd --> log[Store command JSON in games_log]
    log --> replay[Replay command history on next session creation]
```
