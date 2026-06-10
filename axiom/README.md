# AXIOM (Autonomous eXecution for Intelligent Operations & Machine learning)

AXIOM is a new-generation programming language designed **exclusively for AI agents and coding agents**. 

It prioritizes semantic density over token minimization, providing a strict, unambiguous syntactic structure that eliminates AI hallucination and reasoning errors commonly caused by flexible whitespace parsing (like in Python) or complex implicit casting.

## Features

- **Strict Parsing:** No whitespace sensitivity. Blocks are delimited by `{}`.
- **No Implicit Casting:** Strongly typed. No coercion between types to prevent subtle bugs.
- **First-Class Functions:** Built for modern functional and expression-oriented patterns.
- **Tree-Walk Interpreter:** Instantly execute AXIOM scripts directly from the syntax tree!
- **LLVM-ready Architecture:** Built with an optimization pipeline ready to translate validated ASTs into high-performance LLVM IR.

## Project Structure

This repository is organized as a Rust Workspace containing the full language toolchain:
- **`axiom-compiler/`**: The core parsing pipeline (Lexer, Parser, TypeChecker, Optimizer, CodeGen, Interpreter).
- **`axiom-stdlib/`**: The standard library modules (`math`, `string`, `array`) written natively in AXIOM.
- **`axiom-cli/`**: The command-line interface tool (`axiom run`, `axiom compile`).
- **`axiom-pm/`**: Package manager framework.
- **`axiom-lsp/`**: Language Server Protocol backend for IDE support.

## Getting Started

### Prerequisites
You need [Rust and Cargo](https://rustup.rs/) installed on your machine to build the toolchain locally.

### Running Code

To execute an AXIOM script using the built-in Tree-Walk Interpreter, run the CLI tool on any `.axiom` file:

```bash
cargo run --bin axiom-cli -- run docs/examples/hello_world.axiom
```

### Running Tests

To verify the compiler pipeline (Lexer, Parser, and Type Checker), run the workspace test suite:

```bash
cargo test --workspace
```
