use crate::aml::{name::Target, ops::FindSetLeftBitOp, prefixed::prefixed, term::TermArg};

parser_struct_alloc!(
    struct FindSetLeftBit {
        operand: TermArg<A>,
        target: Target<A>,
    },
    prefixed(FindSetLeftBitOp::p, (TermArg::p, Target::p))
);
