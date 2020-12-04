use std::fmt::Display;

use super::{expression_syntax::ExpressionSyntax, syntax_token::SyntaxToken};

#[derive(Debug, Clone)]
pub struct UnaryExpressionSyntax {
    pub(crate) operator_token: SyntaxToken,
    pub(crate) operand: Box<ExpressionSyntax>,
}

impl Display for UnaryExpressionSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "UnaryExpressionSyntax")?;
        writeln!(f, "    {}", self.operator_token)?;
        writeln!(f, "    {}", self.operand)?;
        Ok(())
    }
}
