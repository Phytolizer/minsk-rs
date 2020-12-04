use std::fmt::Display;

use super::minsk_type::MinskType;

#[derive(Debug, Clone, Eq)]
pub enum MinskValue {
    Integer(i32),
    Boolean(bool),
}

impl MinskValue {
    pub(crate) fn kind(&self) -> MinskType {
        match self {
            MinskValue::Integer(_) => MinskType::Integer,
            MinskValue::Boolean(_) => MinskType::Boolean,
        }
    }

    pub(crate) fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    pub(crate) fn is_boolean(&self) -> bool {
        matches!(self, Self::Boolean(_))
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
