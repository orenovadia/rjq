#[derive(Debug)]
#[derive(PartialEq)]
pub enum Expression {
    Null,
    This,
}

pub fn parse(command: String) -> Expression {
    Expression::Null
}

#[cfg(test)]
mod tests {
    use crate::parser::{Expression, parse};

    #[test]
    fn parses_null() {
        assert_parsed("null", Expression::Null)
    }

    #[test]
    fn parses_object() {
        assert_parsed(".", Expression::This)
    }

    fn assert_parsed(command: &str, expected: Expression) {
        let actual = parse(command.to_string());
        assert_eq!(expected, actual);
    }
}