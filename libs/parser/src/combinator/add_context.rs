use crate::{
    error::{ParseError, ParseResult, ParserError},
    input::Input,
    parser::Parser,
};
use core::alloc::Allocator;

#[derive(Clone)]
pub struct AddContext<P> {
    pub(crate) parser: P,
    pub(crate) context: &'static str,
}

impl<I, O, E, C, P, A> Parser<I, C, A> for AddContext<P>
where
    I: Input,
    E: ParseError<I, A>,
    P: Parser<I, C, A, Output = O, Error = E>,
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
        match self.parser.parse(input.clone(), context, alloc.clone()) {
            Ok(result) => Ok(result),
            Err(error) => match error {
                ParserError::Error(error) => Err(ParserError::Error(error)),
                ParserError::Failure(error) => {
                    Err(ParserError::Failure(error.add_context(input, self.context)))
                }
            },
        }
    }
}
