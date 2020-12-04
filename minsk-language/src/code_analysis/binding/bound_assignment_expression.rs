use crate::code_analysis::{minsk_type::MinskType, variable_symbol::VariableSymbol};

use super::bound_expression::BoundExpression;

#[derive(Debug)]
pub struct BoundAssignmentExpression {
    pub(crate) variable: VariableSymbol,
    pub(crate) expression: Box<BoundExpression>,
}

impl BoundAssignmentExpression {
    pub(super) fn kind(&self) -> MinskType {
        self.expression.kind()
    }
}
