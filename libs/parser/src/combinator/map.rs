use crate::{
    error::{ParseErrorKind, ParseResult},
    input::Input,
    parser::Parser,
};
use core::alloc::Allocator;

#[derive(Clone)]
pub struct Map<P, F> {
    pub(crate) parser: P,
    pub(crate) f: F,
}

impl<I, O, C, P, F, A> Parser<I, C, A> for Map<P, F>
where
    I: Input,
    P: Parser<I, C, A>,
    F: Fn(P::Output) -> O + Clone,
    A: Allocator,
{
    type Output = O;

    type Error = P::Error;

    fn parse(
        self,
        input: I,
        context: &mut C,
        alloc: A,
    ) -> ParseResult<I, Self::Output, Self::Error> {
        self.parser
            .parse(input.clone(), context, alloc)
            .map(|(input, output)| (input, (self.f)(output)))
            .map_err(|error| error.append(input, ParseErrorKind::Map))
    }
}
