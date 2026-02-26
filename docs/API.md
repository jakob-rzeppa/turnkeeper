# Turnkeeper API Documentation

This document provides an overview of the central API endpoints for the Turnkeeper backend. It describes the available routes, request/response formats, and authentication requirements.

## Error Handling

- All errors are returned as JSON with an appropriate HTTP status code and error message.

**Example error response:**

```json
{
    "error": "Invalid credentials"
}
```

###

## Authentication

Most endpoints require authentication via a JSON Web Token (JWT). Obtain a token using the `/user/login` or `/user/register` endpoints and include it in the `Authorization` header as `Bearer <token>`.

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

| Property    | Value                                |
| ----------- | ------------------------------------ |
| Lifetime    | 30 seconds                           |
| Usage       | Single-use (consumed on first check) |
| Storage     | In-memory (server-side)              |
| Format      | UUID v4                              |
| Game scoped | Yes — ticket is bound to a game ID   |

- Returns 401 if the GM token is missing or invalid.
- The ticket is deleted after validation regardless of success.
- Expired tickets that haven't been used are cleaned up automatically.

---

### GET `/gm/ws/{id}?ticket=<ticket>`

WebSocket upgrade endpoint. Requires a valid ticket obtained from `POST /gm/ws/ticket/{game_id}`.

- Returns 401 if the ticket is missing, expired, already used, or does not match the game ID in the path.
