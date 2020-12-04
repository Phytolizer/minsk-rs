use crate::minsk_value::MinskValue;

use super::binding::{
    bound_binary_operator_kind::BoundBinaryOperatorKind, bound_expression::BoundExpression,
    bound_unary_operator_kind::BoundUnaryOperatorKind,
};

pub(crate) struct Evaluator;

impl Evaluator {
    pub(crate) fn evaluate(root: &BoundExpression) -> MinskValue {
        Self::evaluate_expression(root)
    }

    fn evaluate_expression(root: &BoundExpression) -> MinskValue {
        match root {
            BoundExpression::BoundLiteralExpression(lit) => lit.value.clone(),
            BoundExpression::BoundUnaryExpression(u) => {
                let operand = Self::evaluate_expression(&u.operand).as_int().unwrap();
                match u.operator_kind {
                    BoundUnaryOperatorKind::Identity => MinskValue::Integer(operand),
                    BoundUnaryOperatorKind::Negation => MinskValue::Integer(-operand),
                }
            }
            BoundExpression::BoundBinaryExpression(b) => {
                let left = Self::evaluate_expression(&b.left).as_int().unwrap();
                let right = Self::evaluate_expression(&b.right).as_int().unwrap();
                match b.operator_kind {
                    BoundBinaryOperatorKind::Addition => MinskValue::Integer(left + right),
                    BoundBinaryOperatorKind::Subtraction => MinskValue::Integer(left - right),
                    BoundBinaryOperatorKind::Multiplication => MinskValue::Integer(left * right),
                    BoundBinaryOperatorKind::Division => MinskValue::Integer(left / right),
                }
            }
        }
    }
}
