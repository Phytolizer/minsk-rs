use std::{collections::HashMap, sync::Arc};

use parking_lot::RwLock;

use crate::code_analysis::variable_symbol::VariableSymbol;

pub(crate) struct BoundScope {
    variables: HashMap<String, VariableSymbol>,
    parent: Option<Arc<RwLock<BoundScope>>>,
}

impl BoundScope {
    pub(super) fn new(parent: Option<Arc<RwLock<BoundScope>>>) -> Self {
        Self {
            variables: HashMap::new(),
            parent,
        }
    }

    pub(super) fn parent(&self) -> Option<Arc<RwLock<BoundScope>>> {
        self.parent.clone()
    }

    pub(super) fn try_declare(&mut self, variable: VariableSymbol) -> bool {
        if self.variables.contains_key(variable.name()) {
            return false;
        }

        self.variables.insert(variable.name().to_string(), variable);
        true
    }

    pub(super) fn try_lookup(&self, name: &str) -> Option<VariableSymbol> {
        self.variables
            .get(name)
            .cloned()
            .or_else(|| self.parent.as_ref().and_then(|p| p.read().try_lookup(name)))
    }

    pub(super) fn declared_variables(&self) -> impl Iterator<Item = &VariableSymbol> {
        self.variables.values()
    }
}
