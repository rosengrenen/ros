use crate::aml::{
    aml::{data::integer::byte_data, term::term_arg::TermArg},
    ops::BufferOp,
    pkg, prefixed, Context,
};
use alloc::vec::Vec;
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    multi::many::many,
    parser::Parser,
};

#[derive(Debug)]
pub struct Buffer<A: Allocator> {
    pub len: TermArg<A>,
    pub bytes: Vec<u8, A>,
}

impl<A: Allocator + Clone> Buffer<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let buffer_size = TermArg::p; // => Integer
        prefixed(BufferOp::p, pkg((buffer_size, many(byte_data))))
            .map(|(len, bytes)| Self { len, bytes })
            .add_context("Buffer")
            .parse(input, context, alloc)
    }
}
