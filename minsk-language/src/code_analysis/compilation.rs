use std::collections::HashMap;

use super::{
    binding::binder::Binder, evaluation_result::EvaluationResult, evaluator::Evaluator,
    minsk_value::MinskValue, syntax::syntax_tree::SyntaxTree,
};

pub struct Compilation;

impl Compilation {
    pub fn evaluate(
        syntax: SyntaxTree,
        variables: &mut HashMap<String, MinskValue>,
    ) -> EvaluationResult {
        let mut binder = Binder::new(variables);
        let bound_expression = binder.bind(syntax.root());
        let mut diagnostics = syntax.diagnostics().collect::<Vec<_>>();
        diagnostics.append(&mut binder.diagnostics().collect());
        if diagnostics.len() > 0 {
            return EvaluationResult::Error(diagnostics);
        }
        let value = Evaluator::new(variables).evaluate(&bound_expression);
        EvaluationResult::Value(value)
    }
}
