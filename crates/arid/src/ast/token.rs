/// Represents a token.
#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    /// Token type.
    pub kind: TokenKind,
    /// Position of the first byte.
    pub start: usize,
    /// Position of the last byte.
    pub end: usize,
}

impl Token {
    /// Creates a new token from the given type and span.
    pub const fn new(kind: TokenKind, start: usize, end: usize) -> Token {
        Token { kind, start, end }
    }
}

/// Represents the token types.
#[derive(Debug, Eq, PartialEq)]
pub enum TokenKind {
    /// Opening template delimiter: `{{`, `{%` or `{#`.
    OpenDelim(Delim),
    /// Closing template delimiter: `}}`, `%}` or `#}`.
    CloseDelim(Delim),
    /// Text which will be output as is.
    Raw,
}

/// Represents the template delimiters.
#[derive(Debug, Eq, PartialEq)]
pub enum Delim {
    /// Expression delimiter: `{{` or `}}`.
    Expr,
    /// Statement delimiter: `{%` or `%}`.
    Stmt,
    /// Comment delimiter: `{#` or `#}`.
    Cmnt,
}

#[cfg(test)]
mod tests {
    use crate::ast::{Token, TokenKind::*};

    #[test]
    fn new() {
        let t1 = Token::new(Raw, 5, 6);
        assert_eq!(t1.kind, Raw);
        assert_eq!(t1.start, 5);
        assert_eq!(t1.end, 6);

        let t2 = Token::new(Raw, 12, 12);
        assert_eq!(t2.kind, Raw);
        assert_eq!(t2.start, 12);
        assert_eq!(t2.end, 12);
    }
}
