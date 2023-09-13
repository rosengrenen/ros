use crate::{
    error::{ParseError, ParserError},
    parser::Parser,
};
use core::alloc::Allocator;

pub fn fold<'alloc, I, O, E, P, F, Init, Acc, A>(
    parser: P,
    init: Init,
    f: F,
) -> impl Parser<'alloc, I, A, Output = Acc, Error = E>
where
    I: Clone,
    E: ParseError<'alloc, I, A>,
    P: Parser<'alloc, I, A, Output = O, Error = E>,
    F: Fn(Acc, O) -> Acc,
    Init: Fn() -> Acc,
    A: Allocator,
{
    FoldMN {
        min: 0,
        max: usize::MAX,
        parser,
        init,
        f,
    }
}

pub fn fold1<'alloc, I, O, E, P, F, Init, Acc, A>(
    parser: P,
    init: Init,
    f: F,
) -> impl Parser<'alloc, I, A, Output = Acc, Error = E>
where
    I: Clone,
    E: ParseError<'alloc, I, A>,
    P: Parser<'alloc, I, A, Output = O, Error = E>,
    F: Fn(Acc, O) -> Acc,
    Init: Fn() -> Acc,
    A: Allocator,
{
    FoldMN {
        min: 1,
        max: usize::MAX,
        parser,
        init,
        f,
    }
}

pub fn fold_n<'alloc, I, O, E, P, F, Init, Acc, A>(
    count: usize,
    parser: P,
    init: Init,
    f: F,
) -> impl Parser<'alloc, I, A, Output = Acc, Error = E>
where
    I: Clone,
    E: ParseError<'alloc, I, A>,
    P: Parser<'alloc, I, A, Output = O, Error = E>,
    F: Fn(Acc, O) -> Acc,
    Init: Fn() -> Acc,
    A: Allocator,
{
    FoldMN {
        min: count,
        max: count,
        parser,
        init,
        f,
    }
}

pub fn fold_m_n<'alloc, I, O, E, P, F, Init, Acc, A>(
    min: usize,
    max: usize,
    parser: P,
    init: Init,
    f: F,
) -> impl Parser<'alloc, I, A, Output = Acc, Error = E>
where
    I: Clone,
    E: ParseError<'alloc, I, A>,
    P: Parser<'alloc, I, A, Output = O, Error = E>,
    F: Fn(Acc, O) -> Acc,
    Init: Fn() -> Acc,
    A: Allocator,
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

impl<'alloc, I, O, E, P, Init, Acc, F, A> Parser<'alloc, I, A> for FoldMN<P, Init, F>
where
    I: Clone,
    E: ParseError<'alloc, I, A>,
    P: Parser<'alloc, I, A, Output = O, Error = E>,
    F: Fn(Acc, O) -> Acc,
    Init: Fn() -> Acc,
    A: Allocator,
{
    type Output = Acc;

    type Error = E;

    fn parse(
        &self,
        mut input: I,
        alloc: &'alloc A,
    ) -> crate::error::ParseResult<'alloc, I, Self::Output, Self::Error> {
        let mut folded_output = (self.init)();
        for count in 0..self.max {
            match self.parser.parse(input.clone(), alloc) {
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
