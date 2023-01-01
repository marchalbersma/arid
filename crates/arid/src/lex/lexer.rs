use crate::ast::{Delim, Token, TokenKind::*};
use std::str::Chars;

/// Template lexer.
pub struct Lexer<'a> {
    chars: Chars<'a>,
    len_remaining: usize,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer from the given template string slice.
    pub fn new(template: &'a str) -> Lexer {
        Lexer {
            chars: template.chars(),
            len_remaining: template.len(),
        }
    }

    /// Returns the next token.
    pub fn next_token(&mut self) -> Token {
        let start = self.len_remaining - self.chars.as_str().len();

        let kind = match self.chars.next().unwrap() {
            '{' => match self.chars.next().unwrap() {
                '{' => OpenDelim(Delim::Expr),
                '%' => OpenDelim(Delim::Stmt),
                '#' => OpenDelim(Delim::Cmnt),
                _ => Raw,
            },
            '}' => match self.chars.next().unwrap() {
                '}' => CloseDelim(Delim::Expr),
                _ => Raw,
            },
            '%' => match self.chars.next().unwrap() {
                '}' => CloseDelim(Delim::Stmt),
                _ => Raw,
            },
            '#' => match self.chars.next().unwrap() {
                '}' => CloseDelim(Delim::Cmnt),
                _ => Raw,
            },
            _ => Raw,
        };

        if kind == Raw {
            Token::new(Raw, start, self.len_remaining)
        } else {
            Token::new(
                kind,
                start,
                self.len_remaining - self.chars.as_str().len() - 1,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Delim, Token, TokenKind::*};
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

    #[test]
    fn next_token_delim() {
        let mut l1 = Lexer::new("{{}}");
        assert_eq!(l1.next_token(), Token::new(OpenDelim(Delim::Expr), 0, 1));
        assert_eq!(l1.next_token(), Token::new(CloseDelim(Delim::Expr), 2, 3));

        let mut l2 = Lexer::new("{%%}");
        assert_eq!(l2.next_token(), Token::new(OpenDelim(Delim::Stmt), 0, 1));
        assert_eq!(l2.next_token(), Token::new(CloseDelim(Delim::Stmt), 2, 3));

        let mut l3 = Lexer::new("{##}");
        assert_eq!(l3.next_token(), Token::new(OpenDelim(Delim::Cmnt), 0, 1));
        assert_eq!(l3.next_token(), Token::new(CloseDelim(Delim::Cmnt), 2, 3));
    }
}
