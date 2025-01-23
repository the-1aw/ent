use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Token {
    symbol: TokenType,
    lexem: String,
    line: usize,
}

impl Token {
    pub fn new(symbol: TokenType, lexem: &str, line: usize) -> Self {
        Token {
            symbol,
            lexem: lexem.to_string(),
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "symbol: {:?}, lexem: {}, line: {}",
            self.symbol, self.lexem, self.line
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Commma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two cahracter tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}
