use crate::aml::{data::byte_data, name::NameString, ops::MutexOp, prefixed::prefixed};

parser_struct_alloc!(
    struct Mutex {
        name: NameString<A>,
        flags: u8,
    },
    prefixed(MutexOp::p, (NameString::p, byte_data))
);
