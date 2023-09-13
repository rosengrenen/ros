use crate::{error::ParseResult, parser::Parser};

pub struct Map<P, F> {
    pub(crate) parser: P,
    pub(crate) f: F,
}

impl<P, F> Map<P, F> {
    pub(crate) fn inner<I, O1, O2, E>(&self, result: ParseResult<I, O1, E>) -> ParseResult<I, O2, E>
    where
        F: Fn(O1) -> O2,
    {
        result.map(|(input, output)| (input, (self.f)(output)))
    }
}

impl<I, O, P, F> Parser<I> for Map<P, F>
where
    P: Parser<I>,
    F: Fn(P::Output) -> O,
{
    type Output = O;

    type Error = P::Error;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output, Self::Error> {
        self.inner(self.parser.parse(input))
    }
}
