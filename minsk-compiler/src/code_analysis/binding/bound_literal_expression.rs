use crate::{minsk_type::MinskType, minsk_value::MinskValue};

#[derive(Debug)]
pub(crate) struct BoundLiteralExpression {
    pub(crate) value: MinskValue,
}

impl BoundLiteralExpression {
    pub(super) fn kind(&self) -> MinskType {
        match self.value {
            MinskValue::Integer(_) => MinskType::Integer,
            MinskValue::Boolean(_) => MinskType::Boolean,
        }
    }
}
