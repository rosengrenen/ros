use crate::{
    error::{ParseError, ParseErrorKind, ParseResult, ParserError},
    input::Input,
    parser::Parser,
};
use core::alloc::Allocator;

#[derive(Clone)]
pub struct Opt<P> {
    pub(crate) parser: P,
}

impl<I, O, E, C, P, A> Parser<I, C, A> for Opt<P>
where
    I: Input,
    E: ParseError<I, A>,
    P: Parser<I, C, A, Output = O, Error = E>,
    A: Allocator,
{
    type Output = Option<P::Output>;

    type Error = P::Error;

    fn parse(
        &self,
        input: I,
        context: &mut C,
        alloc: A,
    ) -> ParseResult<I, Self::Output, Self::Error> {
        match self.parser.parse(input.clone(), context, alloc) {
            Ok((input, output)) => Ok((input, Some(output))),
            Err(ParserError::Error(_)) => Ok((input, None)),
            Err(ParserError::Failure(error)) => Err(ParserError::Failure(
                error.append(input, ParseErrorKind::Opt),
            )),
        }
    }
}
