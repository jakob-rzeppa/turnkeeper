# Architecture

## System Overview

```mermaid
graph TB
    subgraph "GM Device"
        GM[GM Client<br/>Vue.js SPA]
        Backend[Backend Server<br/>Rust/Axum]
        DB[(SQLite DB)]

        GM <-->|HTTP + WS| Backend
        Backend <--> DB
    end

    subgraph "User Devices"
        User1[User 1<br/>Vue.js SPA]
        User2[User 2<br/>Vue.js SPA]
        User3[User 3<br/>Vue.js SPA]
    end

    User1 <-->|HTTP| Backend
    User2 <-->|HTTP| Backend
    User3 <-->|HTTP| Backend

    style GM fill:#e1f5ff
    style Backend fill:#fff3e0
    style User1 fill:#f3e5f5
    style User2 fill:#f3e5f5
    style User3 fill:#f3e5f5
```

> **Note:** User WebSocket connections are not yet implemented. Users currently only interact via HTTP (login/register).

## GM Auth / Game Start

```mermaid
sequenceDiagram
    participant GM as GM Client
    participant Backend as Backend Server

    GM->>+Backend: POST /gm/login
    note over Backend: Validate password against GM_PASSWORD env var
    Backend->>-GM: JSON web token

    GM->>Backend: GET /gm/games
    Backend->>GM: List of game metadata (id, name)
    note over GM: Choose a game to resume

    GM->>+Backend: POST /gm/ws/ticket/{game_id}
    note over Backend: Validate JWT, create ticket (30s TTL)
    Backend->>-GM: { url: "ws://.../gm/ws/{id}?ticket=..." }

    GM->>+Backend: WS connect to returned URL
    note over Backend: Validate ticket (single-use)
    Backend->>-GM: WebSocket connection established
    note over Backend: Send full GmGameInfo state
```

## User Auth

```mermaid
sequenceDiagram
    participant User as User Client
    participant Backend as Backend Server

    User->>+Backend: POST /user/login
    note over Backend: Validate user credentials
    Backend->>-User: JSON web token

    note over User: User frontend currently only supports login.<br/>Game connectivity is not yet implemented.
```

> **Note:** The user frontend currently only provides login functionality. There is no game view or WebSocket connection for users yet. User registration exists as a backend endpoint (`POST /user/register`) but the frontend always calls `/user/login`.

## API HTTP Endpoints

### Auth (Unauthenticated)

- `POST /user/login` → JWT
- `POST /user/register` → JWT
- `POST /gm/login` → JWT

### Games

- `GET /user/games` → All game metadata (unauthenticated)
- `GET /gm/games` → All game metadata (GM JWT required)
- `POST /gm/games` → Create new game (GM JWT required)
- `DELETE /gm/games/:id` → Delete game (GM JWT required, **not yet implemented in repository**)

### WebSocket Tickets

- `POST /gm/ws/ticket/:game_id` → Get single-use WS ticket URL (GM JWT required)

## WebSocket Connection

### Authentication

WebSocket connections are authenticated via a ticket-based flow, since the browser WebSocket API does not support custom headers.

```mermaid
sequenceDiagram
    participant Client
    participant Backend

    Client->>+Backend: POST /gm/ws/ticket/{game_id}<br/>Authorization: Bearer <token>
    note over Backend: Validate JWT
    note over Backend: Generate single-use ticket<br/>(UUID v4, 30s TTL)
    Backend->>-Client: { url: "ws://.../gm/ws/{id}?ticket=..." }

    Client->>+Backend: GET /gm/ws/{id}?ticket=...<br/>(WebSocket upgrade)
    note over Backend: Validate & consume ticket
    note over Backend: Verify ticket game_id matches path
    Backend->>-Client: 101 Switching Protocols
```

### Entrypoints

- GM: `/gm/ws/:game_id?ticket=...`

> User WebSocket entrypoints are not yet implemented.

## WebSocket Events

Events are sent as JSON-serialized Rust enum variants. After each event, the server responds with the full `GmGameInfo` game state.

### GM → Backend Events

| Event               | JSON Payload                                 | Description                                        |
| ------------------- | -------------------------------------------- | -------------------------------------------------- |
| `AddPlayer`         | `"AddPlayer"`                                | Adds a new anonymous player to the game            |
| `ChangePlayerOrder` | `{"ChangePlayerOrder": ["id1", "id2", ...]}` | Reorders players by providing ordered player UUIDs |
| `Debug`             | `{"Debug": "message"}`                       | Debug event (prints to server console)             |

### Backend → GM Response

After every event, the server sends the full game state:

```json
{
    "id": "uuid",
    "name": "string",
    "players": [
        {
            "id": "uuid",
            "user": { "id": "uuid", "name": "string" } | null,
            "stats": [
                {
                    "id": "uuid",
                    "key": "string",
                    "value_type": "String" | "Number" | "Boolean",
                    "string_value": "string" | null,
                    "number_value": number | null,
                    "boolean_value": boolean | null
                }
            ]
        }
    ],
    "round_number": 0,
    "current_player_index": 0
}
```

### Player Management Flow

```mermaid
sequenceDiagram
    participant Gm as GM Client
    participant Backend as Backend Server

    Gm->>+Backend: "AddPlayer"
    Backend->>-Gm: Full GmGameInfo state

    Gm->>+Backend: {"ChangePlayerOrder": ["id1","id2",...]}
    Backend->>-Gm: Full GmGameInfo state
```
