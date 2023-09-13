use crate::{
    alloc::parser::ParserAlloc,
    combinator::opt::Opt,
    error::{ParseError, ParseResult},
};
use core::alloc::Allocator;

impl<'alloc, I, O, E, P, A> ParserAlloc<'alloc, I, A> for Opt<P>
where
    I: Clone,
    E: ParseError<I>,
    P: ParserAlloc<'alloc, I, A, Output = O, Error = E>,
    A: Allocator,
{
    type Output = Option<P::Output>;

    type Error = P::Error;

    fn parse_alloc(&self, input: I, alloc: &'alloc A) -> ParseResult<I, Self::Output, Self::Error> {
        Self::inner(self.parser.parse_alloc(input.clone(), alloc), input)
    }
}
