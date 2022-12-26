use crate::ast::{Token, TokenKind::*};
use std::str::Chars;

/// Template lexer.
pub struct Lexer<'a> {
    chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer from the given template string slice.
    pub fn new(template: &'a str) -> Lexer {
        Lexer {
            chars: template.chars(),
        }
    }

    /// Returns the next token.
    pub fn next_token(&mut self) -> Token {
        Token::new(Raw, 0, self.chars.as_str().len())
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Token, TokenKind::*};
    use crate::lex::Lexer;

    #[test]
    fn new() {
        let l1 = Lexer::new("Hello, world!");
        assert_eq!(l1.chars.collect::<String>(), "Hello, world!");

        let l2 = Lexer::new("👋🌍");
        assert_eq!(l2.chars.collect::<String>(), "👋🌍");
    }

    #[test]
    fn next_token_raw() {
        let mut l1 = Lexer::new(r#"fn main() { println!("Hello, world!"); }"#);
        assert_eq!(l1.next_token(), Token::new(Raw, 0, 40));

        let mut l2 = Lexer::new("pub fn add(left: usize, right: usize) -> usize { left + right }");
        assert_eq!(l2.next_token(), Token::new(Raw, 0, 63));
    }
}
