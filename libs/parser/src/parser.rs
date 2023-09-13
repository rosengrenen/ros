use core::{alloc::Allocator, marker::PhantomData};

use crate::{
    alloc::parser::ParserAlloc,
    branch::alt::{Alt, AltHelper},
    combinator::{
        cut::Cut,
        map::Map,
        map_err::MapErr,
        map_res::{MapRes, MapRes1},
        opt::Opt,
        or::Or,
    },
    error::{FromExternalError, ParseError, ParseResult},
};

pub trait Parser<I> {
    type Output;

    type Error: ParseError<I>;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output, Self::Error>;

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
        P: Parser<I, Output = Self::Output, Error = Self::Error>,
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

    fn alt(self) -> Alt<Self>
    where
        Self: Sized + AltHelper<I>,
    {
        Alt { parsers: self }
    }

    fn into_alloc<A>(self) -> IntoAlloc<Self, A>
    where
        Self: Sized,
        A: Allocator,
    {
        IntoAlloc {
            parser: self,
            alloc: PhantomData,
        }
    }
}

impl<I, O, E, F> Parser<I> for F
where
    E: ParseError<I>,
    F: Fn(I) -> ParseResult<I, O, E>,
{
    type Output = O;

    type Error = E;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output, Self::Error> {
        self(input)
    }
}

pub struct IntoAlloc<P, A> {
    parser: P,
    alloc: PhantomData<A>,
}

impl<'alloc, I, P, A> ParserAlloc<'alloc, I, A> for IntoAlloc<P, A>
where
    P: Parser<I>,
    A: Allocator,
{
    type Output = P::Output;

    type Error = P::Error;

    fn parse_alloc(
        &self,
        input: I,
        _alloc: &'alloc A,
    ) -> ParseResult<I, Self::Output, Self::Error> {
        self.parser.parse(input)
    }
}
