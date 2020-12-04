use crate::code_analysis::minsk_type::MinskType;

use super::bound_expression::BoundExpression;

#[derive(Debug)]
pub struct BoundAssignmentExpression {
    pub(crate) name: String,
    pub(crate) expression: Box<BoundExpression>,
}

impl BoundAssignmentExpression {
    pub(super) fn kind(&self) -> MinskType {
        self.expression.kind()
    }
}
