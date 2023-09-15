use crate::{
    error::{ParseError, ParseErrorKind, ParseResult, ParserError},
    input::Input,
    parser::Parser,
};
use core::alloc::Allocator;

pub struct Opt<P> {
    pub(crate) parser: P,
}

impl<I, O, E, P, A> Parser<I, A> for Opt<P>
where
    I: Input,
    E: ParseError<I, A>,
    P: Parser<I, A, Output = O, Error = E>,
    A: Allocator,
{
    type Output = Option<P::Output>;

    type Error = P::Error;

    fn parse(&self, input: I, alloc: A) -> ParseResult<I, Self::Output, Self::Error> {
        match self.parser.parse(input.clone(), alloc) {
            Ok((input, output)) => Ok((input, Some(output))),
            Err(ParserError::Error(_)) => Ok((input, None)),
            Err(ParserError::Failure(error)) => Err(ParserError::Failure(
                error.append(input, ParseErrorKind::Opt),
            )),
        }
    }
}
