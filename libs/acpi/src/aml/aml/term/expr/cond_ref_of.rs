use crate::aml::{
    aml::name::{SuperName, Target},
    ops::CondRefOfOp,
    prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct CondRefOf<A: Allocator> {
    pub name: SuperName<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> CondRefOf<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(CondRefOfOp::p, (SuperName::p, Target::p))
            .map(|(name, target)| Self { name, target })
            .add_context("CondRefOf")
            .parse(input, context, alloc)
    }
}
