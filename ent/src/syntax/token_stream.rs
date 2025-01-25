use std::str::FromStr;

use crate::syntax::lexer::Lexer;
use crate::syntax::lexer_error::LexicalError;
use crate::syntax::token::Token;

#[derive(Debug, PartialEq)]
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

impl FromStr for TokenStream {
    type Err = Vec<LexicalError>;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let lexer = Lexer::new(src);
        lexer.scan()
    }
}

impl From<TokenStream> for Vec<Token> {
    fn from(value: TokenStream) -> Self {
        value.tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::{token::Token, token_type::TokenType};

    #[test]
    fn create_from_str() {
        let src = "a + b";
        let expected = TokenStream {
            tokens: vec![
                Token::new(TokenType::Identifier("a".to_string()), 1),
                Token::new(TokenType::Plus, 1),
                Token::new(TokenType::Identifier("b".to_string()), 1),
            ],
        };
        assert_eq!(expected, TokenStream::from_str(&src).unwrap());
    }

    #[test]
    fn fail_creation_from_str() {
        let src = "\"";
        assert!(TokenStream::from_str(&src).is_err());
    }

    #[test]
    fn from_token_stream_to_token_vec() {
        let expected = vec![
            Token::new(TokenType::Identifier("a".to_string()), 0),
            Token::new(TokenType::Plus, 0),
            Token::new(TokenType::Identifier("b".to_string()), 0),
        ];
        let token_stream = TokenStream {
            tokens: vec![
                Token::new(TokenType::Identifier("a".to_string()), 0),
                Token::new(TokenType::Plus, 0),
                Token::new(TokenType::Identifier("b".to_string()), 0),
            ],
        };

        let vec: Vec<Token> = Vec::from(token_stream);
        assert_eq!(vec, expected);
    }
}
