# Turnkeeper API Documentation

This document provides an overview of the API endpoints for the Turnkeeper backend. It describes the available routes, request/response formats, authentication requirements, and WebSocket event protocol.

## Error Handling

All errors are returned as JSON with an appropriate HTTP status code and error message.

**Example error response:**

```json
{
    "error": "Invalid credentials"
}
```

---

## Authentication

Most endpoints require authentication via a JSON Web Token (JWT). Obtain a token using the login or register endpoints and include it in the `Authorization` header as `Bearer <token>`.

### POST `/user/login`

Authenticate a user and receive a JWT.

**Request Body:**

```json
{
    "name": "string (required)",
    "password": "string (required)"
}
```

**Response:**

```json
{
    "token": "string"
}
```

- Returns 401 if credentials are invalid.

---

### POST `/user/register`

Register a new user and receive a JWT.

**Request Body:**

```json
{
    "name": "string (required)",
    "password": "string (required)"
}
```

**Response:**

```json
{
    "token": "string"
}
```

- Returns 400 if the username is already taken or input is invalid.

---

### POST `/gm/login`

Authenticate as the Game Master. The password is validated against the `GM_PASSWORD` environment variable.

**Request Body:**

```json
{
    "password": "string (required)"
}
```

**Response:**

```json
{
    "token": "string"
}
```

- Returns 401 if the password is incorrect.

---

## Game Management

### GET `/gm/games`

List all games. **Requires:** GM JWT.

**Response:**

```json
[
    { "id": "uuid", "name": "string" },
    ...
]
```

---

### GET `/user/games`

List all games. **Requires:** User JWT.

**Response:** Same format as `GET /gm/games`.

---

### POST `/gm/games`

Create a new game. **Requires:** GM JWT.

**Request Body:**

```json
{
    "name": "string (required)"
}
```

**Response:**

```json
{
    "id": "uuid",
    "name": "string"
}
```

- Returns 400 if the name is invalid or already taken.

---

### DELETE `/gm/games/{id}`

Delete a game. **Requires:** GM JWT.

> **Warning:** The repository implementation for delete is not yet complete (`todo!()`). Calling this endpoint will cause a server panic.

---

## WebSocket Authentication

The browser WebSocket API does not support custom headers, so authentication uses a **ticket-based flow**:

1. The client obtains a short-lived, single-use ticket via an authenticated HTTP endpoint.
2. The client opens the WebSocket using the URL returned by that endpoint (which contains the ticket as a query parameter).
3. The server validates the ticket on the WebSocket upgrade request.

### POST `/gm/ws/ticket/{game_id}`

Obtain a WebSocket connection URL with an embedded authentication ticket.

**Requires:** `Authorization: Bearer <gm_token>`

**Response:**

```json
{
    "url": "ws://localhost:8080/gm/ws/{game_id}?ticket=<ticket>"
}
```

### Ticket Properties

| Property    | Value                                                         |
| ----------- | ------------------------------------------------------------- |
| Lifetime    | 30 seconds                                                    |
| Usage       | Single-use (consumed on connection)                           |
| Storage     | In-memory, as `ConnectionState::Pending` inside `GameSession` |
| Format      | UUID v4                                                       |
| Game scoped | Yes — ticket is bound to the session for a specific game ID   |

- Returns 401 if the GM token is missing or invalid.
- Returns an error if a GM connection or pending ticket already exists for this game session.
- Expired pending tickets are cleaned up opportunistically on the next `gm_pre_connect()` call.

---

### GET `/gm/ws/{id}?ticket=<ticket>`

WebSocket upgrade endpoint. Requires a valid ticket obtained from `POST /gm/ws/ticket/{game_id}`.

- Returns an error if:
    - No pending ticket exists for this session (`NoPendingConnection`)
    - The ticket doesn't match or has expired (`InvalidConnectionToken`)
    - A GM is already connected (`GmAlreadyConnected`)

---

### POST `/user/ws/ticket/{game_id}`

Obtain a WebSocket connection URL with an embedded authentication ticket for a user.

**Requires:** `Authorization: Bearer <user_token>`

**Response:**

```json
{
    "url": "ws://localhost:8080/user/ws/{game_id}?ticket=<ticket>&user_id=<user_id>"
}
```

### Ticket Properties

| Property    | Value                                                             |
| ----------- | ----------------------------------------------------------------- |
| Lifetime    | 30 seconds                                                        |
| Usage       | Single-use (consumed on connection)                               |
| Storage     | In-memory, as `UserConnectionState::Pending` inside `GameSession` |
| Format      | UUID v4                                                           |
| Game scoped | Yes — ticket is bound to the session for a specific game ID       |
| User scoped | Yes — ticket is bound to the authenticated user                   |

- Returns 401 if the user token is missing or invalid.
- Multiple users can obtain tickets for the same game session simultaneously.

---

### GET `/user/ws/{id}?ticket=<ticket>&user_id=<user_id>`

WebSocket upgrade endpoint for users. Requires a valid ticket obtained from `POST /user/ws/ticket/{game_id}`.

- Returns an error if:
    - No pending ticket exists for this user in this session (`NoPendingConnection`)
    - The ticket doesn't match or has expired (`InvalidConnectionToken`)
    - The user is already connected (`UserAlreadyConnected`)

---

## WebSocket Events

Once a WebSocket connection is established, the GM client sends JSON-serialized `GameEvent` messages. After each event, the server responds with the full game state as a `GmGameInfo` JSON object.

### Client → Backend Events

Both GM and user clients send events over WebSocket. Events are JSON-serialized `GameEvent` enum variants.

| Event             | JSON Format                                      | Description                                        |
| ----------------- | ------------------------------------------------ | -------------------------------------------------- |
| AddPlayer         | `"AddPlayer"`                                    | Adds a new anonymous player to the game            |
| ChangePlayerOrder | `{"ChangePlayerOrder": ["uuid1", "uuid2", ...]}` | Reorders players by providing ordered player UUIDs |
| Debug             | `{"Debug": "message"}`                           | Debug event — prints message to server console     |

> **Note:** A `GameEvent::is_user_permitted()` method exists in the domain but is not currently enforced — user clients can send all events.

### Backend → Client Response (GmGameInfo)

After every event, the server broadcasts the full game state to **all** connected clients (GM and users):

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
                    "string_value": "string | null",
                    "number_value": "number | null",
                    "boolean_value": "boolean | null"
                }
            ]
        }
    ],
    "round_number": 0,
    "current_player_index": 0
}
```

> **Note:** Players can exist without an assigned user (`user: null`). Stats are defined in the domain but there are currently no WebSocket events to add, edit, or remove them.

> **Note:** Event logging to the `games_log` table is defined in the schema and repository contract but the implementation is not yet active (commented out).
