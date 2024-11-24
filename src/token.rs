// Single-character tokens:
//   (    )    {    }    ,    .    -    +    ;    /    *
//
// One or two character tokens:
//   !    !=   =    ==   >    >=   <    <=
//
// Literals:
//   identifier    string    number
//
// Keywords:
//   and     class    else     false    fun    for     if 
//   nil     or       print    return   super   this    true 
//   var     while
//
// Special:
//   eof

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals
    Identifier, String, Number,

    // Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof
}

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    Str(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}