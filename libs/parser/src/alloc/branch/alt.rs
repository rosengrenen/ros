use crate::{
    alloc::parser::ParserAlloc,
    error::{ParseError, ParseResult, ParserError},
};
use core::{alloc::Allocator, marker::PhantomData};

pub struct Alt<P, A> {
    pub(crate) parsers: P,
    pub(crate) alloc: PhantomData<A>,
}

impl<'alloc, I, P, A> ParserAlloc<'alloc, I, A> for Alt<P, A>
where
    P: AltHelperAlloc<'alloc, I, A>,
    A: Allocator,
{
    type Output = P::Output;

    type Error = P::Error;

    fn parse_alloc(&self, input: I, alloc: &'alloc A) -> ParseResult<I, Self::Output, Self::Error> {
        self.parsers.parse_alt_alloc(input, alloc)
    }
}

pub trait AltHelperAlloc<'alloc, I, A> {
    type Output;

    type Error: ParseError<I>;

    fn parse_alt_alloc(
        &self,
        input: I,
        alloc: &'alloc A,
    ) -> ParseResult<I, Self::Output, Self::Error>;
}

macro_rules! alt_trait(
    ($parser1:ident, $parser2:ident, $($parser:ident),*) => (
        alt_trait!(__impl $parser1, $parser2; $($parser),*);
    );
    (__impl $($parser: ident),+; $parser1:ident, $($parser2:ident),*) => (
        alt_trait_impl!($($parser),+);
        alt_trait!(__impl $($parser),+, $parser1; $($parser2),*);
    );
    (__impl $($parser: ident),+; $parser1:ident) => (
        alt_trait_impl!($($parser),+);
        alt_trait_impl!($($parser),+, $parser1);
    );
);

macro_rules! alt_trait_impl {
    ($($parsers:ident),+) => {
        impl<'alloc, I, O, E, $($parsers),+, A> AltHelperAlloc<'alloc, I, A> for ($($parsers),+)
        where
            I: Clone,
            E: ParseError<I>,
            $(
                $parsers: ParserAlloc<'alloc, I, A, Output = O, Error = E>,
            )+
            A: Allocator,
        {
            type Output = O;

            type Error = E;

            fn parse_alt_alloc(
                &self,
                input: I,
                alloc: &'alloc A,
            ) -> ParseResult<I, Self::Output, Self::Error> {
                #[allow(non_snake_case)]
                let ($($parsers),+) = self;
                alt_trait_inner!(input, alloc, $($parsers)+)
            }
        }
    };
}

// TODO: use .or() as soon as parsers are references
macro_rules! alt_trait_inner(
    ($input:expr, $alloc:expr, $head:ident $($parsers:ident)+) => (
        match $head.parse_alloc($input.clone(), $alloc) {
            Ok(result) => Ok(result),
            Err(ParserError::Error(_)) => alt_trait_inner!($input, $alloc, $($parsers)+),
            Err(ParserError::Failure(error)) => Err(ParserError::Failure(error)),
        }
    );
    ($input:expr, $alloc:expr, $head:ident) => (
        $head.parse_alloc($input, $alloc)
    );
);

alt_trait!(P1, P2, P3, P4, P5, P6, P7, P8);
