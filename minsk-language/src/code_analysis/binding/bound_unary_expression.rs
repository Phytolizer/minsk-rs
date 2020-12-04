use super::super::minsk_type::MinskType;

use super::{bound_expression::BoundExpression, bound_unary_operator::BoundUnaryOperator};

#[derive(Debug)]
pub struct BoundUnaryExpression {
    pub(crate) op: BoundUnaryOperator,
    pub(crate) operand: Box<BoundExpression>,
}

impl BoundUnaryExpression {
    pub(super) fn kind(&self) -> MinskType {
        self.op.result_type
    }
}
