use std::fmt::Display;

use crate::code_analysis::text::text_span::TextSpan;

use super::{
    expression_syntax::ExpressionSyntax, statement_syntax::StatementSyntax,
    syntax_token::SyntaxToken,
};

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatementSyntax {
    if_keyword: SyntaxToken,
    condition: ExpressionSyntax,
    then_statement: Box<StatementSyntax>,
    else_clause: Option<ElseClauseSyntax>,
}

impl IfStatementSyntax {
    pub(crate) fn span(&self) -> TextSpan {
        TextSpan {
            start: self.if_keyword.span.start,
            end: if let Some(e) = &self.else_clause {
                e.span().end
            } else {
                self.then_statement.span().end
            },
        }
    }

    pub(crate) fn new(
        if_keyword: SyntaxToken,
        condition: ExpressionSyntax,
        then_statement: Box<StatementSyntax>,
        else_clause: Option<ElseClauseSyntax>,
    ) -> Self {
        Self {
            if_keyword,
            condition,
            then_statement,
            else_clause,
        }
    }

    pub(crate) fn condition(&self) -> &ExpressionSyntax {
        &self.condition
    }

    pub(crate) fn then_statement(&self) -> &StatementSyntax {
        &self.then_statement
    }

    pub(crate) fn else_statement(&self) -> Option<&ElseClauseSyntax> {
        self.else_clause.as_ref()
    }
}

impl Display for IfStatementSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "IfStatementSyntax")?;
        writeln!(f, "    {}", self.then_statement)?;
        if let Some(e) = &self.else_clause {
            write!(f, "    {}", e)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElseClauseSyntax {
    else_keyword: SyntaxToken,
    else_statement: Box<StatementSyntax>,
}

impl ElseClauseSyntax {
    pub(crate) fn span(&self) -> TextSpan {
        TextSpan {
            start: self.else_keyword.span.start,
            end: self.else_statement.span().end,
        }
    }

    pub(crate) fn new(else_keyword: SyntaxToken, else_statement: Box<StatementSyntax>) -> Self {
        Self {
            else_keyword,
            else_statement,
        }
    }

    pub(crate) fn else_statement(&self) -> &StatementSyntax {
        &self.else_statement
    }
}

impl Display for ElseClauseSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ElseClauseSyntax")?;
        writeln!(f, "    {}", self.else_statement)?;
        Ok(())
    }
}
