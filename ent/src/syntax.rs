use std::{
    error::Error,
    fmt::Display,
    iter::Peekable,
    str::{Chars, FromStr},
};

use token::{Token, TokenType};

pub mod token;

#[derive(Debug)]
pub struct LexError {
    line: usize,
    msg: String,
}

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[error: {}] {}", self.line, self.msg)
    }
}

impl LexError {
    pub fn new(line: usize, msg: String) -> Self {
        LexError { line, msg }
    }
}

impl Error for LexError {}

#[derive(Debug)]
pub struct TokenStream {
    tokens: Vec<Token>,
}

impl TokenStream {
    pub fn new() -> Self {
        TokenStream { tokens: Vec::new() }
    }

    pub fn append(&mut self, token: Token) {
        self.tokens.push(token);
    }
}

struct Scanner<'a> {
    src: Peekable<Chars<'a>>,
    token_stream: TokenStream,
    errors: Vec<LexError>,
}

impl<'a> Scanner<'a> {
    pub fn new(src: &'a str) -> Self {
        Scanner {
            src: src.chars().peekable(),
            token_stream: TokenStream::new(),
            errors: Vec::new(),
        }
    }

    fn match_token(&mut self, ch: char) -> Result<Option<Token>, LexError> {
        match ch {
            '(' => Ok(Some(Token::new(TokenType::LeftParen, "", 0))),
            ')' => Ok(Some(Token::new(TokenType::RightParen, "", 0))),
            '{' => Ok(Some(Token::new(TokenType::LeftBrace, "", 0))),
            '}' => Ok(Some(Token::new(TokenType::RightBrace, "", 0))),
            ',' => Ok(Some(Token::new(TokenType::Commma, "", 0))),
            '.' => Ok(Some(Token::new(TokenType::Dot, "", 0))),
            '-' => Ok(Some(Token::new(TokenType::Minus, "", 0))),
            '+' => Ok(Some(Token::new(TokenType::Plus, "", 0))),
            ';' => Ok(Some(Token::new(TokenType::Semicolon, "", 0))),
            '*' => Ok(Some(Token::new(TokenType::Star, "", 0))),
            '/' => {
                if self.src.next_if_eq(&'/').is_some() {
                    while let Some(_) = self.src.next_if(|c| *c != '\n') {}
                    Ok(None)
                } else {
                    Ok(Some(Token::new(TokenType::Slash, "", 0)))
                }
            }
            '!' => Ok(Some(Token::new(
                if self.src.next_if_eq(&'=').is_none() {
                    TokenType::Bang
                } else {
                    TokenType::BangEqual
                },
                "",
                0,
            ))),
            '=' => Ok(Some(Token::new(
                if self.src.next_if_eq(&'=').is_none() {
                    TokenType::Equal
                } else {
                    TokenType::EqualEqual
                },
                "",
                0,
            ))),
            '>' => Ok(Some(Token::new(
                if self.src.next_if_eq(&'=').is_none() {
                    TokenType::Greater
                } else {
                    TokenType::GreaterEqual
                },
                "",
                0,
            ))),
            '<' => Ok(Some(Token::new(
                if self.src.next_if_eq(&'=').is_none() {
                    TokenType::Less
                } else {
                    TokenType::LessEqual
                },
                "",
                0,
            ))),
            ' ' | '\r' | '\t' | '\n' => Ok(None),
            c => Err(LexError::new(0, format!("Unknown token {c}"))),
        }
    }

    /// Transform `source` into a TokenStream
    ///
    /// This consumes the Scanner.
    /// For better error reporting, this will scan the whole source regardless of error.
    /// This gives you the ability to warn the user with all Lexical Error at once
    /// to avoid hide and seek with errors shadowed by others later in the source.
    pub fn scan(mut self) -> Result<TokenStream, Vec<LexError>> {
        while let Some(ch) = self.src.next() {
            match self.match_token(ch) {
                Ok(token) => {
                    if let Some(token) = token {
                        self.token_stream.append(token);
                    }
                }
                Err(err) => self.errors.push(err),
            }
        }
        if self.errors.len() > 0 {
            Err(self.errors)
        } else {
            Ok(self.token_stream)
        }
    }
}

impl FromStr for TokenStream {
    type Err = Vec<LexError>;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let scanner = Scanner::new(src);
        scanner.scan()
    }
}

impl From<TokenStream> for Vec<Token> {
    fn from(value: TokenStream) -> Self {
        value.tokens
    }
}
