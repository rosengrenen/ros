use core::alloc::Allocator;
use core::ops::ControlFlow;

use crate::error::ParseError;
use crate::error::ParseErrorKind;
use crate::error::ParserError;
use crate::input::Input;
use crate::parser::Parser;

pub fn fold<I, O, E, C, P, F, Init, Acc, A>(
    parser: P,
    init: Init,
    f: F,
) -> impl Parser<I, C, A, Output = Acc, Error = E>
where
    I: Input,
    E: ParseError<I, A>,
    P: Parser<I, C, A, Output = O, Error = E>,
    F: Fn(Acc, O) -> Acc + Clone,
    Init: Fn() -> Acc + Clone,
    A: Allocator + Clone,
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

pub fn fold1<I, O, E, C, P, F, Init, Acc, A>(
    parser: P,
    init: Init,
    f: F,
) -> impl Parser<I, C, A, Output = Acc, Error = E>
where
    I: Input,
    E: ParseError<I, A>,
    P: Parser<I, C, A, Output = O, Error = E>,
    F: Fn(Acc, O) -> Acc + Clone,
    Init: Fn() -> Acc + Clone,
    A: Allocator + Clone,
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

pub fn fold_n<I, O, E, C, P, F, Init, Acc, A>(
    count: usize,
    parser: P,
    init: Init,
    f: F,
) -> impl Parser<I, C, A, Output = Acc, Error = E>
where
    I: Input,
    E: ParseError<I, A>,
    P: Parser<I, C, A, Output = O, Error = E>,
    F: Fn(Acc, O) -> Acc + Clone,
    Init: Fn() -> Acc + Clone,
    A: Allocator + Clone,
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

pub fn fold_m_n<I, O, E, C, P, F, Init, Acc, A>(
    min: usize,
    max: usize,
    parser: P,
    init: Init,
    f: F,
) -> impl Parser<I, C, A, Output = Acc, Error = E>
where
    I: Input,
    E: ParseError<I, A>,
    P: Parser<I, C, A, Output = O, Error = E>,
    F: Fn(Acc, O) -> Acc + Clone,
    Init: Fn() -> Acc + Clone,
    A: Allocator + Clone,
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

#[derive(Clone)]
pub struct FoldMN<P, Init, F> {
    pub(crate) min: usize,
    pub(crate) max: usize,
    pub(crate) parser: P,
    pub(crate) init: Init,
    pub(crate) f: F,
    pub(crate) kind: ParseErrorKind,
}

impl<I, O, E, C, P, Init, Acc, F, A> Parser<I, C, A> for FoldMN<P, Init, F>
where
    I: Input,
    E: ParseError<I, A>,
    P: Parser<I, C, A, Output = O, Error = E>,
    F: Fn(Acc, O) -> Acc + Clone,
    Init: Fn() -> Acc + Clone,
    A: Allocator + Clone,
{
    type Output = Acc;

    type Error = E;

    fn parse(
        self,
        input: I,
        context: &mut C,
        alloc: A,
    ) -> crate::error::ParseResult<I, Self::Output, Self::Error> {
        let control_flow =
            (0..self.max).try_fold((input, (self.init)()), |(input, folded_outputs), count| {
                match self
                    .parser
                    .clone()
                    .parse(input.clone(), context, alloc.clone())
                {
                    Ok((input, output)) => {
                        ControlFlow::Continue((input, (self.f)(folded_outputs, output)))
                    }
                    Err(ParserError::Error(_)) => {
                        if count < self.min {
                            ControlFlow::Break(Err(ParserError::Error(E::from_error_kind(
                                input,
                                self.kind,
                                alloc.clone(),
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
