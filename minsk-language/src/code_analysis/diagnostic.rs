use std::fmt::Display;

use super::text_span::TextSpan;

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub span: TextSpan,
    pub message: String,
}

impl Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
