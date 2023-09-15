use crate::{
    error::{ParseError, ParseErrorKind, ParserError},
    input::Input,
    parser::Parser,
};
use alloc::vec::Vec;
use core::{alloc::Allocator, ops::ControlFlow};

pub fn many<I, O, E, P, A>(parser: P) -> impl Parser<I, A, Output = Vec<O, A>, Error = E>
where
    I: Input,
    E: ParseError<I, A>,
    P: Parser<I, A, Output = O, Error = E>,
    A: Allocator + Clone,
{
    ManyMN {
        min: 0,
        max: usize::MAX,
        parser,
        kind: ParseErrorKind::Many,
    }
}

pub fn many1<I, O, E, P, A>(parser: P) -> impl Parser<I, A, Output = Vec<O, A>, Error = E>
where
    I: Input,
    E: ParseError<I, A>,
    P: Parser<I, A, Output = O, Error = E>,
    A: Allocator + Clone,
{
    ManyMN {
        min: 1,
        max: usize::MAX,
        parser,
        kind: ParseErrorKind::Many1,
    }
}

pub fn many_n<I, O, E, P, A>(
    count: usize,
    parser: P,
) -> impl Parser<I, A, Output = Vec<O, A>, Error = E>
where
    I: Input,
    E: ParseError<I, A>,
    P: Parser<I, A, Output = O, Error = E>,
    A: Allocator + Clone,
{
    ManyMN {
        min: count,
        max: count,
        parser,
        kind: ParseErrorKind::ManyN,
    }
}

pub fn many_m_n<I, O, E, P, A>(
    min: usize,
    max: usize,
    parser: P,
) -> impl Parser<I, A, Output = Vec<O, A>, Error = E>
where
    I: Input,
    E: ParseError<I, A>,
    P: Parser<I, A, Output = O, Error = E>,
    A: Allocator + Clone,
{
    ManyMN {
        min,
        max,
        parser,
        kind: ParseErrorKind::ManyMN,
    }
}

// TODO: error kind
pub struct ManyMN<P> {
    min: usize,
    max: usize,
    parser: P,
    kind: ParseErrorKind,
}

impl<I, E, P, A> Parser<I, A> for ManyMN<P>
where
    I: Input,
    E: ParseError<I, A>,
    P: Parser<I, A, Error = E>,
    A: Allocator + Clone,
{
    type Output = Vec<P::Output, A>;

    type Error = E;

    fn parse(&self, input: I, alloc: A) -> crate::error::ParseResult<I, Self::Output, Self::Error> {
        let control_flow = (0..self.max).try_fold(
            (input, Vec::new(alloc.clone())),
            |(input, mut outputs), count| {
                match self.parser.parse(input.clone(), alloc.clone()) {
                    Ok((input, output)) => {
                        // TOOD: remove this unwrap
                        outputs.push(output).unwrap();
                        ControlFlow::Continue((input, outputs))
                    }
                    Err(ParserError::Error(_)) => {
                        if count < self.min {
                            ControlFlow::Break(Err(ParserError::Error(E::from_error_kind(
                                input,
                                self.kind,
                                alloc.clone(),
                            ))))
                        } else {
                            ControlFlow::Break(Ok((input, outputs)))
                        }
                    }
                    Err(ParserError::Failure(error)) => ControlFlow::Break(Err(
                        ParserError::Failure(error.append(input, self.kind)),
                    )),
                }
            },
        );
        match control_flow {
            ControlFlow::Continue(res) => Ok(res),
            ControlFlow::Break(res) => res,
        }
    }
}
