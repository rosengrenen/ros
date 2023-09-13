use crate::{
    error::{ParseError, ParseResult},
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
        impl<'alloc, I, $($outputs),+, E, $($parsers),+, A> Parser<'alloc, I, A> for ($($parsers),+)
        where
            E: ParseError<'alloc, I, A>,
            $($parsers: Parser<'alloc, I, A, Output = $outputs, Error = E>,)+
            A: Allocator,
        {
            type Output = ($($outputs),+);

            type Error = E;

            fn parse(&self, input: I, alloc: &'alloc A) -> ParseResult<'alloc, I, Self::Output, Self::Error> {
                #[allow(non_snake_case)]
                let ($($parsers),+) = self;
                $(
                    #[allow(non_snake_case)]
                    let (input, $outputs) = $parsers.parse(input, alloc)?;
                )+

                Ok((input, ($($outputs),+)))
            }
        }
    };
}

tuple_trait!(O1 P1, O2 P2, O3 P3, O4 P4, O5 P5, O6 P6, O7 P7, O8 P8);
