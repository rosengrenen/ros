use super::PkgElement;
use crate::aml::{aml::data::byte_data, ops::PkgOp, pkg, prefixed, Context};
use alloc::vec::Vec;
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    multi::many::many,
    parser::Parser,
};

pub struct Pkg<A: Allocator> {
    pub len: usize,
    pub elements: Vec<PkgElement<A>, A>,
}

impl<A: Allocator + Clone> Pkg<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let num_elements = byte_data;
        prefixed(PkgOp::p, pkg((num_elements, many(PkgElement::p))))
            .map(|(len, elements)| Self {
                len: len as usize,
                elements,
            })
            .add_context("Pkg")
            .parse(input, context, alloc)
    }
}
