use crate::{
    error::{ParseError, ParseErrorKind, ParseResult, ParserError},
    parser::Parser,
};
use core::alloc::Allocator;

pub struct Opt<P> {
    pub(crate) parser: P,
}

impl<'alloc, I, O, E, P, A> Parser<'alloc, I, A> for Opt<P>
where
    I: Clone,
    E: ParseError<'alloc, I, A>,
    P: Parser<'alloc, I, A, Output = O, Error = E>,
    A: Allocator,
{
    type Output = Option<P::Output>;

    type Error = P::Error;

    fn parse(
        &self,
        input: I,
        alloc: &'alloc A,
    ) -> ParseResult<'alloc, I, Self::Output, Self::Error> {
        match self.parser.parse(input.clone(), alloc) {
            Ok((input, output)) => Ok((input, Some(output))),
            Err(ParserError::Error(_)) => Ok((input, None)),
            Err(ParserError::Failure(error)) => Err(ParserError::Failure(
                error.append(input, ParseErrorKind::Opt),
            )),
        }
    }
}
