use crate::aml::{
    aml::{data::integer::byte_data, name::NameString},
    ops::ExternalOp,
    prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct External<A: Allocator> {
    pub name: NameString<A>,
    pub obj_type: u8,
    pub argument_count: u8,
}

impl<A: Allocator + Clone> External<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(ExternalOp::p, (NameString::p, byte_data, byte_data))
            .map(|(name, obj_type, argument_count)| Self {
                name,
                obj_type,
                argument_count,
            })
            .add_context("External")
            .parse(input, context, alloc)
    }
}
