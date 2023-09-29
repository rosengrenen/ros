use crate::branch::alt::{Alt, AltHelper};
use crate::combinator::map_context::MapContext;
use crate::{
    combinator::{
        add_context::AddContext,
        and_then::AndThen,
        boxed::Boxed,
        cut::Cut,
        map::Map,
        map_res::{MapRes, MapRes1},
        opt::Opt,
        or::Or,
    },
    error::{FromExternalError, ParseError, ParseResult},
};
use core::{alloc::Allocator, marker::PhantomData};

pub trait Parser<I, C, A>: Clone
where
    A: Allocator,
{
    type Output;

    type Error: ParseError<I, A>;

    fn parse(
        self,
        input: I,
        context: &mut C,
        alloc: A,
    ) -> ParseResult<I, Self::Output, Self::Error>;

    fn map<F, O>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> O + Clone,
    {
        Map { parser: self, f }
    }

    fn map_context<F>(self, f: F) -> MapContext<Self, F>
    where
        Self: Sized,
        F: Fn(&Self::Output, &mut C) + Clone,
    {
        MapContext { parser: self, f }
    }

    fn map_res<O, E, F>(self, f: F) -> MapRes<Self, F, E>
    where
        Self: Sized,
        Self::Error: FromExternalError<I, E, A>,
        F: Fn(Self::Output) -> Result<O, E> + Clone,
        A: Clone,
    {
        MapRes {
            parser: self,
            f,
            error: PhantomData,
        }
    }

    fn map_res1<O, E, F>(self, f: F) -> MapRes1<Self, F, Self::Error>
    where
        Self: Sized,
        F: Fn(Self::Output) -> Result<O, E> + Clone,
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
        P: Parser<Self::Output, C, A, Output = O, Error = Self::Error>,
    {
        AndThen {
            first: self,
            second: parser,
        }
    }

    fn boxed(self) -> Boxed<Self>
    where
        Self: Sized,
    {
        Boxed { parser: self }
    }

    fn or<P>(self, parser: P) -> Or<Self, P>
    where
        Self: Sized,
        P: Parser<I, C, A, Output = Self::Output, Error = Self::Error>,
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
        Self: Sized + AltHelper<I, C, A>,
    {
        Alt {
            parsers: self,
            alloc: PhantomData,
        }
    }

    fn add_context(self, context: &'static str) -> AddContext<Self>
    where
        Self: Sized,
    {
        AddContext {
            parser: self,
            context,
        }
    }
}

impl<I, O, E, C, F, A> Parser<I, C, A> for F
where
    E: ParseError<I, A>,
    F: Fn(I, &mut C, A) -> ParseResult<I, O, E> + Clone,
    A: Allocator,
{
    type Output = O;

    type Error = E;

    fn parse(
        self,
        input: I,
        context: &mut C,
        alloc: A,
    ) -> ParseResult<I, Self::Output, Self::Error> {
        self(input, context, alloc)
    }
}
