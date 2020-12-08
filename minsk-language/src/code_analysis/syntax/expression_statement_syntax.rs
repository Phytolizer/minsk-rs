use std::fmt::Display;

use crate::code_analysis::text::text_span::TextSpan;

use super::expression_syntax::ExpressionSyntax;

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatementSyntax {
    expression: ExpressionSyntax,
}

impl ExpressionStatementSyntax {
    pub(crate) fn new(expression: ExpressionSyntax) -> Self {
        Self { expression }
    }

    pub(crate) fn expression(&self) -> &ExpressionSyntax {
        &self.expression
    }

    pub(crate) fn span(&self) -> TextSpan {
        self.expression.span()
    }
}

impl Display for ExpressionStatementSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expression)
    }
}
