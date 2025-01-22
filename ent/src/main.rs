use std::env;

fn main() {
    match env::args().len() {
        2 => {
            let path = env::args().nth(1).unwrap();
            ent::run_file(&path);
        }
        1 => ent::run_repl(),
        _ => ent::usage(),
    }
}
