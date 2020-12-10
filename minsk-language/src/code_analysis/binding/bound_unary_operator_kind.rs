#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum BoundUnaryOperatorKind {
    Identity,
    Negation,
    LogicalNegation,
    OnesComplement,
}
