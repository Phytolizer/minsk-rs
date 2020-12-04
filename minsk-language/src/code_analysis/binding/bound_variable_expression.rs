use crate::code_analysis::{minsk_type::MinskType, variable_symbol::VariableSymbol};

#[derive(Debug)]
pub struct BoundVariableExpression {
    pub(crate) variable: VariableSymbol,
}

impl BoundVariableExpression {
    pub(super) fn kind(&self) -> MinskType {
        self.variable.ty
    }
}
