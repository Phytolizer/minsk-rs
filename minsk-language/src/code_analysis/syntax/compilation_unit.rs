use std::fmt::Display;

use crate::code_analysis::text::text_span::TextSpan;

use super::{expression_syntax::ExpressionSyntax, syntax_token::SyntaxToken};

#[derive(Debug, Clone, PartialEq)]
pub struct CompilationUnit {
    pub root: ExpressionSyntax,
    pub end_of_file_token: SyntaxToken,
}

impl CompilationUnit {
    pub fn span(&self) -> TextSpan {
        TextSpan {
            start: self.root.span().start,
            end: self.end_of_file_token.span.end,
        }
    }
}

impl Display for CompilationUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)?;
        write!(f, "{}", self.end_of_file_token)?;
        Ok(())
    }
}
