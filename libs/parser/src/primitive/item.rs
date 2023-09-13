use super::take::take_const;
use crate::{
    error::{ParseError, ParseErrorKind, ParseResult},
    input::Input,
    parser::Parser,
};
use core::marker::PhantomData;

pub fn take_one<I, E>() -> impl Parser<I, Output = I::Item, Error = E>
where
    I: Input,
    I::Item: Clone,
    E: ParseError<I>,
{
    Satisfy {
        pred: |_: &I::Item| true,
        error: PhantomData,
    }
}

pub fn item<I, E>(item: I::Item) -> impl Parser<I, Output = I::Item, Error = E>
where
    I: Input,
    I::Item: Clone + PartialEq,
    E: ParseError<I>,
{
    Satisfy {
        pred: move |i: &I::Item| i == &item,
        error: PhantomData,
    }
}

pub fn satisfy<I, E, P>(pred: P) -> impl Parser<I, Output = I::Item, Error = E>
where
    I: Input,
    I::Item: Clone,
    E: ParseError<I>,
    P: Fn(&I::Item) -> bool,
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
    P: Fn(&I::Item) -> bool,
{
    type Output = I::Item;

    type Error = E;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output, Self::Error> {
        let input_err = input.clone();
        take_const::<1, I, E>()
            .map(|output| output[0].clone())
            .map_res1(|item| match (self.pred)(&item) {
                true => Ok(item),
                false => Err(()),
            })
            .map_err(|_| E::from_error_kind(input_err.clone(), ParseErrorKind::None))
            .parse(input)
    }
}
