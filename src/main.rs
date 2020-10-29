extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::{Pair, Pairs};
use pest::Parser;
use std::process::exit;

#[derive(Parser)]
#[grammar = "jq.pest"]
struct JqParser;

enum Expression {
    Object,
    Attribute(String),
}

fn main() {
    let mut pairs: Pairs<Rule> = JqParser::parse(Rule::identifier, "foo").unwrap_or_else(|e| panic!("{}", e));
    let expression = pairs.next().unwrap();
    println!("{:?}", expression);
    exit(0);

//    parse_expression(expression);
}

fn parse_expression(expression: Pair<Rule>) {
    match expression.as_rule() {
        Rule::expression => print!("{:?}", expression),
        _ => unreachable!()
    };
}
