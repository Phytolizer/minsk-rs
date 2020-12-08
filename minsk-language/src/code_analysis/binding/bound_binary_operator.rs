use crate::code_analysis::syntax::syntax_kind::SyntaxKind;

use super::{super::minsk_type::MinskType, bound_binary_operator_kind::BoundBinaryOperatorKind};

#[derive(Debug, Copy, Clone)]
pub(crate) struct BoundBinaryOperator {
    pub(super) syntax_kind: SyntaxKind,
    pub(crate) kind: BoundBinaryOperatorKind,
    pub(super) left_type: MinskType,
    pub(super) right_type: MinskType,
    pub(super) result_type: MinskType,
}

impl BoundBinaryOperator {
    const fn new(syntax_kind: SyntaxKind, kind: BoundBinaryOperatorKind, ty: MinskType) -> Self {
        Self {
            syntax_kind,
            kind,
            left_type: ty,
            right_type: ty,
            result_type: ty,
        }
    }
    const fn new_with_result_type(
        syntax_kind: SyntaxKind,
        kind: BoundBinaryOperatorKind,
        ty: MinskType,
        result_type: MinskType,
    ) -> Self {
        Self {
            syntax_kind,
            kind,
            left_type: ty,
            right_type: ty,
            result_type,
        }
    }
    pub(super) const fn operators() -> [BoundBinaryOperator; 14] {
        [
            BoundBinaryOperator::new(
                SyntaxKind::Plus,
                BoundBinaryOperatorKind::Addition,
                MinskType::Integer,
            ),
            BoundBinaryOperator::new(
                SyntaxKind::Minus,
                BoundBinaryOperatorKind::Subtraction,
                MinskType::Integer,
            ),
            BoundBinaryOperator::new(
                SyntaxKind::Star,
                BoundBinaryOperatorKind::Multiplication,
                MinskType::Integer,
            ),
            BoundBinaryOperator::new(
                SyntaxKind::Slash,
                BoundBinaryOperatorKind::Division,
                MinskType::Integer,
            ),
            BoundBinaryOperator::new_with_result_type(
                SyntaxKind::EqualsEquals,
                BoundBinaryOperatorKind::Equality,
                MinskType::Integer,
                MinskType::Boolean,
            ),
            BoundBinaryOperator::new_with_result_type(
                SyntaxKind::EqualsEquals,
                BoundBinaryOperatorKind::Equality,
                MinskType::Boolean,
                MinskType::Boolean,
            ),
            BoundBinaryOperator::new_with_result_type(
                SyntaxKind::BangEquals,
                BoundBinaryOperatorKind::Inequality,
                MinskType::Integer,
                MinskType::Boolean,
            ),
            BoundBinaryOperator::new_with_result_type(
                SyntaxKind::BangEquals,
                BoundBinaryOperatorKind::Inequality,
                MinskType::Boolean,
                MinskType::Boolean,
            ),
            BoundBinaryOperator::new_with_result_type(
                SyntaxKind::Less,
                BoundBinaryOperatorKind::LessThan,
                MinskType::Integer,
                MinskType::Boolean,
            ),
            BoundBinaryOperator::new_with_result_type(
                SyntaxKind::LessEquals,
                BoundBinaryOperatorKind::LessOrEquals,
                MinskType::Integer,
                MinskType::Boolean,
            ),
            BoundBinaryOperator::new_with_result_type(
                SyntaxKind::Greater,
                BoundBinaryOperatorKind::GreaterThan,
                MinskType::Integer,
                MinskType::Boolean,
            ),
            BoundBinaryOperator::new_with_result_type(
                SyntaxKind::GreaterEquals,
                BoundBinaryOperatorKind::GreaterOrEquals,
                MinskType::Integer,
                MinskType::Boolean,
            ),
            BoundBinaryOperator::new(
                SyntaxKind::AmpersandAmpersand,
                BoundBinaryOperatorKind::LogicalAnd,
                MinskType::Boolean,
            ),
            BoundBinaryOperator::new(
                SyntaxKind::PipePipe,
                BoundBinaryOperatorKind::LogicalOr,
                MinskType::Boolean,
            ),
        ]
    }

    pub(super) fn bind(
        syntax_kind: SyntaxKind,
        left_type: MinskType,
        right_type: MinskType,
    ) -> Option<BoundBinaryOperator> {
        for op in Self::operators().iter() {
            if op.syntax_kind == syntax_kind
                && op.left_type == left_type
                && op.right_type == right_type
            {
                return Some(*op);
            }
        }
        None
    }
}
