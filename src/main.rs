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

    // read the code to a string
    let code_file = &argv[1];
    let code = std::fs::read_to_string(code_file);
    if code.is_err() {
        return Err(Error::UnableToOpenFile(code_file.clone()));
    }

    // tokenize the code
    let tokens = lexer::tokenize(&code.unwrap())?;

    // parse it
    let ast = parser::parse(&tokens)?;

    // print for now
    println!("tokens = \n{:#?}\n", tokens);
    println!("ast = \n{:#?}\n", ast);

    Ok(())
}
