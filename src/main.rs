use std::process::exit;

use crate::parser::Parser;

mod parser;
mod lexer;
mod runner;

fn main() {
    let expr = Parser::parse(".foo.bar".to_string());
    println!("{:#?}", expr);

    exit(0);
}

