use crate::aml::{
    aml::{data::byte_data, term::TermArg},
    ops::MatchOp,
    prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
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
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(
            MatchOp::p,
            (
                TermArg::p,
                byte_data,
                TermArg::p,
                byte_data,
                TermArg::p,
                TermArg::p,
            ),
        )
        .map(
            |(search_pkg, left_match_opcode, left, right_match_opcode, right, start_index)| Self {
                search_pkg,
                left_match_opcode,
                left,
                right_match_opcode,
                right,
                start_index,
            },
        )
        .add_context("Match")
        .parse(input, context, alloc)
    }
}
