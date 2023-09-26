use crate::aml::{data::word_data, ops::AcquireOp, prefixed::prefixed, term::statement::MutexObj};

parser_struct_alloc!(
    struct Acquire {
        mutex: MutexObj<A>,
        timeout: u16,
    },
    prefixed(AcquireOp::p, (MutexObj::p, word_data))
);
