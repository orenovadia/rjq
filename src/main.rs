use std::{env, io};
use std::io::Read;

use serde_json::Value;

use crate::parser::{Expression, Parser};
use crate::runner::transform;

mod parser;
mod lexer;
mod runner;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("missing filter argument")
    }
    let expr = parse_filter(args);
    println!("expression: {:#?}", &expr);

    let json_input = read_stdin();

    let transformed = transform(json_input, expr);
    println!("result: {}", transformed);
}

fn parse_filter(args: Vec<String>) -> Expression {
    let filter = args.last().unwrap().clone();
    let expr = Parser::parse(filter);
    expr
}

fn read_stdin() -> Value {
    let mut stdin = io::stdin();
    let mut s = String::new();
    stdin.read_to_string(&mut s).expect("failed reading");
    serde_json::from_str(&s).expect("not a json")
}

