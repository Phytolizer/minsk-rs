use super::bound_expression::BoundExpression;

#[derive(Debug)]
pub struct BoundExpressionStatement {
    expression: BoundExpression,
}

impl BoundExpressionStatement {
    pub(crate) fn new(expression: BoundExpression) -> Self {
        Self { expression }
    }

    pub(crate) fn expression(&self) -> &BoundExpression {
        &self.expression
    }
}
