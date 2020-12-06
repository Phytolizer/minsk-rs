use std::fmt::Display;

use crate::code_analysis::text::text_span::TextSpan;

use super::compilation_unit::CompilationUnit;

#[derive(Debug, Clone, PartialEq)]
pub enum SyntaxNode {
    CompilationUnit(CompilationUnit),
}

impl SyntaxNode {
    pub fn span(&self) -> TextSpan {
        match self {
            Self::CompilationUnit(c) => c.span(),
        }
    }
}

impl Display for SyntaxNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "SyntaxNode")?;
        match self {
            Self::CompilationUnit(c) => write!(f, "{}", c),
        }
    }
}
