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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_token() {
        let symbol = TokenType::Number(1234.0);
        let same_symbol = TokenType::Number(1234.0);
        let line = 2;
        assert!(Token { symbol, line } == Token::new(same_symbol, line))
    }

    #[test]
    fn fmt_token() {
        let expected = "symbol: Number(1234.0), line: 2";
        let actual = format!("{}", Token::new(TokenType::Number(1234.0), 2));
        assert_eq!(expected, actual)
    }
}
