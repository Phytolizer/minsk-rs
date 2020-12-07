use std::sync::Arc;

use crate::code_analysis::{diagnostic::Diagnostic, variable_symbol::VariableSymbol};

use super::bound_expression::BoundExpression;

pub(crate) struct BoundGlobalScope {
    previous: Option<Arc<BoundGlobalScope>>,
    diagnostics: Vec<Diagnostic>,
    variables: Vec<VariableSymbol>,
    expression: BoundExpression,
}

impl BoundGlobalScope {
    pub(super) fn new(
        previous: Option<Arc<BoundGlobalScope>>,
        diagnostics: Vec<Diagnostic>,
        variables: Vec<VariableSymbol>,
        expression: BoundExpression,
    ) -> Self {
        Self {
            previous,
            diagnostics,
            variables,
            expression,
        }
    }

    pub(crate) fn previous(&self) -> &Option<Arc<BoundGlobalScope>> {
        &self.previous
    }

    pub(crate) fn diagnostics(&self) -> impl Iterator<Item = Diagnostic> + '_ {
        self.diagnostics.iter().cloned()
    }

    pub(crate) fn variables(&self) -> impl Iterator<Item = VariableSymbol> + '_ {
        self.variables.iter().cloned()
    }

    pub(crate) fn expression(&self) -> &BoundExpression {
        &self.expression
    }
}
