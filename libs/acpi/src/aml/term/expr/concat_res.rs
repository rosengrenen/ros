use crate::aml::{name::Target, ops::ConcatResOp, prefixed::prefixed, term::TermArg};

parser_struct_alloc!(
    struct ConcatRes {
        left: TermArg<A>,
        right: TermArg<A>,
        target: Target<A>,
    },
    prefixed(ConcatResOp::p, (TermArg::p, TermArg::p, Target::p))
);
