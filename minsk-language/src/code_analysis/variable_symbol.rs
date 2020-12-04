use super::minsk_type::MinskType;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct VariableSymbol {
    pub(crate) name: String,
    pub(crate) ty: MinskType,
}
