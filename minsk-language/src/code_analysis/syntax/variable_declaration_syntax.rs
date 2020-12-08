use std::fmt::Display;

use crate::code_analysis::text::text_span::TextSpan;

use super::{expression_syntax::ExpressionSyntax, syntax_token::SyntaxToken};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclarationSyntax {
    keyword_token: SyntaxToken,
    identifier: SyntaxToken,
    equals_token: SyntaxToken,
    initializer: ExpressionSyntax,
}

impl VariableDeclarationSyntax {
    pub(crate) fn new(
        keyword_token: SyntaxToken,
        identifier: SyntaxToken,
        equals_token: SyntaxToken,
        initializer: ExpressionSyntax,
    ) -> Self {
        Self {
            keyword_token,
            identifier,
            equals_token,
            initializer,
        }
    }

    pub(crate) fn span(&self) -> TextSpan {
        TextSpan {
            start: self.keyword_token.span.start,
            end: self.initializer.span().end,
        }
    }

    pub(crate) fn keyword_token(&self) -> &SyntaxToken {
        &self.keyword_token
    }

    pub(crate) fn identifier(&self) -> &SyntaxToken {
        &self.identifier
    }

    pub(crate) fn initializer(&self) -> &ExpressionSyntax {
        &self.initializer
    }
}

impl Display for VariableDeclarationSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VariableDeclarationSyntax({}, {})",
            self.identifier.text, self.initializer
        )
    }
}
