use std::fmt::Display;

use super::{
    binary_expression_syntax::BinaryExpressionSyntax,
    literal_expression_syntax::LiteralExpressionSyntax,
    parenthesized_expression_syntax::ParenthesizedExpressionSyntax,
};

#[derive(Debug, Clone)]
pub(crate) enum ExpressionSyntax {
    LiteralExpressionSyntax(LiteralExpressionSyntax),
    BinaryExpressionSyntax(BinaryExpressionSyntax),
    ParenthesizedExpressionSyntax(ParenthesizedExpressionSyntax),
}

impl Display for ExpressionSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ExpressionSyntax")?;
        match self {
            ExpressionSyntax::LiteralExpressionSyntax(lit) => {
                for line in lit.to_string().split('\n') {
                    writeln!(f, "    {}", line)?;
                }
                Ok(())
            }
            ExpressionSyntax::BinaryExpressionSyntax(b) => {
                for line in b.to_string().split('\n').filter(|l| !l.is_empty()) {
                    writeln!(f, "    {}", line)?;
                }
                Ok(())
            }
            ExpressionSyntax::ParenthesizedExpressionSyntax(p) => {
                for line in p.to_string().split('\n').filter(|l| !l.is_empty()) {
                    writeln!(f, "    {}", line)?;
                }
                Ok(())
            }
        }
    }
}
