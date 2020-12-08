use std::{collections::HashMap, sync::Arc};

use super::{
    binding::{binder::Binder, bound_global_scope::BoundGlobalScope},
    evaluation_result::EvaluationResult,
    evaluator::Evaluator,
    minsk_value::MinskValue,
    syntax::syntax_tree::SyntaxTree,
    variable_symbol::VariableSymbol,
};

#[derive(Clone)]
pub struct Compilation {
    syntax_tree: SyntaxTree,
    previous: Option<Box<Compilation>>,
    global_scope: Option<Arc<BoundGlobalScope>>,
}

impl Compilation {
    fn new_internal(previous: Option<Compilation>, syntax_tree: SyntaxTree) -> Self {
        Self {
            syntax_tree,
            previous: previous.map(Box::new),
            global_scope: None,
        }
    }
    pub fn new(syntax_tree: SyntaxTree) -> Self {
        Self::new_internal(None, syntax_tree)
    }
    pub fn evaluate(
        &mut self,
        variables: &mut HashMap<VariableSymbol, MinskValue>,
    ) -> EvaluationResult {
        let mut diagnostics = self.syntax_tree.diagnostics().collect::<Vec<_>>();
        diagnostics.append(&mut self.global_scope().diagnostics().collect::<Vec<_>>());
        if !diagnostics.is_empty() {
            return Err(diagnostics);
        }
        let value = Evaluator::new(variables).evaluate(self.global_scope().statement());
        Ok(value)
    }

    pub fn continue_with(self, syntax_tree: SyntaxTree) -> Self {
        Compilation::new_internal(Some(self), syntax_tree)
    }

    pub(crate) fn global_scope(&mut self) -> Arc<BoundGlobalScope> {
        if self.global_scope.is_none() {
            self.global_scope = Some(Arc::new(Binder::bind_global_scope(
                self.previous.as_mut().map(|p| p.global_scope()),
                self.syntax_tree.root(),
            )));
        }

        self.global_scope.clone().unwrap()
    }
}
