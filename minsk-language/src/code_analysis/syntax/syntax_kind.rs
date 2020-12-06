use std::fmt::Display;
use strum::EnumIter;

#[derive(Debug, Copy, Clone, Eq, PartialEq, EnumIter, Hash)]
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
    Equals,
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
