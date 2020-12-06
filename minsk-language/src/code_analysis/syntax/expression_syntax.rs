use std::fmt::Display;

use crate::code_analysis::text_span::TextSpan;

use super::{
    assignment_expression_syntax::AssignmentExpressionSyntax,
    binary_expression_syntax::BinaryExpressionSyntax,
    literal_expression_syntax::LiteralExpressionSyntax,
    name_expression_syntax::NameExpressionSyntax,
    parenthesized_expression_syntax::ParenthesizedExpressionSyntax,
    unary_expression_syntax::UnaryExpressionSyntax,
};

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionSyntax {
    Literal(LiteralExpressionSyntax),
    Unary(UnaryExpressionSyntax),
    Binary(BinaryExpressionSyntax),
    Parenthesized(ParenthesizedExpressionSyntax),
    Name(NameExpressionSyntax),
    Assignment(AssignmentExpressionSyntax),
}

impl ExpressionSyntax {
    pub fn span(&self) -> TextSpan {
        match self {
            ExpressionSyntax::Literal(l) => l.span(),
            ExpressionSyntax::Unary(u) => u.span(),
            ExpressionSyntax::Binary(b) => b.span(),
            ExpressionSyntax::Parenthesized(p) => p.span(),
            ExpressionSyntax::Name(n) => n.span(),
            ExpressionSyntax::Assignment(a) => a.span(),
        }
    }
}

fn format_indented(s: &dyn Display, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for line in s.to_string().split('\n') {
        writeln!(f, "    {}", line)?;
    }
    Ok(())
}

impl Display for ExpressionSyntax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ExpressionSyntax")?;
        match self {
            ExpressionSyntax::Literal(l) => format_indented(l, f),
            ExpressionSyntax::Unary(u) => format_indented(u, f),
            ExpressionSyntax::Binary(b) => format_indented(b, f),
            ExpressionSyntax::Parenthesized(p) => format_indented(p, f),
            ExpressionSyntax::Name(n) => format_indented(n, f),
            ExpressionSyntax::Assignment(a) => format_indented(a, f),
        }
    }
}
