use crate::lexer::{Lexer, Token, Type};

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

    fn parse_attribute(mut self) -> Expression {
        let mut current = Expression::This;

        while self.lexer.has_remaining() {
            self.require(Type::Dot);

            match self.maybe(Type::Identifier) {
                Some(Token { token_type: Type::Identifier, text: name }) => {
                    let expression = Box::from(current);
                    current = Expression::Attribute { expression, name }
                }
                None => break,
                unexpected => panic!("unexpected '{:?}' token", unexpected)
            };
        }
        return current;
    }

    fn require(&mut self, expected_type: Type) -> Token {
        let token = self.lexer.next_token().expect("expected token");
        assert_eq!(token.token_type, expected_type);
        token
    }

    fn maybe(&mut self, expected_type: Type) -> Option<Token> {
        let token = self.lexer.next_token()?;
        assert_eq!(token.token_type, expected_type);
        Some(token)
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
        assert_eq!(actual, *expected);
    }
}