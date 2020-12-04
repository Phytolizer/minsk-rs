use crate::code_analysis::minsk_type::MinskType;

#[derive(Debug)]
pub struct BoundVariableExpression {
    pub(crate) name: String,
    pub(crate) ty: MinskType,
}

impl BoundVariableExpression {
    pub(super) fn kind(&self) -> MinskType {
        self.ty
    }
}
