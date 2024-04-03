use core::alloc::Allocator;

use crate::aml::context::Context;
use crate::aml::name::SuperName;
use crate::aml::name::Target;
use crate::aml::ops::CondRefOfOp;
use crate::aml::parser::fail;
use crate::aml::parser::Input;
use crate::aml::parser::ParseResult;

pub struct CondRefOf<A: Allocator> {
    pub name: SuperName<A>,
    pub target: Target<A>,
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

impl<A: Allocator> core::fmt::Debug for CondRefOf<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CondRefOf")
            .field("name", &self.name)
            .field("target", &self.target)
            .finish()
    }
}
