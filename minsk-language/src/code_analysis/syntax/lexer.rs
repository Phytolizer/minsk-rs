use crate::code_analysis::{
    diagnostic_bag::DiagnosticBag, minsk_type::MinskType, text_span::TextSpan,
};

use super::super::minsk_value::MinskValue;
use super::{syntax_facts::SyntaxFacts, syntax_kind::SyntaxKind, syntax_token::SyntaxToken};

pub(super) struct Lexer {
    text: String,
    start: usize,
    position: usize,
    diagnostics: DiagnosticBag,
}

impl Lexer {
    pub(super) fn new(text: String) -> Self {
        Self {
            text,
            start: 0,
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
        self.start = self.position;
        match self.current() {
            '\0' => SyntaxToken::new(SyntaxKind::EndOfFile, self.position, String::new(), None),
            d if d.is_numeric() => {
                while self.current().is_numeric() {
                    self.next();
                }
                let text = &self.text[self.start..self.position];
                let value = match text.parse::<i32>() {
                    Ok(i) => Some(MinskValue::Integer(i)),
                    Err(_) => {
                        self.diagnostics.report_invalid_number(
                            TextSpan {
                                start: self.start,
                                end: self.position,
                            },
                            text,
                            MinskType::Integer,
                        );
                        None
                    }
                };
                SyntaxToken::new(SyntaxKind::Number, self.start, text.to_string(), value)
            }
            w if w.is_whitespace() => {
                while self.current().is_whitespace() {
                    self.next();
                }
                let text = &self.text[self.start..self.position];
                SyntaxToken::new(SyntaxKind::Whitespace, self.start, text.to_string(), None)
            }
            l if l.is_alphabetic() => {
                while self.current().is_alphabetic() {
                    self.next();
                }
                let text = &self.text[self.start..self.position];
                let kind = SyntaxFacts::keyword_kind(text);
                SyntaxToken::new(kind, self.start, text.to_string(), None)
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
            '=' => {
                if self.lookahead() == '=' {
                    self.simple_token(SyntaxKind::EqualsEquals, 2)
                } else {
                    self.simple_token(SyntaxKind::Equals, 1)
                }
            }
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
            self.start,
            self.text[self.position - size..self.position].to_string(),
            None,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::code_analysis::syntax::syntax_tree::SyntaxTree;
    use itertools::Itertools;
    use pretty_assertions::assert_eq;

    use super::*;

    fn lex_token(kind: SyntaxKind, text: &str) {
        let tokens = SyntaxTree::parse_tokens(text.to_string());

        assert_eq!(1, tokens.len());
        assert_eq!(kind, tokens[0].kind);
        assert_eq!(text, tokens[0].text);
    }

    fn lex_token_pair(t1kind: SyntaxKind, t1text: &str, t2kind: SyntaxKind, t2text: &str) {
        let tokens = SyntaxTree::parse_tokens(String::new() + t1text + t2text);

        dbg!(tokens.iter().map(|t| t.kind).collect::<Vec<_>>());
        assert_eq!(2, tokens.len());
        assert_eq!(t1kind, tokens[0].kind);
        assert_eq!(t1text, tokens[0].text);
        assert_eq!(t2kind, tokens[1].kind);
        assert_eq!(t2text, tokens[1].text);
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

        assert_eq!(3, tokens.len());
        assert_eq!(t1kind, tokens[0].kind);
        assert_eq!(t1text, tokens[0].text);
        assert_eq!(separator_kind, tokens[1].kind);
        assert_eq!(separator_text, tokens[1].text);
        assert_eq!(t2kind, tokens[2].kind);
        assert_eq!(t2text, tokens[2].text);
    }

    fn get_tokens() -> Vec<(SyntaxKind, &'static str)> {
        vec![
            // single representation
            (SyntaxKind::FalseKeyword, "false"),
            (SyntaxKind::TrueKeyword, "true"),
            (SyntaxKind::CloseParenthesis, ")"),
            (SyntaxKind::OpenParenthesis, "("),
            (SyntaxKind::BangEquals, "!="),
            (SyntaxKind::EqualsEquals, "=="),
            (SyntaxKind::PipePipe, "||"),
            (SyntaxKind::AmpersandAmpersand, "&&"),
            (SyntaxKind::Equals, "="),
            (SyntaxKind::Bang, "!"),
            (SyntaxKind::Slash, "/"),
            (SyntaxKind::Star, "*"),
            (SyntaxKind::Minus, "-"),
            (SyntaxKind::Plus, "+"),
            // multiple possible representations
            (SyntaxKind::Identifier, "a"),
            (SyntaxKind::Identifier, "abc"),
            (SyntaxKind::Number, "1"),
            (SyntaxKind::Number, "123"),
        ]
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

        if t1kind == SyntaxKind::Identifier && t2kind == SyntaxKind::Identifier {
            return true;
        } else if t1_is_keyword && t2_is_keyword {
            return true;
        } else if t1_is_keyword && t2kind == SyntaxKind::Identifier
            || t2_is_keyword && t1kind == SyntaxKind::Identifier
        {
            return true;
        } else if t1kind == SyntaxKind::Whitespace && t2kind == SyntaxKind::Whitespace {
            return true;
        } else if t1kind == SyntaxKind::Number && t2kind == SyntaxKind::Number {
            return true;
        } else if t1kind == SyntaxKind::Bang && t2kind == SyntaxKind::Equals {
            return true;
        } else if t1kind == SyntaxKind::Equals && t2kind == SyntaxKind::Equals {
            return true;
        } else if t1kind == SyntaxKind::Equals && t2kind == SyntaxKind::EqualsEquals {
            return true;
        } else if t1kind == SyntaxKind::Bang && t2kind == SyntaxKind::EqualsEquals {
            return true;
        }

        false
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
                dbg!((t1text, t2text));
                lex_token_pair(t1kind, t1text, t2kind, t2text);
            }
        }
    }

    #[test]
    fn lexes_token_pairs_with_separators() {
        for (t1kind, t1text, separator_kind, separator_text, t2kind, t2text) in
            get_token_pairs_with_separators()
        {
            lex_token_pair_with_separator(t1kind, t1text, separator_kind, separator_text, t2kind, t2text);
        }
    }
}
