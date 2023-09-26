use crate::aml::{
    name::{SuperName, Target},
    ops::CondRefOfOp,
    prefixed::prefixed,
};

parser_struct_alloc!(
    struct CondRefOf {
        name: SuperName<A>,
        target: Target<A>,
    },
    prefixed(CondRefOfOp::p, (SuperName::p, Target::p))
);
