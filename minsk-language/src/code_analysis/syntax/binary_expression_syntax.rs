use std::fmt::Display;

use crate::code_analysis::text_span::TextSpan;

use super::{expression_syntax::ExpressionSyntax, syntax_token::SyntaxToken};

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpressionSyntax {
    pub(crate) left: Box<ExpressionSyntax>,
    pub(crate) operator_token: SyntaxToken,
    pub(crate) right: Box<ExpressionSyntax>,
}

impl BinaryExpressionSyntax {
    pub fn span(&self) -> TextSpan {
        TextSpan {
            start: self.left.span().start,
            end: self.right.span().end,
        }
    }
}

impl Display for BinaryExpressionSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "BinaryExpressionSyntax")?;
        writeln!(f, "    {}", self.left)?;
        writeln!(f, "    {}", self.operator_token)?;
        writeln!(f, "    {}", self.right)?;
        Ok(())
    }
}
