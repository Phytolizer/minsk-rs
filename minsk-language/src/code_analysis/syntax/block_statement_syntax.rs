use std::fmt::Display;

use crate::code_analysis::text::text_span::TextSpan;

use super::{statement_syntax::StatementSyntax, syntax_token::SyntaxToken};

#[derive(Debug, Clone, PartialEq)]
pub struct BlockStatementSyntax {
    open_brace_token: SyntaxToken,
    statements: Vec<StatementSyntax>,
    close_brace_token: SyntaxToken,
}

impl BlockStatementSyntax {
    pub(crate) fn new(
        open_brace_token: SyntaxToken,
        statements: Vec<StatementSyntax>,
        close_brace_token: SyntaxToken,
    ) -> Self {
        Self {
            open_brace_token,
            statements,
            close_brace_token,
        }
    }

    pub(crate) fn open_brace_token(&self) -> &SyntaxToken {
        &self.open_brace_token
    }

    pub(crate) fn statements(&self) -> &[StatementSyntax] {
        &self.statements
    }

    pub(crate) fn close_brace_token(&self) -> &SyntaxToken {
        &self.close_brace_token
    }

    pub(crate) fn span(&self) -> TextSpan {
        TextSpan {
            start: self.open_brace_token.span.start,
            end: self.close_brace_token.span.end,
        }
    }
}

impl Display for BlockStatementSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        for statement in self.statements() {
            write!(f, "    {}", statement)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}
