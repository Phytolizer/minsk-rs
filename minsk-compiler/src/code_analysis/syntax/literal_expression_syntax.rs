use std::fmt::Display;

use super::syntax_token::SyntaxToken;

#[derive(Debug, Clone)]
pub(crate) struct LiteralExpressionSyntax {
    pub(crate) literal_token: SyntaxToken,
}

impl Display for LiteralExpressionSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "LiteralExpressionSyntax")?;
        write!(f, "    {}", self.literal_token)
    }
}
