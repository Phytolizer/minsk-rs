use crate::{
    code_analysis::syntax::expression_syntax::ExpressionSyntax,
    code_analysis::syntax::parenthesized_expression_syntax::ParenthesizedExpressionSyntax,
    code_analysis::syntax::syntax_kind::SyntaxKind,
    code_analysis::syntax::syntax_node::SyntaxNode,
    code_analysis::syntax::{
        binary_expression_syntax::BinaryExpressionSyntax,
        unary_expression_syntax::UnaryExpressionSyntax,
    },
    minsk_value::MinskValue,
};

use super::{
    super::syntax::literal_expression_syntax::LiteralExpressionSyntax,
    bound_binary_expression::BoundBinaryExpression,
    bound_binary_operator_kind::BoundBinaryOperatorKind, bound_expression::BoundExpression,
    bound_literal_expression::BoundLiteralExpression, bound_unary_expression::BoundUnaryExpression,
    bound_unary_operator_kind::BoundUnaryOperatorKind, minsk_type::MinskType,
};

pub(crate) struct Binder {
    diagnostics: Vec<String>,
}

impl Binder {
    pub(crate) fn new() -> Self {
        Self {
            diagnostics: vec![],
        }
    }

    pub(crate) fn diagnostics(&self) -> &[String] {
        &self.diagnostics
    }

    pub(crate) fn bind(&mut self, syntax: &SyntaxNode) -> BoundExpression {
        let SyntaxNode::ExpressionSyntax(e) = syntax;
        self.bind_expression(e)
    }

    pub(super) fn bind_expression(&mut self, syntax: &ExpressionSyntax) -> BoundExpression {
        match syntax {
            ExpressionSyntax::LiteralExpressionSyntax(l) => self.bind_literal_expression(l),
            ExpressionSyntax::UnaryExpressionSyntax(u) => self.bind_unary_expression(u),
            ExpressionSyntax::BinaryExpressionSyntax(b) => self.bind_binary_expression(b),
            ExpressionSyntax::ParenthesizedExpressionSyntax(p) => {
                self.bind_parenthesized_expression(p)
            }
        }
    }

    fn bind_literal_expression(&mut self, syntax: &LiteralExpressionSyntax) -> BoundExpression {
        let value = match &syntax.literal_token.value {
            Some(v) => v.clone(),
            None => MinskValue::Integer(0),
        };
        BoundExpression::BoundLiteralExpression(BoundLiteralExpression { value })
    }

    fn bind_unary_expression(&mut self, syntax: &UnaryExpressionSyntax) -> BoundExpression {
        let operand = self.bind_expression(&syntax.operand);
        let operator_kind =
            self.bind_unary_operator_kind(syntax.operator_token.kind, operand.kind());
        if let Some(operator_kind) = operator_kind {
            BoundExpression::BoundUnaryExpression(BoundUnaryExpression {
                operator_kind,
                operand: Box::new(operand),
            })
        } else {
            self.diagnostics.push(format!(
                "Unary operator '{}' is not defined for type {}",
                syntax.operator_token.kind,
                operand.kind()
            ));
            operand
        }
    }

    fn bind_binary_expression(&mut self, syntax: &BinaryExpressionSyntax) -> BoundExpression {
        let left = self.bind_expression(&syntax.left);
        let right = self.bind_expression(&syntax.right);
        let operator_kind =
            self.bind_binary_operator_kind(syntax.operator_token.kind, left.kind(), right.kind());
        if let Some(operator_kind) = operator_kind {
            BoundExpression::BoundBinaryExpression(BoundBinaryExpression {
                left: Box::new(left),
                operator_kind,
                right: Box::new(right),
            })
        } else {
            self.diagnostics.push(format!(
                "Binary operator '{}' is not defined for types {} and {}",
                syntax.operator_token.kind,
                left.kind(),
                right.kind()
            ));
            left
        }
    }

    fn bind_parenthesized_expression(
        &mut self,
        syntax: &ParenthesizedExpressionSyntax,
    ) -> BoundExpression {
        self.bind_expression(&syntax.expression)
    }

    fn bind_unary_operator_kind(
        &mut self,
        operator_kind: SyntaxKind,
        ty: MinskType,
    ) -> Option<BoundUnaryOperatorKind> {
        if ty != MinskType::Integer {
            return None;
        }
        match operator_kind {
            SyntaxKind::Plus => Some(BoundUnaryOperatorKind::Identity),
            SyntaxKind::Minus => Some(BoundUnaryOperatorKind::Negation),
            _ => panic!("Unexpected unary operator {}", operator_kind),
        }
    }

    fn bind_binary_operator_kind(
        &mut self,
        operator_kind: SyntaxKind,
        left_ty: MinskType,
        right_ty: MinskType,
    ) -> Option<BoundBinaryOperatorKind> {
        if left_ty != MinskType::Integer || right_ty != MinskType::Integer {
            return None;
        }
        match operator_kind {
            SyntaxKind::Plus => Some(BoundBinaryOperatorKind::Addition),
            SyntaxKind::Minus => Some(BoundBinaryOperatorKind::Subtraction),
            SyntaxKind::Star => Some(BoundBinaryOperatorKind::Multiplication),
            SyntaxKind::Slash => Some(BoundBinaryOperatorKind::Division),
            _ => panic!("Unexpected binary operator {}", operator_kind),
        }
    }
}
