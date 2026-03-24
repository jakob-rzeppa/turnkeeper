# Game Session

```mermaid
classDiagram
direction BT
    class Game {

    }

    class Plugin {

    }

    class GameRuntime {
        + handle_command(GameCommand)
        + get_gm_game_projection()
        + get_user_game_projection(&Id)
    }
    GameRuntime "1" *-- "1" Game : runs
    GameRuntime "1" *-- "0..*" Plugin : uses

    class GameSession {

    }
    GameSession "1" *-- "1" GameRuntime : contains
```

## Action plugin execution

```mermaid
sequenceDiagram
    participant GameSession
    participant GameRuntime

    GameSession ->>+ GameRuntime : Execute Action Plugin by id
    note over GameRuntime : Get the Plugin by id
    note over GameRuntime : Clone the Game State
    create participant PluginRuntime
    GameRuntime ->> PluginRuntime : Create with Plugin and cloned Game
    note over PluginRuntime : Parse code
    note over PluginRuntime : Execute code
    destroy PluginRuntime
    PluginRuntime -->> GameRuntime : Return updated Game if successfull
    note over GameRuntime : Overwrite the current game with the updated
    GameRuntime -->>- GameSession :
```
