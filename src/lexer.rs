#[derive(Debug)]
#[derive(PartialEq)]
pub struct Token {}

pub struct Lexer {
    text: String,
    pos: u32,
}

impl Lexer {
    pub fn on(text: String) -> Lexer
    {
        Lexer { text, pos: 0 }
    }
    pub fn next_token(self: Self) -> Option<Token> {
        Option::None
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, Token};

    #[test]
    fn lex_empty() {
        assert_no_token("")
    }

    fn assert_no_token(text: &str) {
        let actual = first_token(text.to_string());
        assert_eq!(None, actual)
    }

    fn assert_tokenized(text: &str, token: Token) {
        let actual = first_token(text.to_string());
        assert_eq!(Some(token), actual)
    }

    fn first_token(text: String) -> Option<Token> {
        let lex = Lexer::on(text);
        let actual = lex.next_token();
        actual
    }
}