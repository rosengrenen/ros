use crate::{
    error::{ParseErrorKind, ParseResult},
    parser::Parser,
};
use core::alloc::Allocator;

pub struct Map<P, F> {
    pub(crate) parser: P,
    pub(crate) f: F,
}

impl<'alloc, I, O, P, F, A> Parser<'alloc, I, A> for Map<P, F>
where
    I: Clone,
    P: Parser<'alloc, I, A>,
    F: Fn(P::Output) -> O,
    A: Allocator,
{
    type Output = O;

    type Error = P::Error;

    fn parse(
        &self,
        input: I,
        alloc: &'alloc A,
    ) -> ParseResult<'alloc, I, Self::Output, Self::Error> {
        self.parser
            .parse(input.clone(), alloc)
            .map(|(input, output)| (input, (self.f)(output)))
            .map_err(|error| error.append(input, ParseErrorKind::Map))
    }
}
