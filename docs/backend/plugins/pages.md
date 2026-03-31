# Pages

See also:

- [Plugin Docs Index](README.md)
- [Plugin Overview](OVERVIEW.md)
- [Grammar](GRAMMAR.md)
- [Game Session + Runtime](application/GAME_SESSION.md)

Plugins should be able to create custom pages, where input can be set via forms and some code is executed on submit.

## Example

```
page

# The decl code block is used to define variables before the code execution.
#
# Only in the decl block the program is able to declare forms, since they need to be processed before the "normal" code execution.
decl {
    title "Some title";

    form incrementForm {
        title: "Increment Form",
        submit: "Increment",
        fields: {
            incrementBy: int = 0, # with default value
        }
    };

    # You can get values from the previous execution via the display.
    display increment "Increment";
}

# Use form
if (!incrementForm.isSubmitted) {
    throw "from is not submitted";
}

let incrementBy: int = someForm.fields.incrementBy;

# Use display value
let lastValue: int = increment.value;
let newValue = lastValue + incrementBy;

# Set display
increment.value = newValue;
```

## Related Project Docs

- [Main Documentation Index](../README.md)
