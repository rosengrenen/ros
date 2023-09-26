use crate::{
    error::{ParseErrorKind, ParseResult},
    input::Input,
    parser::Parser,
};
use alloc::boxed::Box;
use core::alloc::Allocator;

#[derive(Clone)]
pub struct Boxed<P> {
    pub(crate) parser: P,
}

impl<I, C, P, A> Parser<I, C, A> for Boxed<P>
where
    I: Input,
    P: Parser<I, C, A>,
    A: Allocator + Clone,
{
    type Output = Box<P::Output, A>;

    type Error = P::Error;

    fn parse(
        self,
        input: I,
        context: &mut C,
        alloc: A,
    ) -> ParseResult<I, Self::Output, Self::Error> {
        self.parser
            .map_res1(|inner| Box::new(inner, alloc.clone()))
            .parse(input.clone(), context, alloc.clone())
            .map_err(|error| error.append(input, ParseErrorKind::MapBoxed))
    }
}
