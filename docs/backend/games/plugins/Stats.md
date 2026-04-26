# State Plugin

## Visibility

- hidden: only seen in the code
- private: only seen by the gm
- protected\*: only the own value can be seen by each player
- public: seen by anyone

\*only for player stats

## Example

```rust
// The protected player stat will exist for all players with a default of 2
protected pstat gold: int = 2;
// The public game stat defendingMultiplier is set with a default of 1.5
public stat defendingMultiplier: float = 1.5;
```
