use crate::lexer::{Lexer, Token, Type};
use crate::parser::Expression::Pipe;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Expression {
    Attribute { expression: Box<Expression>, name: String },
    Pipe { left: Box<Expression>, right: Box<Expression> },
    This,
}


#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn parse(command: String) -> Expression {
        let lexer = Lexer::on(command);
        let parser = Parser { lexer };
        parser.parse_expression()
    }

    fn parse_expression(mut self) -> Expression {
        self.parse_attribute()
    }

    fn parse_attribute(&mut self) -> Expression {
        let mut current = Expression::This;
        let mut previous_type = Type::Dot;

        while self.lexer.has_remaining() {
            match self.lexer.next_token() {
                None => {}
                Some(Token{token_type: Type::Dot, text: _}) => {
                    previous_type = Type::Dot;
                }
                Some(Token { token_type: Type::Identifier, text: name }) => {
                    assert_eq!(previous_type, Type::Dot);
                    previous_type = Type::Identifier;
                    let expression = Box::from(current);
                    current = Expression::Attribute { expression, name }
                }
                Some(Token { token_type: Type::Pipe, text: _ }) => {
                    current = Pipe { left: Box::new(current), right: Box::new(self.parse_attribute()) }
                }
            }
        }
        return current;
    }
}


#[cfg(test)]
mod tests {
    use crate::parser::{Expression, Parser};

    #[test]
    fn parses_object() {
        assert_parsed(".", &Expression::This)
    }

    #[test]
    fn parses_attribute() {
        let expected = Expression::Attribute { expression: Box::from(Expression::This), name: "bar".to_string() };
        assert_parsed(".bar", &expected)
    }

    #[test]
    fn parse_two_attributes() {
        let bar = Expression::Attribute { expression: Box::from(Expression::This), name: "bar".to_string() };
        let fiz = Expression::Attribute { expression: Box::from(bar), name: "fiz".to_string() };

        assert_parsed(".bar.fiz", &fiz)
    }
    #[ignore]
    #[test]
    fn just_identifier_is_syntax_error() {
        assert_error("foo")
    }

    fn assert_error(expression: &str) {
        Parser::parse(String::from(expression));
    }

    #[test]
    fn parse_pipe() {
        // TODO:
        let expected = Expression::Pipe {
            left: Box::from(Expression::This),
            right: Box::from(Expression::This),
        };
        assert_parsed(". | .", &expected);
        assert_parsed(".|.", &expected);
    }

    fn assert_parsed(command: &str, expected: &Expression) {
        let actual = Parser::parse(command.to_string());
        println!("{:?}", actual);
        assert_eq!(actual, *expected);
    }
}