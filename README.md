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

# Stuff

What does borrowing mean? Just a temporary use from the owner

Exactly! Think of borrowing like borrowing a book from a library:
fn example_borrowing() {
    let library = String::from("Book");  // Library owns the book
    
    // IMMUTABLE BORROW - Like checking out a book to read
    {
        let reader1 = &library;  // Can read the book
        let reader2 = &library;  // Multiple people can read at once!
        // reader1.push_str("!"); // ❌ Can't modify - just like a library book
    } // Borrowing ends here - book is "returned"
    
    // MUTABLE BORROW - Like checking out a book to write in
    {
        let writer = &mut library;  // Can modify the book
        writer.push_str("!");
        // let reader = &library;   // ❌ Nobody else can read while someone's writing
        // let writer2 = &mut library; // ❌ Can't have two writers
    } // Mutable borrow ends - book is "returned"
    
    // Original owner can use it again
    println!("{}", library);
}
Key borrowing rules:
You can have EITHER:
Many immutable borrows (many readers)
OR one mutable borrow (one writer)
BUT never both at same time
Owner can't use value while it's borrowed
Borrowed value must be returned before owner is dropped
Just like a real library:
Many people can read a book at once
Only one person can write in it at a time
Book must be returned before library closes!