use super::{
    bound_binary_operator_kind::BoundBinaryOperatorKind, bound_expression::BoundExpression,
};

#[derive(Debug)]
pub(crate) struct BoundBinaryExpression {
    pub(crate) left: Box<BoundExpression>,
    pub(crate) operator_kind: BoundBinaryOperatorKind,
    pub(crate) right: Box<BoundExpression>,
}

impl BoundBinaryExpression {
    pub(super) fn kind(&self) -> super::minsk_type::MinskType {
        self.left.kind()
    }
}
