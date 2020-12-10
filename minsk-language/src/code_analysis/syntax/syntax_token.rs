use std::fmt::Display;

use crate::code_analysis::text::text_span::TextSpan;

use super::super::minsk_value::MinskValue;

use super::syntax_kind::SyntaxKind;

#[derive(Debug, Clone, PartialEq)]
pub struct SyntaxToken {
    pub(crate) kind: SyntaxKind,
    pub(crate) position: usize,
    pub(crate) text: String,
    pub(crate) value: Option<MinskValue>,
    pub(crate) span: TextSpan,
}

impl SyntaxToken {
    pub(crate) fn new(
        kind: SyntaxKind,
        position: usize,
        text: String,
        value: Option<MinskValue>,
    ) -> Self {
        Self {
            kind,
            position,
            span: TextSpan {
                start: position,
                end: position + text.chars().count(),
            },
            text,
            value,
        }
    }
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
