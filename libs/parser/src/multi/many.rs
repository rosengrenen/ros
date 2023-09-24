use crate::{
    error::{ParseError, ParseErrorKind, ParseResult},
    input::Input,
    parser::Parser,
};
use alloc::vec::Vec;
use core::alloc::Allocator;

use super::fold::FoldMN;

pub fn many<I, O, E, C, P, A>(parser: P) -> impl Parser<I, C, A, Output = Vec<O, A>, Error = E>
where
    I: Input,
    E: ParseError<I, A>,
    P: Parser<I, C, A, Output = O, Error = E>,
    A: Allocator + Clone,
{
    ManyMN {
        min: 0,
        max: usize::MAX,
        parser,
        kind: ParseErrorKind::Many,
    }
}

pub fn many1<I, O, E, C, P, A>(parser: P) -> impl Parser<I, C, A, Output = Vec<O, A>, Error = E>
where
    I: Input,
    E: ParseError<I, A>,
    P: Parser<I, C, A, Output = O, Error = E>,
    A: Allocator + Clone,
{
    ManyMN {
        min: 1,
        max: usize::MAX,
        parser,
        kind: ParseErrorKind::Many1,
    }
}

pub fn many_n<I, O, E, C, P, A>(
    count: usize,
    parser: P,
) -> impl Parser<I, C, A, Output = Vec<O, A>, Error = E>
where
    I: Input,
    E: ParseError<I, A>,
    P: Parser<I, C, A, Output = O, Error = E>,
    A: Allocator + Clone,
{
    ManyMN {
        min: count,
        max: count,
        parser,
        kind: ParseErrorKind::ManyN,
    }
}

pub fn many_m_n<I, O, E, C, P, A>(
    min: usize,
    max: usize,
    parser: P,
) -> impl Parser<I, C, A, Output = Vec<O, A>, Error = E>
where
    I: Input,
    E: ParseError<I, A>,
    P: Parser<I, C, A, Output = O, Error = E>,
    A: Allocator + Clone,
{
    ManyMN {
        min,
        max,
        parser,
        kind: ParseErrorKind::ManyMN,
    }
}

#[derive(Clone)]
pub struct ManyMN<P> {
    min: usize,
    max: usize,
    parser: P,
    kind: ParseErrorKind,
}

impl<I, E, C, P, A> Parser<I, C, A> for ManyMN<P>
where
    I: Input,
    E: ParseError<I, A>,
    P: Parser<I, C, A, Error = E>,
    A: Allocator + Clone,
{
    type Output = Vec<P::Output, A>;

    type Error = E;

    fn parse(
        self,
        input: I,
        context: &mut C,
        alloc: A,
    ) -> ParseResult<I, Self::Output, Self::Error> {
        FoldMN {
            min: self.min,
            max: self.max,
            parser: self.parser.clone(),
            init: || Vec::new(alloc.clone()),
            f: |mut outputs: Vec<P::Output, A>, output| {
                outputs.push(output).unwrap();
                outputs
            },
            kind: self.kind,
        }
        .parse(input, context, alloc.clone())
    }
}
