use crate::{
    error::{ParseErrorKind, ParseResult},
    input::Input,
    parser::Parser,
};
use core::alloc::Allocator;

pub struct Map<P, F> {
    pub(crate) parser: P,
    pub(crate) f: F,
}

impl<I, O, P, F, A> Parser<I, A> for Map<P, F>
where
    I: Input,
    P: Parser<I, A>,
    F: Fn(P::Output) -> O,
    A: Allocator,
{
    type Output = O;

    type Error = P::Error;

    fn parse(&self, input: I, alloc: A) -> ParseResult<I, Self::Output, Self::Error> {
        self.parser
            .parse(input.clone(), alloc)
            .map(|(input, output)| (input, (self.f)(output)))
            .map_err(|error| error.append(input, ParseErrorKind::Map))
    }
}
