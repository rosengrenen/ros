use super::util::iter_to_array_unchecked;
use crate::{
    error::{ParseError, ParseErrorKind, ParseResult},
    input::Input,
    parser::Parser,
};
use core::marker::PhantomData;

pub fn take<I, E>(count: usize) -> impl Parser<I, Output = I, Error = E>
where
    I: Input,
    E: ParseError<I>,
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

impl<I, E> Parser<I> for Take<E>
where
    I: Input,
    E: ParseError<I>,
{
    type Output = I;

    type Error = E;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output, Self::Error> {
        let (output, input) = input.split_at_index(self.count);
        Ok((input, output))
    }
}

pub fn take_const<const C: usize, I, E>() -> impl Parser<I, Output = [I::Item; C], Error = E>
where
    I: Input,
    E: ParseError<I>,
{
    TakeConst { error: PhantomData }
}

pub struct TakeConst<const C: usize, E> {
    error: PhantomData<E>,
}

impl<const C: usize, I, E> Parser<I> for TakeConst<C, E>
where
    I: Input,
    E: ParseError<I>,
{
    type Output = [I::Item; C];

    type Error = E;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output, Self::Error> {
        take(C)
            .map(|output: I| iter_to_array_unchecked(output.item_iter()))
            .parse(input)
    }
}

pub fn take_while<I, E, P>(pred: P) -> impl Parser<I, Output = I, Error = E>
where
    I: Input,
    E: ParseError<I>,
    P: Fn(I::Item) -> bool,
{
    TakeWhileMN {
        min: 0,
        max: usize::MAX,
        pred,
        kind: ParseErrorKind::None,
        error: PhantomData,
    }
}

pub fn take_while1<I, E, P>(pred: P) -> impl Parser<I, Output = I, Error = E>
where
    I: Input,
    E: ParseError<I>,
    P: Fn(I::Item) -> bool,
{
    TakeWhileMN {
        min: 1,
        max: usize::MAX,
        pred,
        kind: ParseErrorKind::None,
        error: PhantomData,
    }
}

pub fn take_while_n<I, E, P>(n: usize, pred: P) -> impl Parser<I, Output = I, Error = E>
where
    I: Input,
    E: ParseError<I>,
    P: Fn(I::Item) -> bool,
{
    TakeWhileMN {
        min: n,
        max: n,
        pred,
        kind: ParseErrorKind::None,
        error: PhantomData,
    }
}

pub fn take_while1_m_n<I, E, P>(
    min: usize,
    max: usize,
    pred: P,
) -> impl Parser<I, Output = I, Error = E>
where
    I: Input,
    E: ParseError<I>,
    P: Fn(I::Item) -> bool,
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

impl<I, E, P> Parser<I> for TakeWhileMN<P, E>
where
    I: Input,
    E: ParseError<I>,
    P: Fn(I::Item) -> bool,
{
    type Output = I;

    type Error = E;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output, Self::Error> {
        input.split_at_position_m_n(self.min, self.max, |item| !(self.pred)(item), self.kind)
    }
}
