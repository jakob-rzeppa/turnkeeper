# Action Plugin

## Example

```rust
// The player stat will exist for all players with a default of 2
pstat gold: int = 2;
// The game stat defendingMultiplier is set with a default of 1.5
stat defendingMultiplier: float = 1.5;
// The global variable battlesAllowed is set to true. The difference between this and stat is, that a global variable does not show in the gm game overview and can't be displayed in the pages. It still can be changed in the code.
let battlesAllowed: boolean = true;
// A const is a immutable variable and should be preferred to let if possible.
const goldForBattleWinner: int = 200;

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

### On Action

```rust
// A action that is triggered by another action can't have parameters
action afterFight after fight {

}

action beforeFight before fight {

}

// A action can also be triggered by a turn advance (pre defined action)
action doSomething on turnStart {}

action doSomething on turnEnd {}

// The same for rounds
action doSomething on roundStart {}

action doSomething on roundEnd {}
```

When multiple actions are triggered simultaneously, their execution order is not guaranteed.

### Game Object

```
// The global game object
game = {
    currentPlayer: Player,
    players: (playerId, Player), // Use with game.players[id]
    // All the games stats
}
```
