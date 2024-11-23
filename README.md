# RLox

A Rust implementation of the Lox programming language from [Crafting Interpreters](http://craftinginterpreters.com/).

## Project Structure
```
rlox
├── src
│   ├── error.rs     # Error handling
│   ├── lib.rs       # Core interpreter logic
│   └── main.rs      # CLI interface
```

## Usage

```bash
# Run a Lox file
cargo run path/to/file.lox

# Start REPL
cargo run
```

## Development

### Prerequisites
- Rust (stable)
- Cargo

### Commands
```bash
# Build
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Run linter
cargo clippy
```

## License
MIT