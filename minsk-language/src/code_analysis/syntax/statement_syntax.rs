use std::fmt::Display;

use crate::code_analysis::text::text_span::TextSpan;

use super::{
    block_statement_syntax::BlockStatementSyntax,
    expression_statement_syntax::ExpressionStatementSyntax,
};

#[derive(Debug, Clone, PartialEq)]
pub enum StatementSyntax {
    Block(BlockStatementSyntax),
    Expression(ExpressionStatementSyntax),
}

impl StatementSyntax {
    pub(crate) fn span(&self) -> TextSpan {
        match self {
            StatementSyntax::Block(b) => b.span(),
            StatementSyntax::Expression(e) => e.span(),
        }
    }
}

impl Display for StatementSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatementSyntax::Block(b) => write!(f, "{}", b),
            StatementSyntax::Expression(e) => write!(f, "{}", e),
        }
    }
}
