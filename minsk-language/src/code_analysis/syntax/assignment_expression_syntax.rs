use std::fmt::Display;

use super::{expression_syntax::ExpressionSyntax, syntax_token::SyntaxToken};

#[derive(Debug, Clone)]
pub struct AssignmentExpressionSyntax {
    pub(crate) identifier_token: SyntaxToken,
    pub(crate) equals_token: SyntaxToken,
    pub(crate) expression: Box<ExpressionSyntax>,
}

impl Display for AssignmentExpressionSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "AssignmentExpressionSyntax")?;
        writeln!(f, "    {}", self.identifier_token)?;
        writeln!(f, "    {}", self.equals_token)?;
        writeln!(f, "    {}", self.expression)?;
        Ok(())
    }
}
