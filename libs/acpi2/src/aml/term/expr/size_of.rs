use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    name::SuperName,
    ops::SizeOfOp,
    parser::{fail, Input, ParseResult},
};

pub struct SizeOf<A: Allocator> {
    pub name: SuperName<A>,
}

impl<A: Allocator + Clone> SizeOf<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = SizeOfOp::parse(input)?;
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

impl<A: Allocator> core::fmt::Debug for SizeOf<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SizeOf").field("name", &self.name).finish()
    }
}
