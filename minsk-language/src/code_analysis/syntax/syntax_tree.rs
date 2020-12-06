use crate::code_analysis::{
    diagnostic::Diagnostic, diagnostic_bag::DiagnosticBag, text::source_text::SourceText,
};

use super::{
    lexer::Lexer, parser::Parser, syntax_kind::SyntaxKind, syntax_node::SyntaxNode,
    syntax_token::SyntaxToken,
};

#[derive(Debug)]
pub struct SyntaxTree {
    pub text: SourceText,
    pub(super) root: SyntaxNode,
    pub(super) end_of_file_token: SyntaxToken,
    pub(super) diagnostics: DiagnosticBag,
}

impl SyntaxTree {
    pub fn parse<ST: Into<SourceText>>(text: ST) -> Self {
        Parser::new(text.into()).parse()
    }

    pub fn parse_tokens<ST: Into<SourceText>>(text: ST) -> Vec<SyntaxToken> {
        let mut lexer = Lexer::new(text.into());
        let mut tokens = vec![];
        loop {
            let token = lexer.next_token();
            if token.kind == SyntaxKind::EndOfFile {
                break;
            }
            tokens.push(token);
        }
        tokens
    }

    pub fn diagnostics(&self) -> impl Iterator<Item = Diagnostic> + '_ {
        self.diagnostics.iter()
    }

    pub fn root(&self) -> &SyntaxNode {
        &self.root
    }
}
