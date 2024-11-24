use std::io::{self, Write};
use std::fs;
use rlox::lox::Lox;

/// Exit codes following Unix sysexits.h convention:
/// - 64 (EX_USAGE): Command line usage error
/// - 65 (EX_DATAERR): Data format error
/// - 66 (EX_NOINPUT): Cannot open input
/// 
/// These codes are in the range 64-78 reserved for application-specific errors
/// as defined in `sysexits.h`
const EXIT_CODE_USAGE: i32 = 64;    // EX_USAGE
const EXIT_CODE_DATA: i32 = 65;     // EX_DATAERR
const EXIT_CODE_IO: i32 = 66;       // EX_NOINPUT

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            println!("Usage: rlox [script]");
            std::process::exit(EXIT_CODE_USAGE);
        }
    }
}

fn run_file(path: &str) {
    let mut lox = Lox::new();
    match fs::read_to_string(path) {
        Ok(contents) => {
            lox.run(&contents);
            if lox.had_error() {
                std::process::exit(EXIT_CODE_DATA);
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(EXIT_CODE_IO);
        }
    }
}

fn run_prompt() {
    let mut lox = Lox::new();
    loop {
        print!("> ");
        io::stdout().flush().expect("Error flushing to standard output!");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // Ctrl+D (EOF) - a graceful exit vs. Ctrl+C (SIGINT)
            Ok(_) => {
                lox.run(&input);
                lox.reset_error(); // Reset error flag for next line
            },
            Err(e) => {
                eprintln!("Error reading input: {e}");
                break;
            }
        }
    }
}