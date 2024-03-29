use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    name::SuperName,
    ops::RefOfOp,
    parser::{fail, Input, ParseResult},
};

pub struct RefOf<A: Allocator> {
    name: SuperName<A>,
}

impl<A: Allocator + Clone> RefOf<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = RefOfOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (name, input) = SuperName::parse(input, context, alloc)?;
        Ok((Self { name }, input))
    }
}
