# API and Realtime

See also:

- [Architecture](ARCHITECTURE.md)
- [Backend Documentation](backend/README.md)
- [How To Use](HOW_TO_USE.md)

## Authentication Model

Two JWT flows exist:

- GM JWT: for GM routes
- User JWT: for user routes

For websockets a extra ticket endpoint is provided.

## API HTTP Endpoints

### Auth (Unauthenticated)

- `POST /user/login` → JWT
- `POST /user/register` → JWT
- `POST /gm/login` → JWT

### Games

- `GET /user/games` → All game metadata (User JWT required)
- `GET /gm/games` → All game metadata (GM JWT required)
- `POST /gm/games` → Create new game (GM JWT required)
- `DELETE /gm/games/{id}` → Delete game (GM JWT required, **repository not yet implemented**)

### WebSocket Tickets

- `POST /gm/ws/ticket/{game_id}` → Get single-use WS ticket URL (GM JWT required)
- `POST /user/ws/ticket/{game_id}` → Get single-use WS ticket URL (User JWT required)

## WebSocket Commands

Commands are sent as JSON-serialized Rust enum variants. After each command, the server broadcasts the full `GmGameInfo` game state to **all** connected clients (GM and users).

## Command Flow

The backend accepts game commands via WebSocket and then broadcasts full projected game state to connected clients.

For sequence diagrams and details, see [Architecture](ARCHITECTURE.md).
