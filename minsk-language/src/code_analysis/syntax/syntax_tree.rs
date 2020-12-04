use super::{parser::Parser, syntax_node::SyntaxNode, syntax_token::SyntaxToken};

#[derive(Debug)]
pub struct SyntaxTree {
    pub(super) root: SyntaxNode,
    pub(super) end_of_file_token: SyntaxToken,
    pub(super) diagnostics: Vec<String>,
}

impl SyntaxTree {
    pub fn parse(text: String) -> Self {
        let parser = Parser::new(text);
        parser.parse()
    }

    pub fn diagnostics(&self) -> &[String] {
        &self.diagnostics
    }

    pub fn root(&self) -> &SyntaxNode {
        &self.root
    }
}
