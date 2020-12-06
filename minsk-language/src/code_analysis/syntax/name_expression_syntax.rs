use std::fmt::Display;

use crate::code_analysis::text_span::TextSpan;

use super::syntax_token::SyntaxToken;

#[derive(Debug, Clone, PartialEq)]
pub struct NameExpressionSyntax {
    pub(crate) identifier_token: SyntaxToken,
}

impl NameExpressionSyntax {
    pub fn span(&self) -> TextSpan {
        self.identifier_token.span.clone()
    }
}

impl Display for NameExpressionSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "NameExpressionSyntax")?;
        writeln!(f, "    {}", self.identifier_token)?;
        Ok(())
    }
}
