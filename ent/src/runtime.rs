use crate::syntax::token::Token;

pub fn run(tokens: Vec<Token>) {
    for token in tokens.iter() {
        println!("{token}");
    }
}
