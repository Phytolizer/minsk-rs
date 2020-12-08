use super::minsk_type::MinskType;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct VariableSymbol {
    name: String,
    read_only: bool,
    ty: MinskType,
}

impl VariableSymbol {
    pub(crate) fn new(name: String, read_only: bool, ty: MinskType) -> Self {
        Self {
            name,
            read_only,
            ty,
        }
    }

    pub(crate) fn ty(&self) -> MinskType {
        self.ty
    }

    pub(crate) fn read_only(&self) -> bool {
        self.read_only
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}
