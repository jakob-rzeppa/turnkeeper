# Programming Language Overview

## Data

All data is passed by value (cloned). Comparisons are also always by value (and type) and not by reference.

### Types

- `int` (underlying datatype in rust: `i64`)
- `float` (underlying datatype in rust: `f64`)
- `string` (underlying datatype in rust: `String`)
- `bool` (underlying datatype in rust: `bool`)
- `id` (underlying datatype in rust: `Id` value_object -> `Uuid`)
- Lists `<somethig>[]` (underlying datatype in rust: `Vec<>`)
- Every Projection of the Game, User etc. Entities accessible by the scripts is a unique type
    - The accessible Projections are the same as the ones broadcasted to the gm
    - `game`, `player`, `stat`, `tradable`, `user`

## Expressions

Expressions always evaluate to a single value (list counts as one :/).

### Operations

#### Unary

- `!` - negation
- `(type)` - type conversion
    - only for `int -> float`, `float -> int`

#### Binary

- `<int> + <int>` -> `<int>` overflow not possible
- `<float> + <float>` -> `<float>` overflow not possible
- `<int> - <int>` -> `<int>` overflow not possible
- `<float> - <float>` -> `<float>` overflow not possible
- `<int> * <int>` -> `<int>` overflow not possible
- `<float> * <float>` -> `<float>` overflow not possible
- `<int> / <int>` -> `<int>` (devision by zero is error)
- `<float> / <float>` -> `<float>` (devision by zero is error)
- `<int> % <int>` -> `<int>` (modulo)
- `<int> ^ <int>` -> `<int>` (power)
- `<float> ^ <float>` -> `<float>` (power)

## Functions

```
// Without return type
fn count(text: string) {
    ...
}

// With return type
fn count(text: string) -> int {
    if (something) {
        return 5;
    }

    return 1;
}
```

Call functions

```
let c: int = count("text");
```

### Rejecting

```
fn count(text: string) -> int? {
    if (something) {
        reject "some reason";
    }

    return 1;
}
```

Handle the function

```
// Reason relevant
let c: int = count("text") catch (e: string) {
    if (we want to throw) {
        throw "we want to throw";
    }
    // do something and return or throw
    return 2;
}

// Reason irrelevant
let c: int = count("text") catch {
    if (we want to throw) {
        throw "we want to throw";
    }
    // do something and return or throw
    return 2;
}

// In expression
let c: int = count("text") catch { return 5; } + 5;
```

## Access Game State

Game state can be accessed via a call to

- `useGame() -> game`
- `useCurrentPlayer() -> player`

#### Example

```
let currentPlayer: player = useCurrentPlayer();
let currentPlayerId: id = currentPlayer.id;
```

## Execute Game Commands

```
// This ignores if a error happens (should only be used with commands that are not expected to return a error)
exec SendTradable {
    from_id: string;
    to_id: string;
    tradable_id: string;
    amount: 50
};
```

```
// With error handling
exec SendTradable {
    from_id: string;
    to_id: string;
    tradable_id: string;
    amount: 50
} catch (e: string) {
    ...
};

// Message irrelevant
exec SendTradable {
    from_id: string;
    to_id: string;
    tradable_id: string;
    amount: 50
} catch {
    ...
};
```

```
// Without command body
exec NextTurn catch {
    ...
};
```

## Stop execution

To stop the execution of the script run `exit` (successful execution) or `throw <string>`.

## Error handling

## Control flow

### If / else

```
if (bool) {
    ...
} else if (bool) {
    ...
} else {
    ...
}
```

### Loops

```
// While loop
while (bool) {
    ...
}
```
