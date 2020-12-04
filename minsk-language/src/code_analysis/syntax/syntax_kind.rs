use std::fmt::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum SyntaxKind {
    // Tokens
    EndOfFile,
    BadToken,

    Whitespace,
    Number,
    Identifier,

    Plus,
    Minus,
    Star,
    Slash,
    Bang,
    AmpersandAmpersand,
    PipePipe,
    EqualsEquals,
    BangEquals,

    OpenParenthesis,
    CloseParenthesis,

    // Keywowrds
    FalseKeyword,
    TrueKeyword,
}

impl Display for SyntaxKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}