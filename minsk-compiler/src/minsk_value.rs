use std::fmt::Display;

#[derive(Debug, Clone)]
pub(crate) enum MinskValue {
    Integer(i32),
}

impl Display for MinskValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "Integer({})", i),
        }
    }
}
