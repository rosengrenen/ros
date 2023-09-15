use crate::{
    error::{ParseError, ParseErrorKind, ParseResult},
    input::Input,
    parser::Parser,
};
use core::alloc::Allocator;

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
        impl< I, $($outputs),+, E, $($parsers),+, A> Parser<I, A> for ($($parsers),+)
        where
            I: Input,
            E: ParseError< I, A>,
            $($parsers: Parser< I, A, Output = $outputs, Error = E>,)+
            A:Allocator + Clone,
        {
            type Output = ($($outputs),+);

            type Error = E;

            fn parse(&self, input: I, alloc:  A) -> ParseResult< I, Self::Output, Self::Error> {
                #[allow(non_snake_case)]
                let ($($parsers),+) = self;
                $(
                    #[allow(non_snake_case)]
                    let (input, $outputs) = $parsers.parse(input.clone(), alloc.clone())
                        .map_err(|error| error.append(input, ParseErrorKind::Tuple))?;
                )+

                Ok((input, ($($outputs),+)))
            }
        }
    };
}

tuple_trait!(O1 P1, O2 P2, O3 P3, O4 P4, O5 P5, O6 P6, O7 P7, O8 P8, O9 P9, O10 P10, O11 P11, O12 P12, O13 P13, O14 P14, O15 P15, O16 P16);
