use std::collections::HashMap;

use super::{
    binding::binder::Binder, evaluation_result::EvaluationResult, evaluator::Evaluator,
    minsk_value::MinskValue, syntax::syntax_tree::SyntaxTree, variable_symbol::VariableSymbol,
};

pub struct Compilation;

impl Compilation {
    pub fn evaluate(
        syntax: &SyntaxTree,
        variables: &mut HashMap<VariableSymbol, MinskValue>,
    ) -> EvaluationResult {
        let mut binder = Binder::new(variables);
        let bound_expression = binder.bind(&syntax.root().root);
        let mut diagnostics = syntax.diagnostics().collect::<Vec<_>>();
        diagnostics.append(&mut binder.diagnostics().collect());
        if !diagnostics.is_empty() {
            return Err(diagnostics);
        }
        let value = Evaluator::new(variables).evaluate(&bound_expression);
        Ok(value)
    }
}
