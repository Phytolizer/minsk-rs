use super::{diagnostic::Diagnostic, minsk_value::MinskValue};

pub type EvaluationResult = Result<Option<MinskValue>, Vec<Diagnostic>>;
