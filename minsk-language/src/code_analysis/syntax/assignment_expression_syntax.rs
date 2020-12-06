use std::fmt::Display;

use crate::code_analysis::text::text_span::TextSpan;

use super::{expression_syntax::ExpressionSyntax, syntax_token::SyntaxToken};

#[derive(Debug, Clone, PartialEq)]
pub struct AssignmentExpressionSyntax {
    pub(crate) identifier_token: SyntaxToken,
    pub(crate) equals_token: SyntaxToken,
    pub(crate) expression: Box<ExpressionSyntax>,
}

impl AssignmentExpressionSyntax {
    pub fn span(&self) -> TextSpan {
        TextSpan {
            start: self.identifier_token.span.start,
            end: self.expression.span().end,
        }
    }
}

impl Display for AssignmentExpressionSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "AssignmentExpressionSyntax")?;
        writeln!(f, "    {}", self.identifier_token)?;
        writeln!(f, "    {}", self.equals_token)?;
        writeln!(f, "    {}", self.expression)?;
        Ok(())
    }
}
