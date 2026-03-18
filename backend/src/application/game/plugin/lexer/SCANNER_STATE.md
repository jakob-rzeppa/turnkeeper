# Scanner State

The scanner scans the code and turns it into lexemes, to be further processed by the evaluator.

It is based on a finite state machine.

### Overview

```mermaid
stateDiagram-v2

[*] --> None

None --> IdentifierOrKeyword: Alphabetic or _
None --> Integer: Digit
None --> String: Double Quote
None --> SpecialCharacter: all Special characters
```

### IdentifierOrKeyword

```mermaid
stateDiagram-v2

None --> IdentifierOrKeyword: Alphabetic or _
IdentifierOrKeyword --> IdentifierOrKeyword: Alphabetic, Digit, _ or -
IdentifierOrKeyword --> [*]: Other
```

### Integer

```mermaid
stateDiagram-v2

None --> Integer: Digit
Integer --> Integer: Digit
Integer --> Float: Dot
Integer --> [*]: Other

Float --> Float: Digit
Float --> [*]: Other
```

### String

```mermaid
stateDiagram-v2

None --> String: Double Quote
String --> String: Any Character Except Double Quote
String --> [*]: Double Quote
```

## Special Character

```mermaid
stateDiagram-v2

None --> SpecialCharacter: all Special characters
SpecialCharacter --> [*]: other
SpecialCharacter --> DoubleSpecialCharacter: all Special characters
DoubleSpecialCharacter --> [*]
```
