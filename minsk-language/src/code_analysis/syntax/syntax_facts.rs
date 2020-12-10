use super::syntax_kind::SyntaxKind;

pub(super) trait SyntaxFactsExt {
    fn binary_operator_precedence(&self) -> usize;
    fn unary_operator_precedence(&self) -> usize;
}

pub(super) struct SyntaxFacts;

impl SyntaxFacts {
    pub(super) fn keyword_kind(text: &str) -> SyntaxKind {
        match text {
            "true" => SyntaxKind::TrueKeyword,
            "false" => SyntaxKind::FalseKeyword,
            "let" => SyntaxKind::LetKeyword,
            "var" => SyntaxKind::VarKeyword,
            "if" => SyntaxKind::IfKeyword,
            "else" => SyntaxKind::ElseKeyword,
            "while" => SyntaxKind::WhileKeyword,
            "for" => SyntaxKind::ForKeyword,
            "to" => SyntaxKind::ToKeyword,
            _ => SyntaxKind::Identifier,
        }
    }

    pub(super) fn get_text(kind: SyntaxKind) -> Option<&'static str> {
        match kind {
            SyntaxKind::Plus => Some("+"),
            SyntaxKind::Minus => Some("-"),
            SyntaxKind::Star => Some("*"),
            SyntaxKind::Slash => Some("/"),
            SyntaxKind::Bang => Some("!"),
            SyntaxKind::Equals => Some("="),
            SyntaxKind::AmpersandAmpersand => Some("&&"),
            SyntaxKind::PipePipe => Some("||"),
            SyntaxKind::EqualsEquals => Some("=="),
            SyntaxKind::BangEquals => Some("!="),
            SyntaxKind::Less => Some("<"),
            SyntaxKind::LessEquals => Some("<="),
            SyntaxKind::Greater => Some(">"),
            SyntaxKind::GreaterEquals => Some(">="),
            SyntaxKind::OpenParenthesis => Some("("),
            SyntaxKind::CloseParenthesis => Some(")"),
            SyntaxKind::OpenBrace => Some("{"),
            SyntaxKind::CloseBrace => Some("}"),
            SyntaxKind::Ampersand => Some("&"),
            SyntaxKind::Pipe => Some("|"),
            SyntaxKind::Hat => Some("^"),
            SyntaxKind::Tilde => Some("~"),
            SyntaxKind::FalseKeyword => Some("false"),
            SyntaxKind::TrueKeyword => Some("true"),
            SyntaxKind::LetKeyword => Some("let"),
            SyntaxKind::VarKeyword => Some("var"),
            SyntaxKind::IfKeyword => Some("if"),
            SyntaxKind::ElseKeyword => Some("else"),
            SyntaxKind::WhileKeyword => Some("while"),
            SyntaxKind::ForKeyword => Some("for"),
            SyntaxKind::ToKeyword => Some("to"),
            _ => None,
        }
    }
}

impl SyntaxFactsExt for SyntaxKind {
    fn binary_operator_precedence(&self) -> usize {
        match self {
            SyntaxKind::Star | SyntaxKind::Slash => 5,
            SyntaxKind::Plus | SyntaxKind::Minus => 4,
            SyntaxKind::EqualsEquals
            | SyntaxKind::BangEquals
            | SyntaxKind::Less
            | SyntaxKind::LessEquals
            | SyntaxKind::Greater
            | SyntaxKind::GreaterEquals => 3,
            SyntaxKind::Hat | SyntaxKind::Pipe | SyntaxKind::PipePipe => 2,
            SyntaxKind::Ampersand | SyntaxKind::AmpersandAmpersand => 1,
            _ => 0,
        }
    }

    fn unary_operator_precedence(&self) -> usize {
        match self {
            SyntaxKind::Tilde | SyntaxKind::Plus | SyntaxKind::Minus | SyntaxKind::Bang => 6,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::code_analysis::syntax::syntax_tree::SyntaxTree;

    use super::*;
    use pretty_assertions::assert_eq;
    use strum::IntoEnumIterator;
    #[test]
    fn get_text_round_trips() {
        for kind in SyntaxKind::iter() {
            let text = SyntaxFacts::get_text(kind);
            if let Some(text) = text {
                let tokens = SyntaxTree::parse_tokens(text.to_string());

                assert_eq!(1, tokens.len());
                let token = &tokens[0];

                assert_eq!(kind, token.kind);
                assert_eq!(text, token.text);
            }
        }
    }
}
