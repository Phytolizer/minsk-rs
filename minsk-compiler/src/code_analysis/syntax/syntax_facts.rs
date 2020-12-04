use super::syntax_kind::SyntaxKind;

pub(crate) trait SyntaxFacts {
    fn binary_operator_precedence(&self) -> usize;
}

impl SyntaxFacts for SyntaxKind {
    fn binary_operator_precedence(&self) -> usize {
        match self {
            SyntaxKind::Star | SyntaxKind::Slash => 2,
            SyntaxKind::Plus | SyntaxKind::Minus => 1,
            _ => 0,
        }
    }
}
