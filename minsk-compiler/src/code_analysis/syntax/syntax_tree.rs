use super::{parser::Parser, syntax_node::SyntaxNode, syntax_token::SyntaxToken};

#[derive(Debug)]
pub(crate) struct SyntaxTree {
    pub(crate) root: SyntaxNode,
    pub(crate) end_of_file_token: SyntaxToken,
    pub(crate) diagnostics: Vec<String>,
}

impl SyntaxTree {
    pub(crate) fn parse(text: String) -> Self {
        let parser = Parser::new(text);
        parser.parse()
    }

    pub(crate) fn diagnostics(&self) -> &[String] {
        &self.diagnostics
    }

    pub(crate) fn root(&self) -> &SyntaxNode {
        &self.root
    }
}
