use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    data::word_data,
    ops::AcquireOp,
    parser::{fail, Input, ParseResult},
    term::statement::MutexObj,
};

pub struct Acquire<A: Allocator> {
    pub mutex: MutexObj<A>,
    pub timeout: u16,
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

impl<A: Allocator> core::fmt::Debug for Acquire<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Acquire")
            .field("mutex", &self.mutex)
            .field("timeout", &self.timeout)
            .finish()
    }
}
