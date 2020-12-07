use parking_lot::RwLock;
use std::sync::Arc;

use crate::{
    code_analysis::syntax::expression_syntax::ExpressionSyntax,
    code_analysis::syntax::parenthesized_expression_syntax::ParenthesizedExpressionSyntax,
    code_analysis::{
        diagnostic::Diagnostic,
        diagnostic_bag::DiagnosticBag,
        minsk_value::MinskValue,
        syntax::assignment_expression_syntax::AssignmentExpressionSyntax,
        syntax::{
            binary_expression_syntax::BinaryExpressionSyntax, compilation_unit::CompilationUnit,
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
    bound_expression::BoundExpression, bound_global_scope::BoundGlobalScope,
    bound_literal_expression::BoundLiteralExpression, bound_scope::BoundScope,
    bound_unary_expression::BoundUnaryExpression, bound_unary_operator::BoundUnaryOperator,
    bound_variable_expression::BoundVariableExpression,
};

pub struct Binder {
    scope: Arc<RwLock<BoundScope>>,
    diagnostics: DiagnosticBag,
}

impl Binder {
    pub(crate) fn new(parent: Option<Arc<RwLock<BoundScope>>>) -> Self {
        Self {
            scope: Arc::new(RwLock::new(BoundScope::new(parent))),
            diagnostics: DiagnosticBag::new(),
        }
    }

    pub(crate) fn bind_global_scope(
        previous: Option<Arc<BoundGlobalScope>>,
        syntax: &CompilationUnit,
    ) -> BoundGlobalScope {
        let parent_scope = Self::create_parent_scopes(previous.clone());
        let mut binder = Binder::new(parent_scope);
        let expression = binder.bind_expression(syntax.expression());
        let variables = binder
            .scope
            .read()
            .declared_variables()
            .cloned()
            .collect::<Vec<_>>();
        let diagnostics = binder.diagnostics().collect::<Vec<_>>();
        BoundGlobalScope::new(previous, diagnostics, variables, expression)
    }

    pub(crate) fn create_parent_scopes(
        mut previous: Option<Arc<BoundGlobalScope>>,
    ) -> Option<Arc<RwLock<BoundScope>>> {
        let mut stack = Vec::<Arc<BoundGlobalScope>>::new();
        while let Some(prev) = &previous {
            stack.push(prev.clone());

            previous = previous.and_then(|p| p.previous().clone());
        }

        let mut parent: Option<Arc<RwLock<BoundScope>>> = None;
        while !stack.is_empty() {
            let previous = stack.pop().unwrap();
            let mut scope = BoundScope::new(parent);
            for v in previous.variables() {
                scope.try_declare(v);
            }

            parent = Some(Arc::new(RwLock::new(scope)));
        }
        parent
    }

    pub fn diagnostics(&self) -> impl Iterator<Item = Diagnostic> + '_ {
        self.diagnostics.iter()
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
                syntax.operator_token.span,
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
                syntax.operator_token.span,
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
        let variable = self.scope.read().try_lookup(name.clone());
        if let Some(variable) = variable {
            BoundExpression::Variable(BoundVariableExpression { variable })
        } else {
            self.diagnostics
                .report_undefined_name(syntax.identifier_token.span, name);
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

        let variable = VariableSymbol {
            name: name.clone(),
            ty: bound.kind(),
        };
        if !self.scope.write().try_declare(variable.clone()) {
            self.diagnostics
                .report_variable_already_declared(syntax.identifier_token.span, &name);
        }

        BoundExpression::Assignment(BoundAssignmentExpression {
            variable,
            expression: Box::new(bound),
        })
    }
}
