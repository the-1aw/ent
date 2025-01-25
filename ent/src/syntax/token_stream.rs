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
