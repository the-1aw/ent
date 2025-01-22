mod runtime;
mod syntax;

use std::{
    fs,
    io::{self, Write},
    process,
    str::FromStr,
};

use syntax::TokenStream;

pub fn run_file(path: &str) {
    let src = fs::read_to_string(path).unwrap();
    match TokenStream::from_str(&src) {
        Ok(tokens) => runtime::run(tokens.into()),
        Err(lex_errors) => {
            for err in lex_errors.iter() {
                eprintln!("{}", err);
            }
            process::exit(65);
        }
    }
}

pub fn run_repl() {
    let mut buf = String::new();

    loop {
        print!("> ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut buf).expect("IO Failure");
        if buf.len() > 0 {
            match TokenStream::from_str(&buf) {
                Ok(tokens) => runtime::run(tokens.into()),
                Err(lex_errors) => {
                    for err in lex_errors.iter() {
                        eprintln!("{}", err);
                    }
                }
            }
        }
        buf.clear();
    }
}

pub fn usage() {
    println!("Usage: ent [script]");
    process::exit(64);
}
