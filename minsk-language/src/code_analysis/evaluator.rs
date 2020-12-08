use std::collections::HashMap;

use super::{
    binding::{
        bound_assignment_expression::BoundAssignmentExpression,
        bound_binary_expression::BoundBinaryExpression, bound_block_statement::BoundBlockStatement,
        bound_expression_statement::BoundExpressionStatement, bound_statement::BoundStatement,
        bound_unary_expression::BoundUnaryExpression,
        bound_variable_declaration::BoundVariableDeclaration,
        bound_variable_expression::BoundVariableExpression,
    },
    minsk_value::MinskValue,
    variable_symbol::VariableSymbol,
};

use super::binding::{
    bound_binary_operator_kind::BoundBinaryOperatorKind, bound_expression::BoundExpression,
    bound_literal_expression::BoundLiteralExpression,
    bound_unary_operator_kind::BoundUnaryOperatorKind,
};

pub struct Evaluator<'compilation> {
    variables: &'compilation mut HashMap<VariableSymbol, MinskValue>,
    last_value: Option<MinskValue>,
}

impl<'compilation> Evaluator<'compilation> {
    pub fn new(variables: &'compilation mut HashMap<VariableSymbol, MinskValue>) -> Self {
        Self {
            variables,
            last_value: None,
        }
    }

    pub fn evaluate(&mut self, root: &BoundStatement) -> Option<MinskValue> {
        self.evaluate_statement(root);
        self.last_value.clone()
    }

    fn evaluate_statement(&mut self, statement: &BoundStatement) {
        match statement {
            BoundStatement::Block(b) => self.evaluate_block_statement(b),
            BoundStatement::Expression(e) => self.evaluate_expression_statement(e),
            BoundStatement::VariableDeclaration(v) => self.evaluate_variable_declaration(v),
        }
    }

    fn evaluate_variable_declaration(&mut self, v: &BoundVariableDeclaration) {
        let value = self.evaluate_expression(v.initializer());
        self.variables.insert(v.variable().clone(), value.clone());
        self.last_value = Some(value);
    }

    fn evaluate_block_statement(&mut self, b: &BoundBlockStatement) {
        for statement in b.statements() {
            self.evaluate(statement);
        }
    }

    fn evaluate_expression_statement(&mut self, e: &BoundExpressionStatement) {
        self.last_value = Some(self.evaluate_expression(e.expression()));
    }

    fn evaluate_expression(&mut self, root: &BoundExpression) -> MinskValue {
        match root {
            BoundExpression::Literal(lit) => self.evaluate_literal_expression(lit),
            BoundExpression::Unary(u) => self.evaluate_unary_expression(u),
            BoundExpression::Binary(b) => self.evaluate_binary_expression(b),
            BoundExpression::Variable(v) => self.evaluate_variable_expression(v),
            BoundExpression::Assignment(a) => self.evaluate_assignment_expression(a),
        }
    }

    fn evaluate_literal_expression(&mut self, lit: &BoundLiteralExpression) -> MinskValue {
        lit.value.clone()
    }

    fn evaluate_unary_expression(&mut self, u: &BoundUnaryExpression) -> MinskValue {
        let operand = self.evaluate_expression(&u.operand);
        match u.op.kind {
            BoundUnaryOperatorKind::Identity => MinskValue::Integer(operand.as_integer().unwrap()),
            BoundUnaryOperatorKind::Negation => MinskValue::Integer(-operand.as_integer().unwrap()),
            BoundUnaryOperatorKind::LogicalNegation => {
                MinskValue::Boolean(!operand.as_boolean().unwrap())
            }
        }
    }

    fn evaluate_binary_expression(&mut self, b: &BoundBinaryExpression) -> MinskValue {
        let left = self.evaluate_expression(&b.left);
        let right = self.evaluate_expression(&b.right);
        match b.op.kind {
            BoundBinaryOperatorKind::Addition => MinskValue::Integer(
                left.as_integer()
                    .unwrap()
                    .wrapping_add(right.as_integer().unwrap()),
            ),
            BoundBinaryOperatorKind::Subtraction => MinskValue::Integer(
                left.as_integer()
                    .unwrap()
                    .wrapping_sub(right.as_integer().unwrap()),
            ),
            BoundBinaryOperatorKind::Multiplication => MinskValue::Integer(
                left.as_integer()
                    .unwrap()
                    .wrapping_mul(right.as_integer().unwrap()),
            ),
            BoundBinaryOperatorKind::Division => MinskValue::Integer(
                left.as_integer()
                    .unwrap()
                    .wrapping_div(right.as_integer().unwrap()),
            ),
            BoundBinaryOperatorKind::Equality => MinskValue::Boolean(left == right),
            BoundBinaryOperatorKind::Inequality => MinskValue::Boolean(left != right),
            BoundBinaryOperatorKind::LogicalAnd => {
                MinskValue::Boolean(left.as_boolean().unwrap() && right.as_boolean().unwrap())
            }
            BoundBinaryOperatorKind::LogicalOr => {
                MinskValue::Boolean(left.as_boolean().unwrap() || right.as_boolean().unwrap())
            }
        }
    }

    fn evaluate_variable_expression(&mut self, v: &BoundVariableExpression) -> MinskValue {
        self.variables.get(&v.variable).unwrap().clone()
    }

    fn evaluate_assignment_expression(&mut self, a: &BoundAssignmentExpression) -> MinskValue {
        let value = self.evaluate_expression(&a.expression);
        self.variables.insert(a.variable.clone(), value.clone());
        value
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufRead, BufReader};

    use crate::code_analysis::{
        compilation::Compilation, syntax::syntax_tree::SyntaxTree, text::text_span::TextSpan,
    };

    use super::*;
    use spectral::prelude::*;

    #[derive(Debug)]
    struct AnnotatedText {
        text: String,
        spans: Vec<TextSpan>,
    }

    impl AnnotatedText {
        fn indentation(s: &str) -> usize {
            s.chars()
                .enumerate()
                .find_map(|(i, c)| if !c.is_whitespace() { Some(i) } else { None })
                // unwrap b/c we filtered whitespace-only lines
                .unwrap()
        }

        fn dedent(text: &str) -> Vec<String> {
            let lines = BufReader::new(text.as_bytes())
                .lines()
                .map(|line| line.unwrap())
                .filter(|line| !line.trim().is_empty())
                .collect::<Vec<_>>();
            let min_indentation = lines
                .iter()
                .min_by(|&a, &b| Self::indentation(a).cmp(&Self::indentation(b)))
                .map(|line| Self::indentation(line))
                // unwrap because we reject whitespace-only input
                .unwrap();
            let lines = lines
                .iter()
                .map(|line| line.chars().skip(min_indentation).collect::<String>())
                .collect::<Vec<_>>();
            lines
        }
        fn parse(text: &str) -> Self {
            let text = Self::dedent(text);
            let mut spans = Vec::<TextSpan>::new();
            let mut start_positions = Vec::<usize>::new();

            let mut position = 0;
            let text = text
                .join("\n")
                .chars()
                .filter(|&c| {
                    if c == '[' {
                        start_positions.push(position);
                        false
                    } else if c == ']' {
                        if start_positions.is_empty() {
                            panic!("unmatched ']'");
                        }
                        let start = start_positions.pop().unwrap();
                        let end = position;
                        let span = TextSpan { start, end };
                        spans.push(span);
                        false
                    } else {
                        position += 1;
                        true
                    }
                })
                .collect();
            if !start_positions.is_empty() {
                panic!("unmatched '['");
            }
            AnnotatedText { text, spans }
        }
    }

    #[test]
    fn variable_declaration_reports_redeclaration() {
        let text = "
            {
                var x = 10
                var y = 100
                {
                    var x = 10
                }
                var [x] = 5
            }
            ";
        let diagnostics = "
            Variable 'x' has already been declared
            ";

        assert_has_diagnostics(text, diagnostics);
    }

    #[test]
    fn name_expression_reports_undefined() {
        let text = "[x] * 10";
        let diagnostics = "
            Variable 'x' doesn't exist
            ";

        assert_has_diagnostics(text, diagnostics);
    }

    #[test]
    fn bad_unary_operator_is_reported() {
        let text = "[-]true";
        let diagnostics = "
            Unary operator '-' is not defined for type Boolean
        ";
        assert_has_diagnostics(text, diagnostics);
    }

    #[test]
    fn bad_binary_operator_is_reported() {
        let text = "3 [&&] 7";
        let diagnostics = "
            Binary operator '&&' is not defined for types Integer and Integer
            ";
        assert_has_diagnostics(text, diagnostics);
    }

    #[test]
    fn cannot_assign_let_binding() {
        let text = "
            {
                let x = 5
                x [=] 4
            }
            ";
        let diagnostics = "
            Variable 'x' is immutable and cannot be assigned to
        ";
        assert_has_diagnostics(text, diagnostics);
    }

    #[test]
    fn cannot_convert_bool_to_int() {
        let text = "
            {
                var x = 10
                x = [true]
            }
        ";
        let diagnostics = "
            Cannot convert Boolean to Integer
        ";
        assert_has_diagnostics(text, diagnostics);
    }

    fn assert_has_diagnostics(text: &str, diagnostics: &str) {
        let annotated_text = AnnotatedText::parse(text);
        let syntax_tree = SyntaxTree::parse(annotated_text.text.clone());
        let mut compilation = Compilation::new(syntax_tree);
        let result = compilation.evaluate(&mut HashMap::new());
        let expected_diagnostics = AnnotatedText::dedent(diagnostics);
        asserting!("result is error").that(&result).is_err();
        let result = result.unwrap_err();

        if annotated_text.spans.len() != expected_diagnostics.len() {
            panic!("mismatch between span count and diagnostic count");
        }
        asserting!("same number of diagnostics")
            .that(&expected_diagnostics.len())
            .is_equal_to(&result.len());
        for (i, (diagnostic, span)) in expected_diagnostics
            .iter()
            .zip(annotated_text.spans)
            .enumerate()
        {
            let actual_span = result[i].span;
            let actual_diagnostic = &result[i].message;
            asserting!("messages match")
                .that(actual_diagnostic)
                .is_equal_to(diagnostic);
            asserting!("spans match")
                .that(&actual_span)
                .is_equal_to(&span);
        }
    }

    #[test]
    fn annotated_text_dedents_correctly() {
        let text = "
            test
            set
            ";
        asserting!("strips indentation")
            .that(&AnnotatedText::dedent(text))
            .is_equal_to(&vec!["test".to_string(), "set".to_string()]);
    }

    fn try_evaluate(text: &str, expected: Option<MinskValue>) {
        let syntax_tree = SyntaxTree::parse(text.to_string());

        let actual = Compilation::new(syntax_tree)
            .evaluate(&mut HashMap::<VariableSymbol, MinskValue>::new());

        asserting!("evaluated value")
            .that(&actual)
            .is_ok()
            .is_equal_to(&expected);
    }

    #[test]
    fn evaluates_correctly() {
        for (text, expected) in [
            ("1", MinskValue::Integer(1)),
            ("+1", MinskValue::Integer(1)),
            ("-1", MinskValue::Integer(-1)),
            ("14 + 12", MinskValue::Integer(26)),
            ("12 - 3", MinskValue::Integer(9)),
            ("4 * 2", MinskValue::Integer(8)),
            ("9 / 3", MinskValue::Integer(3)),
            ("(10)", MinskValue::Integer(10)),
            ("12 == 3", MinskValue::Boolean(false)),
            ("3 == 3", MinskValue::Boolean(true)),
            ("12 != 3", MinskValue::Boolean(true)),
            ("3 != 3", MinskValue::Boolean(false)),
            ("true == false", MinskValue::Boolean(false)),
            ("true == true", MinskValue::Boolean(true)),
            ("false == false", MinskValue::Boolean(true)),
            ("true != false", MinskValue::Boolean(true)),
            ("false != false", MinskValue::Boolean(false)),
            ("true", MinskValue::Boolean(true)),
            ("false", MinskValue::Boolean(false)),
            ("!true", MinskValue::Boolean(false)),
            ("!false", MinskValue::Boolean(true)),
            ("{ var a = 0 (a = 10) * a }", MinskValue::Integer(100)),
        ]
        .iter()
        {
            println!("{} => {}", text, expected);
            try_evaluate(text, Some(expected.clone()));
        }
    }
}
