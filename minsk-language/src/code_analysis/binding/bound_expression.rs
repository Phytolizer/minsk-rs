use std::fmt::Debug;

use super::{
    super::minsk_type::MinskType, bound_assignment_expression::BoundAssignmentExpression,
    bound_variable_expression::BoundVariableExpression,
};

use super::{
    bound_binary_expression::BoundBinaryExpression,
    bound_literal_expression::BoundLiteralExpression, bound_unary_expression::BoundUnaryExpression,
};

#[derive(Debug)]
pub enum BoundExpression {
    Binary(BoundBinaryExpression),
    Literal(BoundLiteralExpression),
    Unary(BoundUnaryExpression),
    Variable(BoundVariableExpression),
    Assignment(BoundAssignmentExpression),
}

impl BoundExpression {
    pub(super) fn ty(&self) -> MinskType {
        match self {
            BoundExpression::Binary(b) => b.kind(),
            BoundExpression::Literal(l) => l.kind(),
            BoundExpression::Unary(u) => u.kind(),
            BoundExpression::Variable(v) => v.kind(),
            BoundExpression::Assignment(a) => a.kind(),
        }
    }
}
