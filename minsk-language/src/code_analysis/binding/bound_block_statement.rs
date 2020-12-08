use super::bound_statement::BoundStatement;

#[derive(Debug)]
pub struct BoundBlockStatement {
    statements: Vec<BoundStatement>,
}

impl BoundBlockStatement {
    pub(crate) fn new(statements: Vec<BoundStatement>) -> Self {
        Self { statements }
    }

    pub(crate) fn statements(&self) -> &[BoundStatement] {
        &self.statements
    }
}
