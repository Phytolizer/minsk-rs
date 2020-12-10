use std::fmt::Display;

use crate::code_analysis::text::text_span::TextSpan;

use super::{
    compilation_unit::CompilationUnit, if_statement_syntax::ElseClauseSyntax,
    statement_syntax::StatementSyntax,
};

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxNode {
    CompilationUnit(CompilationUnit),
    Statement(StatementSyntax),
    ElseClause(ElseClauseSyntax),
}

impl SyntaxNode {
    pub fn span(&self) -> TextSpan {
        match self {
            SyntaxNode::CompilationUnit(c) => c.span(),
            SyntaxNode::Statement(s) => s.span(),
            SyntaxNode::ElseClause(e) => e.span(),
        }
    }
}

impl Display for SyntaxNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "SyntaxNode")?;
        match self {
            SyntaxNode::CompilationUnit(c) => write!(f, "{}", c),
            SyntaxNode::Statement(s) => write!(f, "{}", s),
            SyntaxNode::ElseClause(e) => write!(f, "{}", e),
        }
    }
}
