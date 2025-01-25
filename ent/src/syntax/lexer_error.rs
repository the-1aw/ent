use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct LexicalError {
    line: usize,
    msg: String,
}

impl Display for LexicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[error: {}] {}", self.line, self.msg)
    }
}

impl LexicalError {
    pub fn new(line: usize, msg: String) -> Self {
        LexicalError { line, msg }
    }
}

impl Error for LexicalError {}
