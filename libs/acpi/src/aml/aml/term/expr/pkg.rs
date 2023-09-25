use crate::aml::{aml::data::byte_data, ops::PkgOp, pkg, prefixed, Context};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

use super::PkgElementList;

pub struct Pkg<A: Allocator> {
    pub len: usize,
    pub elements: PkgElementList<A>,
}

impl<A: Allocator + Clone> Pkg<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let num_elements = byte_data;
        prefixed(PkgOp::p, pkg((num_elements, PkgElementList::p)))
            .map(|(len, elements)| Self {
                len: len as usize,
                elements,
            })
            .add_context("Pkg")
            .parse(input, context, alloc)
    }
}
