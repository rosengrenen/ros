use super::util::iter_to_array_unchecked;
use crate::{
    error::{ParseError, ParseErrorKind, ParseResult},
    input::Input,
    parser::Parser,
};
use core::{alloc::Allocator, marker::PhantomData};

pub fn take<I, E, C, A>(count: usize) -> impl Parser<I, C, A, Output = I, Error = E>
where
    I: Input,
    E: ParseError<I, A>,
    A: Allocator,
{
    Take {
        count,
        error: PhantomData,
    }
}

#[derive(Clone)]
pub struct Take<E> {
    count: usize,
    error: PhantomData<E>,
}

impl<I, E, C, A> Parser<I, C, A> for Take<E>
where
    I: Input,
    E: ParseError<I, A>,
    A: Allocator,
{
    type Output = I;

    type Error = E;

    fn parse(
        self,
        input: I,
        _context: &mut C,
        alloc: A,
    ) -> ParseResult<I, Self::Output, Self::Error> {
        input.split_at_index(self.count, ParseErrorKind::Take, alloc)
    }
}

pub fn take_const<const COUNT: usize, I, E, C, A>(
) -> impl Parser<I, C, A, Output = [I::Item; COUNT], Error = E>
where
    I: Input,
    E: ParseError<I, A>,
    A: Allocator,
{
    TakeConst { error: PhantomData }
}

#[derive(Clone)]
pub struct TakeConst<const COUNT: usize, E> {
    error: PhantomData<E>,
}

impl<const COUNT: usize, I, E, C, A> Parser<I, C, A> for TakeConst<COUNT, E>
where
    I: Input,
    E: ParseError<I, A>,
    A: Allocator,
{
    type Output = [I::Item; COUNT];

    type Error = E;

    fn parse(
        self,
        input: I,
        context: &mut C,
        alloc: A,
    ) -> ParseResult<I, Self::Output, Self::Error> {
        take(COUNT)
            .map(|output: I| iter_to_array_unchecked(output.item_iter()))
            .parse(input.clone(), context, alloc)
            .map_err(|error| error.append(input, ParseErrorKind::TakeConst))
    }
}

pub fn take_while<I, E, C, P, A>(pred: P) -> impl Parser<I, C, A, Output = I, Error = E>
where
    I: Input,
    E: ParseError<I, A>,
    P: Fn(I::Item) -> bool + Clone,
    A: Allocator,
{
    TakeWhileMN {
        min: 0,
        max: usize::MAX,
        pred,
        kind: ParseErrorKind::TakeWhile,
        error: PhantomData,
    }
}

pub fn take_while1<I, E, C, P, A>(pred: P) -> impl Parser<I, C, A, Output = I, Error = E>
where
    I: Input,
    E: ParseError<I, A>,
    P: Fn(I::Item) -> bool + Clone,
    A: Allocator,
{
    TakeWhileMN {
        min: 1,
        max: usize::MAX,
        pred,
        kind: ParseErrorKind::TakeWhile1,
        error: PhantomData,
    }
}

pub fn take_while_n<I, E, C, P, A>(n: usize, pred: P) -> impl Parser<I, C, A, Output = I, Error = E>
where
    I: Input,
    E: ParseError<I, A>,
    P: Fn(I::Item) -> bool + Clone,
    A: Allocator,
{
    TakeWhileMN {
        min: n,
        max: n,
        pred,
        kind: ParseErrorKind::TakeWhileN,
        error: PhantomData,
    }
}

pub fn take_while_m_n<I, E, C, P, A>(
    min: usize,
    max: usize,
    pred: P,
) -> impl Parser<I, C, A, Output = I, Error = E>
where
    I: Input,
    E: ParseError<I, A>,
    P: Fn(I::Item) -> bool + Clone,
    A: Allocator,
{
    TakeWhileMN {
        min,
        max,
        pred,
        kind: ParseErrorKind::TakeWhileMN,
        error: PhantomData,
    }
}

#[derive(Clone)]
pub struct TakeWhileMN<P, E> {
    min: usize,
    max: usize,
    pred: P,
    kind: ParseErrorKind,
    error: PhantomData<E>,
}

impl<I, E, C, P, A> Parser<I, C, A> for TakeWhileMN<P, E>
where
    I: Input,
    E: ParseError<I, A>,
    P: Fn(I::Item) -> bool + Clone,
    A: Allocator,
{
    type Output = I;

    type Error = E;

    fn parse(
        self,
        input: I,
        _context: &mut C,
        alloc: A,
    ) -> ParseResult<I, Self::Output, Self::Error> {
        input.split_at_position_m_n(
            self.min,
            self.max,
            |item| !(self.pred)(item),
            self.kind,
            alloc,
        )
    }
}
