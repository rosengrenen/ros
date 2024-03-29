use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    name::SimpleName,
    ops::CopyObjOp,
    parser::{fail, Input, ParseResult},
    term::TermArg,
};

pub struct CopyObj<A: Allocator> {
    arg: TermArg<A>,
    name: SimpleName<A>,
}

impl<A: Allocator + Clone> CopyObj<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = CopyObjOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (arg, input) = TermArg::parse(input, context, alloc.clone())?;
        let (name, input) = SimpleName::parse(input, alloc)?;
        Ok((Self { arg, name }, input))
    }
}
