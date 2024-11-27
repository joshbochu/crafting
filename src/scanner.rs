use crate::lox::Lox;
use crate::token::{Literal, Token, TokenType};

pub struct Scanner<'a> {
    lox: &'a mut Lox,
    source: String,
    chars: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &str, lox: &'a mut Lox) -> Self {
        Scanner {
            lox,
            source: String::from(source),
            chars: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            None,
            self.line,
        ));

        self.tokens.to_vec()
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.is_match('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.is_match('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.is_match('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.is_match('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            '/' => {
                if self.is_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => {} // Ignore whitespace
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => self.lox.error(self.line, "Unexpected character."),
        }
    }

    pub fn string(&mut self) {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1
            }
            let _ = self.advance();
        }

        if self.is_at_end() {
            self.lox.error(self.line, "Unterminated string");
            return;
        }

        self.advance();

        self.add_token_with_literal(
            TokenType::String,
            Some(Literal::Str(
                self.source[self.start..self.current].to_string(),
            )),
        );
    }

    pub fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        } else {
            return self.chars[self.current];
        }
    }

    pub fn advance(&mut self) -> char {
        let c = self.chars[self.current];
        self.current += 1;
        return c;
    }

    pub fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    pub fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    pub fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.chars[self.current] != expected {
            return false;
        }
        self.current += 1;
        return true;
    }
}
