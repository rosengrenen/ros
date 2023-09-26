use crate::aml::{ops::DerefOfOp, prefixed::prefixed, term::TermArg};

parser_struct_alloc!(
    struct DerefOf {
        obj_ref: TermArg<A>,
    },
    prefixed(DerefOfOp::p, TermArg::p)
);
