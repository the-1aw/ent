use std::{
    iter::Peekable,
    str::{Chars, FromStr},
};

use crate::syntax::keywords;
use crate::syntax::lexer_error::LexicalError;
use crate::syntax::token_type::{Literal, TokenType};
use crate::syntax::{Token, TokenStream};

pub struct Lexer<'a> {
    src: Peekable<Chars<'a>>,
    token_stream: TokenStream,
    current_line: usize,
    errors: Vec<LexicalError>,
}

type TokenScanResult = Result<Option<Token>, LexicalError>;

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Lexer {
            src: src.chars().peekable(),
            token_stream: TokenStream::new(),
            errors: Vec::new(),
            current_line: 1,
        }
    }

    fn scan_string(&mut self) -> TokenScanResult {
        let is_not_str_end = |c: &char| *c != '"';
        let src = self.src.clone();
        let str_begining_line = self.current_line;
        while let Some(c) = self.src.next_if(is_not_str_end) {
            if c == '\n' {
                self.current_line += 1
            }
        }
        if let None = self.src.next() {
            return Err(LexicalError::new(
                str_begining_line,
                "Unterminated string".to_string(),
            ));
        }
        let str = String::from_iter(src.take_while(is_not_str_end));
        Ok(Some(Token::new(
            TokenType::Literal(Literal::String(str)),
            str_begining_line,
        )))
    }

    fn scan_number(&mut self, digit: char) -> TokenScanResult {
        let is_digit_or_dot = |c: &char| match *c {
            '.' => true,
            '0'..='9' => true,
            _ => false,
        };
        let src = self.src.clone();
        while let Some(_) = self.src.next_if(is_digit_or_dot) {}
        let num_str = format!(
            "{digit}{}",
            String::from_iter(src.take_while(is_digit_or_dot))
        );
        match f64::from_str(&num_str) {
            Ok(num) => Ok(Some(Token::new(
                TokenType::Literal(Literal::Number(num)),
                self.current_line,
            ))),
            Err(_) => Err(LexicalError::new(
                self.current_line,
                "Invalid number".to_string(),
            )),
        }
    }

    fn is_identifier_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn scan_identifier(&mut self, c: char) -> TokenScanResult {
        let src = self.src.clone();
        while let Some(_) = self.src.next_if(|c| Lexer::is_identifier_char(*c)) {}
        let identifier = format!(
            "{c}{}",
            String::from_iter(src.take_while(|c| Lexer::is_identifier_char(*c)))
        );
        match keywords::match_reserved_word(&identifier) {
            Some(symbol) => Ok(Some(Token::new(symbol, self.current_line))),
            None => Ok(Some(Token::new(
                TokenType::Identifier(identifier),
                self.current_line,
            ))),
        }
    }

    fn scan_token(&mut self, c: char) -> TokenScanResult {
        match c {
            '(' => Ok(Some(Token::new(TokenType::LeftParen, self.current_line))),
            ')' => Ok(Some(Token::new(TokenType::RightParen, self.current_line))),
            '{' => Ok(Some(Token::new(TokenType::LeftBrace, self.current_line))),
            '}' => Ok(Some(Token::new(TokenType::RightBrace, self.current_line))),
            ',' => Ok(Some(Token::new(TokenType::Commma, self.current_line))),
            '.' => Ok(Some(Token::new(TokenType::Dot, self.current_line))),
            '-' => Ok(Some(Token::new(TokenType::Minus, self.current_line))),
            '+' => Ok(Some(Token::new(TokenType::Plus, self.current_line))),
            ';' => Ok(Some(Token::new(TokenType::Semicolon, self.current_line))),
            '*' => Ok(Some(Token::new(TokenType::Star, self.current_line))),
            '"' => self.scan_string(),
            '/' => {
                if self.src.next_if_eq(&'/').is_some() {
                    while let Some(_) = self.src.next_if(|c| *c != '\n') {}
                    Ok(None)
                } else {
                    Ok(Some(Token::new(TokenType::Slash, self.current_line)))
                }
            }
            '!' => Ok(Some(Token::new(
                if self.src.next_if_eq(&'=').is_none() {
                    TokenType::Bang
                } else {
                    TokenType::BangEqual
                },
                self.current_line,
            ))),
            '=' => Ok(Some(Token::new(
                if self.src.next_if_eq(&'=').is_none() {
                    TokenType::Equal
                } else {
                    TokenType::EqualEqual
                },
                self.current_line,
            ))),
            '>' => Ok(Some(Token::new(
                if self.src.next_if_eq(&'=').is_none() {
                    TokenType::Greater
                } else {
                    TokenType::GreaterEqual
                },
                self.current_line,
            ))),
            '<' => Ok(Some(Token::new(
                if self.src.next_if_eq(&'=').is_none() {
                    TokenType::Less
                } else {
                    TokenType::LessEqual
                },
                self.current_line,
            ))),
            '\n' => {
                self.current_line += 1;
                Ok(None)
            }
            ' ' | '\t' | '\r' => Ok(None),
            c if c.is_ascii_digit() => self.scan_number(c),
            c if Lexer::is_identifier_char(c) => self.scan_identifier(c),
            c => Err(LexicalError::new(
                self.current_line,
                format!("Unknown token {c}"),
            )),
        }
    }

    /// Transform `source` into a TokenStream
    ///
    /// This consumes the Lexer.
    /// For better error reporting, this will scan the whole source regardless of error.
    /// This gives you the ability to warn the user with all Lexical Error at once
    /// to avoid hide and seek with errors shadowed by others later in the source.
    pub fn scan(mut self) -> Result<TokenStream, Vec<LexicalError>> {
        while let Some(ch) = self.src.next() {
            match self.scan_token(ch) {
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
