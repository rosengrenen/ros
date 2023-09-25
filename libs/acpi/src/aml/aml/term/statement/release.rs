use crate::aml::{ops::ReleaseOp, prefixed, Context};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

use super::MutexObj;

#[derive(Debug)]
pub struct Release<A: Allocator> {
    pub mutex: MutexObj<A>,
}

impl<A: Allocator + Clone> Release<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(ReleaseOp::p, MutexObj::p)
            .map(|mutex| Self { mutex })
            .add_context("Release")
            .parse(input, context, alloc)
    }
}
