use crate::aml::{data::byte_data, ops::BufferOp, pkg_len::pkg, prefixed::prefixed, term::TermArg};
use alloc::vec::Vec;
use parser::multi::many::many;

parser_struct_alloc!(
    struct Buffer {
        len: TermArg<A>,
        bytes: Vec<u8, A>,
    },
    prefixed(BufferOp::p, pkg((TermArg::p, many(byte_data))))
);
