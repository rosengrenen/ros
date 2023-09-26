use crate::aml::{data::byte_data, ops::AcquireOp, prefixed::prefixed, term::statement::MutexObj};

parser_struct_alloc!(
    struct Acquire {
        mutex: MutexObj<A>,
        timeout: u8,
    },
    prefixed(AcquireOp::p, (MutexObj::p, byte_data))
);
