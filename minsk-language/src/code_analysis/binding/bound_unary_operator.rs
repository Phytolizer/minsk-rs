use crate::code_analysis::syntax::syntax_kind::SyntaxKind;

use super::super::minsk_type::MinskType;
use super::bound_unary_operator_kind::BoundUnaryOperatorKind;

#[derive(Debug, Copy, Clone)]
pub(crate) struct BoundUnaryOperator {
    pub(super) syntax_kind: SyntaxKind,
    pub(crate) kind: BoundUnaryOperatorKind,
    pub(super) operand_type: MinskType,
    pub(super) result_type: MinskType,
}

impl BoundUnaryOperator {
    const fn new(syntax_kind: SyntaxKind, kind: BoundUnaryOperatorKind, ty: MinskType) -> Self {
        Self {
            syntax_kind,
            kind,
            operand_type: ty,
            result_type: ty,
        }
    }
}

impl BoundUnaryOperator {
    pub(super) const fn operators() -> [BoundUnaryOperator; 3] {
        [
            BoundUnaryOperator::new(
                SyntaxKind::Bang,
                BoundUnaryOperatorKind::LogicalNegation,
                MinskType::Boolean,
            ),
            BoundUnaryOperator::new(
                SyntaxKind::Plus,
                BoundUnaryOperatorKind::Identity,
                MinskType::Integer,
            ),
            BoundUnaryOperator::new(
                SyntaxKind::Minus,
                BoundUnaryOperatorKind::Negation,
                MinskType::Integer,
            ),
        ]
    }

    pub(super) fn bind(
        syntax_kind: SyntaxKind,
        operand_type: MinskType,
    ) -> Option<BoundUnaryOperator> {
        for op in Self::operators().iter() {
            if op.syntax_kind == syntax_kind && op.operand_type == operand_type {
                return Some(*op);
            }
        }
        None
    }
}
