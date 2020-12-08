use std::fmt::Display;

use crate::code_analysis::text::text_span::TextSpan;

use super::{statement_syntax::StatementSyntax, syntax_token::SyntaxToken};

#[derive(Debug, Clone, PartialEq)]
pub struct CompilationUnit {
    statement: StatementSyntax,
    end_of_file_token: SyntaxToken,
}

impl CompilationUnit {
    pub fn new(statement: StatementSyntax, end_of_file_token: SyntaxToken) -> Self {
        Self {
            statement,
            end_of_file_token,
        }
    }

    pub fn statement(&self) -> &StatementSyntax {
        &self.statement
    }

    pub fn end_of_file_token(&self) -> &SyntaxToken {
        &self.end_of_file_token
    }

    pub fn span(&self) -> TextSpan {
        TextSpan {
            start: self.statement.span().start,
            end: self.end_of_file_token.span.end,
        }
    }
}

impl Display for CompilationUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.statement)?;
        write!(f, "{}", self.end_of_file_token)?;
        Ok(())
    }
}
