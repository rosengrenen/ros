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
    FoldMN {
        min: 0,
        max: usize::MAX,
        parser,
        init,
        f,
    }
}

pub fn fold1<I, O, E, P, F, Init, Acc>(
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
    FoldMN {
        min: 1,
        max: usize::MAX,
        parser,
        init,
        f,
    }
}

pub fn fold_n<I, O, E, P, F, Init, Acc>(
    count: usize,
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
    FoldMN {
        min: count,
        max: count,
        parser,
        init,
        f,
    }
}

pub fn fold_m_n<I, O, E, P, F, Init, Acc>(
    min: usize,
    max: usize,
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
    FoldMN {
        min,
        max,
        parser,
        init,
        f,
    }
}

// TODO: error kind
pub struct FoldMN<P, Init, F> {
    min: usize,
    max: usize,
    parser: P,
    init: Init,
    f: F,
}

impl<I, O, E, P, Init, Acc, F> Parser<I> for FoldMN<P, Init, F>
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
        for count in 0..self.max {
            match self.parser.parse(input.clone()) {
                Ok((next_input, output)) => {
                    folded_output = (self.f)(folded_output, output);
                    input = next_input;
                }
                Err(ParserError::Error(error)) => {
                    if count < self.min {
                        return Err(ParserError::Error(error));
                    }

                    break;
                }
                Err(ParserError::Failure(error)) => return Err(ParserError::Failure(error)),
            }
        }

        Ok((input, folded_output))
    }
}
