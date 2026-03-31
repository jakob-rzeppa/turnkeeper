# Infrastructure Layer

Infrastructure adapts external systems (HTTP, WS, SQLite, JWT) to application contracts and handles the underlying program that contains the application and domain layers.

## Modules

- `infrastructure/http`: REST routes and handlers
- `infrastructure/websocket`: ticket endpoints, WS upgrades, session manager, WS connection wrapper
- `infrastructure/auth`: JWT generators/validators + auth middleware
- `infrastructure/persistence`: SQLx repositories and DB pool setup
- `infrastructure/error`: translation from domain/application errors to HTTP errors

## Route Surface

- GM: login, games CRUD (delete WIP), users list, WS ticket and WS connect
- User: login/register, games list, users list, WS ticket and WS connect
