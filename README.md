# RLox

A Rust implementation of the Lox programming language from [Crafting Interpreters](http://craftinginterpreters.com/).

## Project Structure
```
rlox
├── src
│   ├── error.rs     # Error handling
│   ├── lib.rs       # Core interpreter logic
│   ├── main.rs      # CLI interface
│   ├── scanner.rs   # Lexical analysis
│   ├── token.rs     # Token definitions
│   └── tests        # Test suite
```

## Usage
```bash
cargo run [path/to/file.lox]  # Run a file or start REPL
```

## Development
```bash
cargo build   # Build project
cargo test    # Run tests
cargo fmt     # Format code
cargo clippy  # Run linter
```

## License
MIT