use crate::{
    error::{ParseError, ParseResult, ParserError},
    parser::Parser,
};

pub struct Alt<P> {
    pub(crate) parsers: P,
}

impl<I, P> Parser<I> for Alt<P>
where
    P: AltHelper<I>,
{
    type Output = P::Output;

    type Error = P::Error;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output, Self::Error> {
        self.parsers.parse_alt(input)
    }
}

pub trait AltHelper<I> {
    type Output;

    type Error: ParseError<I>;

    fn parse_alt(&self, input: I) -> ParseResult<I, Self::Output, Self::Error>;
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
        impl<I, O, E, $($parsers),+> AltHelper<I> for ($($parsers),+)
        where
            I: Clone,
            E: ParseError<I>,
            $(
                $parsers: Parser<I,Output = O, Error = E>,
            )+
        {
            type Output = O;

            type Error = E;

            fn parse_alt(
                &self,
                input: I,
            ) -> ParseResult<I, Self::Output, Self::Error> {
                #[allow(non_snake_case)]
                let ($($parsers),+) = self;
                alt_trait_inner!(input, $($parsers)+)
            }
        }
    };
}

// TODO: use .or() as soon as parsers are references
macro_rules! alt_trait_inner(
    ($input:expr, $head:ident $($parsers:ident)+) => (
        match $head.parse($input.clone()) {
            Ok(result) => Ok(result),
            Err(ParserError::Error(_)) => alt_trait_inner!($input, $($parsers)+),
            Err(ParserError::Failure(error)) => Err(ParserError::Failure(error)),
        }
    );
    ($input:expr, $head:ident) => (
        $head.parse($input)
    );
);

alt_trait!(P1, P2, P3, P4, P5, P6, P7, P8);
