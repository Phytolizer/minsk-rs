use crate::minsk_value::MinskValue;

use super::syntax::{expression_syntax::ExpressionSyntax, syntax_kind::SyntaxKind};

pub(crate) struct Evaluator;

impl Evaluator {
    pub(crate) fn evaluate(root: &ExpressionSyntax) -> i32 {
        Self::evaluate_expression(root)
    }

    fn evaluate_expression(root: &ExpressionSyntax) -> i32 {
        match root {
            ExpressionSyntax::LiteralExpressionSyntax(lit) => {
                let MinskValue::Integer(i) = lit.literal_token.value.as_ref().unwrap();
                *i
            }
            ExpressionSyntax::UnaryExpressionSyntax(u) => {
                let operand = Self::evaluate_expression(&u.operand);
                match u.operator_token.kind {
                    SyntaxKind::Plus => operand,
                    SyntaxKind::Minus => -operand,
                    _ => panic!("Unexpected unary operator {}", u.operator_token.kind),
                }
            }
            ExpressionSyntax::BinaryExpressionSyntax(b) => {
                let left = Self::evaluate_expression(&b.left);
                let right = Self::evaluate_expression(&b.right);
                match b.operator_token.kind {
                    SyntaxKind::Plus => left + right,
                    SyntaxKind::Minus => left - right,
                    SyntaxKind::Star => left * right,
                    SyntaxKind::Slash => left / right,
                    _ => panic!("Unexpected binary operator {}", b.operator_token.kind),
                }
            }
            ExpressionSyntax::ParenthesizedExpressionSyntax(p) => {
                Self::evaluate_expression(&p.expression)
            }
        }
    }
}
