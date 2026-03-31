# Command Processing and Broadcast

Commands enter through active WebSocket connections and are processed by `GameSession` and `GameRuntime`.

## Command Pipeline

```mermaid
sequenceDiagram
    participant Client
    participant Session as GameSession
    participant Runtime as GameRuntime
    participant Repo as GameRepository
    participant GM as GM Connection
    participant Users as User Connections

    Client->>Session: GameCommand
    Session->>Runtime: handle_command(command)

    alt command accepted
      Session->>Repo: log_command(game_id, command)
      Repo-->>Session: ok
    else command rejected
      Session-->>GM: GameError projection
    end

    note over Session: broadcast_game_state
    Session-->>GM: GmGameState
    Session-->>Users: UserGameInfo for each user
```

## Command Source and Permission Notes

- GM connections can submit all game commands.
- User connections are checked by `command.is_user_permitted(user_id)` before execution.

## Runtime Dispatch Model

```mermaid
flowchart TB
    cmd[GameCommand enum variant] --> dispatch[GameRuntime::handle_command]
    dispatch --> game[Game aggregate methods]
    game --> ok{success?}
    ok -- Yes --> persist[log command in games_log]
    ok -- No --> gmerr[send GameError to GM]
    persist --> broadcast[broadcast latest projections]
    gmerr --> broadcast
```
