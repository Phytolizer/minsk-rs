use super::{bound_expression::BoundExpression, bound_unary_operator_kind::BoundUnaryOperatorKind};

#[derive(Debug)]
pub(crate) struct BoundUnaryExpression {
    pub(crate) operator_kind: BoundUnaryOperatorKind,
    pub(crate) operand: Box<BoundExpression>,
}

impl BoundUnaryExpression {
    pub(super) fn kind(&self) -> super::minsk_type::MinskType {
        self.operand.kind()
    }
}
