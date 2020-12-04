use crate::minsk_type::MinskType;

use super::{bound_binary_operator::BoundBinaryOperator, bound_expression::BoundExpression};

#[derive(Debug)]
pub(crate) struct BoundBinaryExpression {
    pub(crate) left: Box<BoundExpression>,
    pub(crate) op: BoundBinaryOperator,
    pub(crate) right: Box<BoundExpression>,
}

impl BoundBinaryExpression {
    pub(super) fn kind(&self) -> MinskType {
        self.left.kind()
    }
}
