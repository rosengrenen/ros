use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    name::Target,
    ops::IndexOp,
    parser::{fail, Input, ParseResult},
    term::TermArg,
};

pub struct Index<A: Allocator> {
    pub buf_pkg_str_obj: TermArg<A>,
    pub index_value: TermArg<A>,
    pub target: Target<A>,
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

impl<A: Allocator> core::fmt::Debug for Index<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Index")
            .field("buf_pkg_str_obj", &self.buf_pkg_str_obj)
            .field("index_value", &self.index_value)
            .field("target", &self.target)
            .finish()
    }
}
