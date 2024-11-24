use crate::token::Token;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {
            source: source.to_string(),
            tokens: Vec::new() // On init
        }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        Vec::new() // TODO
    }
}