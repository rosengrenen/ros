use crate::aml::{name::SuperName, ops::SizeOfOp, prefixed::prefixed};

parser_struct_alloc!(
    struct SizeOf {
        name: SuperName<A>,
    },
    prefixed(SizeOfOp::p, SuperName::p)
);
