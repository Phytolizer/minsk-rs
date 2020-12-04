use std::fmt::Display;

use super::expression_syntax::ExpressionSyntax;

#[derive(Debug)]
pub(crate) enum SyntaxNode {
    ExpressionSyntax(ExpressionSyntax),
}

impl Display for SyntaxNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "SyntaxNode")?;
        match self {
            Self::ExpressionSyntax(e) => write!(f, "{}", e),
        }
    }
}
