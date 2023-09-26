use crate::aml::{data::byte_data, name::NameString, ops::ExternalOp, prefixed::prefixed};

parser_struct_alloc!(
    struct External {
        name: NameString<A>,
        obj_type: u8,
        argument_count: u8,
    },
    prefixed(ExternalOp::p, (NameString::p, byte_data, byte_data))
);
