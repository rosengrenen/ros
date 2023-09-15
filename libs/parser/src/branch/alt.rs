use crate::{
    error::{ParseError, ParseErrorKind, ParseResult, ParserError},
    input::Input,
    parser::Parser,
};
use core::{alloc::Allocator, marker::PhantomData};

pub struct Alt<P, A> {
    pub(crate) parsers: P,
    pub(crate) alloc: PhantomData<A>,
}

impl<I, P, A> Parser<I, A> for Alt<P, A>
where
    P: AltHelper<I, A>,
    A: Allocator,
{
    type Output = P::Output;

    type Error = P::Error;

    fn parse(&self, input: I, alloc: A) -> ParseResult<I, Self::Output, Self::Error> {
        self.parsers.parse_alt(input, alloc)
    }
}

pub trait AltHelper<I, A: Allocator> {
    type Output;

    type Error: ParseError<I, A>;

    fn parse_alt(&self, input: I, alloc: A) -> ParseResult<I, Self::Output, Self::Error>;
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
        impl< I, O, E, $($parsers),+, A> AltHelper<I, A> for ($($parsers),+)
        where
            I: Input,
            E: ParseError< I, A>,
            $(
                $parsers: Parser< I, A, Output = O, Error = E>,
            )+
            A:Allocator + Clone,
        {
            type Output = O;

            type Error = E;

            fn parse_alt(
                &self,
                input: I,
                alloc: A,
            ) -> ParseResult< I, Self::Output, Self::Error> {
                #[allow(non_snake_case)]
                let ($($parsers),+) = self;
                alt_trait_inner!(input.clone(), alloc.clone(), $($parsers)+)
                    .map_err(|error| error.append(input, ParseErrorKind::Alt))
            }
        }
    };
}

macro_rules! alt_trait_inner(
    ($input:expr, $alloc:expr, $head:ident $($parsers:ident)+) => (
        match $head.parse($input.clone(), $alloc) {
            Ok(result) => Ok(result),
            Err(ParserError::Error(_)) => alt_trait_inner!($input, $alloc, $($parsers)+),
            Err(ParserError::Failure(error)) => Err(ParserError::Failure(error)),
        }
    );
    ($input:expr, $alloc:expr, $head:ident) => (
        $head.parse($input, $alloc)
    );
);

alt_trait!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16);
