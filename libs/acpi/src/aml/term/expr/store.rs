use crate::aml::{name::SuperName, ops::StoreOp, prefixed::prefixed, term::TermArg};

parser_struct_alloc!(
    struct Store {
        term: TermArg<A>,
        name: SuperName<A>,
    },
    prefixed(StoreOp::p, (TermArg::p, SuperName::p))
);
