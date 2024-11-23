use std::{env, fs, io::{self, Write}, process};

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
    loop {
        print!("> ");
        io::stdout().flush().expect("Error flushing to standard output!");
        loop {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => run(&input),
                Err(e) => {
                    eprintln!("Error reading input: {e}");
                    break;
                }
            }
        }
    }
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