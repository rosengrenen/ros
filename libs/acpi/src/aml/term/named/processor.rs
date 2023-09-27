use crate::aml::{
    data::{byte_data, dword_data},
    name::NameString,
    ops::ProcessorOp,
    pkg_len::pkg,
    prefixed::prefixed,
    term::TermObj,
};
use alloc::vec::Vec;
use parser::multi::many::many;

parser_struct_alloc!(
    struct Processor {
        name: NameString<A>,
        proc_id: u8,
        pblk_addr: u32,
        pblk_len: u8,
        terms: Vec<TermObj<A>, A>,
    },
    prefixed(
        ProcessorOp::p,
        pkg((
            NameString::p,
            byte_data,
            dword_data,
            byte_data,
            many(TermObj::p)
        )),
    )
);
