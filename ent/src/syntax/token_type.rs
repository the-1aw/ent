use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    False,
    True,
    Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(str) => write!(f, "{str}"),
            Self::Number(num) => write!(f, "{num}"),
            Self::False => write!(f, "false"),
            Self::True => write!(f, "true"),
            Self::Nil => write!(f, "nil"),
        }
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
    Literal(Literal),
    Identifier(String),

    // Keywords.
    And,
    Class,
    Else,
    Fun,
    For,
    If,
    Or,
    Print,
    Return,
    Super,
    This,
    Var,
    While,

    EOF,
}
