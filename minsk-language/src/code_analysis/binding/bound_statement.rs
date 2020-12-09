use super::{
    bound_block_statement::BoundBlockStatement,
    bound_expression_statement::BoundExpressionStatement, bound_if_statement::BoundIfStatement,
    bound_variable_declaration::BoundVariableDeclaration,
    bound_while_statement::BoundWhileStatement,
};

#[derive(Debug)]
pub enum BoundStatement {
    Block(BoundBlockStatement),
    Expression(BoundExpressionStatement),
    If(BoundIfStatement),
    VariableDeclaration(BoundVariableDeclaration),
    While(BoundWhileStatement),
}
