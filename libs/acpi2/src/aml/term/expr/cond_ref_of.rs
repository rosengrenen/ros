use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    name::{SuperName, Target},
    ops::CondRefOfOp,
    parser::{fail, Input, ParseResult},
};

pub struct CondRefOf<A: Allocator> {
    name: SuperName<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> CondRefOf<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = CondRefOfOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (name, input) = SuperName::parse(input, context, alloc.clone())?;
        let (target, input) = Target::parse(input, context, alloc)?;
        Ok((Self { name, target }, input))
    }
}
