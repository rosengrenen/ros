use crate::aml::{name::SuperName, ops::RefOfOp, prefixed::prefixed};

parser_struct_alloc!(
    struct RefOf {
        name: SuperName<A>,
    },
    prefixed(RefOfOp::p, SuperName::p)
);
