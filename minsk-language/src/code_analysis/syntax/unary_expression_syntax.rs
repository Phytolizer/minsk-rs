use std::fmt::Display;

use crate::code_analysis::text_span::TextSpan;

use super::{expression_syntax::ExpressionSyntax, syntax_token::SyntaxToken};

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpressionSyntax {
    pub(crate) operator_token: SyntaxToken,
    pub(crate) operand: Box<ExpressionSyntax>,
}

impl UnaryExpressionSyntax {
    pub fn span(&self) -> TextSpan {
        TextSpan {
            start: self.operator_token.span.start,
            end: self.operand.span().end,
        }
    }
}

impl Display for UnaryExpressionSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "UnaryExpressionSyntax")?;
        writeln!(f, "    {}", self.operator_token)?;
        writeln!(f, "    {}", self.operand)?;
        Ok(())
    }
}
