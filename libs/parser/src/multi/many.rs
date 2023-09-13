use crate::{
    error::{ParseError, ParserError},
    parser::Parser,
};
use alloc::vec::Vec;
use core::{alloc::Allocator, ops::ControlFlow};

pub fn many<'alloc, I, O, E, P, A>(
    parser: P,
) -> impl Parser<'alloc, I, A, Output = Vec<'alloc, O, A>, Error = E>
where
    I: Clone,
    E: ParseError<'alloc, I, A>,
    P: Parser<'alloc, I, A, Output = O, Error = E>,
    A: Allocator + 'alloc,
{
    ManyMN {
        min: 0,
        max: usize::MAX,
        parser,
    }
}

pub fn many1<'alloc, I, O, E, P, A>(
    parser: P,
) -> impl Parser<'alloc, I, A, Output = Vec<'alloc, O, A>, Error = E>
where
    I: Clone,
    E: ParseError<'alloc, I, A>,
    P: Parser<'alloc, I, A, Output = O, Error = E>,
    A: Allocator + 'alloc,
{
    ManyMN {
        min: 1,
        max: usize::MAX,
        parser,
    }
}

pub fn many_n<'alloc, I, O, E, P, A>(
    count: usize,
    parser: P,
) -> impl Parser<'alloc, I, A, Output = Vec<'alloc, O, A>, Error = E>
where
    I: Clone,
    E: ParseError<'alloc, I, A>,
    P: Parser<'alloc, I, A, Output = O, Error = E>,
    A: Allocator + 'alloc,
{
    ManyMN {
        min: count,
        max: count,
        parser,
    }
}

pub fn many_m_n<'alloc, I, O, E, P, A>(
    min: usize,
    max: usize,
    parser: P,
) -> impl Parser<'alloc, I, A, Output = Vec<'alloc, O, A>, Error = E>
where
    I: Clone,
    E: ParseError<'alloc, I, A>,
    P: Parser<'alloc, I, A, Output = O, Error = E>,
    A: Allocator + 'alloc,
{
    ManyMN { min, max, parser }
}

// TODO: error kind
pub struct ManyMN<P> {
    min: usize,
    max: usize,
    parser: P,
}

impl<'alloc, I, A, P> Parser<'alloc, I, A> for ManyMN<P>
where
    I: Clone,
    P: Parser<'alloc, I, A>,
    A: Allocator + 'alloc,
{
    type Output = Vec<'alloc, P::Output, A>;

    type Error = P::Error;

    fn parse(
        &self,
        input: I,
        alloc: &'alloc A,
    ) -> crate::error::ParseResult<'alloc, I, Self::Output, Self::Error> {
        let control_flow =
            (0..self.max).try_fold((input, Vec::new(alloc)), |(input, mut outputs), count| {
                match self.parser.parse(input.clone(), alloc) {
                    Ok((input, output)) => {
                        // TOOD: remove this unwrap
                        outputs.push(output).unwrap();
                        ControlFlow::Continue((input, outputs))
                    }
                    Err(ParserError::Error(error)) => {
                        if count < self.min {
                            ControlFlow::Break(Err(ParserError::Error(error)))
                        } else {
                            ControlFlow::Break(Ok((input, outputs)))
                        }
                    }
                    Err(ParserError::Failure(error)) => {
                        ControlFlow::Break(Err(ParserError::Failure(error)))
                    }
                }
            });
        match control_flow {
            ControlFlow::Continue(res) => Ok(res),
            ControlFlow::Break(res) => res,
        }
    }
}
