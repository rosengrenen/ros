use crate::{
    error::{ParseError, ParseErrorKind, ParserError},
    parser::Parser,
};
use core::{alloc::Allocator, ops::ControlFlow};

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
        kind: ParseErrorKind::Fold,
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
        kind: ParseErrorKind::Fold1,
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
        kind: ParseErrorKind::FoldN,
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
        kind: ParseErrorKind::FoldMN,
    }
}

// TODO: error kind
pub struct FoldMN<P, Init, F> {
    min: usize,
    max: usize,
    parser: P,
    init: Init,
    f: F,
    kind: ParseErrorKind,
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
        input: I,
        alloc: &'alloc A,
    ) -> crate::error::ParseResult<'alloc, I, Self::Output, Self::Error> {
        let control_flow =
            (0..self.max).try_fold((input, (self.init)()), |(input, folded_outputs), count| {
                match self.parser.parse(input.clone(), alloc) {
                    Ok((input, output)) => {
                        ControlFlow::Continue((input, (self.f)(folded_outputs, output)))
                    }
                    Err(ParserError::Error(_)) => {
                        if count < self.min {
                            ControlFlow::Break(Err(ParserError::Error(E::from_error_kind(
                                input, self.kind, alloc,
                            ))))
                        } else {
                            ControlFlow::Break(Ok((input, folded_outputs)))
                        }
                    }
                    Err(ParserError::Failure(error)) => ControlFlow::Break(Err(
                        ParserError::Failure(error.append(input, self.kind)),
                    )),
                }
            });
        match control_flow {
            ControlFlow::Continue(res) => Ok(res),
            ControlFlow::Break(res) => res,
        }
    }
}
