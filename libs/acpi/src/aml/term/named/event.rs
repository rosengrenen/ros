use crate::aml::{name::NameString, ops::EventOp, prefixed::prefixed};

parser_struct_alloc!(
    struct Event {
        name: NameString<A>,
    },
    prefixed(EventOp::p, NameString::p)
);
