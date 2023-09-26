use crate::aml::{name::SimpleName, ops::CopyObjOp, prefixed::prefixed, term::TermArg};

parser_struct_alloc!(
    struct CopyObj {
        arg: TermArg<A>,
        name: SimpleName<A>,
    },
    prefixed(CopyObjOp::p, (TermArg::p, SimpleName::p))
);
