use std::fmt::Display;

use crate::code_analysis::text::text_span::TextSpan;

use super::{expression_syntax::ExpressionSyntax, syntax_token::SyntaxToken};

#[derive(Debug, Clone, PartialEq)]
pub struct CompilationUnit {
    expression: ExpressionSyntax,
    end_of_file_token: SyntaxToken,
}

impl CompilationUnit {
    pub fn new(expression: ExpressionSyntax, end_of_file_token: SyntaxToken) -> Self {
        Self {
            expression,
            end_of_file_token,
        }
    }

    pub fn expression(&self) -> &ExpressionSyntax {
        &self.expression
    }

    pub fn end_of_file_token(&self) -> &SyntaxToken {
        &self.end_of_file_token
    }

    pub fn span(&self) -> TextSpan {
        TextSpan {
            start: self.expression.span().start,
            end: self.end_of_file_token.span.end,
        }
    }
}

impl Display for CompilationUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expression)?;
        write!(f, "{}", self.end_of_file_token)?;
        Ok(())
    }
}
