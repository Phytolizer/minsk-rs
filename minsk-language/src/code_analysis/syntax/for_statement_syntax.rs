use std::fmt::Display;

use crate::code_analysis::text::text_span::TextSpan;

use super::{
    expression_syntax::ExpressionSyntax, statement_syntax::StatementSyntax,
    syntax_token::SyntaxToken,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ForStatementSyntax {
    for_keyword: SyntaxToken,
    identifier: SyntaxToken,
    equals_token: SyntaxToken,
    lower_bound: Box<ExpressionSyntax>,
    to_keyword: SyntaxToken,
    upper_bound: Box<ExpressionSyntax>,
    body: Box<StatementSyntax>,
}

impl ForStatementSyntax {
    pub(crate) fn new(
        for_keyword: SyntaxToken,
        identifier: SyntaxToken,
        equals_token: SyntaxToken,
        lower_bound: Box<ExpressionSyntax>,
        to_keyword: SyntaxToken,
        upper_bound: Box<ExpressionSyntax>,
        body: Box<StatementSyntax>,
    ) -> Self {
        Self {
            for_keyword,
            identifier,
            equals_token,
            lower_bound,
            to_keyword,
            upper_bound,
            body,
        }
    }

    pub(crate) fn span(&self) -> TextSpan {
        TextSpan {
            start: self.for_keyword.span.start,
            end: self.upper_bound.span().end,
        }
    }

    pub(crate) fn identifier(&self) -> &SyntaxToken {
        &self.identifier
    }

    pub(crate) fn lower_bound(&self) -> &ExpressionSyntax {
        &self.lower_bound
    }

    pub(crate) fn upper_bound(&self) -> &ExpressionSyntax {
        &self.upper_bound
    }

    pub(crate) fn body(&self) -> &StatementSyntax {
        &self.body
    }
}

impl Display for ForStatementSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ForStatementSyntax")?;
        writeln!(f, "{}..{}", self.lower_bound, self.upper_bound)?;
        Ok(())
    }
}
