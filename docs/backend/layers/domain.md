# Domain Layer

The domain layer models core game and user rules **without outside dependencies**.

## Modules

- `domain/user`
- `domain/game`
- `domain/gm`

## Game Aggregate

```mermaid
classDiagram
    class Game {
      id: Id
      name: String
      players
      next_turn()
      add_player(id)
      add_tradable(id, name, initial)
    }

    class Player {
      id: Id
      user_id
      stats
      tradables
    }

    class Stat {
      id: Id
      key: StatKey
      value: StatValue
    }

    class Tradable {
      id: Id
      name: String
      value: f64
    }

    Game "1" *-- "0..*" Player
    Player "1" *-- "0..*" Stat
    Player "1" *-- "0..*" Tradable
```

## User Aggregate

```mermaid
classDiagram
    class User {
      id: Id
      name: UserName
      password: UserPassword
      check_password(password)
    }
```

## Invariants and Rules

- Game identity and user identity are UUID-based IDs.
- Domain constructors and value objects enforce validation boundaries.
- Domain methods return domain-specific errors instead of infra errors.
- User management is separate from the game aggregate; game uses user identity references.

## Projections

The clients don't get to see the game aggregate itself. They only get a projection. This projection differs depending on who wants to see the game state. This way we seperate the connection from the actual data.
