use super::super::{minsk_type::MinskType, minsk_value::MinskValue};

#[derive(Debug)]
pub struct BoundLiteralExpression {
    pub(crate) value: MinskValue,
}

impl BoundLiteralExpression {
    pub(super) fn kind(&self) -> MinskType {
        match self.value {
            MinskValue::Integer(_) => MinskType::Integer,
            MinskValue::Boolean(_) => MinskType::Boolean,
            MinskValue::Null => MinskType::Null,
        }
    }
}
