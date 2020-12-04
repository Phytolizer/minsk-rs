use super::bound_expression::BoundExpression;

#[derive(Debug)]
enum BoundNode {
    BoundExpression(BoundExpression),
}
