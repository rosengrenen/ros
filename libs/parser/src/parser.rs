use crate::{
    combinator::{
        and_then::AndThen,
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

pub trait Parser<I, A>
where
    A: Allocator,
{
    type Output;

    type Error: ParseError<I, A>;

    fn parse(&self, input: I, alloc: A) -> ParseResult<I, Self::Output, Self::Error>;

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
        Self::Error: FromExternalError<I, E2, A>,
        F: Fn(Self::Output) -> Result<O2, E2>,
        A: Clone,
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
        A: Clone,
    {
        MapRes1 {
            parser: self,
            f,
            error: PhantomData,
        }
    }

    fn and_then<O, P>(self, parser: P) -> AndThen<Self, P>
    where
        Self: Sized,
        P: Parser<Self::Output, A, Output = O, Error = Self::Error>,
    {
        AndThen {
            first: self,
            second: parser,
        }
    }

    fn or<P>(self, parser: P) -> Or<Self, P>
    where
        Self: Sized,
        P: Parser<I, A, Output = Self::Output, Error = Self::Error>,
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
        Self: Sized + AltHelper<I, A>,
    {
        Alt {
            parsers: self,
            alloc: PhantomData,
        }
    }
}

impl<I, O, E, F, A> Parser<I, A> for F
where
    E: ParseError<I, A>,
    F: Fn(I, A) -> ParseResult<I, O, E>,
    A: Allocator,
{
    type Output = O;

    type Error = E;

    fn parse(&self, input: I, alloc: A) -> ParseResult<I, Self::Output, Self::Error> {
        self(input, alloc)
    }
}
