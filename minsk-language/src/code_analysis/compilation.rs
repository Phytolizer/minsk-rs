use super::{
    binding::binder::Binder, evaluation_result::EvaluationResult, evaluator::Evaluator,
    syntax::syntax_tree::SyntaxTree,
};

pub struct Compilation;

impl Compilation {
    pub fn evaluate(syntax: SyntaxTree) -> EvaluationResult {
        let mut binder = Binder::new();
        let bound_expression = binder.bind(syntax.root());
        let mut diagnostics = syntax.diagnostics().to_vec();
        diagnostics.append(&mut binder.diagnostics().to_vec());
        if diagnostics.len() > 0 {
            return EvaluationResult::Error(diagnostics);
        }
        let value = Evaluator::evaluate(&bound_expression);
        EvaluationResult::Value(value)
    }
}
