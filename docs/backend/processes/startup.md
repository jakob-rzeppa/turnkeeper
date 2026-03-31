# Startup and Bootstrapping

The backend startup flow is implemented in `backend/src/main.rs`, `backend/src/lib.rs` and `infrastructure/persistence/db.rs`.

## Runtime State Built at Boot

```mermaid
flowchart LR
    pool[SqlitePool] --> rm[RepositoryManager]
    auth[AuthManager] --> state[AppState]
    sessions[GameSessionManager] --> state
    rm --> state
    state --> app[Axum Router]
```

## Failure Points

- missing `.env` in local setup
- invalid/missing env variables (`GM_PASSWORD`, JWT secrets)
- DB connection/migration failure
- TCP bind failure on port `8080`
