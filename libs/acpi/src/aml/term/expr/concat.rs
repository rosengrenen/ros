use crate::aml::{name::Target, ops::ConcatOp, prefixed::prefixed, term::TermArg};

parser_struct_alloc!(
    struct Concat {
        left: TermArg<A>,
        right: TermArg<A>,
        target: Target<A>,
    },
    prefixed(ConcatOp::p, (TermArg::p, TermArg::p, Target::p))
);
