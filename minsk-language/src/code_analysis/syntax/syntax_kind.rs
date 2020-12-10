use std::fmt::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq, strum::EnumIter, Hash)]
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
    Less,
    LessEquals,
    Greater,
    GreaterEquals,
    Tilde,
    Ampersand,
    Pipe,
    Hat,

    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,

    // Keywowrds
    FalseKeyword,
    TrueKeyword,
    LetKeyword,
    VarKeyword,
    IfKeyword,
    ElseKeyword,
    WhileKeyword,
    ForKeyword,
    ToKeyword,
}

impl Display for SyntaxKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
