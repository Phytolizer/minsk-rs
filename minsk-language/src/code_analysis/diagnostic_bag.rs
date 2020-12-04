use super::{
    diagnostic::Diagnostic, minsk_type::MinskType, syntax::syntax_kind::SyntaxKind,
    text_span::TextSpan,
};

#[derive(Debug)]
pub(super) struct DiagnosticBag {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticBag {
    pub fn new() -> Self {
        Self {
            diagnostics: vec![],
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Diagnostic> + '_ {
        self.diagnostics.iter().cloned()
    }

    pub fn report<S: AsRef<str>>(&mut self, span: TextSpan, message: S) {
        self.diagnostics.push(Diagnostic {
            span,
            message: message.as_ref().to_string(),
        })
    }

    pub fn report_invalid_number(&mut self, span: TextSpan, text: &str, ty: MinskType) {
        self.report(span, format!("The number {} isn't a valid {}.", text, ty));
    }

    pub fn report_bad_character(&mut self, position: usize, c: char) {
        self.report(
            TextSpan {
                start: position,
                end: position + 1,
            },
            format!("bad character input: '{}'", c),
        )
    }

    pub(crate) fn report_unexpected_token(
        &mut self,
        span: TextSpan,
        actual_kind: SyntaxKind,
        expected_kind: SyntaxKind,
    ) {
        let message = format!(
            "Unexpected token <{}>, expected <{}>",
            actual_kind, expected_kind
        );
        self.report(span, message);
    }

    pub(crate) fn report_undefined_unary_operator(
        &mut self,
        span: TextSpan,
        operator_text: &str,
        ty: MinskType,
    ) {
        let message = format!(
            "Unary operator '{}' is not defined for type {}",
            operator_text, ty
        );
        self.report(span, message);
    }

    pub(crate) fn report_undefined_binary_operator(
        &mut self,
        span: TextSpan,
        operator_text: &str,
        left_ty: MinskType,
        right_ty: MinskType,
    ) {
        let message = format!(
            "Binary operator '{}' is not defined for types {} and {}",
            operator_text, left_ty, right_ty
        );
        self.report(span, message);
    }

    pub(crate) fn report_undefined_name(&mut self, span: TextSpan, name: &str) {
        let message = format!("Variable '{}' doesn't exist", name);
        self.report(span, message);
    }
}
