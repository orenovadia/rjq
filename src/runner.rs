use serde_json::Value;

use crate::parser::Expression;

pub fn transform(value: Value, expr: impl Apply) -> Value { expr.transform(value) }

pub trait Apply {
    fn transform(&self, value: Value) -> Value;
}

impl Apply for Expression {
    fn transform(&self, value: Value) -> Value {
        match self {
            Expression::Attribute { expression, name } => {
                let inner = expression.transform(value);
                match inner {
                    Value::Object(map) => {
                        // TODO: can we do without cloning the value we get out of the map?
                        map.get(name).cloned().unwrap_or(Value::Null)
                    }
                    _ => Value::Null
                }
            }
            Expression::This => { value }
        }
    }
}


mod tests {
    use serde_json::{json, Value};

    use crate::parser::{Expression, Parser};
    use crate::runner::{Apply, transform};

    #[test]
    fn test_this() {
        let value: Value = json!({"a":4});
        assert_transformed_to(value.clone(), Expression::This, value);
    }

    #[test]
    fn test_attribute() {
        let value: Value = json!({"a":4});
        let this = Box::from(Expression::This);
        let this_dot_a = Expression::Attribute { expression: this, name: "a".to_string() };
        assert_transformed_to(value.clone(), this_dot_a, json!(4));
    }

    #[test]
    fn test_attribute_missing_returns_null() {
        let value: Value = json!( {"a":4});
        let this = Box::from(Expression::This);
        let this_dot_a = Expression::Attribute { expression: this, name: "b".to_string() };
        assert_transformed_to(value.clone(), this_dot_a, Value::Null);
    }

    #[test]
    fn test_nested_attribute() {
        let value: Value = json!({"a": {"b":"foo"} });
        let expression = Parser::parse(".a.b".to_string());
        assert_transformed_to(value.clone(), expression, json!("foo"));
    }

    #[test]
    fn test_nested_attribute_missing() {
        let value: Value = json!({"a": {"b":"foo"} });
        let expression = Parser::parse(".a.c".to_string());
        assert_transformed_to(value.clone(), expression, Value::Null);
    }

    #[test]
    fn test_attribute_of_non_object_resolves_to_null() {
        // perhaps better yelling in this case?
        let value: Value = json!({"a": 5 });
        let expression = Parser::parse(".a.c".to_string());
        assert_transformed_to(value.clone(), expression, Value::Null);
    }

    fn assert_transformed_to(value: Value, expr: impl Apply, expected: Value) {
        let actual: Value = transform(value, expr);
        assert_eq!(actual, expected);
    }
}