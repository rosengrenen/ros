use crate::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};
use core::alloc::Allocator;

#[derive(Clone)]
pub struct MapContext<P, F> {
    pub(crate) parser: P,
    pub(crate) f: F,
}

impl<I, O, E, C, P, F, A> Parser<I, C, A> for MapContext<P, F>
where
    I: Input,
    E: ParseError<I, A>,
    P: Parser<I, C, A, Output = O, Error = E>,
    F: Fn(&O, &mut C) + Clone,
    A: Allocator + Clone,
{
    type Output = O;

    type Error = E;

    fn parse(
        self,
        input: I,
        context: &mut C,
        alloc: A,
    ) -> ParseResult<I, Self::Output, Self::Error> {
        let (input, output) = self.parser.parse(input, context, alloc)?;
        (self.f)(&output, context);
        Ok((input, output))
    }
}
