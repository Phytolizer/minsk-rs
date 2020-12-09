use crate::code_analysis::variable_symbol::VariableSymbol;

use super::{bound_expression::BoundExpression, bound_statement::BoundStatement};

#[derive(Debug)]
pub struct BoundForStatement {
    variable: VariableSymbol,
    lower_bound: BoundExpression,
    upper_bound: BoundExpression,
    body: Box<BoundStatement>,
}

impl BoundForStatement {
    pub(crate) fn new(
        variable: VariableSymbol,
        lower_bound: BoundExpression,
        upper_bound: BoundExpression,
        body: Box<BoundStatement>,
    ) -> Self {
        Self {
            variable,
            lower_bound,
            upper_bound,
            body,
        }
    }

    pub(crate) fn variable(&self) -> &VariableSymbol {
        &self.variable
    }

    pub(crate) fn lower_bound(&self) -> &BoundExpression {
        &self.lower_bound
    }

    pub(crate) fn upper_bound(&self) -> &BoundExpression {
        &self.upper_bound
    }

    pub(crate) fn body(&self) -> &BoundStatement {
        &self.body
    }
}
