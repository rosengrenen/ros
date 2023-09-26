use crate::aml::{
    data::byte_data, name::NameString, ops::OpRegionOp, prefixed::prefixed, term::TermArg,
};

parser_struct_alloc!(
    struct OpRegion {
        name: NameString<A>,
        space: u8,
        offset: TermArg<A>,
        len: TermArg<A>,
    },
    prefixed(
        OpRegionOp::p,
        (NameString::p, byte_data, TermArg::p, TermArg::p),
    )
);
