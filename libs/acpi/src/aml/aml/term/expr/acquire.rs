use crate::aml::{
    aml::{data::byte_data, term::statement::MutexObj},
    ops::AcquireOp,
    prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub struct Acquire<A: Allocator> {
    pub mutex: MutexObj<A>,
    pub timeout: Timeout,
}

impl<A: Allocator + Clone> Acquire<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(AcquireOp::p, (MutexObj::p, Timeout::p))
            .map(|(mutex, timeout)| Self { mutex, timeout })
            .add_context("Acquire")
            .parse(input, context, alloc)
    }
}

#[derive(Debug)]
pub struct Timeout(u8);

impl Timeout {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        byte_data
            .map(Self)
            .add_context("Timeout")
            .parse(input, context, alloc)
    }
}
