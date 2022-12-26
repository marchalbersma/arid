use std::iter::Peekable;
use std::str::Chars;

/// Template lexer.
pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer from the given template string slice.
    pub fn new(template: &'a str) -> Lexer {
        Lexer {
            chars: template.chars().peekable(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lex::Lexer;

    #[test]
    fn new() {
        let l1 = Lexer::new("Hello, world!");
        assert_eq!(l1.chars.collect::<String>(), "Hello, world!");

        let l2 = Lexer::new("👋🌍");
        assert_eq!(l2.chars.collect::<String>(), "👋🌍");
    }
}
