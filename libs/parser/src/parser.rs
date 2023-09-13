use crate::{
    combinator::{
        cut::Cut,
        map::Map,
        map_res::{MapRes, MapRes1},
        opt::Opt,
        or::Or,
    },
    error::{FromExternalError, ParseError, ParseResult},
};
use core::{alloc::Allocator, marker::PhantomData};

use crate::branch::alt::{Alt, AltHelper};

pub trait Parser<'alloc, I, A>
where
    A: Allocator,
{
    type Output;

    type Error: ParseError<'alloc, I, A>;

    fn parse(
        &self,
        input: I,
        alloc: &'alloc A,
    ) -> ParseResult<'alloc, I, Self::Output, Self::Error>;

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
        Self::Error: FromExternalError<'alloc, I, E2, A>,
        F: Fn(Self::Output) -> Result<O2, E2>,
    {
        MapRes {
            parser: self,
            f,
            error: PhantomData,
        }
    }

    fn map_res1<O2, E2, F>(self, f: F) -> MapRes1<Self, F, E2>
    where
        Self: Sized,
        F: Fn(Self::Output) -> Result<O2, E2>,
    {
        MapRes1 {
            parser: self,
            f,
            error: PhantomData,
        }
    }

    fn or<P>(self, parser: P) -> Or<Self, P>
    where
        Self: Sized,
        P: Parser<'alloc, I, A, Output = Self::Output, Error = Self::Error>,
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
        Self: Sized + AltHelper<'alloc, I, A>,
    {
        Alt {
            parsers: self,
            alloc: PhantomData,
        }
    }
}

impl<'alloc, I, O, E, F, A> Parser<'alloc, I, A> for F
where
    E: ParseError<'alloc, I, A>,
    F: Fn(I, &'alloc A) -> ParseResult<'alloc, I, O, E>,
    A: Allocator + 'alloc,
{
    type Output = O;

    type Error = E;

    fn parse(
        &self,
        input: I,
        alloc: &'alloc A,
    ) -> ParseResult<'alloc, I, Self::Output, Self::Error> {
        self(input, alloc)
    }
}
