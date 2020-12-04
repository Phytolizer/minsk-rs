use std::collections::HashMap;

use crate::{
    code_analysis::syntax::expression_syntax::ExpressionSyntax,
    code_analysis::syntax::parenthesized_expression_syntax::ParenthesizedExpressionSyntax,
    code_analysis::syntax::syntax_node::SyntaxNode,
    code_analysis::{
        diagnostic::Diagnostic,
        diagnostic_bag::DiagnosticBag,
        minsk_value::MinskValue,
        syntax::assignment_expression_syntax::AssignmentExpressionSyntax,
        syntax::{
            binary_expression_syntax::BinaryExpressionSyntax,
            name_expression_syntax::NameExpressionSyntax,
            unary_expression_syntax::UnaryExpressionSyntax,
        },
        variable_symbol::VariableSymbol,
    },
};

use super::{
    super::syntax::literal_expression_syntax::LiteralExpressionSyntax,
    bound_assignment_expression::BoundAssignmentExpression,
    bound_binary_expression::BoundBinaryExpression, bound_binary_operator::BoundBinaryOperator,
    bound_expression::BoundExpression, bound_literal_expression::BoundLiteralExpression,
    bound_unary_expression::BoundUnaryExpression, bound_unary_operator::BoundUnaryOperator,
    bound_variable_expression::BoundVariableExpression,
};

pub struct Binder<'compilation> {
    variables: &'compilation mut HashMap<VariableSymbol, MinskValue>,
    diagnostics: DiagnosticBag,
}

impl<'compilation> Binder<'compilation> {
    pub fn new(variables: &'compilation mut HashMap<VariableSymbol, MinskValue>) -> Self {
        Self {
            variables,
            diagnostics: DiagnosticBag::new(),
        }
    }

    pub fn diagnostics(&self) -> impl Iterator<Item = Diagnostic> + '_ {
        self.diagnostics.iter()
    }

    pub fn bind(&mut self, syntax: &SyntaxNode) -> BoundExpression {
        let SyntaxNode::ExpressionSyntax(e) = syntax;
        self.bind_expression(e)
    }

    pub(super) fn bind_expression(&mut self, syntax: &ExpressionSyntax) -> BoundExpression {
        match syntax {
            ExpressionSyntax::Literal(l) => self.bind_literal_expression(l),
            ExpressionSyntax::Unary(u) => self.bind_unary_expression(u),
            ExpressionSyntax::Binary(b) => self.bind_binary_expression(b),
            ExpressionSyntax::Parenthesized(p) => self.bind_parenthesized_expression(p),
            ExpressionSyntax::Name(n) => self.bind_name_expression(n),
            ExpressionSyntax::Assignment(a) => self.bind_assignment_expression(a),
        }
    }

    fn bind_literal_expression(&mut self, syntax: &LiteralExpressionSyntax) -> BoundExpression {
        let value = match &syntax.value {
            Some(v) => v.clone(),
            None => MinskValue::Integer(0),
        };
        BoundExpression::Literal(BoundLiteralExpression { value })
    }

    fn bind_unary_expression(&mut self, syntax: &UnaryExpressionSyntax) -> BoundExpression {
        let operand = self.bind_expression(&syntax.operand);
        let operator = BoundUnaryOperator::bind(syntax.operator_token.kind, operand.kind());
        if let Some(op) = operator {
            BoundExpression::Unary(BoundUnaryExpression {
                op,
                operand: Box::new(operand),
            })
        } else {
            self.diagnostics.report_undefined_unary_operator(
                syntax.operator_token.span.clone(),
                &syntax.operator_token.text,
                operand.kind(),
            );
            operand
        }
    }

    fn bind_binary_expression(&mut self, syntax: &BinaryExpressionSyntax) -> BoundExpression {
        let left = self.bind_expression(&syntax.left);
        let right = self.bind_expression(&syntax.right);
        let operator =
            BoundBinaryOperator::bind(syntax.operator_token.kind, left.kind(), right.kind());
        if let Some(op) = operator {
            BoundExpression::Binary(BoundBinaryExpression {
                left: Box::new(left),
                op,
                right: Box::new(right),
            })
        } else {
            self.diagnostics.report_undefined_binary_operator(
                syntax.operator_token.span.clone(),
                &syntax.operator_token.text,
                left.kind(),
                right.kind(),
            );
            left
        }
    }

    fn bind_parenthesized_expression(
        &mut self,
        syntax: &ParenthesizedExpressionSyntax,
    ) -> BoundExpression {
        self.bind_expression(&syntax.expression)
    }

    fn bind_name_expression(&mut self, syntax: &NameExpressionSyntax) -> BoundExpression {
        let name = &syntax.identifier_token.text;
        let variable =
            self.variables
                .iter()
                .find_map(|v| if &v.0.name == name { Some(v.0) } else { None });
        if let Some(variable) = variable.cloned() {
            BoundExpression::Variable(BoundVariableExpression { variable })
        } else {
            self.diagnostics
                .report_undefined_name(syntax.identifier_token.span.clone(), name);
            BoundExpression::Literal(BoundLiteralExpression {
                value: MinskValue::Integer(0),
            })
        }
    }

    fn bind_assignment_expression(
        &mut self,
        syntax: &AssignmentExpressionSyntax,
    ) -> BoundExpression {
        let name = syntax.identifier_token.text.clone();
        let bound = self.bind_expression(&syntax.expression);

        let existing_variable = self.variables.keys().find(|k| k.name == name).cloned();
        if let Some(existing_variable) = existing_variable {
            self.variables.remove(&existing_variable);
        }
        let variable = VariableSymbol {
            name,
            ty: bound.kind(),
        };
        self.variables.insert(variable.clone(), MinskValue::Null);

        BoundExpression::Assignment(BoundAssignmentExpression {
            variable,
            expression: Box::new(bound),
        })
    }
}
