use super::{
    bound_block_statement::BoundBlockStatement,
    bound_expression_statement::BoundExpressionStatement,
    bound_variable_declaration::BoundVariableDeclaration,
};

#[derive(Debug)]
pub enum BoundStatement {
    Block(BoundBlockStatement),
    Expression(BoundExpressionStatement),
    VariableDeclaration(BoundVariableDeclaration),
}