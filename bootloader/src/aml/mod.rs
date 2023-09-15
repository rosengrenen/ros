pub mod data;
pub mod misc;
pub mod name;
pub mod term;

use self::name::NameString;
use crate::sprintln;
use core::{alloc::Allocator, marker::PhantomData};
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    multi::many::many,
    parser::Parser,
    primitive::{fail::fail, item::take_one, take::take},
};

pub fn definition_blocks<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    alloc: A,
) -> ParseResult<I, (), E> {
    many(def_scope)
        .map(|output| {
            sprintln!("got {}", output.len());
        })
        .parse(input, alloc)
}

pub fn def_scope<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    alloc: A,
) -> ParseResult<I, NameString<A>, E> {
    // let (input, pkg_len) = preceded(item(0x10), pkg.cut()).parse(input, alloc)?;
    // let (rest, input) = take(pkg_len).parse(input, alloc)?;
    // let (_, output) = name_string.cut().parse(input, alloc)?;
    // Ok((rest, output))
    // let (input, pkg_len) = preceded(item(0x10), pkg.cut()).parse(input, alloc)?;
    // let (rest, input) = take(pkg_len).parse(input, alloc)?;
    let (rest, output) = NameString::p.cut().parse(input, alloc)?;
    Ok((rest, output))
}

pub fn pkg_length<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    alloc: A,
) -> ParseResult<I, usize, E> {
    let (input, lead_byte) = take_one().parse(input, alloc.clone())?;
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
        .parse(input, alloc)
}

pub fn pkg<I, E, O, P, A>(parser: P) -> impl Parser<I, A, Output = O, Error = E>
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

pub struct Pkg<P, E> {
    parser: P,
    error: PhantomData<E>,
}

impl<I, O, P, E: ParseError<I, A>, A: Allocator + Clone> Parser<I, A> for Pkg<P, E>
where
    I: Input<Item = u8>,
    E: ParseError<I, A>,
    P: Parser<I, A, Output = O, Error = E>,
    A: Allocator + Clone,
{
    type Output = O;

    type Error = E;

    fn parse(&self, input: I, alloc: A) -> ParseResult<I, Self::Output, Self::Error> {
        let (input, pkg_len) = pkg_length(input, alloc.clone())?;
        let (input, rest) = take(pkg_len).parse(input, alloc.clone())?;
        let (input, output) = self.parser.parse(input, alloc.clone())?;
        fail().parse(input, alloc)?;
        Ok((rest, output))
        // take(pkg_len)
        //     .and_then((self.parser, fail()))
        //     .map(|(output, _)| output)
        //     .parse(input, alloc)
    }
}
