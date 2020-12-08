use crate::code_analysis::{
    diagnostic_bag::DiagnosticBag,
    minsk_type::MinskType,
    text::{source_text::SourceText, text_span::TextSpan},
};

use super::super::minsk_value::MinskValue;
use super::{syntax_facts::SyntaxFacts, syntax_kind::SyntaxKind, syntax_token::SyntaxToken};

pub(super) struct Lexer {
    text: SourceText,
    start: usize,
    position: usize,
    kind: SyntaxKind,
    value: Option<MinskValue>,
    diagnostics: DiagnosticBag,
}

impl Lexer {
    pub(super) fn new(text: SourceText) -> Self {
        Self {
            text,
            start: 0,
            position: 0,
            kind: SyntaxKind::BadToken,
            value: None,
            diagnostics: DiagnosticBag::new(),
        }
    }

    fn lookahead(&self) -> char {
        self.text.get(self.position + 1).unwrap_or('\0')
    }
    fn current(&self) -> char {
        self.text.get(self.position).unwrap_or('\0')
    }
    fn next(&mut self) {
        self.position += 1;
    }

    pub(super) fn diagnostics(self) -> DiagnosticBag {
        self.diagnostics
    }

    pub(crate) fn next_token(&mut self) -> SyntaxToken {
        self.start = self.position;
        self.kind = SyntaxKind::BadToken;
        self.value = None;
        match self.current() {
            '\0' => self.kind = SyntaxKind::EndOfFile,
            d if d.is_numeric() => self.read_number_token(),
            w if w.is_whitespace() => self.read_whitespace(),
            l if l.is_alphabetic() => self.read_identifier_or_keyword(),
            '+' => {
                self.kind = SyntaxKind::Plus;
                self.next();
            }
            '-' => {
                self.kind = SyntaxKind::Minus;
                self.next();
            }
            '*' => {
                self.kind = SyntaxKind::Star;
                self.next();
            }
            '/' => {
                self.kind = SyntaxKind::Slash;
                self.next();
            }
            '(' => {
                self.kind = SyntaxKind::OpenParenthesis;
                self.next();
            }
            ')' => {
                self.kind = SyntaxKind::CloseParenthesis;
                self.next();
            }
            '{' => {
                self.kind = SyntaxKind::OpenBrace;
                self.next();
            }
            '}' => {
                self.kind = SyntaxKind::CloseBrace;
                self.next();
            }
            '!' => {
                if self.lookahead() == '=' {
                    self.kind = SyntaxKind::BangEquals;
                    self.position += 2;
                } else {
                    self.kind = SyntaxKind::Bang;
                    self.next();
                }
            }
            '&' if self.lookahead() == '&' => {
                self.kind = SyntaxKind::AmpersandAmpersand;
                self.position += 2;
            }
            '|' if self.lookahead() == '|' => {
                self.kind = SyntaxKind::PipePipe;
                self.position += 2;
            }
            '=' => {
                if self.lookahead() == '=' {
                    self.kind = SyntaxKind::EqualsEquals;
                    self.position += 2;
                } else {
                    self.kind = SyntaxKind::Equals;
                    self.next();
                }
            }
            _ => {
                self.diagnostics
                    .report_bad_character(self.position, self.current());
                self.next();
            }
        }
        let text = match SyntaxFacts::get_text(self.kind) {
            Some(t) => t.to_string(),
            None => self.text[TextSpan {
                start: self.start,
                end: self.position,
            }]
            .iter()
            .collect::<String>(),
        };

        SyntaxToken::new(self.kind, self.start, text, self.value.clone())
    }

    fn read_number_token(&mut self) {
        while self.current().is_numeric() {
            self.next();
        }

        let text = self.text[TextSpan {
            start: self.start,
            end: self.position,
        }]
        .iter()
        .collect::<String>();
        self.value = match text.parse() {
            Ok(v) => Some(MinskValue::Integer(v)),
            Err(_) => {
                self.diagnostics.report_invalid_number(
                    TextSpan {
                        start: self.start,
                        end: self.position,
                    },
                    &text,
                    MinskType::Integer,
                );
                return;
            }
        };
        self.kind = SyntaxKind::Number;
    }

    fn read_whitespace(&mut self) {
        while self.current().is_whitespace() {
            self.next();
        }
        self.kind = SyntaxKind::Whitespace;
        self.value = None;
    }

    fn read_identifier_or_keyword(&mut self) {
        while self.current().is_alphabetic() {
            self.next();
        }
        let text = self.text[TextSpan {
            start: self.start,
            end: self.position,
        }]
        .iter()
        .collect::<String>();
        self.kind = SyntaxFacts::keyword_kind(&text);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::iter::FromIterator;

    use crate::code_analysis::syntax::syntax_tree::SyntaxTree;
    use itertools::Itertools;
    use spectral::prelude::*;
    use strum::IntoEnumIterator;

    use super::*;

    fn lex_token(kind: SyntaxKind, text: &str) {
        let tokens = SyntaxTree::parse_tokens(text.to_string());

        asserting!("tokens length").that(&tokens).has_length(1);
        asserting!("token kind")
            .that(&tokens[0].kind)
            .is_equal_to(&kind);
        asserting!("token text")
            .that(&tokens[0].text.as_str())
            .is_equal_to(&text);
    }

    fn lex_token_pair(t1kind: SyntaxKind, t1text: &str, t2kind: SyntaxKind, t2text: &str) {
        let tokens = SyntaxTree::parse_tokens(String::new() + t1text + t2text);

        asserting!("tokens length").that(&tokens).has_length(2);
        asserting!("token 1 kind")
            .that(&tokens[0].kind)
            .is_equal_to(t1kind);
        asserting!("token 1 text")
            .that(&tokens[0].text.as_str())
            .is_equal_to(&t1text);
        asserting!("token 2 kind")
            .that(&tokens[1].kind)
            .is_equal_to(t2kind);
        asserting!("token 2 text")
            .that(&tokens[1].text.as_str())
            .is_equal_to(&t2text);
    }

    fn lex_token_pair_with_separator(
        t1kind: SyntaxKind,
        t1text: &str,
        separator_kind: SyntaxKind,
        separator_text: &str,
        t2kind: SyntaxKind,
        t2text: &str,
    ) {
        let tokens = SyntaxTree::parse_tokens(String::new() + t1text + separator_text + t2text);

        asserting!("tokens length").that(&tokens).has_length(3);
        asserting!("token 1 kind")
            .that(&tokens[0].kind)
            .is_equal_to(&t1kind);
        asserting!("token 1 text")
            .that(&tokens[0].text.as_str())
            .is_equal_to(&t1text);
        asserting!("separator kind")
            .that(&tokens[1].kind)
            .is_equal_to(&separator_kind);
        asserting!("separator text")
            .that(&tokens[1].text.as_str())
            .is_equal_to(&separator_text);
        asserting!("token 2 kind")
            .that(&tokens[2].kind)
            .is_equal_to(&t2kind);
        asserting!("token 2 text")
            .that(&tokens[2].text.as_str())
            .is_equal_to(&t2text);
    }

    fn get_tokens() -> Vec<(SyntaxKind, &'static str)> {
        let fixed_tokens = SyntaxKind::iter()
            .map(|k| (k, SyntaxFacts::get_text(k)))
            .filter_map(|(k, t)| if let Some(t) = t { Some((k, t)) } else { None })
            .collect::<Vec<_>>();
        let dynamic_tokens = vec![
            (SyntaxKind::Identifier, "a"),
            (SyntaxKind::Identifier, "abc"),
            (SyntaxKind::Number, "1"),
            (SyntaxKind::Number, "123"),
        ];
        fixed_tokens.iter().cloned().chain(dynamic_tokens).collect()
    }

    fn get_separators() -> Vec<(SyntaxKind, &'static str)> {
        vec![
            (SyntaxKind::Whitespace, " "),
            (SyntaxKind::Whitespace, "  "),
            (SyntaxKind::Whitespace, "\r"),
            (SyntaxKind::Whitespace, "\n"),
            (SyntaxKind::Whitespace, "\r\n"),
        ]
    }

    fn get_token_pairs() -> Vec<(SyntaxKind, &'static str, SyntaxKind, &'static str)> {
        get_tokens()
            .iter()
            .cloned()
            .cartesian_product(get_tokens())
            .map(|((t1kind, t1text), (t2kind, t2text))| (t1kind, t1text, t2kind, t2text))
            .collect()
    }

    fn get_token_pairs_with_separators() -> Vec<(
        SyntaxKind,
        &'static str,
        SyntaxKind,
        &'static str,
        SyntaxKind,
        &'static str,
    )> {
        let mut token_pairs_with_separators = Vec::new();
        for (t1kind, t1text) in get_tokens() {
            for (t2kind, t2text) in get_tokens() {
                if requires_separator(t1kind, t2kind) {
                    for (separator_kind, separator_text) in get_separators() {
                        token_pairs_with_separators.push((
                            t1kind,
                            t1text,
                            separator_kind,
                            separator_text,
                            t2kind,
                            t2text,
                        ));
                    }
                }
            }
        }
        token_pairs_with_separators
    }

    fn requires_separator(t1kind: SyntaxKind, t2kind: SyntaxKind) -> bool {
        let t1_is_keyword = t1kind.to_string().ends_with("Keyword");
        let t2_is_keyword = t2kind.to_string().ends_with("Keyword");

        t1kind == SyntaxKind::Identifier && t2kind == SyntaxKind::Identifier
            || t1_is_keyword && t2_is_keyword
            || t1_is_keyword && t2kind == SyntaxKind::Identifier
            || t2_is_keyword && t1kind == SyntaxKind::Identifier
            || t1kind == SyntaxKind::Whitespace && t2kind == SyntaxKind::Whitespace
            || t1kind == SyntaxKind::Number && t2kind == SyntaxKind::Number
            || t1kind == SyntaxKind::Bang && t2kind == SyntaxKind::Equals
            || t1kind == SyntaxKind::Equals && t2kind == SyntaxKind::Equals
            || t1kind == SyntaxKind::Equals && t2kind == SyntaxKind::EqualsEquals
            || t1kind == SyntaxKind::Bang && t2kind == SyntaxKind::EqualsEquals
    }

    #[test]
    fn tests_all_token_kinds() {
        let all_token_kinds = HashSet::<SyntaxKind>::from_iter(SyntaxKind::iter());
        let tested_token_kinds = HashSet::from_iter(
            get_tokens()
                .iter()
                .cloned()
                .chain(get_separators())
                .map(|(k, _)| k),
        );

        let mut untested_token_kinds = HashSet::<SyntaxKind>::from_iter(
            all_token_kinds.difference(&tested_token_kinds).cloned(),
        );
        untested_token_kinds.remove(&SyntaxKind::BadToken);
        untested_token_kinds.remove(&SyntaxKind::EndOfFile);

        asserting!("all tokens tested")
            .that(&untested_token_kinds)
            .is_equal_to(HashSet::<SyntaxKind>::new());
    }

    #[test]
    fn lexes_token() {
        for (kind, text) in get_tokens() {
            lex_token(kind, text);
        }
    }

    #[test]
    fn lexes_token_pairs() {
        for (t1kind, t1text, t2kind, t2text) in get_token_pairs() {
            if t1kind != SyntaxKind::Whitespace && !requires_separator(t1kind, t2kind) {
                lex_token_pair(t1kind, t1text, t2kind, t2text);
            }
        }
    }

    #[test]
    fn lexes_token_pairs_with_separators() {
        for (t1kind, t1text, separator_kind, separator_text, t2kind, t2text) in
            get_token_pairs_with_separators()
        {
            lex_token_pair_with_separator(
                t1kind,
                t1text,
                separator_kind,
                separator_text,
                t2kind,
                t2text,
            );
        }
    }
}
