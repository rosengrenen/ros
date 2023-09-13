use super::take::take_const;
use crate::{
    error::{ParseError, ParseErrorKind, ParseResult},
    input::Input,
    parser::Parser,
};
use core::{alloc::Allocator, marker::PhantomData};

pub fn take_one<'alloc, I, E, A>() -> impl Parser<'alloc, I, A, Output = I::Item, Error = E>
where
    I: Input,
    I::Item: Clone,
    E: ParseError<'alloc, I, A>,
    A: Allocator,
{
    Satisfy {
        pred: |_: &I::Item| true,
        error: PhantomData,
    }
}

pub fn item<'alloc, I, E, A>(
    item: I::Item,
) -> impl Parser<'alloc, I, A, Output = I::Item, Error = E>
where
    I: Input,
    I::Item: Clone + PartialEq,
    E: ParseError<'alloc, I, A>,
    A: Allocator,
{
    Satisfy {
        pred: move |i: &I::Item| i == &item,
        error: PhantomData,
    }
}

pub fn satisfy<'alloc, I, E, P, A>(
    pred: P,
) -> impl Parser<'alloc, I, A, Output = I::Item, Error = E>
where
    I: Input,
    I::Item: Clone,
    E: ParseError<'alloc, I, A>,
    P: Fn(&I::Item) -> bool,
    A: Allocator,
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

impl<'alloc, I, E, P, A> Parser<'alloc, I, A> for Satisfy<P, E>
where
    I: Input,
    I::Item: Clone,
    E: ParseError<'alloc, I, A>,
    P: Fn(&I::Item) -> bool,
    A: Allocator,
{
    type Output = I::Item;

    type Error = E;

    fn parse(
        &self,
        input: I,
        alloc: &'alloc A,
    ) -> ParseResult<'alloc, I, Self::Output, Self::Error> {
        let input_err = input.clone();
        take_const::<1, I, E, A>()
            .map(|output| output[0].clone())
            .map_res1(|item| match (self.pred)(&item) {
                true => Ok(item),
                false => Err(()),
            })
            .map_err(|_| E::from_error_kind(input_err.clone(), ParseErrorKind::None, alloc))
            .parse(input, alloc)
    }
}
