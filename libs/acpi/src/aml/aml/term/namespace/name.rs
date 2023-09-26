use crate::aml::{
    aml::{data::DataRefObj, name::NameString},
    ops::NameOp,
    prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct Name<A: Allocator> {
    pub name: NameString<A>,
    pub data: DataRefObj<A>,
}

impl<A: Allocator + Clone> Name<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(NameOp::p, (NameString::p, DataRefObj::p))
            .map(|(name, data)| Self { name, data })
            .add_context("Name")
            .parse(input, context, alloc)
    }
}
