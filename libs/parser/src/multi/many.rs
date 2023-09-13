use crate::{
    error::{ParseError, ParserError},
    parser::Parser,
};
use alloc::vec::Vec;
use core::alloc::Allocator;

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
        mut input: I,
        alloc: &'alloc A,
    ) -> crate::error::ParseResult<'alloc, I, Self::Output, Self::Error> {
        let mut outputs = Vec::new(alloc);
        for count in 0..self.max {
            match self.parser.parse(input.clone(), alloc) {
                Ok((next_input, output)) => {
                    // TOOD: remove this unwrap
                    outputs.push(output).unwrap();
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

        Ok((input, outputs))
    }
}
