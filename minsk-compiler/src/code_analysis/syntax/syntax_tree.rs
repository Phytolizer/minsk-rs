use super::{syntax_node::SyntaxNode, syntax_token::SyntaxToken};

#[derive(Debug)]
pub(crate) struct SyntaxTree {
    root: SyntaxNode,
    end_of_file_token: SyntaxToken,
    diagnostics: Vec<String>,
}

impl SyntaxTree {
    pub(crate) fn new(
        root: SyntaxNode,
        end_of_file_token: SyntaxToken,
        diagnostics: Vec<String>,
    ) -> Self {
        Self {
            root,
            end_of_file_token,
            diagnostics,
        }
    }

    pub(crate) fn diagnostics(&self) -> &[String] {
        &self.diagnostics
    }

    pub(crate) fn root(&self) -> &SyntaxNode {
        &self.root
    }
}
