use parking_lot::RwLock;
use std::sync::Arc;

use crate::{
    code_analysis::syntax::expression_syntax::ExpressionSyntax,
    code_analysis::syntax::parenthesized_expression_syntax::ParenthesizedExpressionSyntax,
    code_analysis::{
        diagnostic::Diagnostic,
        diagnostic_bag::DiagnosticBag,
        minsk_type::MinskType,
        minsk_value::MinskValue,
        syntax::assignment_expression_syntax::AssignmentExpressionSyntax,
        syntax::{
            binary_expression_syntax::BinaryExpressionSyntax,
            block_statement_syntax::BlockStatementSyntax, compilation_unit::CompilationUnit,
            expression_statement_syntax::ExpressionStatementSyntax,
            if_statement_syntax::IfStatementSyntax, name_expression_syntax::NameExpressionSyntax,
            statement_syntax::StatementSyntax, syntax_kind::SyntaxKind,
            unary_expression_syntax::UnaryExpressionSyntax,
            variable_declaration_syntax::VariableDeclarationSyntax,
            while_statement_syntax::WhileStatementSyntax,
        },
        variable_symbol::VariableSymbol,
    },
};

use super::{
    super::syntax::literal_expression_syntax::LiteralExpressionSyntax,
    bound_assignment_expression::BoundAssignmentExpression,
    bound_binary_expression::BoundBinaryExpression, bound_binary_operator::BoundBinaryOperator,
    bound_block_statement::BoundBlockStatement, bound_expression::BoundExpression,
    bound_expression_statement::BoundExpressionStatement, bound_global_scope::BoundGlobalScope,
    bound_if_statement::BoundIfStatement, bound_literal_expression::BoundLiteralExpression,
    bound_scope::BoundScope, bound_statement::BoundStatement,
    bound_unary_expression::BoundUnaryExpression, bound_unary_operator::BoundUnaryOperator,
    bound_variable_declaration::BoundVariableDeclaration,
    bound_variable_expression::BoundVariableExpression, bound_while_statement::BoundWhileStatement,
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
        let statement = binder.bind_statement(syntax.statement());
        let variables = binder
            .scope
            .read()
            .declared_variables()
            .cloned()
            .collect::<Vec<_>>();
        let mut diagnostics = binder.diagnostics().collect::<Vec<_>>();

        if let Some(previous) = &previous {
            diagnostics.append(&mut previous.diagnostics().collect::<Vec<_>>());
        }

        BoundGlobalScope::new(previous, diagnostics, variables, statement)
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

    fn bind_statement(&mut self, syntax: &StatementSyntax) -> BoundStatement {
        match syntax {
            StatementSyntax::Block(b) => self.bind_block_statement(b),
            StatementSyntax::Expression(e) => self.bind_expression_statement(e),
            StatementSyntax::If(i) => self.bind_if_statement(i),
            StatementSyntax::VariableDeclaration(v) => self.bind_variable_declaration(v),
            StatementSyntax::While(w) => self.bind_while_statement(w),
        }
    }

    fn bind_while_statement(&mut self, syntax: &WhileStatementSyntax) -> BoundStatement {
        let condition = self.bind_expression_with_type(syntax.condition(), MinskType::Boolean);
        let body = self.bind_statement(syntax.body());
        BoundStatement::While(BoundWhileStatement::new(condition, Box::new(body)))
    }

    fn bind_if_statement(&mut self, syntax: &IfStatementSyntax) -> BoundStatement {
        let condition = self.bind_expression_with_type(syntax.condition(), MinskType::Boolean);
        let then_statement = self.bind_statement(syntax.then_statement());
        let else_statement = syntax
            .else_statement()
            .map(|e| self.bind_statement(e.else_statement()));
        BoundStatement::If(BoundIfStatement::new(
            condition,
            Box::new(then_statement),
            else_statement.map(Box::new),
        ))
    }

    fn bind_variable_declaration(&mut self, syntax: &VariableDeclarationSyntax) -> BoundStatement {
        let name = &syntax.identifier().text;
        let read_only = syntax.keyword_token().kind == SyntaxKind::LetKeyword;
        let initializer = self.bind_expression(syntax.initializer());
        let variable = VariableSymbol::new(name.to_string(), read_only, initializer.ty());

        if !self.scope.write().try_declare(variable.clone()) {
            self.diagnostics
                .report_variable_already_declared(syntax.identifier().span, &name);
        }

        BoundStatement::VariableDeclaration(BoundVariableDeclaration::new(variable, initializer))
    }

    fn bind_block_statement(&mut self, syntax: &BlockStatementSyntax) -> BoundStatement {
        let mut statements = Vec::<BoundStatement>::new();
        self.scope = Arc::new(RwLock::new(BoundScope::new(Some(self.scope.clone()))));
        for statement in syntax.statements() {
            let statement = self.bind_statement(statement);
            statements.push(statement);
        }

        let parent = self.scope.read().parent().unwrap();
        self.scope = parent;
        BoundStatement::Block(BoundBlockStatement::new(statements))
    }

    fn bind_expression_statement(&mut self, syntax: &ExpressionStatementSyntax) -> BoundStatement {
        let expression = self.bind_expression(syntax.expression());
        BoundStatement::Expression(BoundExpressionStatement::new(expression))
    }

    fn bind_expression_with_type(
        &mut self,
        syntax: &ExpressionSyntax,
        ty: MinskType,
    ) -> BoundExpression {
        let result = self.bind_expression(syntax);
        if result.ty() != ty {
            self.diagnostics
                .report_cannot_convert(syntax.span(), result.ty(), ty);
        }
        result
    }

    fn bind_expression(&mut self, syntax: &ExpressionSyntax) -> BoundExpression {
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
        let operator = BoundUnaryOperator::bind(syntax.operator_token.kind, operand.ty());
        if let Some(op) = operator {
            BoundExpression::Unary(BoundUnaryExpression {
                op,
                operand: Box::new(operand),
            })
        } else {
            self.diagnostics.report_undefined_unary_operator(
                syntax.operator_token.span,
                &syntax.operator_token.text,
                operand.ty(),
            );
            operand
        }
    }

    fn bind_binary_expression(&mut self, syntax: &BinaryExpressionSyntax) -> BoundExpression {
        let left = self.bind_expression(&syntax.left);
        let right = self.bind_expression(&syntax.right);
        let operator = BoundBinaryOperator::bind(syntax.operator_token.kind, left.ty(), right.ty());
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
                left.ty(),
                right.ty(),
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
        let variable = self.scope.read().try_lookup(&name);
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

        let maybe_variable = self.scope.read().try_lookup(&name);
        let variable = if let Some(v) = maybe_variable {
            v
        } else {
            self.diagnostics
                .report_undefined_name(syntax.identifier_token.span, &name);
            return bound;
        };

        if variable.read_only() {
            self.diagnostics
                .report_cannot_assign(syntax.equals_token.span, &name);
        }

        if bound.ty() != variable.ty() {
            self.diagnostics.report_cannot_convert(
                syntax.expression.span(),
                bound.ty(),
                variable.ty(),
            );
        }

        BoundExpression::Assignment(BoundAssignmentExpression {
            variable,
            expression: Box::new(bound),
        })
    }
}
