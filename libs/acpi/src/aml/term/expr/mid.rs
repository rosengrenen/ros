use crate::aml::{name::Target, ops::MidOp, prefixed::prefixed, term::TermArg};

parser_struct_alloc!(
    struct Mid {
        mid_obj: TermArg<A>,
        term1: TermArg<A>,
        term2: TermArg<A>,
        target: Target<A>,
    },
    prefixed(MidOp::p, (TermArg::p, TermArg::p, TermArg::p, Target::p))
);
