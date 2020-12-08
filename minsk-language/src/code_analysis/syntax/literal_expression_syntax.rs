use std::fmt::Display;

use crate::code_analysis::text::text_span::TextSpan;

use super::super::minsk_value::MinskValue;

use super::syntax_token::SyntaxToken;

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralExpressionSyntax {
    pub(crate) literal_token: SyntaxToken,
    pub(crate) value: Option<MinskValue>,
}

impl LiteralExpressionSyntax {
    pub(crate) fn new(literal_token: SyntaxToken) -> Self {
        Self {
            value: literal_token.value.clone(),
            literal_token,
        }
    }

    pub fn span(&self) -> TextSpan {
        self.literal_token.span
    }
}

impl Display for LiteralExpressionSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "LiteralExpressionSyntax")?;
        write!(f, "    {}", self.literal_token)
    }
}
