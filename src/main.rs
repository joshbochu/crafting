use std::{env, fs, process};

const EXIT_CODE_USAGE: i32 = 64;  // Command line usage error
const EXIT_CODE_IOERR: i32 = 74;  // IO error

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            println!("Usage: rlox [script]");
            process::exit(EXIT_CODE_USAGE)
        }
    }
}

fn run_prompt() {
     
}

fn run_file(path: &str) {
    match fs::read_to_string(path) {
        Ok(contents) => run(&contents),
        Err(error) => {
            eprintln!("Error reading file: {}", error);
            process::exit(EXIT_CODE_IOERR);
        }
    }
}

fn run(source: &str) {
    println!("Source code to interpret: {}", source);
}