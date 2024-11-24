use crate::scanner::Scanner;

pub struct Lox {
    had_error: bool
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false }
    }
    
    pub fn run(&mut self, source: &str) {
        let scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        for token in tokens {
            println!("{:?}", token);
        }
    }
    
    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message)
    }
    
    fn report(&mut self, line: usize, where_: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, where_, message);
        self.had_error = true
    }
    
    pub fn reset_error(&mut self) {
        self.had_error = false;
    }
    
    pub fn had_error(&self) -> bool {
        self.had_error
    }
}