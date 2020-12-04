use std::fmt::Display;

use crate::minsk_value::MinskValue;

use super::syntax_kind::SyntaxKind;

#[derive(Debug, Clone)]
pub(crate) struct SyntaxToken {
    pub(crate) kind: SyntaxKind,
    pub(crate) position: usize,
    pub(crate) text: String,
    pub(crate) value: Option<MinskValue>,
}

impl Display for SyntaxToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: '{}'", self.kind, self.text)?;
        if let Some(literal) = &self.value {
            write!(f, " {}", literal)?;
        }
        Ok(())
    }
}
