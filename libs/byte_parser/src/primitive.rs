use core::{marker::PhantomData, mem::MaybeUninit};

use crate::{
    error::{ParseError, ParseErrorKind, ParseResult, ParserError},
    parser::Parser,
};

use super::input::Input;

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

fn iter_to_array_unchecked<const C: usize, T, I>(mut iter: I) -> [T; C]
where
    I: Iterator<Item = T>,
{
    // SAFETY: all values are written before reading, so any uninitialized values are overwritten
    let mut array = unsafe { MaybeUninit::<[T; C]>::zeroed().assume_init() };
    for item in array.iter_mut() {
        *item = iter.next().unwrap();
    }

    array
}

pub fn take_one<I, E>() -> impl Parser<I, Output = I::Item, Error = E>
where
    I: Input,
    I::Item: Clone,
    E: ParseError<I>,
{
    TakeOne { error: PhantomData }
}

pub struct TakeOne<E> {
    error: PhantomData<E>,
}

impl<I, E> Parser<I> for TakeOne<E>
where
    I: Input,
    I::Item: Clone,
    E: ParseError<I>,
{
    type Output = I::Item;

    type Error = E;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output, Self::Error> {
        take_const::<1, I, E>()
            .map(|output| output[0].clone())
            .parse(input)
    }
}

pub fn take_while<I, E, P>(pred: P) -> impl Parser<I, Output = I, Error = E>
where
    I: Input,
    E: ParseError<I>,
    P: Fn(I::Item) -> bool,
{
    TakeWhile {
        pred,
        error: PhantomData,
    }
}

pub struct TakeWhile<P, E> {
    pred: P,
    error: PhantomData<E>,
}

impl<I, E, P> Parser<I> for TakeWhile<P, E>
where
    I: Input,
    E: ParseError<I>,
    P: Fn(I::Item) -> bool,
{
    type Output = I;

    type Error = E;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output, Self::Error> {
        input.split_at_position0(|item| !(self.pred)(item))
    }
}

pub fn take_while1<I, E, P>(pred: P) -> impl Parser<I, Output = I, Error = E>
where
    I: Input,
    E: ParseError<I>,
    P: Fn(I::Item) -> bool,
{
    TakeWhile1 {
        pred,
        error: PhantomData,
    }
}

pub struct TakeWhile1<P, E> {
    pred: P,
    error: PhantomData<E>,
}

impl<I, E, P> Parser<I> for TakeWhile1<P, E>
where
    I: Input,
    E: ParseError<I>,
    P: Fn(I::Item) -> bool,
{
    type Output = I;

    type Error = E;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output, Self::Error> {
        input.split_at_position1(|item| !(self.pred)(item), ParseErrorKind::None)
    }
}

pub fn item<I, E>(item: I::Item) -> impl Parser<I, Output = I::Item, Error = E>
where
    I: Input,
    I::Item: Clone + PartialEq,
    E: ParseError<I>,
{
    Item {
        item,
        error: PhantomData,
    }
}

pub struct Item<I, E> {
    item: I,
    error: PhantomData<E>,
}

impl<I, E> Parser<I> for Item<I::Item, E>
where
    I: Input,
    I::Item: Clone + PartialEq,
    E: ParseError<I>,
{
    type Output = I::Item;

    type Error = E;

    fn parse(&self, input0: I) -> ParseResult<I, Self::Output, Self::Error> {
        match take_one::<I, E>().parse(input0.clone()) {
            Ok((input1, item)) => {
                if self.item != item {
                    return Err(ParserError::Error(E::from_error_kind(
                        input0,
                        ParseErrorKind::None,
                    )));
                }

                Ok((input1, item))
            }
            Err(ParserError::Error(_)) => Err(ParserError::Error(E::from_error_kind(
                input0,
                ParseErrorKind::None,
            ))),
            Err(ParserError::Failure(error)) => Err(ParserError::Failure(error)),
        }
    }
}

pub fn satisfy<I, E, P>(pred: P) -> impl Parser<I, Output = I::Item, Error = E>
where
    I: Input,
    I::Item: Clone,
    E: ParseError<I>,
    P: Fn(I::Item) -> bool,
{
    Satisfy {
        pred,
        error: PhantomData,
    }
}

pub struct Satisfy<P, E> {
    pred: P,
    error: PhantomData<E>,
}

impl<I, E, P> Parser<I> for Satisfy<P, E>
where
    I: Input,
    I::Item: Clone,
    E: ParseError<I>,
    P: Fn(I::Item) -> bool,
{
    type Output = I::Item;

    type Error = E;

    fn parse(&self, input0: I) -> ParseResult<I, Self::Output, Self::Error> {
        match take_one::<I, E>().parse(input0.clone()) {
            Ok((input1, item)) => {
                if !(self.pred)(item.clone()) {
                    return Err(ParserError::Error(E::from_error_kind(
                        input0,
                        ParseErrorKind::None,
                    )));
                }

                Ok((input1, item))
            }
            Err(ParserError::Error(_)) => Err(ParserError::Error(E::from_error_kind(
                input0,
                ParseErrorKind::None,
            ))),
            Err(ParserError::Failure(error)) => Err(ParserError::Failure(error)),
        }
    }
}
