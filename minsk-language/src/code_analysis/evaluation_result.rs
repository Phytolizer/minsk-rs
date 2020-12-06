use super::{diagnostic::Diagnostic, minsk_value::MinskValue};

#[derive(Debug)]
pub enum EvaluationResult {
    Error(Vec<Diagnostic>),
    Value(MinskValue),
}
