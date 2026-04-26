# Action Plugin

## Example

```rust
// The player stat will exist for all players with a default of 2
pstat gold: int = 2;
// The game stat defendingMultiplier is set with a default of 1.5
stat defendingMultiplier: float = 1.5;

// A action fight is defined.
action fight(attackingPowerLevel: int, defendingPowerLevel: int, defendingPlayer: Player) {
    // Game stats can be used like normal variables in the code, but they are immutable by conventional means. Use game.<stat> to access it.
    if (attackingPowerLevel > defendingPowerLevel * game.defendingMultiplier) {
        // You mutate player stats with pset <playerId> <stat> = <value>;

        // There are multiple ways to access players.
        // You can access a player via game.players[id].<stat>

        // You can use the global game.currentPlayer
        pset currentPlayer.id gold += 200;

        // You can add a : Player to the action parameters -> a dropdown in the frontend
        pset defendingPlayer.id gold -= 200;
    } else {
        // To mutate a game stat use the set keyword.
        set defendingMultiplier = 1.6; // For some reason we change the multiplier here?
    }
}
```

All Actions, Stats etc. must be defined at the root level of the code.

## On Action

```rust
// A action that is triggered by another action can't have parameters
action afterFight after fight {

}

action beforeFight before fight {

}

// A action can also be triggered by a turn advance (pre defined action)
action doSomething on TurnStart {}

action doSomething on TurnEnd {}

// The same for rounds
action doSomething on RoundStart {}

action doSomething on RoundEnd {}
```

When multiple actions are triggered simultaneously, their execution order is not guaranteed.

## Visibility

- hidden: only from the code
- private: only the gm
- public: anyone

```

```

Actions that are triggered by another action can't have visibility modifiers.

## Game Object

```
// The global game object
game = {
    currentPlayer: Player,
    players: (playerId, Player), // Use with game.players[id]
    // All the games stats
}
```
