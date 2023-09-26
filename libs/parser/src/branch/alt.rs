use crate::{
    error::{ParseError, ParseErrorKind, ParseResult, ParserError},
    input::Input,
    parser::Parser,
};
use core::{alloc::Allocator, marker::PhantomData};

#[derive(Clone)]
pub struct Alt<P, A> {
    pub(crate) parsers: P,
    pub(crate) alloc: PhantomData<A>,
}

impl<I, C, P, A> Parser<I, C, A> for Alt<P, A>
where
    P: AltHelper<I, C, A>,
    A: Allocator + Clone,
{
    type Output = P::Output;

    type Error = P::Error;

    fn parse(
        self,
        input: I,
        context: &mut C,
        alloc: A,
    ) -> ParseResult<I, Self::Output, Self::Error> {
        self.parsers.parse_alt(input, context, alloc)
    }
}

pub trait AltHelper<I, C, A: Allocator>: Clone {
    type Output;

    type Error: ParseError<I, A>;

    fn parse_alt(
        self,
        input: I,
        context: &mut C,
        alloc: A,
    ) -> ParseResult<I, Self::Output, Self::Error>;
}

impl<I, O, E, C, P1, A> AltHelper<I, C, A> for (P1,)
where
    I: Input,
    E: ParseError<I, A>,
    P1: Parser<I, C, A, Output = O, Error = E>,
    A: Allocator + Clone,
{
    type Output = O;

    type Error = E;

    fn parse_alt(
        self,
        input: I,
        context: &mut C,
        alloc: A,
    ) -> ParseResult<I, Self::Output, Self::Error> {
        self.0
            .parse(input.clone(), context, alloc.clone())
            .map_err(|error| error.append(input.clone(), ParseErrorKind::Alt))
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
        impl<I, O, E, C, $($parsers),+, A> AltHelper<I, C, A> for ($($parsers),+)
        where
            I: Input,
            E: ParseError<I, A>,
            $(
                $parsers: Parser<I, C, A, Output = O, Error = E>,
            )+
            A: Allocator + Clone,
        {
            type Output = O;

            type Error = E;

            fn parse_alt(
                self,
                input: I,
                context: &mut C,
                alloc: A,
            ) -> ParseResult<I, Self::Output, Self::Error> {
                #[allow(non_snake_case)]
                let ($($parsers),+) = self.clone();
                $(
                    match $parsers
                        .parse(input.clone(), context, alloc.clone())
                        .map_err(|error| error.append(input.clone(), ParseErrorKind::Alt))
                    {
                        Ok(res) => return Ok(res),
                        Err(ParserError::Error(_)) => (),
                        Err(ParserError::Failure(e)) => return Err(ParserError::Failure(e)),
                    }
                )+

                Err(ParserError::Error(E::from_error_kind(
                    input,
                    ParseErrorKind::Alt,
                    alloc,
                )))
            }
        }
    };
}

alt_trait!(
    P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18, P19, P20, P21,
    P22, P23, P24, P25, P26, P27, P28, P29, P30, P31, P32
);
