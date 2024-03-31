use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    data::byte_data,
    ops::MatchOp,
    parser::{fail, Input, ParseResult},
    term::TermArg,
};

pub struct Match<A: Allocator> {
    pub search_pkg: TermArg<A>,
    pub left_match_opcode: u8,
    pub left: TermArg<A>,
    pub right_match_opcode: u8,
    pub right: TermArg<A>,
    pub start_index: TermArg<A>,
}

impl<A: Allocator + Clone> Match<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = MatchOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (search_pkg, input) = TermArg::parse(input, context, alloc.clone())?;
        let (left_match_opcode, input) = byte_data(input)?;
        let (left, input) = TermArg::parse(input, context, alloc.clone())?;
        let (right_match_opcode, input) = byte_data(input)?;
        let (right, input) = TermArg::parse(input, context, alloc.clone())?;
        let (start_index, input) = TermArg::parse(input, context, alloc)?;
        let this = Self {
            search_pkg,
            left_match_opcode,
            left,
            right_match_opcode,
            right,
            start_index,
        };
        Ok((this, input))
    }
}

impl<A: Allocator> core::fmt::Debug for Match<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Match")
            .field("search_pkg", &self.search_pkg)
            .field("left_match_opcode", &self.left_match_opcode)
            .field("left", &self.left)
            .field("right_match_opcode", &self.right_match_opcode)
            .field("right", &self.right)
            .field("start_index", &self.start_index)
            .finish()
    }
}
