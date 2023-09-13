use crate::{
    combinator::{cut::Cut, map::Map, map_err::MapErr, map_res::MapRes, opt::Opt, or::Or},
    error::{FromExternalError, ParseError, ParseResult},
};
use core::{alloc::Allocator, marker::PhantomData};

use super::branch::alt::{Alt, AltHelperAlloc};

pub trait ParserAlloc<'alloc, I, A>
where
    A: Allocator,
{
    type Output;

    type Error: ParseError<I>;

    fn parse_alloc(&self, input: I, alloc: &'alloc A) -> ParseResult<I, Self::Output, Self::Error>;

    fn map<F, O2>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> O2,
    {
        Map { parser: self, f }
    }

    fn map_res<O2, E2, F>(self, f: F) -> MapRes<Self, F, E2>
    where
        Self: Sized,
        Self::Error: FromExternalError<I, E2>,
        F: Fn(Self::Output) -> Result<O2, E2>,
    {
        MapRes {
            parser: self,
            f,
            error: PhantomData,
        }
    }

    fn map_err<F>(self, f: F) -> MapErr<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Error) -> Self::Error,
    {
        MapErr { parser: self, f }
    }

    fn or<P>(self, parser: P) -> Or<Self, P>
    where
        Self: Sized,
        P: ParserAlloc<'alloc, I, A, Output = Self::Output, Error = Self::Error>,
    {
        Or {
            first: self,
            second: parser,
        }
    }

    fn opt(self) -> Opt<Self>
    where
        Self: Sized,
    {
        Opt { parser: self }
    }

    fn cut(self) -> Cut<Self>
    where
        Self: Sized,
    {
        Cut { parser: self }
    }

    fn alt(self) -> Alt<Self, A>
    where
        Self: Sized + AltHelperAlloc<'alloc, I, A>,
    {
        Alt {
            parsers: self,
            alloc: PhantomData,
        }
    }
}

impl<'alloc, I, O, E, F, A> ParserAlloc<'alloc, I, A> for F
where
    E: ParseError<I>,
    F: Fn(I, &'alloc A) -> ParseResult<I, O, E>,
    A: Allocator + 'alloc,
{
    type Output = O;

    type Error = E;

    fn parse_alloc(&self, input: I, alloc: &'alloc A) -> ParseResult<I, Self::Output, Self::Error> {
        self(input, alloc)
    }
}
