use crate::aml::{name::Target, ops::ModOp, prefixed::prefixed, term::TermArg};

parser_struct_alloc!(
    struct Mod {
        dividend: TermArg<A>,
        divisor: TermArg<A>,
        target: Target<A>,
    },
    prefixed(ModOp::p, (TermArg::p, TermArg::p, Target::p))
);
