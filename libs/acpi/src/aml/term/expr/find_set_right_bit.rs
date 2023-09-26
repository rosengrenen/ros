use crate::aml::{name::Target, ops::FindSetRightBitOp, prefixed::prefixed, term::TermArg};

parser_struct_alloc!(
    struct FindSetRightBit {
        operand: TermArg<A>,
        target: Target<A>,
    },
    prefixed(FindSetRightBitOp::p, (TermArg::p, Target::p))
);
