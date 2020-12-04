use std::fmt::Display;

#[derive(Debug, Clone, Eq)]
pub enum MinskValue {
    Integer(i32),
    Boolean(bool),
    Null,
}

impl MinskValue {
    pub(crate) fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    pub(crate) fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
    }

    pub(crate) fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    pub(crate) fn as_integer(&self) -> Option<i32> {
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

impl PartialEq for MinskValue {
    fn eq(&self, other: &Self) -> bool {
        match self {
            MinskValue::Integer(i) => other.is_integer() && other.as_integer().unwrap() == *i,
            MinskValue::Boolean(b) => other.is_boolean() && other.as_boolean().unwrap() == *b,
            MinskValue::Null => other.is_null(),
        }
    }
}

impl Display for MinskValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::Null => write!(f, "null"),
        }
    }
}
