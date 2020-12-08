use super::{bound_expression::BoundExpression, bound_statement::BoundStatement};

#[derive(Debug)]
pub(super) enum BoundNode {
    Statement(BoundStatement),
    Expression(BoundExpression),
}
