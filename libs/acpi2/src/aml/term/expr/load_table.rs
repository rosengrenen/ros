use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    ops::LoadTableOp,
    parser::{fail, Input, ParseResult},
    term::TermArg,
};

pub struct LoadTable<A: Allocator> {
    arg1: TermArg<A>,
    arg2: TermArg<A>,
    arg3: TermArg<A>,
    arg4: TermArg<A>,
    arg5: TermArg<A>,
    arg6: TermArg<A>,
}

impl<A: Allocator + Clone> LoadTable<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = LoadTableOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (arg1, input) = TermArg::parse(input, context, alloc.clone())?;
        let (arg2, input) = TermArg::parse(input, context, alloc.clone())?;
        let (arg3, input) = TermArg::parse(input, context, alloc.clone())?;
        let (arg4, input) = TermArg::parse(input, context, alloc.clone())?;
        let (arg5, input) = TermArg::parse(input, context, alloc.clone())?;
        let (arg6, input) = TermArg::parse(input, context, alloc)?;
        let this = Self {
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
            arg6,
        };
        Ok((this, input))
    }
}
