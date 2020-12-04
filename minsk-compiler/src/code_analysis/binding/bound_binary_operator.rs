use crate::{code_analysis::syntax::syntax_kind::SyntaxKind, minsk_type::MinskType};

use super::bound_binary_operator_kind::BoundBinaryOperatorKind;

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
    pub(super) const fn operators() -> [BoundBinaryOperator; 6] {
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
