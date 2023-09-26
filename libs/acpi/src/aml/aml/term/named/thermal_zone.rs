use crate::aml::{
    aml::{name::NameString, term::term_obj::TermObj},
    ops::ThermalZoneOp,
    pkg, prefixed, Context,
};
use alloc::vec::Vec;
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    multi::many::many,
    parser::Parser,
};

#[derive(Debug)]
pub struct ThermalZone<A: Allocator> {
    pub name: NameString<A>,
    pub terms: Vec<TermObj<A>, A>,
}

impl<A: Allocator + Clone> ThermalZone<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(ThermalZoneOp::p, pkg((NameString::p, many(TermObj::p))))
            .map(|(name, terms)| Self { name, terms })
            .add_context("ThermalZone")
            .parse(input, context, alloc)
    }
}
