use super::syntax_kind::SyntaxKind;

pub(super) trait SyntaxFactsExt {
    fn binary_operator_precedence(&self) -> usize;
    fn unary_operator_precedence(&self) -> usize;
}

pub(super) struct SyntaxFacts;

impl SyntaxFacts {
    pub(super) fn keyword_kind(text: &str) -> SyntaxKind {
        match text {
            "true" => SyntaxKind::TrueKeyword,
            "false" => SyntaxKind::FalseKeyword,
            _ => SyntaxKind::Identifier,
        }
    }
}

impl SyntaxFactsExt for SyntaxKind {
    fn binary_operator_precedence(&self) -> usize {
        match self {
            SyntaxKind::Star | SyntaxKind::Slash => 2,
            SyntaxKind::Plus | SyntaxKind::Minus => 1,
            _ => 0,
        }
    }

    fn unary_operator_precedence(&self) -> usize {
        match self {
            SyntaxKind::Plus | SyntaxKind::Minus => 3,
            _ => 0,
        }
    }
}
