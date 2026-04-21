# Game Structure

The `Game` is a collection of plugins and configuration describing a specific board game. It serves as a blueprint for the `GameInstance`.

[Read more](./the_game.md)

---

The `GameInstance` is a specific instance of a game, played with some users. It contains a snapshot of the `Game` - to avoid invalid game states, when the blueprint changes during a game - and the current state of the played game.

[Read more](./the_game_instance.md)
