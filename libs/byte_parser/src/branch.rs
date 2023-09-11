use crate::{
    error::{ParseError, ParseErrorKind, ParseResult, ParserError},
    parser::Parser,
};

pub fn alt<I, O, E, P>(parsers: P) -> impl Parser<I, Output = O, Error = E>
where
    E: ParseError<I>,
    Alt<P>: Parser<I, Output = O, Error = E>,
{
    Alt { parsers }
}

pub struct Alt<P> {
    parsers: P,
}

impl<I, O, E, P1> Parser<I> for Alt<(P1,)>
where
    I: Clone,
    E: ParseError<I>,
    P1: Parser<I, Output = O, Error = E>,
{
    type Output = O;

    type Error = E;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output, Self::Error> {
        match self.parsers.0.parse(input.clone()) {
            Ok(result) => Ok(result),
            Err(ParserError::Error(_)) => Err(ParserError::Error(E::from_error_kind(
                input,
                ParseErrorKind::None,
            ))),
            Err(ParserError::Failure(error)) => Err(ParserError::Failure(error)),
        }
    }
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
        impl<I, O, E, $($parsers),+> Parser<I> for Alt<($($parsers),+)>
        where
            I: Clone,
            E: ParseError<I>,
            $(
                $parsers: Parser<I, Output = O, Error = E>,
            )+
        {
            type Output = O;

            type Error = E;

            fn parse(&self, input: I) -> ParseResult<I, Self::Output, Self::Error> {
                alt_trait_inner!(0, self, input, $($parsers)+)
            }
        }
    };
}

macro_rules! alt_trait_inner(
    ($iter:tt, $self:expr, $input:expr, $head:ident $($parsers:ident)+) => ({
        match $self.parsers.$iter.parse($input.clone()) {
            Ok(result) => return Ok(result),
            Err(ParserError::Error(_)) => succ!($iter, alt_trait_inner!($self, $input, $($parsers)+)),
            Err(ParserError::Failure(error)) => Err(ParserError::Failure(error)),
        }
    });
    ($iter:tt, $self:expr, $input:expr, $head:ident) => ({
        match $self.parsers.$iter.parse($input.clone()) {
            Ok(result) => Ok(result),
            Err(ParserError::Error(_)) => Err(ParserError::Error(E::from_error_kind(
                $input,
                ParseErrorKind::None,
            ))),
            Err(ParserError::Failure(error)) => Err(ParserError::Failure(error)),
        }
    });
);

alt_trait!(P1, P2, P3, P4, P5, P6, P7, P8);

impl<I, O, E, F> Parser<I> for F
where
    E: ParseError<I>,
    F: Fn(I) -> ParseResult<I, O, E>,
{
    type Output = O;

    type Error = E;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output, Self::Error> {
        self(input)
    }
}
