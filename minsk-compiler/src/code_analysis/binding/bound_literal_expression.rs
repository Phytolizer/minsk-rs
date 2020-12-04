use crate::minsk_value::MinskValue;

use super::minsk_type::MinskType;

#[derive(Debug)]
pub(crate) struct BoundLiteralExpression {
    pub(crate) value: MinskValue,
}

impl BoundLiteralExpression {
    pub(super) fn kind(&self) -> MinskType {
        match self.value {
            MinskValue::Integer(_) => MinskType::Integer,
        }
    }
}
