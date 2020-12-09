use std::fmt::Display;

use crate::code_analysis::text::text_span::TextSpan;

use super::{
    expression_syntax::ExpressionSyntax, statement_syntax::StatementSyntax,
    syntax_token::SyntaxToken,
};

#[derive(Debug, Clone, PartialEq)]
pub struct WhileStatementSyntax {
    while_keyword: SyntaxToken,
    condition: ExpressionSyntax,
    body: Box<StatementSyntax>,
}

impl WhileStatementSyntax {
    pub(crate) fn new(
        while_keyword: SyntaxToken,
        condition: ExpressionSyntax,
        body: Box<StatementSyntax>,
    ) -> Self {
        Self {
            while_keyword,
            condition,
            body,
        }
    }

    pub(crate) fn span(&self) -> TextSpan {
        TextSpan {
            start: self.while_keyword.span.start,
            end: self.body.span().end,
        }
    }

    pub(crate) fn condition(&self) -> &ExpressionSyntax {
        &self.condition
    }

    pub(crate) fn body(&self) -> &StatementSyntax {
        &self.body
    }
}

impl Display for WhileStatementSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WhileStatementSyntax")?;
        writeln!(f, "{}", self.condition)?;
        writeln!(f, "    {}", self.body)?;
        Ok(())
    }
}
