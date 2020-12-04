use std::fmt::Display;

use super::syntax_token::SyntaxToken;

#[derive(Debug, Clone)]
pub struct NameExpressionSyntax {
    pub(crate) identifier_token: SyntaxToken,
}

impl Display for NameExpressionSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "NameExpressionSyntax")?;
        writeln!(f, "    {}", self.identifier_token)?;
        Ok(())
    }
}
