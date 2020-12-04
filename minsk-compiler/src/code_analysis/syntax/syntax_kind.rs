use std::fmt::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum SyntaxKind {
    // Tokens
    EndOfFile,
    BadToken,

    Whitespace,
    Number,

    Plus,
    Minus,
    Star,
    Slash,

    OpenParenthesis,
    CloseParenthesis,
}

impl Display for SyntaxKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
