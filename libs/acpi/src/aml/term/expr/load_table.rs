use crate::aml::{ops::LoadTableOp, prefixed::prefixed, term::TermArg};

parser_struct_alloc!(
    struct LoadTable {
        arg1: TermArg<A>,
        arg2: TermArg<A>,
        arg3: TermArg<A>,
        arg4: TermArg<A>,
        arg5: TermArg<A>,
        arg6: TermArg<A>,
    },
    prefixed(
        LoadTableOp::p,
        (
            TermArg::p,
            TermArg::p,
            TermArg::p,
            TermArg::p,
            TermArg::p,
            TermArg::p,
        ),
    )
);
