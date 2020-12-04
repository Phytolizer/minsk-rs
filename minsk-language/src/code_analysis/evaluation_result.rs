use super::{diagnostic::Diagnostic, minsk_value::MinskValue};

pub enum EvaluationResult {
    Error(Vec<Diagnostic>),
    Value(MinskValue),
}
