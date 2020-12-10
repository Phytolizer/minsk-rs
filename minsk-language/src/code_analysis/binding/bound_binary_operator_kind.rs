#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum BoundBinaryOperatorKind {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    LogicalAnd,
    LogicalOr,
    Equality,
    Inequality,
    LessThan,
    LessOrEquals,
    GreaterThan,
    GreaterOrEquals,
}
