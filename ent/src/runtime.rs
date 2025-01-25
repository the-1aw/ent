use crate::syntax::Token;

pub fn run(tokens: Vec<Token>) {
    for token in tokens.iter() {
        println!("{token}");
    }
}
