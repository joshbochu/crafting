use crate::token::Token;

pub struct Scanner {
    source: String
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {
            source: source.to_string()
        }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        Vec::new()
    }
}