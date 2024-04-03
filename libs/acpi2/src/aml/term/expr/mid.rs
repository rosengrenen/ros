use core::alloc::Allocator;

use crate::aml::context::Context;
use crate::aml::name::Target;
use crate::aml::ops::MidOp;
use crate::aml::parser::fail;
use crate::aml::parser::Input;
use crate::aml::parser::ParseResult;
use crate::aml::term::TermArg;

pub struct Mid<A: Allocator> {
    pub mid_obj: TermArg<A>,
    pub term1: TermArg<A>,
    pub term2: TermArg<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> Mid<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = MidOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (mid_obj, input) = TermArg::parse(input, context, alloc.clone())?;
        let (term1, input) = TermArg::parse(input, context, alloc.clone())?;
        let (term2, input) = TermArg::parse(input, context, alloc.clone())?;
        let (target, input) = Target::parse(input, context, alloc)?;
        let this = Self {
            mid_obj,
            term1,
            term2,
            target,
        };
        Ok((this, input))
    }
}

impl<A: Allocator> core::fmt::Debug for Mid<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Mid")
            .field("mid_obj", &self.mid_obj)
            .field("term1", &self.term1)
            .field("term2", &self.term2)
            .field("target", &self.target)
            .finish()
    }
}
