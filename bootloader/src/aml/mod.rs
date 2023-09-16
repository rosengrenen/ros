pub mod data;
pub mod misc;
pub mod name;
pub mod ops;
pub mod term;

use core::{alloc::Allocator, marker::PhantomData};
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
    primitive::{fail::fail, item::take_one, take::take},
};

pub fn pkg_length<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    alloc: A,
) -> ParseResult<I, usize, E> {
    let (input, lead_byte) = take_one()
        .add_context("pkg_length")
        .parse(input, alloc.clone())?;
    let extra_bytes = (lead_byte >> 6) as usize;
    if extra_bytes == 0 {
        return Ok((input, lead_byte as usize - 1));
    }

    take(extra_bytes)
        .map_res1(move |extra_bytes: I| {
            if lead_byte & 0b0011_0000 != 0 {
                return Err(());
            }

            let mut pkg_length = (lead_byte & 0xf) as usize;
            for (i, b) in extra_bytes.item_iter().enumerate() {
                pkg_length |= (b as usize) << (i * 8 + 4);
            }

            Ok(pkg_length - 1 - extra_bytes.input_len())
        })
        .add_context("pkg_length")
        .parse(input, alloc)
}

pub fn pkg<I, E, O, P, A>(parser: &P) -> Pkg<'_, P, E>
where
    I: Input<Item = u8>,
    E: ParseError<I, A>,
    P: Parser<I, A, Output = O, Error = E>,
    A: Allocator + Clone,
{
    Pkg {
        parser,
        error: PhantomData,
    }
}

pub struct Pkg<'p, P, E> {
    parser: &'p P,
    error: PhantomData<E>,
}

impl<'p, I, O, P, E: ParseError<I, A>, A: Allocator + Clone> Parser<I, A> for Pkg<'p, P, E>
where
    I: Input<Item = u8>,
    E: ParseError<I, A>,
    P: Parser<I, A, Output = O, Error = E>,
    A: Allocator + Clone,
{
    type Output = O;

    type Error = E;

    fn parse(&self, input: I, alloc: A) -> ParseResult<I, Self::Output, Self::Error> {
        let (input, pkg_len) = pkg_length.add_context("pkg").parse(input, alloc.clone())?;
        // let (rest, input) = take(pkg_len)
        //     .add_context("pkg")
        //     .parse(input, alloc.clone())?;
        // let (input, output) = self.parser.cut().parse(input, alloc.clone())?;
        // fail().add_context("pkg").parse(input, alloc)?;
        // Ok((rest, output))
        take(pkg_len)
            .and_then(&(self.parser, &fail()))
            .map(|(output, _)| output)
            .add_context("pkg")
            .parse(input, alloc)
    }
}
