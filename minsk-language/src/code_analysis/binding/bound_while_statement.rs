use super::{bound_expression::BoundExpression, bound_statement::BoundStatement};

#[derive(Debug)]
pub struct BoundWhileStatement {
    condition: BoundExpression,
    body: Box<BoundStatement>,
}

impl BoundWhileStatement {
    pub(crate) fn new(condition: BoundExpression, body: Box<BoundStatement>) -> Self {
        Self { condition, body }
    }

    pub(crate) fn condition(&self) -> &BoundExpression {
        &self.condition
    }

    pub(crate) fn body(&self) -> &BoundStatement {
        &self.body
    }
}
