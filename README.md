# rlox - A Rust Implementation of Lox

This is a Rust implementation of the Lox programming language. The project will implement a complete interpreter for the Lox language, including lexing, parsing, and interpretation.

## Overview

Lox is a simple, dynamically-typed programming language designed for learning about language implementation. This project implements the following features:

- Lexical analysis (tokenization)
- Parsing
- Abstract Syntax Tree (AST) generation
- Interpretation
- Error handling and reporting

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## Building

To build the project:

```bash
cargo build
```

For release build:

```bash
cargo build --release
```

## Testing

The project includes comprehensive tests for each component. To run the tests:

```bash
# Run all tests
cargo test

# Run specific test modules
cargo test scanner::basic
cargo test scanner::literals
cargo test scanner::keywords
cargo test scanner::errors
cargo test scanner::comments
cargo test scanner::edge_cases

# Run tests with output
cargo test -- --nocapture

# Run tests with detailed output
cargo test -- --nocapture --show-output
```

### Test Coverage

To view test coverage, first install cargo-tarpaulin:

```bash
cargo install cargo-tarpaulin
```

Then run:

```bash
cargo tarpaulin
```

## Usage

To run a Lox source file:

```bash
cargo run -- path/to/script.lox
```

## Language Features

The implementation will support the following Lox features:

- Variables and assignment
- Basic arithmetic operations
- Comparison operators
- Logical operators
- Control flow (if/else, while, for)
- Functions
- Classes and instances
- Closures
- Standard library functions

## Example Code

Here's a simple example of Lox code that this interpreter can(should**) run:

```lox
fun fibonacci(n) {
    if (n <= 1) return n;
    return fibonacci(n - 1) + fibonacci(n - 2);
}

for (var i = 0; i < 10; i = i + 1) {
    print fibonacci(i);
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Crafting Interpreters](http://craftinginterpreters.com/) by Robert Nystrom
- The Rust programming language and its ecosystem 
