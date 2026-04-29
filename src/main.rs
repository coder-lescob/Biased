mod lexer;
mod parser;
mod error;

use std::env;
use crate::error::Error;

static COLOR_RED: &str = "\x1b[38;5;196m";
static DEFAULT_COLOR: &str = "\x1b[0m";

fn main() -> Result<(), Error> {
    let argv: Vec<String> = env::args().collect();

    if argv.len() != 2 {
        eprintln!("{COLOR_RED}usage{DEFAULT_COLOR}: {} <code file>", &argv[0]);
        return Ok(());
    }

    let code_file = &argv[1];
    let code = std::fs::read_to_string(code_file).expect("unable to open input file");
    let tokens = lexer::tokenize(&code)?;

    println!("tokens = \n{:#?}\n", tokens);
    println!("ast = \n{:#?}\n", parser::parse(tokens)?);

    Ok(())
}
