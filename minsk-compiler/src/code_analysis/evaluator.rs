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
                let operand = Self::evaluate_expression(&u.operand);
                match u.op.kind {
                    BoundUnaryOperatorKind::Identity => {
                        MinskValue::Integer(operand.as_integer().unwrap())
                    }
                    BoundUnaryOperatorKind::Negation => {
                        MinskValue::Integer(-operand.as_integer().unwrap())
                    }
                    BoundUnaryOperatorKind::LogicalNegation => {
                        MinskValue::Boolean(!operand.as_boolean().unwrap())
                    }
                }
            }
            BoundExpression::BoundBinaryExpression(b) => {
                let left = Self::evaluate_expression(&b.left);
                let right = Self::evaluate_expression(&b.right);
                match b.op.kind {
                    BoundBinaryOperatorKind::Addition => MinskValue::Integer(
                        left.as_integer().unwrap() + right.as_integer().unwrap(),
                    ),
                    BoundBinaryOperatorKind::Subtraction => MinskValue::Integer(
                        left.as_integer().unwrap() - right.as_integer().unwrap(),
                    ),
                    BoundBinaryOperatorKind::Multiplication => MinskValue::Integer(
                        left.as_integer().unwrap() * right.as_integer().unwrap(),
                    ),
                    BoundBinaryOperatorKind::Division => MinskValue::Integer(
                        left.as_integer().unwrap() / right.as_integer().unwrap(),
                    ),
                    BoundBinaryOperatorKind::Equality => MinskValue::Boolean(left == right),
                    BoundBinaryOperatorKind::Inequality => MinskValue::Boolean(left != right),
                    BoundBinaryOperatorKind::LogicalAnd => MinskValue::Boolean(
                        left.as_boolean().unwrap() && right.as_boolean().unwrap(),
                    ),
                    BoundBinaryOperatorKind::LogicalOr => MinskValue::Boolean(
                        left.as_boolean().unwrap() || right.as_boolean().unwrap(),
                    ),
                }
            }
        }
    }
}
