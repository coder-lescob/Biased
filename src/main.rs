mod lexer;

use std::env;

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() != 2 {
        eprintln!("\x1b[38;5;196musage\x1b[0m: {} <code file>", &argv[0]);
        return;
    }

    let code_file = &argv[1];
    let code = std::fs::read_to_string(code_file).expect("unable to open input file");
    let tokens = lexer::tokenize(&code).expect("\x1b[38;5;196mLexical error\x1b[0m");

    println!("{:#?}", tokens);
}
