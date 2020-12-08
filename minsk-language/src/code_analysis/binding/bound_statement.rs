use super::{
    bound_block_statement::BoundBlockStatement,
    bound_expression_statement::BoundExpressionStatement,
};

#[derive(Debug)]
pub enum BoundStatement {
    Block(BoundBlockStatement),
    Expression(BoundExpressionStatement),
}
