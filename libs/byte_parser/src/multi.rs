use crate::{
    error::{ParseError, ParserError},
    parser::Parser,
};

pub fn fold<I, O, E, P, F, Init, Acc>(
    parser: P,
    init: Init,
    f: F,
) -> impl Parser<I, Output = Acc, Error = E>
where
    I: Clone,
    E: ParseError<I>,
    P: Parser<I, Output = O, Error = E>,
    F: Fn(Acc, O) -> Acc,
    Init: Fn() -> Acc,
{
    Fold { parser, init, f }
}

pub struct Fold<P, Init, F> {
    parser: P,
    init: Init,
    f: F,
}

impl<I, O, E, P, Init, Acc, F> Parser<I> for Fold<P, Init, F>
where
    I: Clone,
    E: ParseError<I>,
    P: Parser<I, Output = O, Error = E>,
    F: Fn(Acc, O) -> Acc,
    Init: Fn() -> Acc,
{
    type Output = Acc;

    type Error = E;

    fn parse(&self, mut input: I) -> crate::error::ParseResult<I, Self::Output, Self::Error> {
        let mut folded_output = (self.init)();

        loop {
            match self.parser.parse(input.clone()) {
                Ok((next_input, output)) => {
                    folded_output = (self.f)(folded_output, output);
                    input = next_input;
                }
                Err(ParserError::Error(_)) => {
                    return Ok((input, folded_output));
                }
                Err(ParserError::Failure(error)) => return Err(ParserError::Failure(error)),
            }
        }
    }
}
