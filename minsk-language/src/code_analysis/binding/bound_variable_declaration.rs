use crate::code_analysis::variable_symbol::VariableSymbol;

use super::bound_expression::BoundExpression;

#[derive(Debug)]
pub struct BoundVariableDeclaration {
    variable: VariableSymbol,
    initializer: BoundExpression,
}

impl BoundVariableDeclaration {
    pub(super) fn new(variable: VariableSymbol, initializer: BoundExpression) -> Self {
        Self {
            variable,
            initializer,
        }
    }

    pub(crate) fn variable(&self) -> &VariableSymbol {
        &self.variable
    }

    pub(crate) fn initializer(&self) -> &BoundExpression {
        &self.initializer
    }
}
