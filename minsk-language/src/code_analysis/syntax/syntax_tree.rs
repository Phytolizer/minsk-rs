use crate::code_analysis::{
    diagnostic::Diagnostic, diagnostic_bag::DiagnosticBag, text::source_text::SourceText,
};

use super::{
    compilation_unit::CompilationUnit, lexer::Lexer, parser::Parser, syntax_kind::SyntaxKind,
    syntax_token::SyntaxToken,
};

#[derive(Debug, Clone)]
pub struct SyntaxTree {
    text: SourceText,
    root: CompilationUnit,
    diagnostics: DiagnosticBag,
}

impl SyntaxTree {
    fn new(text: SourceText) -> Self {
        let mut parser = Parser::new(text.clone());
        let root = parser.parse_compilation_unit();
        let diagnostics = parser.diagnostics();
        Self {
            text,
            root,
            diagnostics,
        }
    }

    pub fn parse<ST: Into<SourceText>>(text: ST) -> Self {
        Self::new(text.into())
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

    pub fn root(&self) -> &CompilationUnit {
        &self.root
    }

    pub fn text(&self) -> &SourceText {
        &self.text
    }
}
