use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    name::Target,
    ops::IndexOp,
    parser::{fail, Input, ParseResult},
    term::TermArg,
};

pub struct Index<A: Allocator> {
    buf_pkg_str_obj: TermArg<A>,
    index_value: TermArg<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> Index<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = IndexOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (buf_pkg_str_obj, input) = TermArg::parse(input, context, alloc.clone())?;
        let (index_value, input) = TermArg::parse(input, context, alloc.clone())?;
        let (target, input) = Target::parse(input, context, alloc)?;
        let this = Self {
            buf_pkg_str_obj,
            index_value,
            target,
        };
        Ok((this, input))
    }
}
