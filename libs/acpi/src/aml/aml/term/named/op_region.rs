use crate::aml::{
    aml::{data::integer::byte_data, name::NameString, term::term_arg::TermArg},
    ops::OpRegionOp,
    prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct OpRegion<A: Allocator> {
    pub name: NameString<A>,
    pub space: RegionSpace,
    pub offset: TermArg<A>,
    pub len: TermArg<A>,
}

impl<A: Allocator + Clone> OpRegion<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let region_offset = TermArg::p; // => Integer
        let region_len = TermArg::p; // => Integer
        prefixed(
            OpRegionOp::p,
            (NameString::p, RegionSpace::p, region_offset, region_len),
        )
        .map(|(name, space, offset, len)| Self {
            name,
            space,
            offset,
            len,
        })
        .add_context("OpRegion")
        .parse(input, context, alloc)
    }
}

#[derive(Debug)]
pub struct RegionSpace(u8);

impl RegionSpace {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        byte_data
            .map(Self)
            .add_context("RegionSpace")
            .parse(input, context, alloc)
    }
}
