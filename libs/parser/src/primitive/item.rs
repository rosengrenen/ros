use super::take::take_const;
use crate::{
    error::{ParseError, ParseErrorKind, ParseResult},
    input::Input,
    parser::Parser,
};
use core::{alloc::Allocator, marker::PhantomData};

pub fn take_one<I, E, A>() -> impl Parser<I, A, Output = I::Item, Error = E>
where
    I: Input,
    I::Item: Clone,
    E: ParseError<I, A>,
    A: Allocator + Clone,
{
    Satisfy {
        pred: |_: &I::Item| true,
        kind: ParseErrorKind::TakeOne,
        error: PhantomData,
    }
}

pub fn item<I, E, A>(item: I::Item) -> impl Parser<I, A, Output = I::Item, Error = E>
where
    I: Input,
    I::Item: Clone + PartialEq,
    E: ParseError<I, A>,
    A: Allocator + Clone,
{
    Satisfy {
        pred: move |i: &I::Item| i == &item,
        kind: ParseErrorKind::Item,
        error: PhantomData,
    }
}

pub fn satisfy<I, E, P, A>(pred: P) -> impl Parser<I, A, Output = I::Item, Error = E>
where
    I: Input,
    I::Item: Clone,
    E: ParseError<I, A>,
    P: Fn(&I::Item) -> bool,
    A: Allocator + Clone,
{
    Satisfy {
        pred,
        kind: ParseErrorKind::Satisfy,
        error: PhantomData,
    }
}

pub struct Satisfy<P, E> {
    pred: P,
    kind: ParseErrorKind,
    error: PhantomData<E>,
}

impl<I, E, P, A> Parser<I, A> for Satisfy<P, E>
where
    I: Input,
    I::Item: Clone,
    E: ParseError<I, A>,
    P: Fn(&I::Item) -> bool,
    A: Allocator + Clone,
{
    type Output = I::Item;

    type Error = E;

    fn parse(&self, input: I, alloc: A) -> ParseResult<I, Self::Output, Self::Error> {
        take_const::<1, I, E, A>()
            .map(|output| output[0].clone())
            .map_res1(|item| match (self.pred)(&item) {
                true => Ok(item),
                false => Err(()),
            })
            .parse(input.clone(), alloc)
            .map_err(|error| error.append(input, self.kind))
    }
}
