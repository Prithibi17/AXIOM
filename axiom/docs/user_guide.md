# AXIOM User Guide

Welcome to AXIOM, the programming language designed explicitly for AI-driven code generation and execution!

## Core Concepts

AXIOM's syntax is strictly defined to prevent parsing ambiguities, making it exceptionally efficient for AI agents.

### Variables
Use `let` for immutable variables and `mut` for mutable ones.
```axiom
let name = "AI Agent"
mut counter = 0
```

### Functions
Functions are first-class citizens.
```axiom
fn add(a: Num, b: Num) -> Num {
    a + b
}
```

### Control Flow
AXIOM does not rely on whitespace indentation. Blocks are strictly delimited by `{ }`.
```axiom
if x > 0 {
    print("Positive")
} else {
    print("Negative")
}
```
