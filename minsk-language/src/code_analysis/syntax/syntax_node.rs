use std::fmt::Display;

use crate::code_analysis::text_span::TextSpan;

use super::expression_syntax::ExpressionSyntax;

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxNode {
    ExpressionSyntax(ExpressionSyntax),
}

impl SyntaxNode {
    pub fn span(&self) -> TextSpan {
        match self {
            Self::ExpressionSyntax(e) => e.span(),
        }
    }
}

impl Display for SyntaxNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "SyntaxNode")?;
        match self {
            Self::ExpressionSyntax(e) => write!(f, "{}", e),
        }
    }
}
