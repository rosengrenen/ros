use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    data::word_data,
    ops::AcquireOp,
    parser::{fail, Input, ParseResult},
    term::statement::MutexObj,
};

pub struct Acquire<A: Allocator> {
    mutex: MutexObj<A>,
    timeout: u16,
}

impl<A: Allocator + Clone> Acquire<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = AcquireOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (mutex, input) = MutexObj::parse(input, context, alloc)?;
        let (timeout, input) = word_data(input)?;
        Ok((Self { mutex, timeout }, input))
    }
}
