# Application Layer

The application layer orchestrates use cases and stateful game sessions.

## Modules

- `application/user`: register, login, authenticate, list users
- `application/gm`: GM login and JWT validation use cases
- `application/game`: game create/delete/overview + runtime + session lifecycle
- `application/plugin`: plugin lexer/parser/runtime/debugger
- `application/common`: connection abstractions

## Request Handler Pattern for non-stateful Requests

Each use case is represented by a handler that depends on contracts.

```mermaid
flowchart LR
    req[HTTP/WebSocket Input] --> handler[Request/Command Handler]
    handler --> contracts[Repository/JWT Contracts]
    contracts --> impls[Infrastructure Implementations]
    handler --> result[Response DTO or Domain Error]
```

## Stateful Realtime Game Management

```mermaid
sequenceDiagram
    participant Client
    participant WsHandler
    participant SessionManager
    participant GameSession
    participant GameRuntime

    Client->>WsHandler: request ticket + connect
    WsHandler->>SessionManager: get_or_create_session(game_id)
    SessionManager->>GameSession: create/load if needed
    WsHandler->>GameSession: gm_connect/user_connect
    loop every command
        Client->>GameSession: GameCommand
        GameSession->>GameRuntime: handle_command(command)
        GameSession-->>Client: broadcast updated projection
    end
```
