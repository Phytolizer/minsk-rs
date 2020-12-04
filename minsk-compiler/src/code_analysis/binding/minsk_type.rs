use std::fmt::Display;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(super) enum MinskType {
    Integer,
}

impl Display for MinskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
