use super::{diagnostic::Diagnostic, minsk_value::MinskValue};

pub type EvaluationResult = Result<MinskValue, Vec<Diagnostic>>;
