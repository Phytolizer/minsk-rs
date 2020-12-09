use super::{bound_expression::BoundExpression, bound_statement::BoundStatement};

#[derive(Debug)]
pub struct BoundIfStatement {
    condition: BoundExpression,
    then_statement: Box<BoundStatement>,
    else_statement: Option<Box<BoundStatement>>,
}

impl BoundIfStatement {
    pub(crate) fn new(
        condition: BoundExpression,
        then_statement: Box<BoundStatement>,
        else_statement: Option<Box<BoundStatement>>,
    ) -> Self {
        Self {
            condition,
            then_statement,
            else_statement,
        }
    }

    pub(crate) fn condition(&self) -> &BoundExpression {
        &self.condition
    }

    pub(crate) fn then_statement(&self) -> &BoundStatement {
        &self.then_statement
    }

    pub(crate) fn else_statement(&self) -> Option<&BoundStatement> {
        self.else_statement.as_ref().map(|e| e as &BoundStatement)
    }
}
