# Abstract Syntax Tree Structure

The AST is build upon many structs. These represent each node.

If there are multiple possible child nodes, enums are used. These have no other purpose than differentiating between different nodes. They shall not have any extra attributes (only the struct at position 0).

---

- Each node must contain information about the position of the node in the source code.
- The attributes of a node should only accessible via immutable refs - the AST should be created once and not modified.
- For values used directly by the runtime use the types from plugin::common
- A node part of a enum must end with the name of the enum
