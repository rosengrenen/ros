use core::marker::PhantomData;

use crate::error::{FromExternalError, ParseError, ParseErrorKind, ParseResult, ParserError};

pub trait Parser<I> {
    type Output;

    type Error: ParseError<I>;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output, Self::Error>;

    fn map<F, O2>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> O2,
    {
        Map { parser: self, f }
    }

    fn map_res<O2, E2, F>(self, f: F) -> MapRes<Self, F, E2>
    where
        Self: Sized,
        Self::Error: FromExternalError<I, E2>,
        F: Fn(Self::Output) -> Result<O2, E2>,
    {
        MapRes {
            parser: self,
            f,
            error: PhantomData,
        }
    }

    // map opt

    // then

    // fn or<P>(self, parser: P) -> Or<Self, P>
    // where
    //     Self: Sized,
    //     P: Parser<I, Output = Self::Output, Error = Self::Error>,
    // {
    //     Or {
    //         first_parser: self,
    //         second_parser: parser,
    //     }
    // }

    fn opt(self) -> Opt<Self>
    where
        Self: Sized,
    {
        Opt { parser: self }
    }

    fn cut(self) -> Cut<Self>
    where
        Self: Sized,
    {
        Cut { parser: self }
    }
}

pub struct Map<P, F> {
    parser: P,
    f: F,
}

impl<I, O2, P, F> Parser<I> for Map<P, F>
where
    P: Parser<I>,
    F: Fn(P::Output) -> O2,
{
    type Output = O2;

    type Error = P::Error;

    fn parse(&self, input: I) -> ParseResult<I, O2, Self::Error> {
        self.parser
            .parse(input)
            .map(|(input, output)| (input, (self.f)(output)))
    }
}

pub struct MapRes<P, F, E> {
    parser: P,
    f: F,
    error: PhantomData<E>,
}

impl<I, O1, O2, E1, E2, P, F> Parser<I> for MapRes<P, F, E2>
where
    I: Clone,
    E1: ParseError<I> + FromExternalError<I, E2>,
    P: Parser<I, Output = O1, Error = E1>,
    F: Fn(O1) -> Result<O2, E2>,
{
    type Output = O2;

    type Error = P::Error;

    fn parse(&self, input: I) -> ParseResult<I, O2, Self::Error> {
        self.parser.parse(input).and_then(|(input, output)| {
            let output = {
                let input = input.clone();
                (self.f)(output).map_err(move |error| {
                    ParserError::Error(Self::Error::from_external_error(
                        input,
                        ParseErrorKind::None,
                        error,
                    ))
                })?
            };
            Ok((input, output))
        })
    }
}

// pub struct Or<P1, P2> {
//     first_parser: P1,
//     second_parser: P2,
// }

// impl<I, O, P1, P2, E> Parser<I> for Or<P1, P2>
// where
//     I: Clone,
//     P1: Parser<I, Output = O, Error = E>,
//     P2: Parser<I, Output = O, Error = E>,
//     E: ParseError<I>,
// {
//     type Output = O;

//     type Error = E;

//     fn parse(&self, input: I) -> ParseResult<I, Self::Output, Self::Error> {
//         match self.first_parser.parse(input) {
//             Ok(result) => Ok(result),
//             Err(ParserError::Error(error)) => match self.second_parser.parse(input) {
//                 Ok(result) => Ok(result),
//                 Err(ParserError::Error(error)) => Err(ParserError::Error(
//                     error.append(E::from_error_kind(input, ParseErrorKind::None)),
//                 )),
//                 Err(ParserError::Failure(error)) => Err(ParserError::Failure(error)),
//             },
//             Err(ParserError::Failure(error)) => Err(ParserError::Failure(error)),
//         }
//     }
// }

pub struct Opt<P> {
    parser: P,
}

impl<I, O, E, P> Parser<I> for Opt<P>
where
    I: Clone,
    E: ParseError<I>,
    P: Parser<I, Output = O, Error = E>,
{
    type Output = Option<P::Output>;

    type Error = P::Error;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output, Self::Error> {
        match self.parser.parse(input.clone()) {
            Ok((input, output)) => Ok((input, Some(output))),
            Err(ParserError::Error(_)) => Ok((input, None)),
            Err(ParserError::Failure(error)) => Err(ParserError::Failure(error)),
        }
    }
}

pub struct Cut<P> {
    parser: P,
}

impl<I, P> Parser<I> for Cut<P>
where
    P: Parser<I>,
{
    type Output = P::Output;

    type Error = P::Error;

    fn parse(&self, input: I) -> ParseResult<I, Self::Output, Self::Error> {
        match self.parser.parse(input) {
            Ok(result) => Ok(result),
            Err(ParserError::Error(error)) => Err(ParserError::Failure(error)),
            Err(ParserError::Failure(error)) => Err(ParserError::Failure(error)),
        }
    }
}

impl<I, P1, O1, E> Parser<I> for (P1,)
where
    P1: Parser<I, Output = O1, Error = E>,
    E: ParseError<I>,
{
    type Output = (O1,);

    type Error = E;

    fn parse(&self, input0: I) -> ParseResult<I, Self::Output, Self::Error> {
        let (input1, output1) = self.0.parse(input0)?;
        Ok((input1, (output1,)))
    }
}

macro_rules! tuple_trait(
    ($output1:ident $parser1:ident, $output2:ident $parser2:ident, $($output:ident $parser:ident),*) => (
        tuple_trait!(__impl $output1 $parser1, $output2 $parser2; $($output $parser),*);
    );
    (__impl $($output:ident $parser:ident),+; $output1:ident $parser1:ident, $($output2:ident $parser2:ident),*) => (
        tuple_trait_impl!($($output $parser),+);
        tuple_trait!(__impl $($output $parser),+ , $output1 $parser1; $($output2 $parser2),*);
    );
    (__impl $($output:ident $parser:ident),+; $output1:ident $parser1:ident) => (
        tuple_trait_impl!($($output $parser),+);
        tuple_trait_impl!($($output $parser),+, $output1 $parser1);
    );
);

macro_rules! tuple_trait_impl {
    ($($outputs:ident $parsers:ident),+) => {
        impl<I, $($outputs),+, E, $($parsers),+> Parser<I> for ($($parsers),+)
        where
            E: ParseError<I>,
            $($parsers: Parser<I, Output = $outputs, Error = E>,)+
        {
            type Output = ($($outputs),+);

            type Error = E;

            fn parse(&self, input: I) -> ParseResult<I, ($($outputs),+,), E> {
                #[allow(non_snake_case)]
                let ($($parsers),+) = self;
                $(
                    #[allow(non_snake_case)]
                    let (input, $outputs) = $parsers.parse(input)?;
                )+

                Ok((input, ($($outputs),+)))
            }
        }
    };
}

tuple_trait!(O1 P1, O2 P2, O3 P3, O4 P4, O5 P5, O6 P6, O7 P7, O8 P8);
