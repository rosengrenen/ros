use alloc::iter::IteratorCollectIn;
use alloc::vec::Vec;
use core::alloc::Allocator;

use crate::aml::context::Context;
use crate::aml::ops::BufferOp;
use crate::aml::parser::fail;
use crate::aml::parser::fail_if_not_empty;
use crate::aml::parser::take;
use crate::aml::parser::Input;
use crate::aml::parser::ParseResult;
use crate::aml::pkg_len::pkg;
use crate::aml::term::TermArg;

pub struct Buffer<A: Allocator> {
    pub len: TermArg<A>,
    pub bytes: Vec<u8, A>,
}

impl<A: Allocator + Clone> Buffer<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = BufferOp::parse(input)?;
        let (pkg_input, input) = pkg(input)?;
        let (buffer, pkg_input) = fail(Self::parse_inner(pkg_input, context, alloc))?;
        fail_if_not_empty(pkg_input)?;
        Ok((buffer, input))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (len, input) = TermArg::parse(input, context, alloc.clone())?;
        let input_len = input.bytes.len();
        let (bytes, input) = take(input, input_len)?;
        let bytes = bytes.iter().copied().collect_in(alloc).unwrap();
        Ok((Self { len, bytes }, input))
    }
}

impl<A: Allocator> core::fmt::Debug for Buffer<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Buffer")
            .field("len", &self.len)
            .field("bytes", &self.bytes)
            .finish()
    }
}
