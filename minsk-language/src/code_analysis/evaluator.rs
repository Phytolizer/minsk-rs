use std::collections::HashMap;

use super::{minsk_value::MinskValue, variable_symbol::VariableSymbol};

use super::binding::{
    bound_binary_operator_kind::BoundBinaryOperatorKind, bound_expression::BoundExpression,
    bound_unary_operator_kind::BoundUnaryOperatorKind,
};

pub struct Evaluator<'compilation> {
    variables: &'compilation mut HashMap<VariableSymbol, MinskValue>,
}

impl<'compilation> Evaluator<'compilation> {
    pub fn new(variables: &'compilation mut HashMap<VariableSymbol, MinskValue>) -> Self {
        Self { variables }
    }

    pub fn evaluate(&mut self, root: &BoundExpression) -> MinskValue {
        self.evaluate_expression(root)
    }

    fn evaluate_expression(&mut self, root: &BoundExpression) -> MinskValue {
        match root {
            BoundExpression::Literal(lit) => lit.value.clone(),
            BoundExpression::Unary(u) => {
                let operand = self.evaluate_expression(&u.operand);
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
            BoundExpression::Binary(b) => {
                let left = self.evaluate_expression(&b.left);
                let right = self.evaluate_expression(&b.right);
                match b.op.kind {
                    BoundBinaryOperatorKind::Addition => MinskValue::Integer(
                        left.as_integer()
                            .unwrap()
                            .wrapping_add(right.as_integer().unwrap()),
                    ),
                    BoundBinaryOperatorKind::Subtraction => MinskValue::Integer(
                        left.as_integer()
                            .unwrap()
                            .wrapping_sub(right.as_integer().unwrap()),
                    ),
                    BoundBinaryOperatorKind::Multiplication => MinskValue::Integer(
                        left.as_integer()
                            .unwrap()
                            .wrapping_mul(right.as_integer().unwrap()),
                    ),
                    BoundBinaryOperatorKind::Division => MinskValue::Integer(
                        left.as_integer()
                            .unwrap()
                            .wrapping_div(right.as_integer().unwrap()),
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
            BoundExpression::Variable(v) => self.variables.get(&v.variable).unwrap().clone(),
            BoundExpression::Assignment(a) => {
                let value = self.evaluate_expression(&a.expression);
                self.variables.insert(a.variable.clone(), value.clone());
                value
            }
        }
    }
}
