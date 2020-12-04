use crate::code_analysis::{
    diagnostic_bag::DiagnosticBag, minsk_type::MinskType, text_span::TextSpan,
};

use super::super::minsk_value::MinskValue;
use super::{syntax_facts::SyntaxFacts, syntax_kind::SyntaxKind, syntax_token::SyntaxToken};

pub(super) struct Lexer {
    text: String,
    position: usize,
    diagnostics: DiagnosticBag,
}

impl Lexer {
    pub(super) fn new(text: String) -> Self {
        Self {
            text,
            position: 0,
            diagnostics: DiagnosticBag::new(),
        }
    }

    fn lookahead(&self) -> char {
        self.text.chars().nth(self.position + 1).unwrap_or('\0')
    }
    fn current(&self) -> char {
        self.text.chars().nth(self.position).unwrap_or('\0')
    }
    fn next(&mut self) {
        self.position += 1;
    }

    pub(super) fn diagnostics(self) -> DiagnosticBag {
        self.diagnostics
    }

    pub(crate) fn next_token(&mut self) -> SyntaxToken {
        match self.current() {
            '\0' => SyntaxToken::new(SyntaxKind::EndOfFile, self.position, String::new(), None),
            d if d.is_numeric() => {
                let start = self.position;
                while self.current().is_numeric() {
                    self.next();
                }
                let text = &self.text[start..self.position];
                let value = match text.parse::<i32>() {
                    Ok(i) => Some(MinskValue::Integer(i)),
                    Err(_) => {
                        self.diagnostics.report_invalid_number(
                            TextSpan {
                                start,
                                end: self.position,
                            },
                            text,
                            MinskType::Integer,
                        );
                        None
                    }
                };
                SyntaxToken::new(SyntaxKind::Number, start, text.to_string(), value)
            }
            w if w.is_whitespace() => {
                let start = self.position;
                while self.current().is_whitespace() {
                    self.next();
                }
                let text = &self.text[start..self.position];
                SyntaxToken::new(SyntaxKind::Whitespace, start, text.to_string(), None)
            }
            l if l.is_alphabetic() => {
                let start = self.position;
                while self.current().is_alphabetic() {
                    self.next();
                }
                let text = &self.text[start..self.position];
                let kind = SyntaxFacts::keyword_kind(text);
                SyntaxToken::new(kind, start, text.to_string(), None)
            }
            '+' => self.simple_token(SyntaxKind::Plus, 1),
            '-' => self.simple_token(SyntaxKind::Minus, 1),
            '*' => self.simple_token(SyntaxKind::Star, 1),
            '/' => self.simple_token(SyntaxKind::Slash, 1),
            '(' => self.simple_token(SyntaxKind::OpenParenthesis, 1),
            ')' => self.simple_token(SyntaxKind::CloseParenthesis, 1),
            '!' => {
                if self.lookahead() == '=' {
                    self.simple_token(SyntaxKind::BangEquals, 2)
                } else {
                    self.simple_token(SyntaxKind::Bang, 1)
                }
            }
            '&' if self.lookahead() == '&' => self.simple_token(SyntaxKind::AmpersandAmpersand, 2),
            '|' if self.lookahead() == '|' => self.simple_token(SyntaxKind::PipePipe, 2),
            '=' if self.lookahead() == '=' => self.simple_token(SyntaxKind::EqualsEquals, 2),
            _ => {
                self.diagnostics
                    .report_bad_character(self.position, self.current());
                self.simple_token(SyntaxKind::BadToken, 1)
            }
        }
    }

    fn simple_token(&mut self, kind: SyntaxKind, size: usize) -> SyntaxToken {
        self.position += size;
        SyntaxToken::new(
            kind,
            self.position - size,
            self.text[self.position - size..self.position].to_string(),
            None,
        )
    }
}
