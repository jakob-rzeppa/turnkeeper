# The Game Instance

The `GameInstance` is a specific instance of a game, played with some users. It contains a snapshot of the `Game` - to avoid invalid game states, when the blueprint changes during a game - and the current state of the played game.

## Structure

```mermaid
classDiagram
    class GameInstance {
        game : Game
        name : String
    }

    class Player {
        id : String
        user_id : String?
    }
    GameInstance "1" *-- "0..*" Player : played_by

    class Stat {
        name : String
        value : StatValue
    }
    Player *-- "0..*" Stat : has

    class StatValue <<enumeration>> {
        INT(i64)
        FLOAT(f64)
        BOOLEAN(bool)
        STRING(String)
    }
```
