use std::fmt::Display;

use crate::code_analysis::text::text_span::TextSpan;

use super::{
    block_statement_syntax::BlockStatementSyntax,
    expression_statement_syntax::ExpressionStatementSyntax,
    for_statement_syntax::ForStatementSyntax, if_statement_syntax::IfStatementSyntax,
    variable_declaration_syntax::VariableDeclarationSyntax,
    while_statement_syntax::WhileStatementSyntax,
};

#[derive(Debug, Clone, PartialEq)]
pub enum StatementSyntax {
    Block(BlockStatementSyntax),
    Expression(ExpressionStatementSyntax),
    For(ForStatementSyntax),
    If(IfStatementSyntax),
    VariableDeclaration(VariableDeclarationSyntax),
    While(WhileStatementSyntax),
}

impl StatementSyntax {
    pub(crate) fn span(&self) -> TextSpan {
        match self {
            StatementSyntax::Block(b) => b.span(),
            StatementSyntax::Expression(e) => e.span(),
            StatementSyntax::For(f) => f.span(),
            StatementSyntax::If(i) => i.span(),
            StatementSyntax::VariableDeclaration(v) => v.span(),
            StatementSyntax::While(w) => w.span(),
        }
    }
}

impl Display for StatementSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatementSyntax::Block(b) => write!(f, "{}", b),
            StatementSyntax::Expression(e) => write!(f, "{}", e),
            StatementSyntax::For(o) => write!(f, "{}", o),
            StatementSyntax::If(i) => write!(f, "{}", i),
            StatementSyntax::VariableDeclaration(v) => write!(f, "{}", v),
            StatementSyntax::While(w) => write!(f, "{}", w),
        }
    }
}
