use std::fmt::Display;

use crate::syntax::token_type::TokenType;

#[derive(Debug, PartialEq)]
pub struct Token {
    symbol: TokenType,
    line: usize,
}

impl Token {
    pub fn new(symbol: TokenType, line: usize) -> Self {
        Token { symbol, line }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "symbol: {:?}, line: {}", self.symbol, self.line)
    }
}
