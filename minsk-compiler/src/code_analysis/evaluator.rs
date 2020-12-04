use crate::minsk_value::MinskValue;

use super::binding::{
    bound_binary_operator_kind::BoundBinaryOperatorKind, bound_expression::BoundExpression,
    bound_unary_operator_kind::BoundUnaryOperatorKind,
};

pub(crate) struct Evaluator;

impl Evaluator {
    pub(crate) fn evaluate(root: &BoundExpression) -> i32 {
        Self::evaluate_expression(root)
    }

    fn evaluate_expression(root: &BoundExpression) -> i32 {
        match root {
            BoundExpression::BoundLiteralExpression(lit) => {
                let MinskValue::Integer(i) = lit.value;
                i
            }
            BoundExpression::BoundUnaryExpression(u) => {
                let operand = Self::evaluate_expression(&u.operand);
                match u.operator_kind {
                    BoundUnaryOperatorKind::Identity => operand,
                    BoundUnaryOperatorKind::Negation => -operand,
                }
            }
            BoundExpression::BoundBinaryExpression(b) => {
                let left = Self::evaluate_expression(&b.left);
                let right = Self::evaluate_expression(&b.right);
                match b.operator_kind {
                    BoundBinaryOperatorKind::Addition => left + right,
                    BoundBinaryOperatorKind::Subtraction => left - right,
                    BoundBinaryOperatorKind::Multiplication => left * right,
                    BoundBinaryOperatorKind::Division => left / right,
                }
            }
        }
    }
}
