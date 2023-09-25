use string_builder::Builder;

use crate::lexer::Type::Identifier;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Token {
    pub token_type: Type,
    pub text: String,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Type {
    Dot,
    Identifier,
    Pipe,
}

#[derive(Debug)]
pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    total: usize,
}

impl Lexer {
    pub fn on(text: String) -> Lexer {
        let chars: Vec<char> = text.chars().collect();
        let total = chars.len();
        Lexer { chars, pos: 0, total }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.pos == self.total {
            None
        } else { self.remaining() }
    }

    fn remaining(&mut self) -> Option<Token> {
        while self.has_remaining() {
            let next = self.advance();

            return match next {
                '.' => Some(Token { token_type: Type::Dot, text: ".".to_string() }),
                'a'..='z' | 'A'..='Z' => Some(Token { token_type: Identifier, text: self.consume_alnum(next) }),
                '|' => Some(Token{token_type: Type::Pipe, text: "|".to_string()}),
                _ => None
            };
        }
        unreachable!()
    }

    fn consume_alnum(&mut self, first: char) -> String {
        let mut builder = Builder::new(1);
        builder.append(first);

        let mut done = false;
        while self.has_remaining() && !done {
            let peek = self.peek();

            match peek {
                None => done = true,
                Some(c) => {
                    if c.is_alphanumeric() {
                        builder.append(c);
                        self.advance();
                    } else { done = true; }
                }
            }
        }
        return builder.string().unwrap();
    }

    pub fn has_remaining(&self) -> bool {
        self.pos < self.total
    }

    fn advance(&mut self) -> char {
        assert!(self.has_remaining());
        let c = self.peek().unwrap();
        self.pos += 1;
        c
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).map(char::clone)
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, Token};
    use crate::lexer::Type::{Dot, Identifier};

    #[test]
    fn lex_empty() {
        assert_no_token("")
    }

    #[test]
    fn test_dot() {
        assert_tokenized(".", Token { token_type: Dot, text: ".".to_string() });
    }

    #[test]
    fn test_identifier() {
        assert_tokenized("bar", Token { token_type: Identifier, text: "bar".to_string() });
    }

    #[test]
    fn test_identifier_with_numbers() {
        assert_tokenized("a3b42", Token { token_type: Identifier, text: "a3b42".to_string() });
    }

    #[test]
    fn test_dot_identifier() {
        let dot = Token { token_type: Dot, text: ".".to_string() };
        let ident = Token { token_type: Identifier, text: "bar".to_string() };
        assert_tokenized_many(".bar", vec![dot, ident]);
    }

    #[test]
    fn test_dot_identifier_many() {
        let dot = Token { token_type: Dot, text: ".".to_string() };
        let ident_ab = Token { token_type: Identifier, text: "ab".to_string() };
        let ident_cd = Token { token_type: Identifier, text: "cd".to_string() };
        let expected = vec![dot.clone(), ident_ab, dot, ident_cd];

        assert_tokenized_many(".ab.cd", expected);
    }

    fn assert_no_token(text: &str) {
        let actual = first_token(text.to_string());
        assert_eq!(actual, None)
    }

    fn assert_tokenized(text: &str, token: Token) {
        let actual = first_token(text.to_string());
        assert_eq!(actual, Some(token))
    }

    fn assert_tokenized_many(text: &str, tokens: Vec<Token>) {
        let mut lex = Lexer::on(text.to_string());
        let mut actual = vec![];
        while lex.has_remaining() {
            actual.push(lex.next_token().expect("must be a token"));
        }
        assert_eq!(actual, tokens);
    }

    fn first_token(text: String) -> Option<Token> {
        let mut lex = Lexer::on(text);
        println!("{:#?}", lex);

        let actual = lex.next_token();
        actual
    }
}