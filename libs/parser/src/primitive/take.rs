use super::util::iter_to_array_unchecked;
use crate::{
    error::{ParseError, ParseErrorKind, ParseResult},
    input::Input,
    parser::Parser,
};
use core::{alloc::Allocator, marker::PhantomData};

pub fn take<'alloc, I, E, A>(count: usize) -> impl Parser<'alloc, I, A, Output = I, Error = E>
where
    I: Input,
    E: ParseError<'alloc, I, A>,
    A: Allocator,
{
    Take {
        count,
        error: PhantomData,
    }
}

pub struct Take<E> {
    count: usize,
    error: PhantomData<E>,
}

impl<'alloc, I, E, A> Parser<'alloc, I, A> for Take<E>
where
    I: Input,
    E: ParseError<'alloc, I, A>,
    A: Allocator,
{
    type Output = I;

    type Error = E;

    fn parse(
        &self,
        input: I,
        _alloc: &'alloc A,
    ) -> ParseResult<'alloc, I, Self::Output, Self::Error> {
        let (output, input) = input.split_at_index(self.count);
        Ok((input, output))
    }
}

pub fn take_const<'alloc, const C: usize, I, E, A>(
) -> impl Parser<'alloc, I, A, Output = [I::Item; C], Error = E>
where
    I: Input,
    E: ParseError<'alloc, I, A>,
    A: Allocator,
{
    TakeConst { error: PhantomData }
}

pub struct TakeConst<const C: usize, E> {
    error: PhantomData<E>,
}

impl<'alloc, const C: usize, I, E, A> Parser<'alloc, I, A> for TakeConst<C, E>
where
    I: Input,
    E: ParseError<'alloc, I, A>,
    A: Allocator,
{
    type Output = [I::Item; C];

    type Error = E;

    fn parse(
        &self,
        input: I,
        alloc: &'alloc A,
    ) -> ParseResult<'alloc, I, Self::Output, Self::Error> {
        take(C)
            .map(|output: I| iter_to_array_unchecked(output.item_iter()))
            .parse(input, alloc)
    }
}

pub fn take_while<'alloc, I, E, P, A>(pred: P) -> impl Parser<'alloc, I, A, Output = I, Error = E>
where
    I: Input,
    E: ParseError<'alloc, I, A>,
    P: Fn(I::Item) -> bool,
    A: Allocator,
{
    TakeWhileMN {
        min: 0,
        max: usize::MAX,
        pred,
        kind: ParseErrorKind::None,
        error: PhantomData,
    }
}

pub fn take_while1<'alloc, I, E, P, A>(pred: P) -> impl Parser<'alloc, I, A, Output = I, Error = E>
where
    I: Input,
    E: ParseError<'alloc, I, A>,
    P: Fn(I::Item) -> bool,
    A: Allocator,
{
    TakeWhileMN {
        min: 1,
        max: usize::MAX,
        pred,
        kind: ParseErrorKind::None,
        error: PhantomData,
    }
}

pub fn take_while_n<'alloc, I, E, P, A>(
    n: usize,
    pred: P,
) -> impl Parser<'alloc, I, A, Output = I, Error = E>
where
    I: Input,
    E: ParseError<'alloc, I, A>,
    P: Fn(I::Item) -> bool,
    A: Allocator,
{
    TakeWhileMN {
        min: n,
        max: n,
        pred,
        kind: ParseErrorKind::None,
        error: PhantomData,
    }
}

pub fn take_while1_m_n<'alloc, I, E, P, A>(
    min: usize,
    max: usize,
    pred: P,
) -> impl Parser<'alloc, I, A, Output = I, Error = E>
where
    I: Input,
    E: ParseError<'alloc, I, A>,
    P: Fn(I::Item) -> bool,
    A: Allocator,
{
    TakeWhileMN {
        min,
        max,
        pred,
        kind: ParseErrorKind::None,
        error: PhantomData,
    }
}

pub struct TakeWhileMN<P, E> {
    min: usize,
    max: usize,
    pred: P,
    kind: ParseErrorKind,
    error: PhantomData<E>,
}

impl<'alloc, I, E, P, A> Parser<'alloc, I, A> for TakeWhileMN<P, E>
where
    I: Input,
    E: ParseError<'alloc, I, A>,
    P: Fn(I::Item) -> bool,
    A: Allocator,
{
    type Output = I;

    type Error = E;

    fn parse(
        &self,
        input: I,
        alloc: &'alloc A,
    ) -> ParseResult<'alloc, I, Self::Output, Self::Error> {
        input.split_at_position_m_n(
            self.min,
            self.max,
            |item| !(self.pred)(item),
            self.kind,
            alloc,
        )
    }
}
