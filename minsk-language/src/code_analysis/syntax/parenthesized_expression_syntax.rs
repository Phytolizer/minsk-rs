use std::fmt::Display;

use crate::code_analysis::text::text_span::TextSpan;

use super::{expression_syntax::ExpressionSyntax, syntax_token::SyntaxToken};

#[derive(Debug, Clone, PartialEq)]
pub struct ParenthesizedExpressionSyntax {
    pub(crate) open_parenthesis_token: SyntaxToken,
    pub(crate) expression: Box<ExpressionSyntax>,
    pub(crate) close_parenthesis_token: SyntaxToken,
}

impl ParenthesizedExpressionSyntax {
    pub fn span(&self) -> TextSpan {
        TextSpan {
            start: self.open_parenthesis_token.span.start,
            end: self.close_parenthesis_token.span.end,
        }
    }
}

impl Display for ParenthesizedExpressionSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ParenthesizedExpressionSyntax")?;
        writeln!(f, "    {}", self.open_parenthesis_token)?;
        writeln!(f, "    {}", self.expression)?;
        writeln!(f, "    {}", self.close_parenthesis_token)?;
        Ok(())
    }
}
