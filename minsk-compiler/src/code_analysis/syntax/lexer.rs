use crate::minsk_value::MinskValue;

use super::{syntax_kind::SyntaxKind, syntax_token::SyntaxToken};

pub(super) struct Lexer {
    text: String,
    position: usize,
    diagnostics: Vec<String>,
}

impl Lexer {
    pub(super) fn new(text: String) -> Self {
        Self {
            text,
            position: 0,
            diagnostics: vec![],
        }
    }

    fn current(&self) -> char {
        self.text.chars().nth(self.position).unwrap_or('\0')
    }
    fn next(&mut self) {
        self.position += 1;
    }

    pub(super) fn diagnostics(self) -> Vec<String> {
        self.diagnostics
    }

    pub(crate) fn next_token(&mut self) -> SyntaxToken {
        match self.current() {
            '\0' => SyntaxToken {
                kind: SyntaxKind::EndOfFile,
                position: self.position,
                text: String::new(),
                value: None,
            },
            d if d.is_numeric() => {
                let start = self.position;
                while self.current().is_numeric() {
                    self.next();
                }
                let text = &self.text[start..self.position];
                let value = match text.parse::<i32>() {
                    Ok(i) => Some(MinskValue::Integer(i)),
                    Err(_) => {
                        self.diagnostics.push(format!(
                            "ERROR: The number {} cannot be represented by a 32-bit signed integer",
                            text
                        ));
                        None
                    }
                };
                SyntaxToken {
                    kind: SyntaxKind::Number,
                    position: start,
                    text: text.to_string(),
                    value,
                }
            }
            d if d.is_whitespace() => {
                let start = self.position;
                while self.current().is_whitespace() {
                    self.next();
                }
                let text = &self.text[start..self.position];
                SyntaxToken {
                    kind: SyntaxKind::Whitespace,
                    position: start,
                    text: text.to_string(),
                    value: None,
                }
            }
            '+' => self.simple_token(SyntaxKind::Plus, 1),
            '-' => self.simple_token(SyntaxKind::Minus, 1),
            '*' => self.simple_token(SyntaxKind::Star, 1),
            '/' => self.simple_token(SyntaxKind::Slash, 1),
            '(' => self.simple_token(SyntaxKind::OpenParenthesis, 1),
            ')' => self.simple_token(SyntaxKind::CloseParenthesis, 1),
            _ => {
                self.diagnostics
                    .push(format!("ERROR: bad character input: {}", self.current()));
                self.simple_token(SyntaxKind::BadToken, 1)
            }
        }
    }

    fn simple_token(&mut self, kind: SyntaxKind, size: usize) -> SyntaxToken {
        self.position += size;
        SyntaxToken {
            kind,
            position: self.position - size,
            text: self.text[self.position - size..self.position].to_string(),
            value: None,
        }
    }
}
