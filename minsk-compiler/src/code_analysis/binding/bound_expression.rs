use std::fmt::Debug;

use super::{
    bound_binary_expression::BoundBinaryExpression,
    bound_literal_expression::BoundLiteralExpression, bound_unary_expression::BoundUnaryExpression,
    minsk_type::MinskType,
};

#[derive(Debug)]
pub(crate) enum BoundExpression {
    BoundBinaryExpression(BoundBinaryExpression),
    BoundLiteralExpression(BoundLiteralExpression),
    BoundUnaryExpression(BoundUnaryExpression),
}

impl BoundExpression {
    pub(super) fn kind(&self) -> MinskType {
        match self {
            BoundExpression::BoundBinaryExpression(b) => b.kind(),
            BoundExpression::BoundLiteralExpression(l) => l.kind(),
            BoundExpression::BoundUnaryExpression(u) => u.kind(),
        }
    }
}
