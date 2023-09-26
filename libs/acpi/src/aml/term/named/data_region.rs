use crate::aml::{name::NameString, ops::DataRegionOp, prefixed::prefixed, term::TermArg};

parser_struct_alloc!(
    struct DataRegion {
        name: NameString<A>,
        term1: TermArg<A>,
        term2: TermArg<A>,
        term3: TermArg<A>,
    },
    prefixed(
        DataRegionOp::p,
        (NameString::p, TermArg::p, TermArg::p, TermArg::p),
    )
);
