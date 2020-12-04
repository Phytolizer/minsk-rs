use std::fmt::Display;

#[derive(Debug, Clone)]
pub(crate) enum MinskValue {
    Integer(i32),
    Boolean(bool),
}

impl MinskValue {
    pub(crate) fn as_int(&self) -> Option<i32> {
        match self {
            Self::Integer(i) => Some(*i),
            _ => None,
        }
    }

    pub(crate) fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(b) => Some(*b),
            _ => None,
        }
    }
}

impl Display for MinskValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i),
            Self::Boolean(b) => write!(f, "{}", b),
        }
    }
}
