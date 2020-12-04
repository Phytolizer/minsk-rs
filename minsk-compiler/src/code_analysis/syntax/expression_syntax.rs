use std::fmt::Display;

use super::{
    binary_expression_syntax::BinaryExpressionSyntax,
    literal_expression_syntax::LiteralExpressionSyntax,
    parenthesized_expression_syntax::ParenthesizedExpressionSyntax,
    unary_expression_syntax::UnaryExpressionSyntax,
};

#[derive(Debug, Clone)]
pub(crate) enum ExpressionSyntax {
    LiteralExpressionSyntax(LiteralExpressionSyntax),
    UnaryExpressionSyntax(UnaryExpressionSyntax),
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
            }
            ExpressionSyntax::UnaryExpressionSyntax(u) => {
                for line in u.to_string().split('\n').filter(|l| !l.is_empty()) {
                    writeln!(f, "    {}", line)?;
                }
            }
            ExpressionSyntax::BinaryExpressionSyntax(b) => {
                for line in b.to_string().split('\n').filter(|l| !l.is_empty()) {
                    writeln!(f, "    {}", line)?;
                }
            }
            ExpressionSyntax::ParenthesizedExpressionSyntax(p) => {
                for line in p.to_string().split('\n').filter(|l| !l.is_empty()) {
                    writeln!(f, "    {}", line)?;
                }
            }
        }
        Ok(())
    }
}
