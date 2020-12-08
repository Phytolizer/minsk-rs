use std::fmt::Display;

use crate::code_analysis::text::text_span::TextSpan;

use super::{
    block_statement_syntax::BlockStatementSyntax,
    expression_statement_syntax::ExpressionStatementSyntax,
    variable_declaration_syntax::VariableDeclarationSyntax,
};

#[derive(Debug, Clone, PartialEq)]
pub enum StatementSyntax {
    Block(BlockStatementSyntax),
    Expression(ExpressionStatementSyntax),
    VariableDeclaration(VariableDeclarationSyntax),
}

impl StatementSyntax {
    pub(crate) fn span(&self) -> TextSpan {
        match self {
            StatementSyntax::Block(b) => b.span(),
            StatementSyntax::Expression(e) => e.span(),
            StatementSyntax::VariableDeclaration(v) => v.span(),
        }
    }
}

impl Display for StatementSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatementSyntax::Block(b) => write!(f, "{}", b),
            StatementSyntax::Expression(e) => write!(f, "{}", e),
            StatementSyntax::VariableDeclaration(v) => write!(f, "{}", v),
        }
    }
}
