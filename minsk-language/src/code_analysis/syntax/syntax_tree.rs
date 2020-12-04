use crate::code_analysis::{diagnostic::Diagnostic, diagnostic_bag::DiagnosticBag};

use super::{parser::Parser, syntax_node::SyntaxNode, syntax_token::SyntaxToken};

#[derive(Debug)]
pub struct SyntaxTree {
    pub(super) root: SyntaxNode,
    pub(super) end_of_file_token: SyntaxToken,
    pub(super) diagnostics: DiagnosticBag,
}

impl SyntaxTree {
    pub fn parse(text: String) -> Self {
        let parser = Parser::new(text);
        parser.parse()
    }

    pub fn diagnostics(&self) -> impl Iterator<Item = Diagnostic> + '_ {
        self.diagnostics.iter()
    }

    pub fn root(&self) -> &SyntaxNode {
        &self.root
    }
}
