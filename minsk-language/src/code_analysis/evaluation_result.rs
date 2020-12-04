use super::minsk_value::MinskValue;

pub enum EvaluationResult {
    Error(Vec<String>),
    Value(MinskValue),
}
